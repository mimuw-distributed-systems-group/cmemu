---
name: B sequential execution
description: "Test timing and counters of multiple B instructions one after another"
dumped_symbols:
    times: 12 words
    flags: 12 words
    cpicnts: 12 words
    lsucnts: 12 words
configurations:
- { code: "gpram", lbEn: true, nops: 0, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 0, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 0, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 0, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 1, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 1, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 1, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 1, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 2, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 2, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 2, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 2, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 3, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 3, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 3, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 3, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 4, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 4, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 4, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 4, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 5, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 5, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 5, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 5, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 6, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 6, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 6, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 6, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 7, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 7, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 7, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 7, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 8, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 8, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 8, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 8, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 9, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 9, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 9, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 9, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 10, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 10, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 10, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 10, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 11, bInstr: 'b.n' }
- { code: "sram", lbEn: true, nops: 11, bInstr: 'b.n' }
- { code: "flash", lbEn: false, nops: 11, bInstr: 'b.n' }
- { code: "flash", lbEn: true, nops: 11, bInstr: 'b.n' }
- { code: "gpram", lbEn: true, nops: 0, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 0, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 0, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 0, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 1, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 1, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 1, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 1, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 2, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 2, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 2, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 2, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 3, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 3, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 3, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 3, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 4, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 4, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 4, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 4, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 5, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 5, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 5, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 5, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 6, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 6, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 6, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 6, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 7, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 7, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 7, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 7, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 8, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 8, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 8, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 8, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 9, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 9, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 9, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 9, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 10, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 10, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 10, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 10, bInstr: 'b.w' }
- { code: "gpram", lbEn: true, nops: 11, bInstr: 'b.w' }
- { code: "sram", lbEn: true, nops: 11, bInstr: 'b.w' }
- { code: "flash", lbEn: false, nops: 11, bInstr: 'b.w' }
- { code: "flash", lbEn: true, nops: 11, bInstr: 'b.w' }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 12 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

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
{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {% for i in range(reps) %}
        {{bInstr}} jump_target_{{counter_idx}}_{{reps}}_{{i}}
        {% for i in range(nops) %}
        nop.n
        {% endfor %}
    jump_target_{{counter_idx}}_{{reps}}_{{i}}:
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r8, r3, r4)}}

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
