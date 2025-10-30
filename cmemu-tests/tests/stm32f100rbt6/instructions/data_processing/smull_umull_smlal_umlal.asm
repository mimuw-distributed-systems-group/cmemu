---
name: SMULL/UMULL/SMLAL/UMLAL instructions tests
description: "Timing and correctness test of multiple SMULL/UMULL/SMLAL/UMLAL instructions one after another"
dumped_symbols:
  results: 40 words
  times: 20 words
  flags: 20 words
configurations:
# SMULL takes 3 cycles
- { code: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
# SMULL takes 4 cycles
- { code: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smull.w" }
# SMULL takes 5 cycles
- { code: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smull.w" }
- { code: flash, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smull.w" }
# UMULL takes 3 cycles
- { code: sram, lbEn: True, r6Value: "#0x00000042", r7Value: "#0x0000A870", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: True, r6Value: "#0x00000042", r7Value: "#0x0000A870", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: False, r6Value: "#0x00000042", r7Value: "#0x0000A870", repetitions: 20, testedInstr: "umull.w" }
# UMULL takes 4 cycles
- { code: sram, lbEn: True, r6Value: "#0x80EFC9F4", r7Value: "#0x00001A00", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: True, r6Value: "#0x80EFC9F4", r7Value: "#0x00001A00", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: False, r6Value: "#0x80EFC9F4", r7Value: "#0x00001A00", repetitions: 20, testedInstr: "umull.w" }
# UMULL takes 5 cycles
- { code: sram, lbEn: True, r6Value: "#0xA942F28C", r7Value: "#0x977E0B46", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: True, r6Value: "#0xA942F28C", r7Value: "#0x977E0B46", repetitions: 20, testedInstr: "umull.w" }
- { code: flash, lbEn: False, r6Value: "#0xA942F28C", r7Value: "#0x977E0B46", repetitions: 20, testedInstr: "umull.w" }
# SMLAL takes 3 cycles
- { code: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
# SMLAL takes 5 cycles
- { code: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "smlal.w" }
# SMLAL takes 6 cycles
- { code: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "smlal.w" }
# SMLAL takes 7 cycles
- { code: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "smlal.w" }
# UMLAL takes 3 cycles
- { code: sram, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x00002DA0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
# UMLAL takes 5 cycles
- { code: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x00000480", repetitions: 20, testedInstr: "umlal.w" }
# UMLAL takes 6 cycles
- { code: sram, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x402DF0A0", r7Value: "#0x04800000", repetitions: 20, testedInstr: "umlal.w" }
# UMLAL takes 7 cycles
- { code: sram, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: True, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "umlal.w" }
- { code: flash, lbEn: False, r6Value: "#0x98395B39", r7Value: "#0x824704EA", repetitions: 20, testedInstr: "umlal.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Prepare input values
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Clean result registers
    mov.w  r8, #0
    mov.w  r9, #0

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
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r8, r9, r6, r7
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    sub.w r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r8, r3, r4)}}
    {{saveValue('results', r9, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr
{% endblock %}
