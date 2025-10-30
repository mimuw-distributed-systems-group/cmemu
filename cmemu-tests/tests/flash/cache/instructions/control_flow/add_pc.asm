---
name: BX
description: "Timing test of bx to address in register"
dumped_symbols:
  results: 10 words
  times: 10 words
configurations:
- { code: "sram", lbEn: True, nopCountRange: 8, cache_enabled: True }
- { code: "flash", lbEn: False, nopCountRange: 8, cache_enabled: True }
- { code: "flash", lbEn: True, nopCountRange: 8, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% for nops in range(nopCountRange) %}
    @ Prepare branch arguments
    ldr.w  r4, =jump_{{nops}}_source
    ldr.w  r5, =jump_{{nops}}_target
    sub.w  r5, r4
    sub.w  r5, #4 @ Because the PC = source_ddress + 4

    @ Prepare add arguments
    mov.w  r6, #42
    mov.w  r7, #1

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    @ Jump to jump_{{nops}}_target
jump_{{nops}}_source:
    add.n pc, r5
    @ These `add`s shouldn't execute
    add.w  r6, r7
    add.w  r6, r7
    add.w  r6, r7
    add.w  r6, r7

.align 3
    {% for i in range(nops) %}
        nop.n
    {% endfor %}
jump_{{nops}}_target:
    @ Get finish time
    ldr.w  r1, [r0, {{CYCCNT}}]
    bl.w save
{% endfor %}

    b.w end_label


.align 3
.thumb_func
save:
    subs.n r2, r1, r2

    {{saveTime(r2, r1, r3)}}
    {{saveResult(r6, r1, r3)}}

    bx.n lr

{% endblock %}
