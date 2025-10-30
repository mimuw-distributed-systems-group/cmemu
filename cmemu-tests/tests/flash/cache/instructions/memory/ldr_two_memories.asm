---
name: LDR from two memories tests
description: "Timing test of LDR, when doing consecutive accesses to different memory types"
dumped_symbols:
  times: 20 words # 10 (repetitions) * 2 (LDR widths)
  flags: 20 words
  cpicnts: 20 words
  lsucnts: 20 words
configurations:
- { code: "sram", addr0: "sram", addr1: "sram", lbEn: True, cache_enabled: True }
- { code: "sram", addr0: "sram", addr1: "flash", lbEn: True, cache_enabled: True }
- { code: "sram", addr0: "sram", addr1: "flash", lbEn: False, cache_enabled: True }
- { code: "sram", addr0: "flash", addr1: "sram", lbEn: True, cache_enabled: True }
- { code: "sram", addr0: "flash", addr1: "sram", lbEn: False, cache_enabled: True }
- { code: "sram", addr0: "flash", addr1: "flash", lbEn: True, cache_enabled: True }
- { code: "sram", addr0: "flash", addr1: "flash", lbEn: False, cache_enabled: True }
- { code: "flash", addr0: "sram", addr1: "sram", lbEn: True, cache_enabled: True }
- { code: "flash", addr0: "sram", addr1: "sram", lbEn: False, cache_enabled: True }
- { code: "flash", addr0: "sram", addr1: "flash", lbEn: True, cache_enabled: True }
- { code: "flash", addr0: "sram", addr1: "flash", lbEn: False, cache_enabled: True }
- { code: "flash", addr0: "flash", addr1: "sram", lbEn: True, cache_enabled: True }
- { code: "flash", addr0: "flash", addr1: "sram", lbEn: False, cache_enabled: True }
- { code: "flash", addr0: "flash", addr1: "flash", lbEn: True, cache_enabled: True }
- { code: "flash", addr0: "flash", addr1: "flash", lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 10 %}
{% set save_func_reg = "r9" %}
{% set counter_reg = "r8" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r3, dwt

    @ Prepare LDR input values
    ldr.w r5, =cell_0
    ldr.w r6, =cell_1

    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}         
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
{% for width in ["n", "w"] %}
{% for reps in range(repetitions) %}
    mov.w r7, #0

    @ Reset flash line buffer
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r7

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r3, {{counter_reg}}]

    {% for i in range(reps) %}
        ldr.{{width}} r1, [r5]
        ldr.{{width}} r4, [r6]
    {% endfor %}

    @ Get finish counter value
    ldr.w r0, [r3, {{counter_reg}}]
    
    blx.n {{save_func_reg}}
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r10

.thumb_func
save_times_and_flags:
    mrs.w r7, apsr
    sub.w r0, r0, r2
    
    {{saveValue("times", r0, r1, r2)}}
    {{saveValue("flags", r7, r1, r2)}}
    
    bx.n lr

.thumb_func
save_cpicnt:
    sub.w r0, r0, r2
    and.w r0, r0, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r0, r1, r2)}}

    bx.n lr

.thumb_func
save_lsucnt:
    sub.w r0, r0, r2
    and.w r0, r0, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r0, r1, r2)}}

    bx.n lr

{{ section(addr0) }}
.align 4
cell_0: .word cell_0

{{ section(addr1) }}
.align 4
cell_1: .word cell_1

{% endblock %}
