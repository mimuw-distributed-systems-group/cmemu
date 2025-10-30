---
name: Impact of load operations on execute- and decode-time branches.
description: >
    Tests impact of LDR, LDM, LDMDB, and LDRD operations
    with and without writebacks on execute- and decode-time branches.
dumped_symbols:
  # 27 cases * 2 (branch or not)
  times: 54 words
  flags: 54 words
  cpicnts: 54 words
  lsucnts: 54 words
configurations:
- { code: "gpram", lbEn: true }
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set memory = "sram" if code != "sram" else "gpram" %}
{% set memory_cell = 'memory_cell' %}


@ Test whether register dependency and writebacks have impact
@ on "conditional" decode-time branches.
@ Test is "load op; mov.n pc, lr".
@ r4 holds base register. r6 & r7 are "sink registers" -
@ they can be freely overwritten.
@ Note: `mov pc, lr` is favored, because it does not change instruction set.

{% set test_cases = [] %}

@ LDR
{% set test_cases = test_cases + [
    "ldr.w r6, [r4];",
    "ldr.w lr, [r4];",
    "ldr.w r6, [lr];",

    "ldr.w r6, [r4, #4]!;",
    "ldr.w lr, [r4, #4]!;",
    "ldr.w r6, [lr, #4]!;",

    "ldr.w r6, [r4], #4;",
    "ldr.w lr, [r4], #4;",
    "ldr.w r6, [lr], #4;",
] %}
@ LDRD
{% set test_cases = test_cases + [
    "ldrd.w r6, r7, [r4];",
    "ldrd.w r6, r7, [lr];",

    "ldrd.w r6, r7, [r4, #8]!;",
    "ldrd.w r6, r7, [lr, #8]!;",

    "ldrd.w r6, r7, [r4], #8;",
    "ldrd.w r6, r7, [lr], #8;",
] %}
@ LDM
{% set test_cases = test_cases + [
    "ldm.w r4, {r6, r7};",
    "ldm.w lr, {r6, r7};",
    "ldm.w r4, {r6, lr};",

    "ldm.w r4!, {r6, r7};",
    "ldm.w lr!, {r6, r7};",
    "ldm.w r4!, {r6, lr};",
] %}
@ LDMDB
{% set test_cases = test_cases + [
    "ldmdb.w r4, {r6, r7};",
    "ldmdb.w lr, {r6, r7};",
    "ldmdb.w r4, {r6, lr};",

    "ldmdb.w r4!, {r6, r7};",
    "ldmdb.w lr!, {r6, r7};",
    "ldmdb.w r4!, {r6, lr};",
] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    {% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
        mov.w r1, {{counter}}
        ldr.w r9, ={{save_func}}

        bl.w tested_code
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov.w r11, lr
{% for instr in test_cases %}
{% set case_no = loop.index %}
{% for do_branch in [False, True] %}
{% set branch_label = "case_label_{}_{}".format(case_no, do_branch) %}
    @ Prepare registers
    adr.w r5, {{branch_label}}
    bl.w initialize
    mov.w lr, r5

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    ldr.w  r8, [r0, r1]

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, r2

    @ Execute instructions
    {{instr}}
    {{"mov.n pc, lr" if do_branch else "nop.n"}}
    .align 3  @ LDMDB might decrement LR, so align beforehand
    nop.w; nop.w; @ Keeps alignment to 8 bytes
{{branch_label}}:
    nop.w; nop.w;

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, r2

    @ Get finish counter value
    ldr.w  r12, [r0, r1]

    blx.n r9

{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r11

.align 2
initialize:
    @ Clear flags
    mov.w r10, #0
    msr.w apsr_nzcvq, r10

    @ Prepare reg with zero value
    mov.w r2, #0

    @ Load address of memory cell
    ldr.w r4, ={{memory_cell}}

    @ Write address to the memory, in case it's going to be read from memory.
    str.w r5, [r4, #-8]
    str.w r5, [r4, #-4]
    str.w r5, [r4, #0]
    str.w r5, [r4, #4]

    bx.n lr

.align 2
.thumb_func
save_time_and_flags:
    mrs.w r2, apsr
    sub.w r8, r12, r8

    {{saveValue("times", r8, r10, r3)}}
    {{saveValue("flags", r2, r10, r3)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r8, r12, r8
    and.w r8, r8, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r8, r10, r3)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r8, r12, r8
    and.w r8, r8, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r8, r10, r3)}}

    bx.n lr


{{ section(memory) }}
.align 3
    .rept 4; .word 0x0; .endr
{{memory_cell}}:
    .rept 4; .word 0x0; .endr

{% endblock %}
