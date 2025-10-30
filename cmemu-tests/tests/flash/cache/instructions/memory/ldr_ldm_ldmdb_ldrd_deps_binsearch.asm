---
name: Dependencies for load operations.
description: >
  Tests dependencies of LDR, LDM, LDMDB, and LDRD operations
  with and without writebacks.

  Tested instruction is followed by ADD (no AGU phase)
  or LDR (with AGU phase).
  Decode-time branches are going to be tested separately.
dumped_symbols:
  # 6 repetitions * ((152 test cases // 3 minimal number of parts) + 2 remainder (152-152//3*3))
  times: 312 words
  flags: 312 words
  cpicnts: 312 words
  lsucnts: 312 words
configurations:
- { code: "flash", memory: "sram", lbEn: False, part: 0, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set memory_cell = 'memory_cell' %}
{% set repetitions = 6 %}

{% set ns = namespace(test_cases = []) %}

@ Test dependency on every possible register: base, target, and independent.
@ Should show when following instruction is stalled.
@ Note: `ADD (reg)` is used, so we don't have to worry `ADD RD, #0`
@       would be treated as NOP and optimized.
@       r2 holds #0. r3 is trash register and can be modified.
@ Note: `movs.n` & `adds.n` used, so the code takes less space.
@ Note: `movs.n` is used to restore original base address register value
@       that was changed by writeback.
{% for instr in ["adds.n {reg}, r2;"] %}
    @ LDM.W
    {% set ns.test_cases = ns.test_cases + [
        ["add.w r5, #1;", instr.format(reg = "r6")],

        ["add.w r5, #1;", instr.format(reg = "r4")],
        ["add.w r5, #1;", instr.format(reg = "r5")],
        ["add.w r5, #1;", instr.format(reg = "r6")],
        ["add.w r5, #1;", instr.format(reg = "r7")],

        ["add.w r5, #1;", instr.format(reg = "r4"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r6"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r6"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r7, r5;"],
    ] %}
    @ LDMDB
    {% set ns.test_cases = ns.test_cases + [
        ["add.w r5, #1;", instr.format(reg = "r4")],
        ["add.w r5, #1;", instr.format(reg = "r5")],
        ["add.w r5, #1;", instr.format(reg = "r6")],

        ["add.w r5, #1;", instr.format(reg = "r4")],
        ["add.w r5, #1;", instr.format(reg = "r5")],
        ["add.w r5, #1;", instr.format(reg = "r6")],
        ["add.w r5, #1;", instr.format(reg = "r7")],

        ["add.w r5, #1;", instr.format(reg = "r4"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r6"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r5, r6;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r6"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r7, r5;"],
    ] %}
{% endfor %}
{% for instr in ["adds.n r3, #1;"] %}
    @ LDR
    {% set ns.test_cases = ns.test_cases + [
        ["adds.n r5, #1;", instr.format(reg = "r4")],
        ["adds.n r5, #1;", instr.format(reg = "r5")],
        ["adds.n r5, #1;", instr.format(reg = "r7")],

        ["add.w r5, #1;", instr.format(reg = "r4")],
        ["add.w r5, #1;", instr.format(reg = "r5")],
        ["add.w r5, #1;", instr.format(reg = "r7")],

        ["add.w r5, #1;", instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r4"), instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r4"), instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), instr.format(reg = "r5"), "movs.n r7, r5;"],

        ["add.w r5, #1;", instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r4"), instr.format(reg = "r5"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r4"), instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r5"), instr.format(reg = "r7"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), instr.format(reg = "r4"), "movs.n r7, r5;"],
        ["add.w r5, #1;", instr.format(reg = "r7"), instr.format(reg = "r5"), "movs.n r7, r5;"],
    ] %}
{% endfor %}

{% set test_cases_len = ns.test_cases | length %}
{% set test_parts = {"gpram": 5, "sram": 5, "flash": 1}[code] %}
{% if 0 <= part < test_parts %}
    {% set part_len = test_cases_len // test_parts %}
    @ None for last element to include the remaining few elements.
    {% set ns.test_cases = ns.test_cases[part_len * part : (part_len * (part+1) if part < test_parts - 1 else none)] %}
{% else %}
    {% set ns.test_cases = panic("invalid part") %}
{% endif %}

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
{% for case in ns.test_cases %}
{% set case_len = case | length %}
{% for instrs_used in range(1, 1 + repetitions) %}
    @ Prepare registers
    bl.w initialize

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    ldr.w  r8, [r0, r1]

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, r2

    @ Execute instructions
    {% for i in range(instrs_used) %}
        {{case[i % case_len]}}
    {% endfor %}

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

    @ Load address of memory cell
    ldr.w r4, ={{memory_cell}}
    mov.w r5, r4
    mov.w r6, r4
    mov.w r7, r4

    @ Prepare reg with zero value
    mov.w r2, #0

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
    .rept 4
    .word {{memory_cell}}
    .endr
{{memory_cell}}:
    .rept 4
    .word {{memory_cell}}
    .endr

{% endblock %}
