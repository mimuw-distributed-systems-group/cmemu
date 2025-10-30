---
name: CMN/CMP/TEQ/TST tests
description: "Timing and correctness test of CMN, CMP, TEQ and TST"
dumped_symbols:
  times: 70 words
  flags: 70 words
configurations:
# CMN tests.
- { code: gpram, lbEn: True, testedInstr: "cmn.n", lreg: "r5", rreg: "r6" }
- { code: gpram, lbEn: True, testedInstr: "cmn.w", lreg: "r5", rreg: "r11" }
- { code: sram, lbEn: True, testedInstr: "cmn.n", lreg: "r5", rreg: "r6" }
- { code: sram, lbEn: True, testedInstr: "cmn.w", lreg: "r9", rreg: "r11" }
- { code: flash, lbEn: True, testedInstr: "cmn.n", lreg: "r6", rreg: "r5" }
- { code: flash, lbEn: True, testedInstr: "cmn.w", lreg: "r9", rreg: "r11" }
- { code: flash, lbEn: False, testedInstr: "cmn.n", lreg: "r6", rreg: "r5" }
- { code: flash, lbEn: False, testedInstr: "cmn.w", lreg: "r9", rreg: "r11" }

- { code: gpram, lbEn: True, testedInstr: "cmn.w", lreg: "r9", rreg: "imm" }
- { code: sram, lbEn: True, testedInstr: "cmn.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: True, testedInstr: "cmn.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: False, testedInstr: "cmn.w", lreg: "r9", rreg: "imm" }

# Registers specified to test all variants of CMP instruction. Both low registers will use T1, otherwise T2 variant.
# see [ARM-ARM] A7.7.28
- { code: gpram, lbEn: True, testedInstr: "cmp.n", lreg: "r5", rreg: "r6" }
- { code: gpram, lbEn: True, testedInstr: "cmp.w", lreg: "r5", rreg: "r11" }
- { code: sram, lbEn: True, testedInstr: "cmp.n", lreg: "r9", rreg: "r11" }
- { code: sram, lbEn: True, testedInstr: "cmp.w", lreg: "r5", rreg: "r6" }
- { code: flash, lbEn: True, testedInstr: "cmp.n", lreg: "r6", rreg: "r5" }
- { code: flash, lbEn: True, testedInstr: "cmp.w", lreg: "r9", rreg: "r11" }
- { code: flash, lbEn: False, testedInstr: "cmp.n", lreg: "r6", rreg: "r5" }
- { code: flash, lbEn: False, testedInstr: "cmp.w", lreg: "r9", rreg: "r11" }

