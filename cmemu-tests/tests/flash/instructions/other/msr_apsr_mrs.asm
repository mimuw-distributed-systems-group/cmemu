---
name: Modifies APSR with MSR
description: >
    Timing and correctness test of "msr apsr_nzcvq, rX" instruction.

    TODO: add some tests of msr apsr which doesn't use mrs when we have
          stable it instruction or conditional branches
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

{# APSR flags are on 5 highest bits (i.e. 31..=27), here we iterate over #}
{# all possible APSR values (msr ignores lower bits so they may be 0)    #}
{% for flags in range(32) %}
    @ Prepare input values (poor man shift here, jinja doesn't support '<<')
    mov.w r8, #{{flags * 2**27}}

    @ Reset APSR nzcv flags to 0
    mov.w r5, #1
    adds.n r5, r5, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    @ Set tested flags
    msr.w  apsr_nzcvq, r8

    @ Test new flags
    mrs.w r7, apsr

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(r7, r3, r4)}}

    bx.n lr
{% endblock %}
