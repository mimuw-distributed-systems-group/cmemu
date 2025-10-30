---
name: LDR from dwt and memory tests
description: "Timing test of LDR, when doing consecutive accesses to DWT and memory"
dumped_symbols:
  times: 40 words # 20 repetitions * 2 LDR instructions
  flags: 40 words
  cpicnts: 40 words
  lsucnts: 40 words
configurations:
# - { code: "sram", addr: "sram", lbEn: True }
# - { code: "sram", addr: "flash", lbEn: True }
# - { code: "sram", addr: "flash", lbEn: False }
- { code: "flash", addr: "sram", lbEn: True }
- { code: "flash", addr: "sram", lbEn: False }
- { code: "flash", addr: "flash", lbEn: True }
- { code: "flash", addr: "flash", lbEn: False }
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

    @ Prepare LDR input values
    @ move address of DWT to r5
    mov.w  r5, r3
    ldr.w  r6, =cell_0
    @ 4 is equal to CYCCNT offset from DWT, but it's also an offset of cell_1 from cell_0
    mov.w  r7, #4
    
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
    mov.w r8, #0
    msr.w apsr_nzcvq, r8

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r3, {{counter}}]

    {% for i in range(reps) %}
        {{ldrInstr}} r1, [r5, r7]
        {{ldrInstr}} r4, [r6, r7]
    {% endfor %}

    @ Get finish counter value
    ldr.w  r0, [r3, {{counter}}]

    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

save_times_and_flags:
    mrs.w r8, apsr
    sub.w r2, r0, r2

    {{saveValue("times", r2, r10, r11)}}
    {{saveValue("flags", r8, r10, r11)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r0, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r2, r10, r11)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r0, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r2, r10, r11)}}
    
    bx.n lr

{{ section(addr) }}
.align 4
cell_0: .word cell_0
@ cell_1 is used indirectly by loading cell_0 with offset 4
cell_1: .word cell_1

{% endblock %}
