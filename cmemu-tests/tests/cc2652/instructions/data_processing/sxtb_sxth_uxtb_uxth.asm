---
name: SXTB/SXTH/UTXB/UTXH tests
description: "Timing and correctness test of SXTB/SXTH/UTXB/UTXH instructions"
dumped_symbols:
  results: 340 words
  times: 340 words
  flags: 340 words
configurations:
# STXB
- { code: sram, lbEn: True, testedInstr: "sxtb.w" }
- { code: flash, lbEn: True, testedInstr: "sxtb.w" }
- { code: flash, lbEn: False, testedInstr: "sxtb.w" }
- { code: sram, lbEn: True, testedInstr: "sxtb.n" }
- { code: flash, lbEn: True, testedInstr: "sxtb.n" }
- { code: flash, lbEn: False, testedInstr: "sxtb.n" }
# STXH
- { code: sram, lbEn: True, testedInstr: "sxth.w" }
- { code: flash, lbEn: True, testedInstr: "sxth.w" }
- { code: flash, lbEn: False, testedInstr: "sxth.w" }
- { code: sram, lbEn: True, testedInstr: "sxth.n" }
- { code: flash, lbEn: True, testedInstr: "sxth.n" }
- { code: flash, lbEn: False, testedInstr: "sxth.n" }
# UTXB
- { code: sram, lbEn: True, testedInstr: "uxtb.w" }
- { code: flash, lbEn: True, testedInstr: "uxtb.w" }
- { code: flash, lbEn: False, testedInstr: "uxtb.w" }
- { code: sram, lbEn: True, testedInstr: "uxtb.n" }
- { code: flash, lbEn: True, testedInstr: "uxtb.n" }
- { code: flash, lbEn: False, testedInstr: "uxtb.n" }
# UTXH
- { code: sram, lbEn: True, testedInstr: "uxth.w" }
- { code: flash, lbEn: True, testedInstr: "uxth.w" }
- { code: flash, lbEn: False, testedInstr: "uxth.w" }
- { code: sram, lbEn: True, testedInstr: "uxth.n" }
- { code: flash, lbEn: True, testedInstr: "uxth.n" }
- { code: flash, lbEn: False, testedInstr: "uxth.n" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set registerValues = ["0x0000ffff", "0x000ffff0", "0x00ffff00", "0x0ffff000", "0xffff0000", "0x00005835", "0x0005d850", "0x005d3800", "0x05d38000", "0x05823500", "0x03020581", "0x720f0350", "0x05003f01", "0x230f0300", "0x70050f40", "0xff050f40", "0x700fff40"] %}
{% set rotations = [0, 8, 16, 24] if testedInstr[-1] == "w" else [0] %}

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
    mov.w r8, 0    @ current index
    ldr.w r9, =values

loop_begin:
    cmp.w r8, {{ registerValues | length }}
    bne.n loop_body
    b.w end_label

loop_body:
    @ Prepare input value
    ldr.w r6, [r9, r8, LSL 2]

{% for rot in rotations %}
{% for reps in range(1, 6) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for rep in range(reps) %}
        {{testedInstr}} r7, r6, ROR #{{rot}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}
{% endfor %}

    @ Increase counter
    add.w r8, 1
    b.w loop_begin

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r7, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

{{ section("flash") }}

.align 4
values:
{% for val in registerValues %}
    .word {{ val }}
{% endfor %}

{% endblock %}
