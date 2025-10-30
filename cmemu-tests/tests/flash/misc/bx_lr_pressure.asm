---
name: Check PIQ strain when jumping straight to bx.n lr
description: It is to check when the slots are freed.
dumped_symbols:
  times: auto
  flags: auto
  results: auto
  results2: auto
configurations:
    - {code: flash, lbEn: True, instrs: ["bl.w", "blx.n"]}
    - {code: flash, lbEn: False, instrs: ["bl.w", "blx.n"]}
    - {code: sram, lbEn: True, instrs: ["blx.n"]}
    - {code: sram, lbEn: True, instrs: ["bl.w"]}
    - {code: gpram, lbEn: True, instrs: ["blx.n"]}
    - {code: gpram, lbEn: True, instrs: ["bl.w"]}
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for pad in ('', 'nop.n', 'nop.w', 'add.w r6, 0') %}
{% for x_cycles1 in range(1, 5) %}
{% for x_cycles2 in range(1, 8) %}
{% for instr in instrs %}
  {% set x_loader1, x_word_exec1 = n_x_cycles(x_cycles1, "r3", "r4") %}
  {% set x_loader2, x_word_exec2 = n_x_cycles(x_cycles2, "r5", "r6") %}
  {% set jump_label = uniq_label("bx_n") %}
  {% set next_label = uniq_label("next") %}
  ldr r7, ={{jump_label}}

  @ Prepare register contents
  {{x_loader1}}
  {{x_loader2}}

  .align 4
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]
  {{x_word_exec1}}
  {{x_word_exec2}}
  {{ assert_aligned(2) }}

  {% if instr == 'bl.w' %}
      bl.w {{jump_label}}
  {% else %}
      blx.n r7
  {% endif %}

  {{pad}}
  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]
  b.w {{next_label}}

.align 2
.thumb_func
{{jump_label}}:
  bx.n lr

 .ltorg

.thumb_func
{{next_label}}:
  bl.w save
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

save:
  mrs.w r1, apsr
  subs.n r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
  {{saveValue("results", r5, r3, r4)}}
  {{saveValue("results2", r6, r3, r4)}}
  bx.n lr
{% endblock %}
