---
name: Ldr + Str instruction tests
description: "Timing and correctness test of LDR and STR instruction"
dumped_symbols:
  times: 64 words # 2 (strCellNums) * 2 (widths) * 16 repetitions
  flags: 64 words
  cpicnts: 64 words
  lsucnts: 64 words
configurations:
- { code: sram, ldrMemory: sram, strMemory: sram, lbEn: True }
- { code: sram, ldrMemory: flash, strMemory: sram, lbEn: True }
- { code: sram, ldrMemory: flash, strMemory: sram, lbEn: False }
- { code: flash, ldrMemory: sram, strMemory: sram, lbEn: True }
- { code: flash, ldrMemory: sram, strMemory: sram, lbEn: False }
- { code: flash, ldrMemory: flash, strMemory: sram, lbEn: True }
- { code: flash, ldrMemory: flash, strMemory: sram, lbEn: False }
...
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set save_func_reg = "r10" %}
{% set counter_reg = "r9" %}
{% set ldrCellNum = 0 %}
{% set strCellNums = [0, 1] %}

{% if ldrMemory != strMemory %}
    {% set strCellNums = [0] %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r8, dwt

    @ Prepare LDR input value
    ldr.w r5, =mem_{{ldrMemory}}_{{ldrCellNum}}

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
    mov r11, lr
{% for strCellNum in strCellNums %}
    @ Prepare STR input value    
    movw.w r6, #:lower16:mem_{{strMemory}}_{{strCellNum}}
    movt.w r6, #:upper16:mem_{{strMemory}}_{{strCellNum}}
{% for width in ["n", "w"] %}
{% for reps in range(16) %}
    mov.w r7, #0
    
    @ Reset flash line buffer
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r7

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r8, {{counter_reg}}]

    {% for i in range(reps) %}
        ldr.{{width}} r0, [r5]
        str.{{width}} r1, [r6] 
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r8, {{counter_reg}}]
    
    blx.n {{save_func_reg}}
{% endfor %}
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r11

.thumb_func
save_times_and_flags:
    mrs.w r7, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r7, r3, r4)}}

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

{{ section("flash") }}
.align 4
mem_flash_0: .word 0xCAFE

{{ section("sram")}}
.align 4
mem_sram_0: .word 0xCAFE
.align 4
mem_sram_1: .word 0xBEE5

{% endblock %}
