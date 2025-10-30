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
      pad1: ['', 'nop.w', 'nop.n',]
      pad2: ['', 'add.w r1, r1', 'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
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
{% for reg_name, ldr_dep in [("GPIO::DIN31_0", False), ("GPIO::DIN31_0", True), ("GPIO::EVFLAGS", False)] %}
{% set addr = sysbus_addresses[reg_name] %}
{% for lw in itertools.product('nw', repeat=4) %}
{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1\n add.n r1, r1') %} # for 8-bytes alignment
{% for x_cycles in range(5) %}
  {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r11") %}

  {{ mov_const_2w(r4, addr) }}
  {{ mov_const_2w(r5, "0x20000000") }}
  {{ x_loader }}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  @ TODO: configuration with n_x_cycles of the same cycle count
  ldr.{{lw.1}} {{ r5 if ldr_dep else r7 }}, [r4]
  ldr.{{lw.2}} r7, [r5]

  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}
  ldr.{{lw.3}}  r3, [r0, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
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

{% endblock %}
