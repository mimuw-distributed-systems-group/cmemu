---
name: STR (immediate) with writeback instruction tests
description: >
    Timing and correctness test.
dumped_symbols:
  results: 6 words
  times: 6 words
  cpicnts: 6 words
  lsucnts: 6 words
  written_memory: user-defined
configurations:
- { code: gpram, memory: gpram, lbEn: true }
- { code: gpram, memory: sram, lbEn: true }
- { code: sram, memory: gpram, lbEn: true }
- { code: sram, memory: sram, lbEn: true }
- { code: flash, memory: gpram, lbEn: false }
- { code: flash, memory: sram, lbEn: false }
- { code: flash, memory: gpram, lbEn: true }
- { code: flash, memory: sram, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set data_regs = ["r7", "r8", "r9"] %}

{% block code %}
    @ Prepare cycle counter timer address
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

{% for counter, save_func in [(CYCCNT, "save_time_results_data"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for preindex in [True, False] %}
{% for reps in range(1, 4) %}
    @ Prepare ldr input values and reset flash line buffer
    ldr.w  r6, =rep_{{2*reps+(1 if preindex else 0)}}_memory
    mov.w  r7, #32
    mov.w  r8, #33
    mov.w  r9, #34
    ldr.w  r2, [r7, r7]

    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {% for i in range(reps) %}
        str.w {{data_regs[i]}}, {% if preindex %} [r6, 4]! {% else %} [r6], 4 {% endif %}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]
    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.thumb_func
save_time_results_data:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r6, r3, r4)}}
    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{{ section(memory) }}
.global written_memory
.align 4
written_memory:
{% for reps in range(8) %}
rep_{{reps}}_memory:
    .word {{ reps*4 }}
    .word {{ reps*4 + 1 }}
    .word {{ reps*4 + 2 }}
    .word {{ reps*4 + 3 }}
{% endfor %}
.size written_memory, .-written_memory
{% endblock %}
