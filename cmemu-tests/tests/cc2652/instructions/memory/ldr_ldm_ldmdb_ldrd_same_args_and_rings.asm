---
name: Two extra scenarios for load operations.
description: >
    Tests LDR, LDM, LDMDB, and LDRD operations on two extra scenarios.
    For better coverage, the instructions are mixed with each other,
    and have "modifiers" like writeback and register dependency.

    One scenario is operating on load operations where "rT = rN"
    (one of loaded registers is also the base register).

    Second scenario is operating on a "ring"/cycle.
    "i-th instruction writes base address of (i-1 % ring size)-instruction".
    For ring of size 2, instruction depends on output of previous instruction,
    while for size 3, it depends on result of the instruction before
    previous instruction.
dumped_symbols:
  # 6 repetitions * 142 test cases / 2 minimal number of parts
  times: 426 words
  flags: 426 words
  cpicnts: 426 words
  lsucnts: 426 words
configurations:

- { code: "sram", memory: "sram", lbEn: true, part: 0 }
- { code: "sram", memory: "sram", lbEn: true, part: 1 }
- { code: "sram", memory: "sram", lbEn: true, part: 2 }
- { code: "sram", memory: "sram", lbEn: true, part: 3 }
- { code: "sram", memory: "sram", lbEn: true, part: 4 }
- { code: "sram", memory: "flash", lbEn: true, part: 0 }
- { code: "sram", memory: "flash", lbEn: true, part: 1 }
- { code: "sram", memory: "flash", lbEn: true, part: 2 }
- { code: "sram", memory: "flash", lbEn: true, part: 3 }
- { code: "sram", memory: "flash", lbEn: true, part: 4 }
- { code: "sram", memory: "flash", lbEn: false, part: 0 }
- { code: "sram", memory: "flash", lbEn: false, part: 1 }
- { code: "sram", memory: "flash", lbEn: false, part: 2 }
- { code: "sram", memory: "flash", lbEn: false, part: 3 }
- { code: "sram", memory: "flash", lbEn: false, part: 4 }

- { code: "flash", memory: "sram", lbEn: true, part: 0 }
- { code: "flash", memory: "sram", lbEn: true, part: 1 }
- { code: "flash", memory: "sram", lbEn: false, part: 0 }
- { code: "flash", memory: "sram", lbEn: false, part: 1 }
- { code: "flash", memory: "flash", lbEn: true, part: 0 }
- { code: "flash", memory: "flash", lbEn: true, part: 1 }
- { code: "flash", memory: "flash", lbEn: false, part: 0 }
- { code: "flash", memory: "flash", lbEn: false, part: 1 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set memory_cell = 'memory_cell' %}
{% set repetitions = 6 %}

{% set ns = namespace(test_cases = []) %}

@ "rt = rn" with narrow LDR & LDM.
@ Two cases: with and without register dependencies.
{% set ns.test_cases = ns.test_cases + [
    ["ldr.n r5, [r5];"],
    ["ldm.n r5, {r5};"],
    ["ldr.n r5, [r5];", "ldr.n r6, [r6];"],
    ["ldm.n r5, {r5};", "ldm.n r6, {r6};"],
    ["ldr.n r5, [r5];", "ldm.n r5, {r5};"],
    ["ldr.n r5, [r5];", "ldm.n r6, {r6};"],
    ["ldr.n r5, [r5];", "ldr.n r5, [r5];", "ldm.n r5, {r5};"],
    ["ldr.n r5, [r5];", "ldr.n r6, [r6];", "ldm.n r7, {r7};"],
    ["ldr.n r5, [r5];", "ldm.n r5, {r5};", "ldm.n r5, {r5};"],
    ["ldr.n r5, [r5];", "ldm.n r6, {r6};", "ldm.n r7, {r7};"],
    ["ldr.n r5, [r5];", "ldr.n r5, [r5];", "ldm.n r5, {r5};", "ldm.n r5, {r5};"],
    ["ldr.n r5, [r5];", "ldr.n r6, [r6];", "ldm.n r5, {r5};", "ldm.n r6, {r6};"],
] %}

