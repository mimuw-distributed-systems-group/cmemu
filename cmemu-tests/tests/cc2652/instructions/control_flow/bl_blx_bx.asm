---
name: BL/BLX+BX
description: "Timing test of `BL label; BX lr` and `BLX rm, BX lr` combinations"
dumped_symbols:
  results: 24 words
  times: 24 words
  flags: 24 words
  cpicnts: 24 words
  lsucnts: 24 words
configurations:
# bl tests
# mov lr
- { code: "sram", lbEn: true, instr: "bl.w", saveLRInstr: "mov.w  lr, r8" }
- { code: "flash", lbEn: false, instr: "bl.w", saveLRInstr: "mov.w  lr, r8" }
- { code: "flash", lbEn: true, instr: "bl.w", saveLRInstr: "mov.w  lr, r8" }
# umull lr, rx
- { code: "sram", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w lr, r10, r8, r9" }
- { code: "flash", lbEn: false, instr: "bl.w", saveLRInstr: "umull.w lr, r10, r8, r9" }
- { code: "flash", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w lr, r10, r8, r9" }
# umull rx, lr
- { code: "sram", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w r10, lr, r8, r9" }
- { code: "flash", lbEn: false, instr: "bl.w", saveLRInstr: "umull.w r10, lr, r8, r9" }
- { code: "flash", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w r10, lr, r8, r9" }
# umull ry, rx
- { code: "sram", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w r10, r11, r8, r9" }
- { code: "flash", lbEn: false, instr: "bl.w", saveLRInstr: "umull.w r10, r11, r8, r9" }
- { code: "flash", lbEn: true, instr: "bl.w", saveLRInstr: "umull.w r10, r11, r8, r9" }

# blx tests
# mov lr
- { code: "sram", lbEn: true, instr: "blx.n", saveLRInstr: "mov.w  lr, r8" }
- { code: "flash", lbEn: false, instr: "blx.n", saveLRInstr: "mov.w  lr, r8" }
- { code: "flash", lbEn: true, instr: "blx.n", saveLRInstr: "mov.w  lr, r8" }
# umull lr, rx
- { code: "sram", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w lr, r10, r8, r9" }
- { code: "flash", lbEn: false, instr: "blx.n", saveLRInstr: "umull.w lr, r10, r8, r9" }
- { code: "flash", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w lr, r10, r8, r9" }
# umull rx, lr
- { code: "sram", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w r10, lr, r8, r9" }
- { code: "flash", lbEn: false, instr: "blx.n", saveLRInstr: "umull.w r10, lr, r8, r9" }
- { code: "flash", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w r10, lr, r8, r9" }
# umull ry, rx
- { code: "sram", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w r10, r11, r8, r9" }
- { code: "flash", lbEn: false, instr: "blx.n", saveLRInstr: "umull.w r10, r11, r8, r9" }
- { code: "flash", lbEn: true, instr: "blx.n", saveLRInstr: "umull.w r10, r11, r8, r9" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set instrRepRange = 4 %}
{% set nopCountRange = 6 %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    @ Prepare add arguments
    mov.w  r5, #42
    mov.w  r6, #1
    @ Prepare {{saveLRInstr}} arguments
    ldr.w  r8, =0x0000FFFF
    ldr.w  r9, =0xFFFF0000

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
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for reps in range(instrRepRange) %}
{% for nops in range(nopCountRange) %}
    {% if instr == "blx.n" %}
        @ Load address
        ldr.w  r7, =jump_{{nops}}_target
    {% endif %}

    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {% for i in range(reps) %}
        @ Save something to lr
        {{saveLRInstr}}
    {% endfor %}
    @ Jump to jump_{{nops}}_target
    {% if instr == "bl.w" %}
        bl.w  jump_{{nops}}_target
    {% elif instr == "blx.n" %}
        blx.n r7
    {% else %}
        panic!
    {% endif %}
    @ This ADD should execute only once, after return
    add.w  r5, r6

    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

@ Different aligment addresses, that bl jumps to.
{% for nops in range(nopCountRange) %}
.align 4
    {% for i in range(nops) %}
        nop.n
    {% endfor %}
.thumb_func
jump_{{nops}}_target:
    @ Just return to bl call
    bx.n lr
    @ This ADD shouldn't execute
    add.w  r5, r6
{% endfor %}

    .ltorg

.align 2
.thumb_func
save_time_flags_and_result:
    mrs.w r1, apsr
    subs.n r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    {{saveValue("flags", r1, r3, r4)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr
{% endblock %}
