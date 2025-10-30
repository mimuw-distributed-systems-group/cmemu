---
name: MOV.W ThumbExpandImm_C tests
description: >-
    Timing and correctness test of mov.w instruction
    with various immediate values (using ThumbExpandImm_C from [ARM-ARM]).
dumped_symbols:
  results: 28 words
  times: 28 words
  flags: 28 words
configurations:
- { code: "sram", lbEn: True }
- { code: "flash", lbEn: True }
- { code: "flash", lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set imms = [
    '0000_0000_0000_0000_0000_0000_1110_1011' | int(base=2),
    '0000_0000_1110_1011_0000_0000_1110_1011' | int(base=2),
    '1110_1011_0000_0000_1110_1011_0000_0000' | int(base=2),
    '1110_1011_1110_1011_1110_1011_1110_1011' | int(base=2),
    '00000000000000000000000_11101011_0' | int(base=2),
    '0000000000000000000000_11101011_00' | int(base=2),
    '000000000000000000000_11101011_000' | int(base=2),
    '00000000000000000000_11101011_0000' | int(base=2),
    '0000000000000000000_11101011_00000' | int(base=2),
    '000000000000000000_11101011_000000' | int(base=2),
    '00000000000000000_11101011_0000000' | int(base=2),
    '0000000000000000_11101011_00000000' | int(base=2),
    '000000000000000_11101011_000000000' | int(base=2),
    '00000000000000_11101011_0000000000' | int(base=2),
    '0000000000000_11101011_00000000000' | int(base=2),
    '000000000000_11101011_000000000000' | int(base=2),
    '00000000000_11101011_0000000000000' | int(base=2),
    '0000000000_11101011_00000000000000' | int(base=2),
    '000000000_11101011_000000000000000' | int(base=2),
    '00000000_11101011_0000000000000000' | int(base=2),
    '0000000_11101011_00000000000000000' | int(base=2),
    '000000_11101011_000000000000000000' | int(base=2),
    '00000_11101011_0000000000000000000' | int(base=2),
    '0000_11101011_00000000000000000000' | int(base=2),
    '000_11101011_000000000000000000000' | int(base=2),
    '00_11101011_0000000000000000000000' | int(base=2),
    '0_11101011_00000000000000000000000' | int(base=2),
     '11101011_000000000000000000000000' | int(base=2),
] %}

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
{% for imm in imms %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    mov.w r6, {{imm}}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
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
