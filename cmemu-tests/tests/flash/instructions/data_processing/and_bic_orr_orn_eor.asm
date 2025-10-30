---
name: AND/BIC/ORR/ORN/EOR instructions tests
description: Timing and correctness test
dumped_symbols:
  results: 100 words
  times: 100 words
  flags: 100 words
configurations:
# AND tests:
- { code: gpram, lbEn: true, testedInstr: "ands.n", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "and.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "and.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "ands.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "ands.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "ands.n", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "and.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "and.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "ands.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "ands.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "ands.n", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "ands.n", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "and.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "and.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "and.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "and.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "ands.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "ands.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "ands.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "ands.w", instrType: imm }
# BIC tests:
- { code: gpram, lbEn: true, testedInstr: "bics.n", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "bic.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "bic.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "bics.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "bics.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "bics.n", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "bic.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "bic.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "bics.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "bics.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "bics.n", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "bics.n", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "bic.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "bic.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "bic.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "bic.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "bics.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "bics.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "bics.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "bics.w", instrType: imm }
# ORR tests:
- { code: gpram, lbEn: true, testedInstr: "orrs.n", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "orr.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "orr.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "orrs.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "orrs.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "orrs.n", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "orr.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "orr.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "orrs.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "orrs.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "orrs.n", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "orrs.n", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "orr.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "orr.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "orr.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "orr.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "orrs.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "orrs.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "orrs.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "orrs.w", instrType: reg }
# ORN tests:
- { code: gpram, lbEn: true, testedInstr: "orn.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "orn.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "orns.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "orns.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "orn.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "orn.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "orns.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "orns.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "orn.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "orn.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "orn.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "orn.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "orns.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "orns.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "orns.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "orns.w", instrType: reg }
# EOR tests:
- { code: gpram, lbEn: true, testedInstr: "eors.n", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "eor.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "eor.w", instrType: reg }
- { code: gpram, lbEn: true, testedInstr: "eors.w", instrType: imm }
- { code: gpram, lbEn: true, testedInstr: "eors.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "eors.n", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "eor.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "eor.w", instrType: reg }
- { code: sram, lbEn: true, testedInstr: "eors.w", instrType: imm }
- { code: sram, lbEn: true, testedInstr: "eors.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "eors.n", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "eors.n", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "eor.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "eor.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "eor.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "eor.w", instrType: reg }
- { code: flash, lbEn: true, testedInstr: "eors.w", instrType: imm }
- { code: flash, lbEn: false, testedInstr: "eors.w", instrType: imm }
- { code: flash, lbEn: true, testedInstr: "eors.w", instrType: reg }
- { code: flash, lbEn: false, testedInstr: "eors.w", instrType: reg }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}

{% set testedValues = ["#0xffffffff", "#0x77777777", "#0xd5d5d5d5", "#0x00ff00ff", "#0xff00ff00", "#0x5d5d5d5d", "#0x00000000"] %}
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

{% for leftID in range(testedValues|length) %}
{% for rightID in range(leftID+1) %}
{% for reps in range(1, 4) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare input values
    mov.w  r6, {{testedValues[leftID]}}
    {% if instrType == "reg" %}
        mov.w  r7, {{testedValues[rightID]}}
    {% endif %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r6, r6, {{"r7" if instrType == "reg" else testedValues[rightID]}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

{% endfor %}
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
