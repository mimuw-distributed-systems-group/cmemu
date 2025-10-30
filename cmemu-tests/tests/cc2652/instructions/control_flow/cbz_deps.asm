---
name: CB{N}Z register dependencies tests
description: "Timing test of CB{N}Z register dependencies."
dumped_symbols:
  times: 24 words
  flags: 24 words
  cpicnts: 24 words
  lsucnts: 24 words
  foldcnts: 24 words
configurations:
- { code: "flash", lbEn: true, addr: "sram", preInstr: "ldr.w r5, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "str.w r5, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "ldr.w r7, [r1]", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "str.w r7, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "adds.n r7, r5", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "adds.w r7, r5", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "adds.n r5, 0", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "adds.w r5, 0", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "nop.n", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: true, addr: "sram", preInstr: "nop.w", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "ldr.w r5, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "str.w r5, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "ldr.w r7, [r1]", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "str.w r7, [r1]", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "adds.n r7, r5", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "adds.w r7, r5", cbzInstr: "cbnz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "nop.n", cbzInstr: "cbz.n" }
- { code: "flash", lbEn: false, addr: "sram", preInstr: "nop.w", cbzInstr: "cbz.n" }
...
{% set nopsRange = 4 if code == "flash" else 3 %}
{% set repetitions = 6 %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ r6 contains default value of 0
    mov.w  r6, #0

    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"), (FOLDCNT, "save_foldcnt")] %}
    {% set counter_idx = loop.index %}
{% for nops in range(nopsRange) %}
{% for reps in range(repetitions) %}
    @ Prepare test input
    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter}}]

    {% for i in range(reps) %}
        {{preInstr}}
        {{cbzInstr}} r7, jump_target_{{counter_idx}}_{{reps}}_{{nops}}_{{i}}
        @ These `ADD`s shouldn't execute
        add.w  r6, r6
        add.w  r6, r6
        add.w  r6, r6
        add.w  r6, r6
    .align 4
        {% for j in range(nops) %}
            nop.n
        {% endfor %}
    jump_target_{{counter_idx}}_{{reps}}_{{nops}}_{{i}}:
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Registers r1, r5, r7 can be operated by inserted instruction
    @ r1 contains safe memory address in {{addr}}, with initial value of 42
    ldr.w  r1, =cell_0
    @ r5 contains default value of 1
    mov.w  r5, #1
    @ r7 is compared to zero by branch instruction and has default value of 0
    mov.w  r7, #0

    bx.n lr

save_time_and_flags:
    mrs.w r1, apsr
    subs.n r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
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

save_foldcnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ FOLDCNT is 8-bit wide
    {{saveValue("foldcnts", r2, r3, r4)}}
    bx.n lr

{{ section(addr) }}
.align 4
cell_0:       .word 0x2a
{% endblock %}
