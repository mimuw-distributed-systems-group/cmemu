---
name: LDRD (immediate) with writeback instruction tests
description: >
    Timing and correctness test.
dumped_symbols:
  times: 6 words     # 2 (pre/postindex) * 3 (repetitions)
  results: 42 words  # ... * 7 (registers saved)
  flags: 6 words
  cpicnts: 6 words
  lsucnts: 6 words
configurations:
- { code: gpram, lbEn: True, memory: gpram }
- { code: gpram, lbEn: True, memory: sram }
- { code: gpram, lbEn: True, memory: flash }
- { code: gpram, lbEn: False, memory: flash }
- { code: sram, lbEn: True, memory: gpram }
- { code: sram, lbEn: True, memory: sram }
- { code: sram, lbEn: True, memory: flash }
- { code: sram, lbEn: False, memory: flash }
- { code: flash, lbEn: True, memory: gpram }
- { code: flash, lbEn: False, memory: gpram }
- { code: flash, lbEn: True, memory: sram }
- { code: flash, lbEn: False, memory: sram }
- { code: flash, lbEn: True, memory: flash }
- { code: flash, lbEn: False, memory: flash }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set data_reg_pairs = ["r4, r5", "r7, r8", "r9, r10"] %}

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
{% for counter, save_func in [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for preindex in [True, False] %}
{% for reps in range(1, 4) %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Prepare ldr input values and reset flash line buffer
    ldr.w  r6, =memory
    mov.w  r4, #0
    mov.w  r5, #0
    mov.w  r7, #0
    mov.w  r8, #0
    mov.w  r9, #0
    mov.w  r10, #0

    @ Reset flash line buffer
    ldr.w  r1, [r7, r7]

    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r1, [r0, {{counter}}]

    {% for i in range(reps) %}
        ldrd.w {{data_reg_pairs[i]}}, {% if preindex %} [r6, #8]! {% else %} [r6], #8 {% endif %}
    {% endfor %}

    @ Get finish counter value
    ldr.w  r2, [r0, {{counter}}]
    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_results_and_flags:
    mrs.w r11, apsr
    sub.w r1, r2, r1

    {{saveValue("times", r1, r2, r3)}}
    {{saveValue("results", r4, r2, r3)}}
    {{saveValue("results", r5, r2, r3)}}
    {{saveValue("results", r6, r2, r3)}}
    {{saveValue("results", r7, r2, r3)}}
    {{saveValue("results", r8, r2, r3)}}
    {{saveValue("results", r9, r2, r3)}}
    {{saveValue("results", r10, r2, r3)}}
    {{saveValue("flags", r11, r2, r3)}}
    
    bx.n lr

save_cpicnt:
    sub.w r1, r2, r1
    ands.w r1, r1, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r1, r2, r3)}}

    bx.n lr

save_lsucnt:
    sub.w r1, r2, r1
    ands.w r1, r1, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r1, r2, r3)}}
    
    bx.n lr

{{ section(memory) }}
.align 4
memory:
    .word 0
    .word 0
    .word 0x01234567
    .word 0x89ABCDEF
    .word 0x98765432
    .word 0x56789ABC
    .word 0x87456234
    .word 0x09635781
    .word 0x93468531
    .word 0x24678576
    .word 0
    .word 0
{% endblock %}
