---
name: LDR (immediate) with writeback instruction tests (data dependencies)
description: >
    Check if execution time of pre-/postindexed LDR is correct after earlier LDR/ADD instruction.
dumped_symbols:
  results: 8 words # 2 (saved registers) * 2 (preinstr) * 2 (preindex)
  times: 4 words # 2 (preinstr) * 2 (preindex)
  flags: 4 words
  cpicnts: 4 words
  lsucnts: 4 words
configurations:
- { code: gpram, memory: gpram, lbEn: True }
- { code: gpram, memory: sram, lbEn: True }
- { code: gpram, memory: flash, lbEn: True }
- { code: gpram, memory: flash, lbEn: False }
- { code: sram, memory: gpram, lbEn: True }
- { code: sram, memory: sram, lbEn: True }
- { code: sram, memory: flash, lbEn: True }
- { code: sram, memory: flash, lbEn: False }
- { code: flash, memory: gpram, lbEn: True }
- { code: flash, memory: gpram, lbEn: False }
- { code: flash, memory: sram, lbEn: True }
- { code: flash, memory: sram, lbEn: False }
- { code: flash, memory: flash, lbEn: True }
- { code: flash, memory: flash, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set save_func_reg = "r5" %}
{% set counter_reg = "r1" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    
    {% for counter, save_func in [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}         
        mov.w {{counter_reg}}, {{counter}}
        ldr.w {{save_func_reg}}, ={{save_func}}
        
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
    mov r10, lr
{% for preinstr in ["add.w", "ldr.w"] %}
{% for preindex in [True, False] %}
    @ Prepare LDR input values and reset flash line buffer
    ldr.w r6, =memory
    mov.w r7, #0
    mov.w r8, #0
    ldr.w r9, =memory_addr
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r8

    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% if preinstr == "add.w" %}
        add.w r6, r6, r8
    {% elif preinstr == "ldr.w" %}
        ldr.w r6, [r9, r8]
    {% else %}
        panic!
    {% endif %}

    ldr.w r7, {% if preindex %} [r6, 4]! {% else %} [r6], 4 {% endif %}

    @ Get finish counter value
    ldr.w r3, [r0, {{counter_reg}}]
    
    blx.n {{save_func_reg}}

{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r10

.thumb_func
save_times_results_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
    {{saveValue("results", r7, r3, r4)}}
    {{saveValue("flags", r8, r3, r4)}}
    
    bx.n lr

.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{{ section(memory) }}
.align 4
memory:
    .word 2
    .word 3
    .word 5
    .word 7

.align 2
memory_addr:
    .word memory
{% endblock %}
