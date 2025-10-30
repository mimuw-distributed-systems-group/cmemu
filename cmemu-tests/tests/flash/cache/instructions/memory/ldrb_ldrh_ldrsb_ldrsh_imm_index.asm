---
name: LDRB/LDRH/LDRSB/LDRSH (immediate) with writeback instructions tests
description: >
  Timing and correctness test.
dumped_symbols:
  times: 24 words     # 2 (pre/postindex) * 3 (repetitions) * 4 (transfer size)
  results: 96 words  # ... * 4 (registers saved)
configurations:
- { code: sram, memory: sram, cache_enabled: True }
- { code: sram, memory: flash, cache_enabled: True }
- { code: flash, memory: sram, cache_enabled: True }
- { code: flash, memory: flash, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set data_regs = ["r7", "r8", "r9"] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

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

{% for size in ['b', 'h', 'sb', 'sh'] %}
    {% set offset = 2 if size in ['h', 'sh'] else 1 %}
{% for preindex in [True, False] %}
{% for reps in range(1, 4) %}
    @ Prepare ldr input values and reset flash line buffer
    ldr.w  r6, =memory
    mov.w  r7, #0
    mov.w  r8, #0
    mov.w  r9, #0

    @ Reset flash line buffer
    ldr.w  r2, [r7, r7]

    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        ldr{{size}}.w {{data_regs[i]}}, {% if preindex %} [r6, {{offset}}]! {% else %} [r6], {{offset}} {% endif %}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r6, r3, r4)}}
    {{saveResult(r7, r3, r4)}}
    {{saveResult(r8, r3, r4)}}
    {{saveResult(r9, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
memory:
    .word 0x01234567
    .word 0x89ABCDEF
{% endblock %}
