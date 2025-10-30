---
name: MLA/MLS+B instructions test
description: "Timing test of MLA/MLS+B instructions"
dumped_symbols:
  times: 2 words # 2 instructions
configurations:
- { code: "gpram", lbEn: True }
- { code: "sram", lbEn: True }
- { code: flash, lbEn: True }
- { code: flash, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set instructions = ["mla.w", "mls.w"] %}

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
{% for instruction in instructions %}
{% set jumpLabel = "jump_section_%s" % instruction %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare input values
    mov.w  r6, #0
    ldr.w  r7, =4260791909
    mov.w  r8, #3050

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]
.align 4
    nop.w
    udiv.w r6, r7, r8  @ takes 10 cycles
    {{instruction}} r6, r6, r7, r8
    b.w {{jumpLabel}}
    nop.w
    nop.w
    nop.w
    nop.w

{{jumpLabel}}:
.align 4

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save

{% endfor %}

b.w end_label

save:
    subs.n r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}

    bx.n lr
{% endblock %}
