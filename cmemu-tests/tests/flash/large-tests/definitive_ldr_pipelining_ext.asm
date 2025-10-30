---
name: Test if mocked addressed have correct waitstates
description:
dumped_symbols:
  counter: auto
  rough_lsucnt: auto
configurations: []
product:
    - code: [flash]
#      cnt: [CYCCNT, CPICNT, LSUCNT]
      cnt: [CYCCNT]
      wback: ['[r4]', '[r4, 4]!']
      pre_pads: [
          ['add.w r1, r1', '',],
          ['add.n r1, r1', 'add.n r1, r1; add.n r1, r1',]
      ]
      pad1: ['nop.w', '', 'it.n ne; bne.n .+2', 'nop.n', ]
      pad2: ['add.w r1, r1', '',  'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
      lbEn: [True, False]
#      wback: ['[r4]', '[r4], 4', '[r4, 4]!']
      addr_set: [0, 1, 2, 3, 4, 5, 6, 7, 8]
      first_off: [0, 2, 1]
      second_off: [0, 2, 1]
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
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}

{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}
{% set addr_sets = [
    [
        ('DIN31_0', 'sram_cell'),
        ('DOUT3_0', 'sram_cell'),
        ('DIN31_0', 'dep'),
        ('sram_gpio', 'dep'),
    ],
    [
        ('flash_aligned', 'dep'),
        ('flash_aligned', 'flash_4'),
        ('flash_4', 'dep'),
        ('flash_4', 'flash_8'),
    ],
    [
        ('gpram_cell', 'dep'),
        ('gpram_cell', 'gpram_cell_4'),
        ('gpram_cell_4', 'gpram_cell'),
        ('gpram_cell', 'gpram_cell'),
    ],
    [
        ('sram_cell', 'dep'),
        ('sram_cell', 'sram_cell_4'),
        ('sram_cell_4', 'sram_cell'),
        ('sram_cell', 'sram_cell'),
    ],
    [
        ('flash_gpram', 'dep'),
        ('flash_gpram', 'gpram_cell'),
        ('flash_sram', 'dep'),
        ('flash_sram', 'sram_cell'),
    ],
    [
        ('gpram_flash', 'dep'),
        ('gpram_flash', 'flash_aligned'),
        ('gpram_sram', 'dep'),
        ('gpram_sram', 'sram_cell'),
    ],
    [
        ('sram_flash', 'dep'),
        ('sram_flash', 'flash_aligned'),
        ('sram_gpram', 'dep'),
        ('sram_gpram', 'gpram_cell'),
    ],
    [
        ('DOUT3_0', 'flash_aligned'),
        ('flash_gpio', 'dep'),
        ('gpram_gpio', 'dep'),
        ('DOUT3_0', 'gpram_cell'),
    ],
    [
        ('DOUT3_0', 'dep'),
        ('DOUT3_0', 'DOUT3_0'),
        ('DIN31_0', 'flash_aligned'),
        ('EVFLAGS', 'dep'),
    ],
]
%}

{% set addrs = addr_sets[addr_set] %}

@ reg assignment
@ r0, r2 - counter base
@ r1 - pad addrs reg
@ r3 - counter dest, temps in save
@ r4 - first ldr base reg
@ r5, r7 - second addr regs
@ r6 - free
@ r8, r9, r10 - x_cyc_stallers
@ r11 - lsu_cnt
@ r12

{% block after %}

.equ DOUT3_0, 0x40022000
.equ DIN31_0, 0x400220C0
.equ EVFLAGS, 0x400220E0

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for stall_i in [0, 2, 6, 7] %}
{% set (stall_loader, stall_exec) = n_x_cycles(stall_i, "r8", "r9") %}
{% for addr, ldr_dep in addrs %}
{% for dep_type in (('reg', 'adds.n', 'adds.w') if ldr_dep == 'dep' else ('',)) %}
{% for x_cycles in range(3) %}
    {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r9", load_2=False) %}
    {% set load_label = uniq_label("init") %}

{% for lw in itertools.product('nw', repeat=3) %}
{% for pre_pad in pre_pads %} # for 8-bytes alignment

  bl.w {{load_label}}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  {{ stall_exec }}

  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  @ TODO: configuration with n_x_cycles of the same cycle count
  @ first_off is embedded in the address
  ldr.{{lw.1 if wback == "[r4]" else "w"}} {{ r5 if dep_type == 'reg' else r7 }}, {{wback}}
  {% if dep_type.startswith("add") %} {{dep_type}} r5, r7, {{ second_off }} {% endif %}
  ldr.{{lw.2}} r7, [r5]

  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}
  ldr.n  r3, [r0, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
{% endfor %}


{% set skipl = uniq_label() %}
b.n {{skipl}}
.align 3
{{load_label}}:
  {{ mov_const_2w(r4, addr if wback[-1] != "!" else (addr ~ "-4")) }}
  {{ mov_const_2w(r5, "0xFFFFFFFF" if dep_type else ldr_dep) }}
  {{ stall_loader }}
  {{ x_loader }}
  mov.w r11, 0
  msr.w apsr_nzcvq, r11
  ldr.w r11, [r11] @ clean flash buffer
  ldr.w r11, [r0, {{LSUCNT}}]
  nop.w
bx.n lr
.ltorg
{{skipl}}:

{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label


save:
  ldr.w r4, [r0, {{LSUCNT}}]
  sub.w r11, r4, r11
  ands.w r11, r11, 0xFF  @ CPICNT is 8-bit wide

  mrs.w r1, apsr
  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r3, r4)}}
@  {{saveValue("results", r7, r3, r4)}}
  {{saveValue("rough_lsucnt", r11, r3, r4)}}
  bx.n lr

{{ section('flash') }}
.align 3
.space {{first_off}}, 0
flash_aligned: .word flash_4
flash_4: .word flash_8
flash_8: flash_gpram: .word gpram_cell
flash_sram: .word sram_cell
flash_gpio: .word {{ sysbus_addresses["GPIO::DIN31_0"] }}

{{ section('gpram') }}
.align 3
.space {{first_off}}, 0
gpram_cell: .word gpram_cell_4
gpram_cell_4: gpram_flash: .word flash_aligned
gpram_sram: .word sram_cell
gpram_gpio: .word {{ sysbus_addresses["GPIO::DIN31_0"] }}

{{ section('sram') }}
.align 3
.space {{first_off}}, 0
sram_cell: .word sram_cell_4
sram_cell_4: sram_flash: .word flash_aligned
sram_gpram: .word gpram_cell
sram_gpio: .word {{ sysbus_addresses["GPIO::DIN31_0"] }}

{% endblock %}
