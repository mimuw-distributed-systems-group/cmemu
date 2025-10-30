---
name: LDR from two memories - check if there is visible line buffer effect
description: >
    Check for timing changes when accessing different or the same addresses on GPRAM.
    This allows to determine if the GPRAM line buffer exists.
dumped_symbols:
  times: 64 words # 16 (repetitions) * 2 (switchGPRAM) * 2 (switchFlash)
  flags: 64 words
  cpicnts: 64 words
  lsucnts: 64 words
configurations:
- { }
...
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 16 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt

    @ Prepare ldr input values
    ldr.w  r2, =cell_0a
    ldr.w  r3, =cell_0b
    ldr.w  r4, =cell_1a
    ldr.w  r5, =cell_1b


{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w  r9, {{counter}}
    ldr.w  r10, ={{save_func}}

    bl.w   tested_code
{% endfor %}
{% endblock %}

{% block after %}
{{ section("sram") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r11, lr
{% for switchGPRAM, switchFlash in [(True, True), (True, False), (False, True), (False, False)] %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w  r6, #0
    msr.w  apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r6, [r8, r9]

    {% for i in range(reps) %}
        ldr.n r0, [{% if switchGPRAM and i % 2 == 0 %}r2{% else %}r3{% endif %}]
        ldr.n r1, [{% if switchFlash and i % 2 == 0 %}r4{% else %}r5{% endif %}]
    {% endfor %}

    @ Get finish time
    ldr.w  r7, [r8, r9]

    @ Save test results
    blx.n  r10

{% endfor %}
{% endfor %}
    @ Return to counters loop
    bx.n  r11

.align 2
.thumb_func
save_times_and_flags:
    sub.w r6, r7, r6
    mrs.w r0, apsr

    {{saveValue("times", r6, r7, r1)}}
    {{saveValue("flags", r0, r7, r1)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r6, r7, r6
    and.w r6, r6, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r6, r7, r1)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r6, r7, r6
    and.w r6, r6, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r6, r7, r1)}}

    bx.n lr

{{ section("gpram") }}
.align 4
cell_0a: .word cell_0a
.align 4
cell_0b: .word cell_0b

{{ section("flash") }}
.align 4
cell_1a: .word cell_1a
.align 4
cell_1b: .word cell_1b

{% endblock %}