@ "rt = rn", but LDM & LDRD have more arguments.
@ Two cases: with and without register dependencies.
{% set ns.test_cases = ns.test_cases + [
    ["ldr.n r5, [r5];", "ldm.n r5, {r4, r5};"],
    ["ldr.n r5, [r5];", "ldm.n r5, {r5, r7};"],

    ["ldr.n r5, [r5];", "ldm.n r6,  {r4, r6};"],
    ["ldr.n r5, [r5];", "ldm.n r6,  {r6, r7};"],
    
    ["ldr.n r5, [r5];", "ldm.w r5, {r4, r5};"],
    ["ldr.n r5, [r5];", "ldm.w r5, {r5, r7};"],

    ["ldr.n r5, [r5];", "ldm.w r6,  {r4, r6};"],
    ["ldr.n r5, [r5];", "ldm.w r6,  {r6, r7};"],

    ["ldr.n r5, [r5];", "ldrd.w r4, r5, [r5];"],
    ["ldr.n r5, [r5];", "ldrd.w r5, r4, [r5];"],

    ["ldr.n r5, [r5];", "ldrd.w r4, r6, [r6];"],
    ["ldr.n r5, [r5];", "ldrd.w r6, r4, [r6];"],

    ["ldm.n r5, {r4, r5};", "ldm.n r5,  {r4, r5};"],
    ["ldm.n r5, {r4, r5};", "ldm.n r5,  {r5, r7};"],
    ["ldm.n r5, {r4, r5};", "ldm.w r5,  {r4, r5};"],
    ["ldm.n r5, {r4, r5};", "ldm.w r5,  {r5, r7};"],
    ["ldm.w r5, {r4, r5};", "ldm.w r5,  {r4, r5};"],
    ["ldm.w r5, {r4, r5};", "ldm.w r5,  {r5, r7};"],
    ["ldm.w r5, {r4, r5};", "ldm.n r5,  {r4, r5};"],
    ["ldm.w r5, {r4, r5};", "ldm.n r5,  {r5, r7};"],

    ["ldm.n r5, {r4, r5};", "ldm.n r6,  {r4, r6};"],
    ["ldm.n r5, {r4, r5};", "ldm.n r6,  {r6, r7};"],
    ["ldm.n r5, {r4, r5};", "ldm.w r6,  {r4, r6};"],
    ["ldm.n r5, {r4, r5};", "ldm.w r6,  {r6, r7};"],
    ["ldm.w r5, {r4, r5};", "ldm.w r6,  {r4, r6};"],
    ["ldm.w r5, {r4, r5};", "ldm.w r6,  {r6, r7};"],
    ["ldm.w r5, {r4, r5};", "ldm.n r6,  {r4, r6};"],
    ["ldm.w r5, {r4, r5};", "ldm.n r6,  {r6, r7};"],

    ["ldmdb.w r5, {r4, r5};", "ldm.w r5, {r4, r5};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.w r5, {r5, r7};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.w r5, {r4, r5};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.w r5, {r5, r7};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.n r5, {r4, r5};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.n r5, {r5, r7};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.n r5, {r4, r5};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.n r5, {r5, r7};"],

    ["ldmdb.w r5, {r4, r5};", "ldm.w r6, {r4, r6};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.w r6, {r6, r7};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.w r6, {r4, r6};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.w r6, {r6, r7};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.n r6, {r4, r6};"],
    ["ldmdb.w r5, {r4, r5};", "ldm.n r6, {r6, r7};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.n r6, {r4, r6};"],
    ["ldmdb.w r5, {r5, r7};", "ldm.n r6, {r6, r7};"],

    ["ldm.w r5, {r4, r5};", "ldrd.w r4, r5, [r5];"],
    ["ldm.w r5, {r4, r5};", "ldrd.w r5, r7, [r5];"],

    ["ldm.w r5, {r5, r7};", "ldrd.w r4, r5, [r5];"],
    ["ldm.w r5, {r5, r7};", "ldrd.w r5, r7, [r5];"],

    ["ldm.w r5, {r5, r7};", "ldrd.w r4, r6, [r6];"],
    ["ldm.w r5, {r5, r7};", "ldrd.w r6, r7, [r6];"],

    ["ldrd.w r4, r5, [r5]"],
    ["ldrd.w r4, r5, [r5]", "ldrd.w r4, r6, [r6]"],

    ["ldm.n r5, {r4, r5};"],
    ["ldm.n r5, {r5, r6};"],
    ["ldm.n r5, {r4, r5};", "ldm.n r6, {r4, r6}"],

    ["ldm.w r5, {r4, r5};"],
    ["ldm.w r5, {r5, r6};"],
    ["ldm.w r5, {r4, r5};", "ldm.w r6, {r4, r6}"],

    ["ldmdb.w r5, {r4, r5};"],
    ["ldmdb.w r5, {r5, r6};"],
    ["ldmdb.w r5, {r4, r5};", "ldmdb.w r6, {r4, r6}"],
] %}

