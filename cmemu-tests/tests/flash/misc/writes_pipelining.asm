---
name: Test proving that AHB can pipeline writes
description: >
    To generate writes repeated STR (immediate) instruction is used.

    The most important configuration is the last one.
    It's timing pattern is: "3, (1)". So each additional STR executes in single cycle.
    Notice also, that memory cells accessed by consecutive STR instructions are not sequential.
    So the accesses cannot be done via single burst transfer,
    but via multiply nonsequential transfers.

dumped_symbols:
  times: 16 words
configurations: 
- { code: gpram, data: gpram, offsets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
- { code: sram, data: sram, offsets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
- { code: gpram, data: sram, offsets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
- { code: gpram, data: gpram, offsets: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] }
- { code: sram, data: sram, offsets: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] }
- { code: gpram, data: sram, offsets: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] }
- { code: gpram, data: gpram, offsets: [11, 14, 7, 4, 5, 1, 0, 10, 2, 8, 13, 9, 6, 12, 3, 15] }
- { code: sram, data: sram, offsets: [11, 14, 7, 4, 5, 1, 0, 10, 2, 8, 13, 9, 6, 12, 3, 15] }
- { code: gpram, data: sram, offsets: [11, 14, 7, 4, 5, 1, 0, 10, 2, 8, 13, 9, 6, 12, 3, 15] }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    @ Prepare str arguments and tested register
    mov.w  r5, #0
    ldr.w  r7, =memory_cell

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
{% for reps in range(16) %}
    isb.w   @ Clear PIQ

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
      str.w r5, [r7, {{offsets[i] * 4}}]
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
{% endfor %}
    b.w end_label

.ltorg

{{ section(data) }}
.align 4
memory_cell:
{% for i in range(16) %}
  .word 0x0
{% endfor %}
{% endblock %}
