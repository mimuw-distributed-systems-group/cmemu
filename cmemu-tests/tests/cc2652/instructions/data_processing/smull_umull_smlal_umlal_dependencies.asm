---
name: SMULL/UMULL/SMLAL/UMLAL register writing order
description: "Timing and correctness test of SMULL/UMULL/SMLAL/UMLAL followed by LDR"
dumped_symbols:
  results: 60 words
  times: 30 words
  flags: 30 words
configurations:
# SMULL takes 3 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 15, testedInstr: "smull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 15, testedInstr: "smull.w" }
# SMULL takes 4 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 15, testedInstr: "smull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 15, testedInstr: "smull.w" }
# SMULL takes 5 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 15, testedInstr: "smull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 15, testedInstr: "smull.w" }
# UMULL takes 3 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x00000042", r7Value: "#0x0000A870", repetitions: 15, testedInstr: "umull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x00000042", r7Value: "#0x0000A870", repetitions: 15, testedInstr: "umull.w" }
# UMULL takes 4 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x80EFC9F4", r7Value: "#0x00001A00", repetitions: 15, testedInstr: "umull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x80EFC9F4", r7Value: "#0x00001A00", repetitions: 15, testedInstr: "umull.w" }
# UMULL takes 5 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0xA942F28C", r7Value: "#0x977E0B46", repetitions: 15, testedInstr: "umull.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0xA942F28C", r7Value: "#0x977E0B46", repetitions: 15, testedInstr: "umull.w" }
# SMLAL takes 3 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "smlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "smlal.w" }
# SMLAL takes 5 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "smlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "smlal.w" }
# SMLAL takes 6 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 2, testedInstr: "smlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 2, testedInstr: "smlal.w" }
# SMLAL takes 7 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 2, testedInstr: "smlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 2, testedInstr: "smlal.w" }
# UMLAL takes 3 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "umlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "umlal.w" }
# UMLAL takes 5 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "umlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 2, testedInstr: "umlal.w" }
# UMLAL takes 6 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 2, testedInstr: "umlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 2, testedInstr: "umlal.w" }
# UMLAL takes 7 cycles
- { code: flash, data: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 2, testedInstr: "umlal.w" }
- { code: flash, data: sram, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 2, testedInstr: "umlal.w" }
...
{% set resultRegisters = ["r8", "r9"] %}
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
{% for resReg in resultRegisters %}
    @ Prepare input values
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Prepare LDR input values
    mov.w  r8, #0
    mov.w  r9, #0
    ldr.w  r5, =loaded_data
    {{testedInstr}} r8, r9, r6, r7
    sub.w  r5, r5, {{resReg}}

{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r8, #0
    msr.w apsr_nzcvq, r8

    @ Clean result registers
    mov.w  r8, #0
    mov.w  r9, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r8, r9, r6, r7
        @ Value loaded to r4 is ignored, as we are interested only in timing of LDR instruction.
        ldr.w  r4, [r5, {{resReg}}]
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label

save:
    mrs.w r10, apsr
    sub.w r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r8, r3, r4)}}
    {{saveValue('results', r9, r3, r4)}}
    {{saveValue('flags', r10, r3, r4)}}

    bx.n lr

{{ section(data) }}
.align 4
loaded_data: .word 0x0

{% endblock %}
