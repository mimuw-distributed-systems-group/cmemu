---
name: ADD+LDM instructions tests
description: "Timing test of ADD+LDM register dependencies"
dumped_symbols:
  times: 20 words
  flags: 20 words
  cpicnts: 20 words
  lsucnts: 20 words
configurations:
- { code: "gpram", addr: "sram", addrReg: "r2", ldrInstr: "ldm.w" }
- { code: "gpram", addr: "sram", addrReg: "r3", ldrInstr: "ldm.w" }
- { code: "gpram", addr: "sram", addrReg: "r2", ldrInstr: "ldmdb.w" }
- { code: "gpram", addr: "sram", addrReg: "r3", ldrInstr: "ldmdb.w" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 20 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr arguments
    ldr.w  r5, =cell_1
    ldr.w  {{addrReg}}, =cell_0

    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{section(code)}}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    ldr.w  r8, [r0, {{counter}}]

    {% for i in range(reps) %}
        add.w  r2, r5, 0
        {{ldrInstr}}  {{addrReg}}, {r1, r4}
    {% endfor %}

    @ Get finish counter value
    ldr.w  r9, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_and_flags:
    mrs.w r7, apsr
    sub.w r8, r9, r8

    {{saveValue("times", r8, r10, r11)}}
    {{saveValue("flags", r7, r10, r11)}}

    bx.n lr

save_cpicnt:
    sub.w r8, r9, r8
    ands.w r8, r8, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r8, r10, r11)}}

    bx.n lr

save_lsucnt:
    sub.w r8, r9, r8
    ands.w r8, r8, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r8, r10, r11)}}
    
    bx.n lr

{{section(addr)}}
.align 4
        .word 0
        .word 0
        .word 0
        .word 0
cell_0: .word cell_0
cell_1: .word cell_1
        .word 0
        .word 0
        .word 0
        .word 0
{% endblock %}
