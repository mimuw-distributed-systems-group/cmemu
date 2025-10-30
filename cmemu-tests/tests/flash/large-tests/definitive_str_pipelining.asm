---
name: Test if mocked addressed have correct waitstates
description:
dumped_symbols:
  counter: auto
configurations: []
product:
    - code: [flash]
      lbEn: [True, False]
      cnt: [CYCCNT, CPICNT, LSUCNT]
      pad1: ['nop.w','',  'nop.n',]
      pad2: ['add.w r1, r1', '',  'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
      pre_pads: [ ['add.n r1, r1; add.n r1, r1', 'add.n r1, r1', ], ['add.w r1, r1', '',],]
      wback: ['[r4]', '[r4], 4', '[r4, 4]!']
      addr_set: ['ls', 'ls2', 'ls3', 'ss']
...

{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}
{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set sysbus_addresses = {
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}
{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}
{% set addr_sets = {
    'ls': [
        (sysbus_addresses['GPIO::DIN31_0'], 'sram_cell'),
        (sysbus_addresses['GPIO::DIN31_0'], 'dep'),
        ('flash_sram', 'dep'),
        ('flash_sram', 'sram_cell'),
    ],
    'ls2': [
        ('flash_gpram', 'dep'),
        ('flash_gpram', 'gpram_cell'),
        ('gpram_gpram', 'dep'),
        ('gpram_gpram', 'gpram_cell'),
    ],
    'ls3': [
        ('sram_sram', 'dep'),
        ('sram_sram', 'sram_cell'),
        ('gpram_sram', 'dep'),
        ('gpram_sram', 'sram_cell'),
    ],
    'ss': [
        ('sram_cell', 'sram_cell_4'),
        ('gpram_cell', 'gpram_cell_4'),
        ('sram_cell', 'gpram_cell'),
        ('gpram_cell', 'sram_cell'),
    ],
}
%}

{% set addrs = addr_sets[addr_set] %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
@ not ldr_dep for evflags, since is causes flash-flash conflict
{% for addr, ldr_dep in addrs %}
{% for dep_type in (('reg', 'mov.n', 'mov.w') if ldr_dep == 'dep' else ('',)) %}
{% for x_cycles in range(5) %}
    {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r11") %}
    {% set load_label = uniq_label("init") %}

{% for lw in itertools.product('nw', repeat=4) %}
{% for pre_pad in pre_pads %} # for 8-bytes alignment

  bl.w {{load_label}}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  @ TODO: configuration with n_x_cycles of the same cycle count
  {{ 'ldr' if addr_set[0] == 'l' else 'str'}}.{{lw.1 if wback == "[r4]" else "w"}} {{ r5 if dep_type == 'reg' else r7 }}, {{wback}}
  {% if dep_type.startswith("mov") %} {{dep_type}} r5, r7 {% endif %}
  str.{{lw.2}} r7, [r5]

  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}
  ldr.{{lw.3}}  r3, [r0, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
{% endfor %}


{% set skipl = uniq_label() %}
b.n {{skipl}}
{{load_label}}:
  {{ mov_const_2w(r4, addr if wback[-1] != "!" else (addr ~ "-4")) }}
  {{ mov_const_2w(r5, "0xFFFFFFFF" if dep_type else ldr_dep) }}
  {{ x_loader }}
bx.n lr
.ltorg
{{skipl}}:

{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label


save:
  mrs.w r1, apsr
  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r3, r4)}}
  bx.n lr

{{ section('flash') }}
.align 3
flash_aligned: .word flash_4
flash_4: .word flash_8
flash_8: flash_gpram: .word gpram_cell
flash_sram: .word sram_cell
flash_gpio: .word {{ sysbus_addresses["GPIO::DIN31_0"] }}

{{ section('gpram') }}
.align 3
gpram_cell: .word gpram_cell_4
gpram_cell_4: gpram_flash: .word flash_aligned
gpram_sram: .word sram_cell
gpram_gpram: .word gpram_cell

{{ section('sram') }}
.align 3
sram_cell: .word sram_cell_4
sram_cell_4: sram_flash: .word flash_aligned
sram_gpram: .word gpram_cell
sram_sram: .word sram_cell

{% endblock %}