@ Some extra cases with writebacks
{% set ns.test_cases = ns.test_cases + [
    ["ldr.w r6, [r5, #4]!;", "ldr.w r6, [r5, #-4]!;"],
    ["ldr.w r6, [r5, #4]!;", "ldr.w r6, [r5], #-4;"],
    ["ldr.w r6, [r5], #4;",  "ldr.w r6, [r5], #-4;"],

    ["ldrd.w r4, r6, [r5, #8]!;", "ldrd.w r4, r6, [r5, #-8]!;"],
    ["ldrd.w r4, r6, [r5, #8]!;", "ldrd.w r4, r6, [r5], #-8;"],
    ["ldrd.w r4, r6, [r5], #8;",  "ldrd.w r4, r6, [r5], #-8;"],
] %}

@ Ring/cycle of size 2.
@ Mixing LDM & LDRD, with & without writeback.
{% set ns.test_cases = ns.test_cases + [
    ["ldr.n r6, [r5];", "ldr.n r5, [r6];"],
    ["ldr.w r6, [r5, #4]!;", "ldr.w r5, [r6, #4]!;"],
    ["ldr.w r6, [r5], #4;",  "ldr.w r5, [r6], #4;"],
    ["ldm.n r5!, {r6};", "ldm.n r6!, {r5};"],

    ["ldr.w r6, [r5, #4]!;", "ldr.w r5, [r6], #4;"],
    ["ldr.n r6, [r5];",      "ldm.n r6!, {r5};"],
    ["ldr.w r6, [r5, #4]!;", "ldm.n r6!, {r5};"],
    ["ldr.w r6, [r5], #4;",  "ldm.n r6!, {r5};"],
] %}

@ As above, but with LDMDB & multiple args (depending on earlier & later loaded register).
{% set ns.test_cases = ns.test_cases + [
    ["ldm.n r5!, {r4, r6};", "ldm.n r6!, {r5};"],
    ["ldm.n r5!, {r6, r7};", "ldm.n r6!, {r5};"],
    ["ldm.w r5!, {r4, r6};", "ldm.n r6!, {r5};"],
    ["ldm.w r5!, {r6, r7};", "ldm.n r6!, {r5};"],
    ["ldm.w r5,  {r4, r6};", "ldm.n r6!, {r5};"],
    ["ldm.w r5,  {r6, r7};", "ldm.n r6!, {r5};"],

    ["ldmdb.w r5!, {r4, r6};", "ldm.n r6!, {r5};"],
    ["ldmdb.w r5!, {r6, r7};", "ldm.n r6!, {r5};"],
    ["ldmdb.w r5,  {r4, r6};", "ldm.n r6!, {r5};"],
    ["ldmdb.w r5,  {r6, r7};", "ldm.n r6!, {r5};"],

    ["ldr.n r6, [r5];", "ldrd.w r4, r5, [r6]"],
    ["ldr.n r6, [r5];", "ldrd.w r5, r4, [r6]"],

    ["ldr.n r6, [r5];", "ldrd.w r4, r5, [r6, #8]!"],
    ["ldr.n r6, [r5];", "ldrd.w r4, r5, [r6], #8"],

    ["ldm.n r5!, {r4, r6};", "ldrd.w r4, r5, [r6]"],
    ["ldm.n r5!, {r6, r7};", "ldrd.w r4, r5, [r6]"],
    ["ldm.w r5!, {r4, r6};", "ldrd.w r4, r5, [r6]"],
    ["ldm.w r5!, {r6, r7};", "ldrd.w r4, r5, [r6]"],
    ["ldm.w r5,  {r4, r6};", "ldrd.w r4, r5, [r6]"],
    ["ldm.w r5,  {r6, r7};", "ldrd.w r4, r5, [r6]"],

    ["ldm.n r5!, {r4, r6};", "ldrd.w r5, r4, [r6]"],
    ["ldm.n r5!, {r6, r7};", "ldrd.w r5, r4, [r6]"],
    ["ldm.w r5!, {r4, r6};", "ldrd.w r5, r4, [r6]"],
    ["ldm.w r5!, {r6, r7};", "ldrd.w r5, r4, [r6]"],
    ["ldm.w r5,  {r4, r6};", "ldrd.w r5, r4, [r6]"],
    ["ldm.w r5,  {r6, r7};", "ldrd.w r5, r4, [r6]"],
] %}

