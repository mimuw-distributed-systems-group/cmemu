---
name: Test if N-X-cycles and Fetch-testing macros work
description: >
    We know that N-X-cycles should stall the pipeline for exactly N execute cycles.

dumped_symbols:
  times: auto
  flags: auto
  results: auto
  results2: auto
configurations:
- { code: "flash", "lbEn": True}
- { code: "flash", "lbEn": False}
- { code: "sram", "lbEn": False}
- { code: "gpram", "lbEn": False}
...

{% device:trace_mode = False %}
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
{% set loader5, word_exec5 = n_x_cycles(5, "r4", "r6", load_2=False) %}
{% for pad in ('', 'nop.w', 'add.w r6, 0', word_exec5, word_exec5 ~ '\nadd.w r4, 1') %}
{% for x_cycles in range(0, 13) %}
   @ clear flags
  mov.w r1, #0
  msr.w apsr_nzcvq, r1

  @ Prepare register contents
  {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r5", "r6") %}
  {{x_loader}}
  {{loader5}}

  .align 4
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]

  {{pad}}
  {{x_word_exec}}
  {{ assert_aligned(2) }}

  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]
  {{ inc_auto_syms() }}
  bl.w save
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
