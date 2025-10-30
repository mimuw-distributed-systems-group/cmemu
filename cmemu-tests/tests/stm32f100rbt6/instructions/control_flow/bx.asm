---
name: BX
description: "Timing test of bx to address in register"
dumped_symbols:
  results: 8 words
  times: 8 words
  flags: 8 words
  cpicnts: 8 words
  lsucnts: 8 words
configurations:
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: false }
- { code: "flash", lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set nopCountRange = 8 %}

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
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for nops in range(nopCountRange) %}
    @ Prepare ADD arguments
    mov.w  r6, #42

    @ Prepare BX arguments
    ldr.w  r5, =jump_{{nops}}_{{counter_idx}}_target

    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    @ Jump to jump_{{nops}}_{{counter_idx}}_target stored in r5
    bx.n r5
    @ These `add`s shouldn't execute
    add.w  r6, #1
    add.w  r6, #1
    add.w  r6, #1
    add.w  r6, #1

.align 4
    {% for i in range(nops) %}
        nop.n
    {% endfor %}
.thumb_func
jump_{{nops}}_{{counter_idx}}_target:
    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
    b.w end_label

save_time_flags_and_result:
    mrs.w r1, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
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
