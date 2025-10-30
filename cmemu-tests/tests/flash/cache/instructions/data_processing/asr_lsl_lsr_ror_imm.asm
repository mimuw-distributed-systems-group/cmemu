---
name: ASR/LSL/LSR/ROR (immediate) tests
description: "Timing and correctness test of shift instructions from immediate. Listed in [ARM_ARM] A4.4.2 (without RRX)."
dumped_symbols:
  results: 35 words
  times: 35 words
  flags: 35 words
configurations:
# ASR
- { code: "gpram", lbEn: True, testedInstr: "asrs.n" }
- { code: "gpram", lbEn: True, testedInstr: "asr.w" }
- { code: "gpram", lbEn: True, testedInstr: "asrs.w" }
- { code: "sram", lbEn: True, testedInstr: "asrs.n" }
- { code: "sram", lbEn: True, testedInstr: "asr.w" }
- { code: "sram", lbEn: True, testedInstr: "asrs.w" }
- { code: "flash", lbEn: True, testedInstr: "asrs.n" }
- { code: "flash", lbEn: True, testedInstr: "asr.w" }
- { code: "flash", lbEn: True, testedInstr: "asrs.w" }
- { code: "flash", lbEn: False, testedInstr: "asrs.n" }
- { code: "flash", lbEn: False, testedInstr: "asr.w" }
- { code: "flash", lbEn: False, testedInstr: "asrs.w" }

# LSL
- { code: "gpram", lbEn: True, testedInstr: "lsls.n" }
- { code: "gpram", lbEn: True, testedInstr: "lsl.w" }
- { code: "gpram", lbEn: True, testedInstr: "lsls.w" }
- { code: "sram", lbEn: True, testedInstr: "lsls.n" }
- { code: "sram", lbEn: True, testedInstr: "lsl.w" }
- { code: "sram", lbEn: True, testedInstr: "lsls.w" }
- { code: "flash", lbEn: True, testedInstr: "lsls.n" }
- { code: "flash", lbEn: True, testedInstr: "lsl.w" }
- { code: "flash", lbEn: True, testedInstr: "lsls.w" }
- { code: "flash", lbEn: False, testedInstr: "lsls.n" }
- { code: "flash", lbEn: False, testedInstr: "lsl.w" }
- { code: "flash", lbEn: False, testedInstr: "lsls.w" }

# LSR
- { code: "gpram", lbEn: True, testedInstr: "lsrs.n" }
- { code: "gpram", lbEn: True, testedInstr: "lsr.w" }
- { code: "gpram", lbEn: True, testedInstr: "lsrs.w" }
- { code: "sram", lbEn: True, testedInstr: "lsrs.n" }
- { code: "sram", lbEn: True, testedInstr: "lsr.w" }
- { code: "sram", lbEn: True, testedInstr: "lsrs.w" }
- { code: "flash", lbEn: True, testedInstr: "lsrs.n" }
- { code: "flash", lbEn: True, testedInstr: "lsr.w" }
- { code: "flash", lbEn: True, testedInstr: "lsrs.w" }
- { code: "flash", lbEn: False, testedInstr: "lsrs.n" }
- { code: "flash", lbEn: False, testedInstr: "lsr.w" }
- { code: "flash", lbEn: False, testedInstr: "lsrs.w" }

# ROR
- { code: "gpram", lbEn: True, testedInstr: "ror.w" }
- { code: "gpram", lbEn: True, testedInstr: "rors.w" }
- { code: "sram", lbEn: True, testedInstr: "ror.w" }
- { code: "sram", lbEn: True, testedInstr: "rors.w" }
- { code: "flash", lbEn: True, testedInstr: "ror.w" }
- { code: "flash", lbEn: True, testedInstr: "rors.w" }
- { code: "flash", lbEn: False, testedInstr: "ror.w" }
- { code: "flash", lbEn: False, testedInstr: "rors.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set shiftRange = 32 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    @ Prepare input value, we load it from memory to enforce encoding.
    @ It is close, so we can use LDR (literal).
    ldr.w  r6, shifted_value

    b.w    tested_code

.align 2
shifted_value: .word 0xf7f7b5b5

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for shift in range(1, shiftRange) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    b.n rep_{{shift}}_label
.align 4
rep_{{shift}}_label:
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{testedInstr}} r7, r6, #{{shift}}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(r7, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr
{% endblock %}
