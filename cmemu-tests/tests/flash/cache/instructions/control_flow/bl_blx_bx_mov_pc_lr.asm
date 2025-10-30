---
name: BL/BLX/BX/MOV_PC plus write to LR
description: >-
  Timing test of `save (lr|rx); BL/BLX/BX/MOV_pc lr`.
  The goal is to verify whether branch is decode- or execute-time,
  including cases with and without register dependency if applicable.
dumped_symbols:
  # 24 = 4 instructions * 3 movReg * 2 bReg
  times: 24 words
  flags: 24 words
  cpicnts: 24 words
  lsucnts: 24 words
configurations:
- { code: "sram", lbEn: True, cache_enabled: True }
- { code: "flash", lbEn: False, cache_enabled: True }
- { code: "flash", lbEn: True, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set movRegs = ["", "lr", "r8"] %}
{% set bRegs = ["lr", "r8"] %}
{% set instructions = ["bl.w", "blx.n", "bx.n", "mov.n pc, "] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    {% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
        mov.w r10, {{counter}}
        ldr.w r11, ={{save_func}}

        bl.w  tested_code
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
    mov.n r12, lr

{% for instr in instructions %}
{% for movReg in movRegs %}
{% for bReg in bRegs %}
    @ Prepare {{instr}} input values
    {% set jump_label = "jump_{}_{}_{}_target".format(instr[:instr.find(".")], movReg, bReg) %}
    adr.w  r5, {{jump_label}}+1
    mov.w  {{bReg}}, r5

    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w r2, [r0, r10]

    @ {{movReg}} might be {{bReg}} when testing register dependency.
    {% if movReg != "" %}
        mov.w {{movReg}}, r5
    {% endif %}
    @ Jump to {{jump_label}}
    {% if instr == "bl.w" %}
        {{instr}} {{jump_label}}
    {% else %}
        {{instr}} {{bReg}}
    {% endif %}

    @ Those NOPs ensure jump target isn't in PIQ
    .rept 4; nop.w; .endr
.align 2
.thumb_func
.type {{jump_label}}, %function
{{jump_label}}:

    @ Get finish time
    ldr.w r3, [r0, r10]

    blx.n r11
{% endfor %}
{% endfor %}
{% endfor %}

    @ Return to counters loop.
    bx.n r12

.align 2
.thumb_func
save_time_and_flags:
    mrs.w r6, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r6, r3, r4)}}

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
