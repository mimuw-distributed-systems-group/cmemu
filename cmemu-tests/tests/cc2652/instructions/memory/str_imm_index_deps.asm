---
name: STR (immediate) with writeback instruction tests (data dependencies)
description: >
    Check if execution time of pre-/postindexed STR is correct after earlier LDR/ADD instruction.
dumped_symbols:
  times: 8 words
  results: 8 words
  written_memory: user-defined
configurations:
- { code: sram, memory: sram, lbEn: true }
- { code: flash, memory: sram, lbEn: false }
- { code: flash, memory: sram, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

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

{% for preinstr in ["a:add.w", "a:ldr.w", "d:add.w", "d:ldr.w"] %}
{% set preinstr_index = loop.index - 1 %}
{% for preindex in [True, False] %}
{% set preindex_index = loop.index - 1 %}
    @ Prepare ldr input values and reset flash line buffer
    ldr.w  r6, =memory_{{preinstr_index}}_{{preindex_index}}
    mov.w  r7, #42
    mov.w  r8, #0
    ldr.w  r9, {% if preinstr[0] == 'a' %} =memory_{{preinstr_index}}_{{preindex_index}}_addr {% else %} =memory_data {% endif %}
    ldr.w  r2, [r8, r8]

    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% if preinstr == "a:add.w" %}
        add.w r6, r6, r8
    {% elif preinstr == "a:ldr.w" %}
        ldr.w r6, [r9, r8]
    {% elif preinstr == "d:add.w" %}
        add.w r7, r7, r8
    {% elif preinstr == "d:ldr.w" %}
        ldr.w r7, [r9, r8]
    {% else %}
        panic!
    {% endif %}

    str.w r7, {% if preindex %} [r6, 4]! {% else %} [r6], 4 {% endif %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

{% endfor %}
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r6, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.global written_memory
.align 4
written_memory:
{% for i in range(4) %}
{% for j in range(2) %}
memory_{{i}}_{{j}}:
    .word {{i*8+i*4}}
    .word {{i*8+i*4+1}}
    .word {{i*8+i*4+2}}
    .word {{i*8+i*4+3}}
{% endfor %}
{% endfor %}
.size written_memory, .-written_memory


{% for i in range(4) %}
{% for j in range(2) %}
memory_{{i}}_{{j}}_addr: .word memory_{{i}}_{{j}}
{% endfor %}
{% endfor %}

.align 4
memory_data: .word 42
{% endblock %}
