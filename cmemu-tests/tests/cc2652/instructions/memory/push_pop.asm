---
name: PUSH and POP instructions tests
description: Timing and correctness test
dumped_symbols:
  results: 19 words
  times: 19 words
  data: 44 words
configurations:
- { code: sram, lbEn: true }
- { code: flash, lbEn: false }
- { code: flash, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set register_sets = [
        ["r0"],
        ["r1"],
        ["r2"],
        ["r8"],
        ["r9"],
        ["lr"],
        ["r0", "r1"],
        ["r0", "r2"],
        ["r1", "r2"],
        ["r8", "r9"],
        ["r0", "r8"],
        ["r0", "lr"],
        ["r8", "lr"],
        ["r0", "r1", "r2"],
        ["r0", "r8", "lr"],
        ["r8", "r9", "lr"],
        ["r0", "r1", "r2", "r8"],
        ["r0", "r1", "r2", "r8", "r9"],
        ["r0", "r1", "r2", "r8", "r9", "lr"],
    ]
%}

{% block code %}
    @ Prepare cycle counter address
    ldr.w  r6, dwt

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

{% for registers in register_sets %}
    @ Decide the width of PUSH and POP instructions, prefer narrow encodings.
    @ Note that narrow POP instruction does not support reading to LR
    {% set width = 'w' if 'r8' in registers or 'r9' in registers or 'lr' in registers else 'n' %}

    @ Prepare input values
    {% for i in range(registers|length) %}
        mov.w {{ registers[i] }}, {{i+1}}
    {% endfor %}

    @ Store initial SP
    mov.w r11, sp

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r6, {{CYCCNT}}]

    push.{{width}} { {{ registers|join(", ") }} }
    {% for i in range(registers|length) %}
        mov.w {{ registers[i] }}, 0
    {% endfor %}
    pop.{{width}} { {{ registers|join(", ") }} }

    @ Get finish time
    ldr.w  r5, [r6, {{CYCCNT}}]

    @ Save execution time
    sub.w r5, r5, r4
    {{ saveTime(r5, r3, r4) }}
    @ Save (initial SP - current SP)
    mov.w r4, sp
    sub.w r5, r11, r4
    {{ saveResult(r5, r3, r4) }}
    @ Save registers contents
    {% for rN in registers %}
        {{ saveValue("data", rN, r3, r4) }}
    {% endfor %}

    @ Restore initial SP
    mov.w sp, r11

    b.n after_pool_{{loop.index}}
    .ltorg
after_pool_{{loop.index}}:
{% endfor %}

    b.w end_label
{% endblock %}
