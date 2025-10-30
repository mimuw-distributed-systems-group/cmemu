---
name: LDR (immediate) with writeback instruction tests
description: >
    Timing and correctness test.
dumped_symbols:
  results: 24 words # 4 (saved registers) * 3 (repetitions) * 2 (preindices)
  times: 6 words # 3 (repetitions) * 2 (preindices)
  flags: 6 words
  cpicnts: 6 words
  lsucnts: 6 words
configurations:
- { code: sram, memory: sram, lbEn: True }
- { code: sram, memory: flash, lbEn: True }
- { code: sram, memory: flash, lbEn: False }
- { code: flash, memory: sram, lbEn: True }
- { code: flash, memory: sram, lbEn: False }
- { code: flash, memory: flash, lbEn: True }
- { code: flash, memory: flash, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set data_regs = ["r7", "r8", "r9"] %}
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
{% for preindex in [True, False] %}
{% for reps in range(1, 4) %}
    @ Prepare LDR input values and reset flash line buffer
    ldr.w r6, =rep_{{reps}}_memory
    mov.w r7, #0
    mov.w r8, #0
    mov.w r9, #0
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r8

    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% for i in range(reps) %}
        ldr.w {{data_regs[i]}}, {% if preindex %} [r6, 4]! {% else %} [r6], 4 {% endif %}
    {% endfor %}

    @ Get finish counter value
    ldr.w r3, [r0, {{counter_reg}}]
    
    blx.n {{save_func_reg}}

{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r10

.thumb_func
save_times_results_and_flags:
    mrs.w r11, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
    {{saveValue("results", r7, r3, r4)}}
    {{saveValue("results", r8, r3, r4)}}
    {{saveValue("results", r9, r3, r4)}}
    {{saveValue("flags", r11, r3, r4)}}
    
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
{% for reps in range(4) %}
rep_{{reps}}_memory:
    .word {{ reps*4 }}
    .word {{ reps*4 + 1 }}
    .word {{ reps*4 + 2 }}
    .word {{ reps*4 + 3 }}
{% endfor %}
{% endblock %}
