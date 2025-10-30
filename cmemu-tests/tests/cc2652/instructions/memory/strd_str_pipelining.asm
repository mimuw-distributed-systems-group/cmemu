---
name: STRD+STR instruction pipeling tests
description: "Timing and correctness test showing, that strd doesn't pipeline with str"
dumped_symbols:
  stored_memory: user-defined
  times: 48 words # 4 (offsets) * 12 (repetitions)
configurations:
- { code: sram, lbEn: true, memory: sram }
- { code: flash, lbEn: true, memory: sram }
- { code: flash, lbEn: false, memory: sram }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set values = ["0x01234567", "0x89ABCDEF", "0x6AB2701C"] %}
{% set offsetRange = 4 %}

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

{% for offset_cnt in range(offsetRange) %}
    {% set offset_strd = offset_cnt * 8 %} @ * 2(words) * 4(bytes)
    {% set offset_str = offset_cnt * 8 + 8 %} @ + 2(words) * 4(bytes)
    @ Prepare value to store
    ldr.w  r4, ={{values[0]}}
    ldr.w  r5, ={{values[1]}}
    ldr.w  r6, ={{values[2]}}
    @ Prepare address to store to
    ldr.w  r7, =rep_{{offset_cnt}}_memory

    b.n label_{{offset_cnt}}

@ We store constants close to their usage, to allow longer program.
.ltorg

label_{{offset_cnt}}:
{% for reps in range(1, 12) %}
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r1, [r0, {{CYCCNT}}]

    {% for i in range(reps) %}
        {% if i % 2 == 0 %}
            strd.w r4, r5, [r7, {{offset_strd}}]
        {% else %}
            str.w r6, [r7, {{offset_str}}]
        {% endif %}
    {% endfor %}

    @ Get finish time
    ldr.w  r2, [r0, {{CYCCNT}}]

    bl.w save

{% endfor %}
{% endfor %}

    b.w end_label

.align 2
save:
    subs.n r1, r2, r1
    {{saveTime(r1, r2, r3)}}
    bx.n lr

{{ section(memory) }}
.align 4
.global	stored_memory
stored_memory:
{% for offset_cnt in range(offsetRange) %}
    rep_{{offset_cnt}}_memory: 
    @ There are 2 words for each possible offset value plus one for single store.
    {% for _ in range(offsetRange*2+1) %}
        .word 0
    {% endfor %}
{% endfor %}
.size	stored_memory, .-stored_memory

{% endblock %}
