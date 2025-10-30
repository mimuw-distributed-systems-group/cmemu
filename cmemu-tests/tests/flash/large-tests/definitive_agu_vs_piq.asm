---
name: If LSU instruction is stalled in Decode on AGU, when PIQ is freed
description:
dumped_symbols:
  counter: auto
configurations: []
product:
    - code: [flash]
      lbEn: [True, False]
      cnt: [CYCCNT, CPICNT, LSUCNT]
      pad1: ['', 'nop.w', 'nop.n',]
#      pad2: ['', 'add.w r1, r1', 'add.n r1, r1']
      pad2: ['', 'add.w r1, r1', 'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
      wback: ['[r5]', '[r5], 4', '[r5, 4]!']
      pre_pad: ['', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1; add.n r1, r1']
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

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
@ not ldr_dep for evflags, since is causes flash-flash conflict
@{% for reg_name, ldr_dep in [("GPIO::DIN31_0", False), ("SRAM", True), ("GPIO::EVFLAGS", False)] %}
@{% set addr = sysbus_addresses[reg_name] %}
{% set addr = "0x20000080" %}
{% for lw in itertools.product('nw', repeat=4) %}
{% for stall_cycles in range(10) %}
@{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1\n add.n r1, r1') %} # for 8-bytes alignment
{% for x_cycles in range(5) %}
  {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r11") %}
  {% set stall_loader, stall_word_exec = n_x_cycles(stall_cycles, "r8", "r9") %}

  {{ mov_const_2w(r4, addr if wback[-1] != "!" else (addr ~ "-4")) }}
@  {{ mov_const_2w(r5, "0x20000000") }}
  {{ x_loader }}
  {{ stall_loader }}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  @ TODO: try to use LSU op here from System
  {{ stall_word_exec }}
  mov.{{lw.1}} r5, r4
  @ This should hold AGU, question: when things are freed?
  ldr.{{lw.2 if wback == "[r5]" else "w"}} r7, {{wback}}

   @ Slider
  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}
  ldr.{{lw.3}}  r3, [r0, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
@{% endfor %}
{% endfor %}
{% set skipl = uniq_label() %}
b.w {{skipl}}
.ltorg
{{skipl}}:
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

{% endblock %}
