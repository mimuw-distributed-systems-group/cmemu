---
name: Test fetch queue content after memory conflicts
description: >
    Test consists of hand-crafted sequence of LDR instructions with different memory types.
    Those LDRs are followed by ADD instructions, to determine when queue will be emptied.
dumped_symbols:
  results: 22 words
  times: 22 words
  flags: 22 words
  cpicnts: 22 words
  lsucnts: 22 words
configurations:
- { code: "gpram", addr1: "sram", addr2: "sram" }
- { code: "gpram", addr1: "gpram", addr2: "sram" }
- { code: "gpram", addr1: "sram", addr2: "gpram" }
- { code: "gpram", addr1: "gpram", addr2: "gpram" }
- { code: "sram", addr1: "sram", addr2: "sram" }
- { code: "sram", addr1: "sram", addr2: "gpram" }
- { code: "sram", addr1: "gpram", addr2: "sram" }
- { code: "sram", addr1: "gpram", addr2: "gpram" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{# Three blocks of instructions:
    * Batch of LDRs to addr1, starting with request to DWT counter
    * Batch of LDRs to addr1, with one request to addr2
    * Adds to check fetch queue status, first one is narrow to compensate for odd number of LDR.N
#}
{% set instruction_list = [
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",

    "ldr.w r5, [r6]",
    "ldr.n r5, [r4]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",
    "ldr.n r5, [r6]",

    "adds.n r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0",
    "adds.w r5, #0"
] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr input values
    ldr.w r4, =addr2_memory_cell
    ldr.w r6, =addr1_memory_cell
    mov.w r5, #0    @ Just to have a predictable starting state

{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r1, {{counter}}
    ldr.w r10, ={{save_func}}

    bl.w    tested_code
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
{% for reps in range(instruction_list|length) %}
    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{instruction_list[i]}}
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
    @ Return to counters loop
    bx.n r11

.align 4
.thumb_func
save_time_flags_and_result:
    sub.w r2, r3, r2
    mrs.w r7, apsr

    {{saveValue("times", r2, r3, r8)}}
    {{saveValue("results", r5, r3, r8)}}
    {{saveValue("flags", r7, r3, r8)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r8)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r8)}}

    bx.n lr

{{ section(addr1) }}
.align 4
addr1_memory_cell: .word 42

{{ section(addr2) }}
.align 4
addr2_memory_cell: .word 21
{% endblock %}