@ Ring/cycle of size 3. Depending on value from 2 instructions ago.
@ Mixing as with ring/cycle of size 2 above.
{% set ns.test_cases = ns.test_cases + [
    ["ldr.n r5, [r6];", "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldr.w r5, [r6, #4]!;", "ldr.w r6, [r7, #4]!;", "ldr.w r7, [r5, #4]!;"],
    ["ldr.w r5, [r6], #4;",  "ldr.w r6, [r7], #4;",  "ldr.w r7, [r5], #4;"],
    ["ldm.n r6!, {r5};", "ldm.n r7!, {r6};", "ldm.n r5!, {r7};"],

    ["ldr.n r5, [r6];",  "ldr.w r6, [r7, #4]!;", "ldr.w r7, [r5], #4;"],
    ["ldr.n r5, [r6];",  "ldr.w r6, [r7, #4]!;", "ldm.n r5!, {r7};"],
    ["ldr.n r5, [r6];",  "ldr.w r6, [r7], #4;",  "ldm.n r5!, {r7};"],
    ["ldr.n r5, [r6];",  "ldm.n r7!, {r6};",     "ldm.n r5!, {r7};"],
    ["ldm.n r6!, {r5};", "ldr.w r6, [r7, #4]!;", "ldr.w r7, [r5], #4;"],
] %}

@ As above, but include multi-args ldm & ldmdb.
{% set ns.test_cases = ns.test_cases + [
    ["ldm.n r6!, {r4, r5};", "ldm.n r7!, {r4, r6};", "ldm.n r5!, {r4, r7};"],
    ["ldm.n r5!, {r4, r7};", "ldm.n r6!, {r5, r7};", "ldm.n r4!, {r6, r7};"],
    ["ldm.w r6!, {r4, r5};", "ldm.w r7!, {r4, r6};", "ldm.w r5!, {r4, r7};"],
    ["ldm.w r5!, {r4, r7};", "ldm.w r6!, {r5, r7};", "ldm.w r4!, {r6, r7};"],
    ["ldm.w r6, {r4, r5};",  "ldm.w r7, {r4, r6};",  "ldm.w r5, {r4, r7};"],
    ["ldm.w r5, {r4, r7};",  "ldm.w r6, {r5, r7};",  "ldm.w r4, {r6, r7};"],

    ["ldmdb.w r6!, {r4, r5};", "ldmdb.w r7!, {r4, r6};", "ldmdb.w r5!, {r4, r7};"],
    ["ldmdb.w r5!, {r4, r7};", "ldmdb.w r6!, {r5, r7};", "ldmdb.w r4!, {r6, r7};"],
    ["ldmdb.w r6, {r4, r5};",  "ldmdb.w r7, {r4, r6};",  "ldmdb.w r5, {r4, r7};"],
    ["ldmdb.w r5, {r4, r7};",  "ldmdb.w r6, {r5, r7};",  "ldmdb.w r4, {r6, r7};"],

    ["ldrd.w r4, r5, [r6];",      "ldrd.w r4, r6, [r7];",      "ldrd.w r4, r7, [r5];"],
    ["ldrd.w r4, r5, [r6, #8]!;", "ldrd.w r4, r6, [r7, #8]!;", "ldrd.w r4, r7, [r5, #8]!;"],
    ["ldrd.w r4, r5, [r6], #8;",  "ldrd.w r4, r6, [r7], #8;",  "ldrd.w r4, r7, [r5], #8;"],
    ["ldrd.w r4, r5, [r6];",      "ldrd.w r4, r6, [r7, #8]!;", "ldrd.w r4, r7, [r5], #8;"],

    ["ldrd.w r4, r5, [r6];",      "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldrd.w r5, r4, [r6];",      "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldrd.w r4, r5, [r6, #8]!;", "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldrd.w r5, r4, [r6, #8]!;", "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldrd.w r4, r5, [r6], #8;",  "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
    ["ldrd.w r5, r4, [r6], #8;",  "ldr.n r6, [r7];", "ldr.n r7, [r5];"],
] %}

{% set test_cases_len = ns.test_cases | length %}
{% set test_parts = {"gpram": 5, "sram": 5, "flash": 2}[code] %}
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
    mov r11, lr
{% for case in ns.test_cases %}
{% set case_len = case | length %}
{% for instrs_used in range(1, 1 + repetitions) %}
    @ Prepare registers
    bl.w initialize

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, r8

    @ Execute instructions
    {% for i in range(instrs_used) %}
        {{case[i % case_len]}}
    {% endfor %}

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, r8

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

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
    mov.w r8, #0

    bx.n lr

.align 2
.thumb_func
save_time_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r12)}}
    {{saveValue("flags", r8, r10, r12)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r10, r12)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r10, r12)}}

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
