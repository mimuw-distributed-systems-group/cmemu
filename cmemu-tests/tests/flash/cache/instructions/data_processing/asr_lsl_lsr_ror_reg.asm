---
name: ASR/LSL/LSR/ROR (register) tests
description: "Timing and correctness test of shift instructions from register. Listed in [ARM_ARM] A4.4.2 (without RRX)."
dumped_symbols:
  results: 68 words
  times: 68 words
  flags: 68 words
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
- { code: "gpram", lbEn: True, testedInstr: "rors.n" }
- { code: "gpram", lbEn: True, testedInstr: "ror.w" }
- { code: "gpram", lbEn: True, testedInstr: "rors.w" }
- { code: "sram", lbEn: True, testedInstr: "rors.n" }
- { code: "sram", lbEn: True, testedInstr: "ror.w" }
- { code: "sram", lbEn: True, testedInstr: "rors.w" }
- { code: "flash", lbEn: True, testedInstr: "rors.n" }
- { code: "flash", lbEn: True, testedInstr: "ror.w" }
- { code: "flash", lbEn: True, testedInstr: "rors.w" }
- { code: "flash", lbEn: False, testedInstr: "rors.n" }
- { code: "flash", lbEn: False, testedInstr: "ror.w" }
- { code: "flash", lbEn: False, testedInstr: "rors.w" }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    ldr.w r6, =#0xf7f7b5b5
    ldr.w r8, =#0x77f7b5b4  @ changed youngest & oldest bits

    b.w    tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}

{# "too big" shifts are defined and should be tested, too #}
{% set shiftRange = 34 %}

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for testedValue in ["r6", "r8"] %}
{% for shift in range(shiftRange) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare values
    mov.w r5, #{{shift}}
    mov.w r7, {{testedValue}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{testedInstr}} r7, r7, r5

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
    {{saveResult(r7, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr
{% endblock %}
