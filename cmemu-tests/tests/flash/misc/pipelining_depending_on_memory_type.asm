---
name: Pipelining of STR before LDR when accessing different memories
description: Check if pipelining happen or not
dumped_symbols:
  times: 16 words
configurations:
- { code: gpram, data: sram }
- { code: gpram, data: dwt }
- { code: sram, data: gpram }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set offset = { 'gpram': 0, 'sram': 0, 'dwt': FOLDCNT }[data] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    ldr.w  r1, {{ {'gpram': '=memory', 'sram': '=memory', 'dwt': 'dwt'}[data] }}

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

{% for conflictReg in ["r4", "r5"] %}
{% for reps in range(8) %}
    @ Prepare input values
    mov.w r4, {{reps}}

    @ Align and clear pipeline
    .align 4
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]

    @ Pipelinig guard
    adds.n r6, 0

    {% for i in range(reps) %}
        movs.n {{conflictReg}}, {{i}}
        str.n r4, [r1, {{offset}}]
        ldr.n r5, [r1, {{offset}}]
    {% endfor %}

    @ Pipelining guard
    adds.n r6, 0

    @ Get finish time
    ldr.n r3, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveValue("times", r2, r3, r4)}}
    bx.n lr

{% if data == 'sram' or data == 'gpram' %}
{{ section(data) }}
.align 2
memory: .word 0
{% endif %}

{% endblock %}
