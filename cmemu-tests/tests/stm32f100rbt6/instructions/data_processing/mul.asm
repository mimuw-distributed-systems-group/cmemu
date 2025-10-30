---
name: MUL instruction tests
description: "Timing and correctness test of multiple mul instructions one after another"
dumped_symbols:
  results: 20 words
  times: 20 words
  flags: 20 words
configurations:
# wide instruction
- { code: "sram", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: False, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "sram", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: False, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "sram", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
- { code: "flash", lbEn: True, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
- { code: "flash", lbEn: False, testedInstr: "mul.w", dstReg: "r5", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
# narrow instruction
- { code: "sram", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: False, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x00002DA0", r7Value: "#0x00000480" }
- { code: "sram", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "flash", lbEn: False, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x402DF0A0", r7Value: "#0x00000480" }
- { code: "sram", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
- { code: "flash", lbEn: True, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
- { code: "flash", lbEn: False, testedInstr: "muls.n", dstReg: "r7", r6Value: "#0x98395B39", r7Value: "#0x824704EA" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 20 %}
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
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Prepare input values
    mov.w  {{dstReg}}, #0
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} {{dstReg}}, r6, r7
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r6, apsr
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(dstReg, r3, r4)}}
    {{saveValue('flags', r6, r3, r4)}}

    bx.n lr
{% endblock %}
