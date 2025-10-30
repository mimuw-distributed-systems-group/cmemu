---
name: STR from two memories tests
description: "Timing test of str, when doing consecutive accesses to different memory types"
dumped_symbols:
  times: 30 words
configurations:
- { code: "sram", addr0: "sram", addr1: "sram", strInstr: "str.w" }
- { code: "sram", addr0: "sram", addr1: "sram", strInstr: "str.n" }
- { code: "flash", addr0: "sram", addr1: "sram", strInstr: "str.w" }
- { code: "flash", addr0: "sram", addr1: "sram", strInstr: "str.n" }
...
{% set repetitions = 10 %}
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r3, dwt
    
    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for cntr in [CYCCNT, CPICNT, LSUCNT] %}
    {% set cntr_idx = loop.index %}
    mov.w  r11, {{cntr}}

{% for reps in range(repetitions) %}
    @ Prepare str input values
    ldr.w  r5, =cell_0
    ldr.w  r6, =cell_1
    mov.w  r7, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r3, r11]

    {% for i in range(reps//2) %}
        {{strInstr}} r1, [r5, r7]
        {{strInstr}} r4, [r6, r7]
    {% endfor %}
    {% if reps % 2 == 1 %}
        {{strInstr}} r1, [r5, r7]
    {% endif %}

    @ Get finish time
    ldr.w  r0, [r3, r11]
    bl.w {{ 'save32' if cntr == CYCCNT else 'save8' }}

{% endfor %}
{% endfor %}
    b.w end_label

save32:
    sub.w  r0, r0, r2
    {{saveTime(r0, r1, r2)}}
    bx.n lr

save8:
    sub.w  r0, r0, r2
    and.w  r0, r0, 0xFF
    {{saveTime(r0, r1, r2)}}
    bx.n lr

{{ section(addr0) }}
.align 4
cell_0: .word cell_0

{{ section(addr1) }}
.align 4
cell_1: .word cell_1

{% endblock %}
