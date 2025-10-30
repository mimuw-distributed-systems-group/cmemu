---
name: Verify Flash Line Buffer timings correctness.
description: >
    We will observe how fast flash delivers next words from memory
    by executing wide `ADD` instructions.
dumped_symbols:
  results: 16 words
  times: 16 words
  flags: 16 words
  cpicnts: 16 words
  lsucnts: 16 words
configurations:
- { lbEn: False }
- { lbEn: True }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 16 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r1, {{counter}}
    ldr.w r10, ={{save_func}}

    bl.w    tested_code
{% endfor %}
{% endblock %}

{% block after %}
{{ section("flash") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r11, lr
{% for reps in range(repetitions) %}
    @ Prepare input values
    mov.w  r6, #0

    @ Clear flags
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        add.w r6, #1
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
    @ Return to counters loop
    bx.n r11

.align 2
.thumb_func
save_time_flags_and_result:
    sub.w r2, r3, r2
    mrs.w r5, apsr

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
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