- { code: gpram, lbEn: True, testedInstr: "cmp.n", lreg: "r5", rreg: "imm" }
- { code: gpram, lbEn: True, testedInstr: "cmp.w", lreg: "r9", rreg: "imm" }
- { code: sram, lbEn: True, testedInstr: "cmp.n", lreg: "r5", rreg: "imm" }
- { code: sram, lbEn: True, testedInstr: "cmp.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: True, testedInstr: "cmp.n", lreg: "r5", rreg: "imm" }
- { code: flash, lbEn: True, testedInstr: "cmp.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: False, testedInstr: "cmp.n", lreg: "r5", rreg: "imm" }
- { code: flash, lbEn: False, testedInstr: "cmp.w", lreg: "r9", rreg: "imm" }

# TEQ has only wide encodings, so fewer tests are considered.
# see [ARM-ARM] A7.7.186
- { code: gpram, lbEn: True, testedInstr: "teq.w", lreg: "r5", rreg: "r11" }
- { code: sram, lbEn: True, testedInstr: "teq.w", lreg: "r5", rreg: "r6" }
- { code: flash, lbEn: True, testedInstr: "teq.w", lreg: "r9", rreg: "r11" }
- { code: flash, lbEn: False, testedInstr: "teq.w", lreg: "r9", rreg: "r11" }
- { code: gpram, lbEn: True, testedInstr: "teq.w", lreg: "r9", rreg: "imm" }
- { code: sram, lbEn: True, testedInstr: "teq.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: True, testedInstr: "teq.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: False, testedInstr: "teq.w", lreg: "r9", rreg: "imm" }

# TST tests (same as in cmp tests, both low registers will use T1 encoding otherwise T2).
# see [ARM-ARM] A7.7.188/189
- { code: gpram, lbEn: True, testedInstr: "tst.n", lreg: "r5", rreg: "r6" }
- { code: gpram, lbEn: True, testedInstr: "tst.w", lreg: "r5", rreg: "r11" }
- { code: sram, lbEn: True, testedInstr: "tst.n", lreg: "r5", rreg: "r6" }
- { code: sram, lbEn: True, testedInstr: "tst.w", lreg: "r5", rreg: "r11" }
- { code: flash, lbEn: True, testedInstr: "tst.n", lreg: "r5", rreg: "r6" }
- { code: flash, lbEn: True, testedInstr: "tst.w", lreg: "r5", rreg: "r11" }
- { code: flash, lbEn: False, testedInstr: "tst.n", lreg: "r5", rreg: "r6" }
- { code: flash, lbEn: False, testedInstr: "tst.w", lreg: "r5", rreg: "r11" }

- { code: gpram, lbEn: True, testedInstr: "tst.w", lreg: "r9", rreg: "imm" }
- { code: sram, lbEn: True, testedInstr: "tst.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: True, testedInstr: "tst.w", lreg: "r9", rreg: "imm" }
- { code: flash, lbEn: False, testedInstr: "tst.w", lreg: "r9", rreg: "imm" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{# helper constants #}
{% set m1 = 'FFFFFFFF'|int(base=16) %}
{% set i32min = '80000000'|int(base=16) %}
{% set i32max = i32min - 1 %}
{% set i32big = '70000000'|int(base=16) %}
{% set i32small = 'F0000000'|int(base=16) %}

{# note: not all cases are possible (i.e. NZxx), some might be hacked (i.e. `and` doesn't set V) #}
{% if testedInstr[:3] == "cmn" %}
    {% set input_vals = [
        ("nzcv", 1, 1),
        ("nzCv", i32small, i32big),
        ("nzCV", m1, i32min),
        ("nZcv", 0, 0),
        ("nZCv", m1, 1),
        ("Nzcv", i32min, 1),
        ("NzcV", i32max, 1),
        ("NzCv", m1, m1),
    ] %}
{% elif testedInstr[:3] == "cmp" %}
    {% if rreg == "imm" %}
        {% set input_vals = [
            ("nzCv", 1, 0),
            ("nzCV", i32min, 1),
            ("nZCv", 0, 0),
            ("Nzcv", 0, 1),
            ("NzCv", m1, 0)
        ] %}
        {% if testedInstr == "cmp.w" %}
            {% set input_vals =  input_vals + [("nzcv", 0, m1), ("NzcV", i32big, i32small)] %}
        {% endif %}
    {% else %}
        {% set input_vals = [
            ("nzcv", 0, m1),
            ("nzCv", 1, 0),
            ("nzCV", i32min, 1),
            ("nZCv", 0, 0),
            ("Nzcv", 0, 1),
            ("NzcV", i32big, i32small),
            ("NzcV", i32max, i32small),
            ("nzCv", i32max, 0),
            ("NzCv", m1, 0)
        ] %}
    {% endif %}
{#
   For "teq" and "tst" instructions: shift is always set to 0 for
   register instructions, so `carry_in == carry_out == 1`. Because of that,
   cases with different carry flag aren't tested.
   ThumbExpandImm_C is used to set carry flag for immediate instrucions.
   It demands a big effort to find test cases for setting carry flag, so it
   hasn't been taken under consideration.
#}
{% elif testedInstr[:3] == "teq" %}
    {% set input_vals = [
        ("nzC", 1, 0),
        ("nzC", 0, 1),
        ("nzC", i32max, 0),
        ("nZC", 0, 0),
        ("NzC", 0, m1),
        ("NzC", i32min, 1),
        ("NzC", i32big, i32small),
        ("NzC", i32max, i32small),
        ("NzC", m1, 0)
    ] %}
{% elif testedInstr[:3] == "tst" %}
    {% set input_vals = [
        ("nzC", 1, 1),
        ("nZC", 0, 1),
        ("NzC", m1, m1),
    ] %}
{% else %}
    panic!
{% endif %}

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

{% for reps in range(1, 8) %}
{% for _, lval, rval in input_vals %}
    @ Clear flags
    mov.w r8, #0
    msr.w apsr_nzcvq, r8

    @ Prepare input values
    mov.w {{lreg}}, #{{lval}}
    {% if rreg != "imm" %}
        mov.w {{rreg}}, #{{rval}}
    {% endif %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {% if rreg == "imm" %}
            {{testedInstr}} {{lreg}}, #{{rval}}
        {% else %}
            {{testedInstr}} {{lreg}}, {{rreg}}
        {% endif %}
    {% endfor%}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label

save:
    mrs.w r8, apsr
    sub.w r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('flags', r8, r3, r4)}}

    bx.n lr
{% endblock %}
