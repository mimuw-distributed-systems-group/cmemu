---
name: STRB/STRH (immediate) with writeback instructions tests
description: >
    Timing and correctness test.
dumped_symbols:
  times: 12 words     # 2 (pre/postindex) * 3 (repetitions) * 2 (transfer size)
  results: 12 words   # see times
  stored_memory: user-defined
configurations:
- { code: gpram, lbEn: true, memory: gpram }
- { code: gpram, lbEn: true, memory: sram }
- { code: sram, lbEn: true, memory: gpram }
- { code: sram, lbEn: true, memory: sram }
- { code: flash, lbEn: true, memory: gpram }
- { code: flash, lbEn: true, memory: sram }
- { code: flash, lbEn: false, memory: gpram }
- { code: flash, lbEn: false, memory: sram }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set dataRegs = ["r7", "r8", "r9"] %}
{% set sizeValues = ['b', 'h'] %}
{% set preindexValues = [True, False] %}
{% set repetitionRange = range(1, 4) %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Prepare stored values
    ldr.w  r7, =0x01234567
    ldr.w  r8, =0x89ABCDEF
    ldr.w  r9, =0xEC421B71

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

{% for size in sizeValues %}
    {% set offset = 2 if size == 'h' else 1 %}
{% for preindex in preindexValues %}
{% for reps in repetitionRange %}
    @ Prepare str base address
    ldr.w  r6, =memory_{{reps}}_{{preindex}}_{{size}}
    
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        str{{size}}.w {{dataRegs[i]}}, {% if preindex %} [r6, {{offset}}]! {% else %} [r6], {{offset}} {% endif %}
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
    bx.n lr

{{ section(memory) }}
.align 4
.global	stored_memory
stored_memory:
{% for size in sizeValues %}
{% for preindex in preindexValues %}
{% for reps in repetitionRange %}
    memory_{{reps}}_{{preindex}}_{{size}}: 
        .word 0
        .word 0
{% endfor %}
{% endfor %}
{% endfor %}
.size	stored_memory, .-stored_memory
{% endblock %}
