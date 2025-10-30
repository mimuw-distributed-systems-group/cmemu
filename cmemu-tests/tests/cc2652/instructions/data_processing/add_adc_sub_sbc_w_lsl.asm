---
name: ADD/ADC/SUB/SBC LSL tests
description: "Timing and correctness test of add.w/adc.w/sub.w/sbc.w with LSL"
dumped_symbols:
  results: 32 words
  times: 32 words
  flags: 32 words
configurations:
# add tests
- { code: "sram", lbEn: True, instr: "add.w", srcReg: "r6" }
- { code: "sram", lbEn: True, instr: "adds.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "add.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "adds.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "add.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "adds.w", srcReg: "r6" }
# add with stack pointer tests
- { code: "sram", lbEn: True, instr: "add.w", srcReg: "sp" }
- { code: "sram", lbEn: True, instr: "adds.w", srcReg: "sp" }
- { code: "flash", lbEn: True, instr: "add.w", srcReg: "sp" }
- { code: "flash", lbEn: True, instr: "adds.w", srcReg: "sp" }
- { code: "flash", lbEn: False, instr: "add.w", srcReg: "sp" }
- { code: "flash", lbEn: False, instr: "adds.w", srcReg: "sp" }
# adc tests
- { code: "sram", lbEn: True, instr: "adc.w", srcReg: "r6" }
- { code: "sram", lbEn: True, instr: "adcs.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "adc.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "adcs.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "adc.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "adcs.w", srcReg: "r6" }
# sub tests
- { code: "sram", lbEn: True, instr: "sub.w", srcReg: "r6" }
- { code: "sram", lbEn: True, instr: "subs.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "sub.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "subs.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "sub.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "subs.w", srcReg: "r6" }
# sub with stack pointer tests
- { code: "sram", lbEn: True, instr: "sub.w", srcReg: "sp" }
- { code: "sram", lbEn: True, instr: "subs.w", srcReg: "sp" }
- { code: "flash", lbEn: True, instr: "sub.w", srcReg: "sp" }
- { code: "flash", lbEn: True, instr: "subs.w", srcReg: "sp" }
- { code: "flash", lbEn: False, instr: "sub.w", srcReg: "sp" }
- { code: "flash", lbEn: False, instr: "subs.w", srcReg: "sp" }
# sbc tests
- { code: "sram", lbEn: True, instr: "sbc.w", srcReg: "r6" }
- { code: "sram", lbEn: True, instr: "sbcs.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "sbc.w", srcReg: "r6" }
- { code: "flash", lbEn: True, instr: "sbcs.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "sbc.w", srcReg: "r6" }
- { code: "flash", lbEn: False, instr: "sbcs.w", srcReg: "r6" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set shiftRange = 32 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    @ Store sp value
    ldr.w  r3, =sp_store
    str.w  sp, [r3]

    @ Prepare input values
    mov.w  r6, #0xaf000000
    mov.w  {{srcReg}}, r6
    mov.w  r7, #0x0000002c

    b.w    tested_code
.thumb_func
end_label:
    @ Revert sp value
    ldr.w  r3, =sp_store
    ldr.w  sp, [r3]
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for shift in range(shiftRange) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{instr}} r8, {{srcReg}}, r7, LSL #{{shift}}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r8, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

{{ section("sram") }}
.align 4
sp_store: .word 0x0

{% endblock %}
