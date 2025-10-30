---
name: Check side effects with a weird case, where some numbers of stalls + AGU yields surprising results
description: >
    It turns out, that if in the following code, we change 6 to 5 or 7, the code runs as expected.
    This seems to happens, when ID is directly under D:AGU.
    This test checks if the following instruction is doubly executed or only doubly fetched.

dumped_symbols:
  times: auto
  flags: auto
  results: auto
  results2: auto
configurations:
- { code: flash, lbEn: False }
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
{% for six_is_special in (6, 5, 7) %}
{% set loader6, word_exec6 = n_x_cycles(six_is_special, "r8", "r9") %}
{% for addr in ("0x20000080",  "0x400220C0", "0x400220E0") %}
{% for slider2 in range(0, 4) %}
{% for alt in ([] if slider2 > 0 else ['', 'b.w .+4', 'b.n .+2', 'ldr.n r6, [r3]', 'ldr.w r6, [r3]']) %}
{% for slider in range(0, 6) %}
  {% set slider_loader, slider_word_exec = n_x_cycles(slider, "r10", "r11") %}
  {% set slider2_loader, slider2_word_exec = n_x_cycles(slider2, "r12", "r9", load_2=False) %}
   @ clear flags
  mov.w r1, #0
  mov.w r3, #16
  msr.w apsr_nzcvq, r1

  @ Prepare register contents
  {{ mov_const_2w(r4, addr) }}
  {{loader6}}
  {{slider_loader}}
  {{slider2_loader}}

  .align 3
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]

  {{word_exec6}}
  mov.n r5, r4
  ldr.w r7, [r5] @ addr dep

  {{slider_word_exec}}

  {{slider2_word_exec}}

  {{alt}}
  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
{% set skipl = uniq_label() %}
b.w {{skipl}}
.ltorg
{{skipl}}:
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

.ltorg

save:
  mrs.w r1, apsr
  subs.n r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
  {{saveValue("results", r9, r3, r4)}}
  {{saveValue("results2", r12, r3, r4)}}
  bx.n lr
{% endblock %}
