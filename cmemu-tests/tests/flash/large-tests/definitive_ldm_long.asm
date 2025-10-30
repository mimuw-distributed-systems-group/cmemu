---
name: Tests LDM instructions with long register set
description: With long register-sets we may notice if there is burst&round-robin interaction on main buses
dumped_symbols:
  counter: auto
configurations: []
product:
    -
#      code: [gpram, flash, sram]
      data: [flash, gpram, sram]
      lbEn: [True, False]
#      lbEn: [True]
#      cnt: [CYCCNT, CPICNT, LSUCNT]
      cnt: [CYCCNT, LSUCNT]
      x_cycles_part: [0, 1]
      pad1: ['nop.w', '',  'nop.n', 'add.n r7, r7', 'b.w .+4', 'b.n .+2', 'isb.w']
      pad2: ['add.w r7, r7', 'add.n r7, r7',] # branches act as decode-time branch
      instr: ['ldm.w', 'ldm.n']
      code: [gpram, flash]
      wback: [False, True]
#      part: [1, 2]
...
@ register assignment:
@ r0 - initial counter
@ r1 - addr of addr of dwt, final counter
@ r2 - addr of dwt
@ r3 - veneer to the above
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
{% set register_list = ['r1', 'r2', 'r3', 'r4', 'r5', 'r6', 'r7', 'r8', 'r9', 'pc'] %}
@ Commented for part (1, 2)
@{% set reps_range = [1*part] if code == 'gpram' else [part, part + 2] %}
@ TODO: support multiple with wback
{% set reps_range = [1] if code == 'gpram' or wback else [1, 2] %}

{% block after %}
{{ section(code) }}
.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% for pre_pad in ('', 'add.n r7, r7', 'add.w r7, r7') %} # for 8-bytes alignment
{% for n_regs in range(2, 10 if 'w' in instr else 6-wback) %}
{% for reps in reps_range %}
{% for x_cycles in (range(x_cycles_part*3,x_cycles_part*3+3) if code != 'gpram' else range(x_cycles_part*2,x_cycles_part*2+2)) %}
{% for slide in (range(0, 8) if code != 'gpram' else range(0,3)) %}

  {% set continue_label = uniq_label("continue") %}
  {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r11") %}
  {{ x_loader }}
  ldr.w r1, ={{ 'base_reg' if not wback else 'base_reg_wback' }}
  ldr.w r2, ={{DWT_BASE}}
@  {{ mov_const_2w(r1, "base_reg") }}
@  {{ mov_const_2w(r2, "dwt") }}
  adr.w lr, {{continue_label}}+1

  @ Prepare register contents
  .align {{ 3 if code == 'flash' else 2 }}
  {{ pre_pad }}
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.n  r0, [r2, {{counter}}]
  @ Fill pipeline
  {{ x_word_exec }}

  {% for i in range(reps) %}
  @ writeback cannot have r1 in set
  {{instr}} r1{{ '!' if wback else ''}}, {{ '{%s}' % ','.join(register_list[wback:n_regs+wback]) }}
  {% endfor %}
  {{ pad1 }}

  @ Check for fetch occupancy
  {% for i in range(slide) %}
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {% endfor %}
  ldr.n  r1, [r2, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{{continue_label}}:

{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label


.align 2
jump_to_save:
  ldr.w  r1, [r2, {{counter}}]
save:
  subs.n r2, r1, r0
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r6, r4)}}
  bx.n lr

{{ section(data) }}
.align 3
base_reg:
.word base_reg @ r1
base_reg_wback:
.word {{DWT_BASE}} @ r2
.rept 16
.word jump_to_save+1
.endr


{% endblock %}
