---
name: SSAT/USAT tests
description: "Timing and correctness test of ssat and usat instructions"
dumped_symbols:
  results: 100 words
  times: 100 words
  flags: 100 words
configurations:
- { code: "sram", lbEn: True, tested_val: "0x01234567", testedInstr: "ssat.w" }
- { code: "sram", lbEn: True, tested_val: "0xfedcba98", testedInstr: "ssat.w" }
- { code: "sram", lbEn: True, tested_val: "0x000A0B0C", testedInstr: "ssat.w" }
- { code: "sram", lbEn: True, tested_val: "0x01234567", testedInstr: "usat.w" }
- { code: "sram", lbEn: True, tested_val: "0xfedcba98", testedInstr: "usat.w" }
- { code: "sram", lbEn: True, tested_val: "0x000A0B0C", testedInstr: "usat.w" }
- { code: "flash", lbEn: True, tested_val: "0x01234567", testedInstr: "ssat.w" }
- { code: "flash", lbEn: True, tested_val: "0xfedcba98", testedInstr: "ssat.w" }
- { code: "flash", lbEn: True, tested_val: "0x000A0B0C", testedInstr: "ssat.w" }
- { code: "flash", lbEn: True, tested_val: "0x01234567", testedInstr: "usat.w" }
- { code: "flash", lbEn: True, tested_val: "0xfedcba98", testedInstr: "usat.w" }
- { code: "flash", lbEn: True, tested_val: "0x000A0B0C", testedInstr: "usat.w" }
- { code: "flash", lbEn: False, tested_val: "0x01234567", testedInstr: "ssat.w" }
- { code: "flash", lbEn: False, tested_val: "0xfedcba98", testedInstr: "ssat.w" }
- { code: "flash", lbEn: False, tested_val: "0x000A0B0C", testedInstr: "ssat.w" }
- { code: "flash", lbEn: False, tested_val: "0x01234567", testedInstr: "usat.w" }
- { code: "flash", lbEn: False, tested_val: "0xfedcba98", testedInstr: "usat.w" }
- { code: "flash", lbEn: False, tested_val: "0x000A0B0C", testedInstr: "usat.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 3 %}
{% set tested_range = range(32) if testedInstr == 'usat.w' else range(1, 33) %}

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
{% for saturate_to in tested_range %}
{% for rep in range(1, repetitions) %}
    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Prepare input value
    ldr.w r7, ={{tested_val}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(rep) %}
        {{testedInstr}} r7, #{{saturate_to}}, r7
    {% endfor %}

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
