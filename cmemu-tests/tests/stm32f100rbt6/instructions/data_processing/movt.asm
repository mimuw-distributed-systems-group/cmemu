---
name: MOVT instructions tests
description: Timing and correctness test
dumped_symbols:
  results: 80 words
  times: 80 words
  flags: 80 words
configurations:
- { code: sram, lbEn: True }
- { code: flash, lbEn: True }
- { code: flash, lbEn: False }
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
{% for initv in ["#0xFFFF0000", "#0x0000FFFF", "#0xF0F0F0F0", "#0xFFFFFFFF", "#0x00000000"] %}
{% for msb in range(16) %}  @ most significant bit set in immediate
    ldr.w r6, ={{initv}}

    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    movt.w r6, 0b1{% for i in range(msb) %}0{% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

{% endfor %}
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r6, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}
    bx.n lr

{% endblock %}
