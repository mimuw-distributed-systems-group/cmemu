---
name: Tests LDM instructions with long register set
description: With long register-sets we may notice if there is burst&round-robin interaction on main buses
dumped_symbols:
  counter: auto
configurations: []
product:
    - code: [flash, sram] # gpram
      data: [flash, gpram, sram]
      lbEn: [True, False]
      cnt: [CYCCNT, CPICNT, LSUCNT]
      x_cycles: [0, 6]
      pad2: ['add.w r1, r1'] # branches act as decode-time branch
      #pad2: ['', 'add.w r1, r1', 'add.n r1, r1',] # branches act as decode-time branch
      instr: ['ldm.w', 'ldm.n']
...
@ register assignment:
@ r0 - initial counter
@ r1 - addr of addr of dwt, final counter
@ r2 - addr of dwt
@ remaining -> scratch

@ The idea of this assignment is that we use r1 as base and all registers, but at r2 we load dwt address

{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}

{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}
{% set register_list = ['r1', 'r2', 'r3', 'r4', 'r5', 'r6', 'r7', 'r8', 'r9', 'r10', 'r11'] %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for pre_pad in ('', 'add.n r7, r7', 'add.w r7, r7') %} # for 8-bytes alignment
{% for n_regs in range(2, 11 if 'w' in instr else 7) %}
@{% for x_cycles in (0, 6) %}
{% for slide in (range(1, 6) if pad2 else range(0,0)) %}

  {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r11") %}
  {{ x_loader }}
  {{ mov_const_2w(r1, "base_reg") }}
  {{ mov_const_2w(r2, "dwt") }}

  @ Prepare register contents
  .align 3
  {{ pre_pad }}
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r0, [r2, {{counter}}]
  @ Fill pipeline
  {{ x_word_exec }}
  {{instr}} r1, {{ '{%s}' % ','.join(register_list[:n_regs]) }}

  @ Check for fetch occupancy
  {% for i in range(slide) %}
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {% endfor %}
  ldr.w  r1, [r2, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
@{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

save:
  subs.n r2, r1, r0
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r3, r4)}}
  bx.n lr

{{ section(data) }}
.align 3
base_reg:
.word base_reg @ r1
.word dwt @ r2
.rept 16
.word
.endr


{% endblock %}
