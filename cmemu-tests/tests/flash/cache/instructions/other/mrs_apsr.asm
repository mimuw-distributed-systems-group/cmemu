---
name: Reads APSR with MRS
description: >
    Timing and correctness test of "mrs rX, apsr" instruction.

    TODO: our processor doesn't support operations with saturation,
          however we should check whether Q flag could be written using msr,
          and if it could, we should zero it in this test.
dumped_symbols:
  results: 100 words
  times: 100 words
configurations:
- { code: "gpram", lbEn: true }
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{# helper constants #}
{% set m1 = 'FFFFFFFF'|int(base=16) %}
{% set i32min = '80000000'|int(base=16) %}
{# set i32max = i32min - 1  # not useful until we support mvn instruction #}
{% set i32big = '70000000'|int(base=16) %}

{# jinja2 sadly doesn't support inline comments, so we're naming the cases #}
{# note: not all cases are possible (i.e. NZxx), some might be hacked (i.e. `and` doesn't set V) #}
{% set case_nzcv = ('subs.n', 0, m1) %}
{% set case_nzCv = ('subs.n', 1, 0) %}
{% set case_nzCV = ('adds.n', i32min, m1) %}
{% set case_nZcv = ('adds.n', 0, 0) %}
{% set case_nZCv = ('subs.n', 0, 0) %}
{% set case_nZCV = ('adds.n', i32min, i32min) %}
{% set case_Nzcv = ('subs.n', 0, 1) %}
{% set case_NzcV = ('adds.n', i32big, i32big) %}
{% set case_NzCv = ('subs.n', m1, 0) %}
{% set cmp_vals = [
    case_nzcv,
    case_nzCv,
    case_nzCV,
    case_nZcv,
    case_nZCv,
    case_nZCV,
    case_Nzcv,
    case_NzcV,
    case_NzCv,
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

{% for setFlagsInstr, lval, rval in cmp_vals %}
    @ Prepare input values
    mov.w r5, #{{lval}}
    mov.w r6, #{{rval}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{setFlagsInstr}} r7, r5, r6  @ Note: r7 is trash register
    mrs.w r8, apsr

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(r8, r3, r4)}}

    bx.n lr
{% endblock %}
