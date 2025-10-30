---
name: LDR from DWT tests
description: "Timing test of LDR, when doing consecutive accesses to DWT"
dumped_symbols: 
  times: 40 words # 20 repetitions * 2 LDR instructions
  flags: 40 words
  cpicnts: 40 words
  lsucnts: 40 words
configurations:
# - { code: "sram", lbEn: True }
- { code: "flash", lbEn: True }
- { code: "flash", lbEn: False }
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 20 %}
{% set ldrInstrs = ["ldr.w", "ldr.n"] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set ldrInstrs = ldrInstrs[:1] %}
    {% elif gpram_part == 1 %}
        {% set ldrInstrs = ldrInstrs[1:2] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r3, dwt
    
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
{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for ldrInstr in ldrInstrs %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Prepare LDR input values
    @ move address of dwt to r5
    mov.w  r5, r3
    mov.w  r7, #4

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r3, {{counter}}]

    {% for i in range(reps) %}
        {{ldrInstr}} r1, [r5, r7]
    {% endfor %}

    @ Get finish counter value
    ldr.w  r0, [r3, {{counter}}]
    
    @ Save measurements
    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

save_times_and_flags:
    mrs.w r1, apsr
    sub.w r0, r0, r2

    {{saveValue("times", r0, r2, r4)}}
    {{saveValue("flags", r1, r2, r4)}}

    bx.n lr

save_cpicnt:
    sub.w r0, r0, r2
    ands.w r0, r0, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r0, r2, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r0, r0, r2
    ands.w r0, r0, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r0, r2, r4)}}
    
    bx.n lr

{% endblock %}
