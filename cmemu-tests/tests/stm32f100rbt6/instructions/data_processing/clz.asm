---
name: CLZ instructions tests
description: Timing and correctness test
dumped_symbols:
  results: 100 words
  times: 100 words
  flags: 100 words
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
{% for msb in range(33) %}
{% for reps in range(1, 4) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    {% if msb == 32 %}
        mov.w r7, 0
    {% else %}
        @ Fill bits that are less significant than the most significant (set) bit with different patterns
        @ for `reps` equal to: 1 -> 0b10000..; 2 -> 0b10101..; 3 -> 0b11111..
        ldr.w r7, =0b1{% for i in range(msb) %}{% if reps == 1 or (reps == 2 and i % 2 == 0) %}0{% else %}1{% endif %}{% endfor %}
    {% endif %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        clz.w r6, r7
    {% endfor %}

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
