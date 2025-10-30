---
name: LDR from dwt tests
description: "Timing test of LDR, when doing consecutive accesses to DWT"
dumped_symbols:
  times: 4 words # 4 (nopsRange)
  flags: 4 words
  cpicnts: 4 words
  lsucnts: 4 words
configurations:
- { code: gpram, lbEn: True }
- { code: sram, lbEn: True }
- { code: flash, lbEn: True }
- { code: flash, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set nopsRange = 4 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w  r9, {{counter}}
    ldr.w  r10, ={{save_func}}

    bl.w   tested_code
{% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r11, lr
{% for nopsCount in range(nopsRange) %}
    @ Clean flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    {% for _ in range(nopsCount) %}
    nop.n
    {% endfor %}
    isb.w

    @ Get start counter value
    ldr.n  r2, [r0, r1]

    @ Get finish counter value
    ldr.n  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
    @ Return to counters loop
    bx.n r11

.align 2
.thumb_func
save_times_and_flags:
    sub.w r2, r3, r2
    mrs.w r5, apsr

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r5, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr


{% endblock %}
