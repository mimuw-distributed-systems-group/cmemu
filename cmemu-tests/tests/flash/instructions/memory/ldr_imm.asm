---
name: LDR (immediate) instruction tests
description: "Timing and correctness test"
dumped_symbols: 
  results: 96 words # 24 (repetitions) * 4 (encodings)
  times: 96 words
  flags: 96 words
  cpicnts: 96 words
  lsucnts: 96 words
configurations:
- { code: "gpram", memory: "gpram", lbEn: True, gpram_part: 0 }
- { code: "gpram", memory: "gpram", lbEn: True, gpram_part: 1 }
- { code: "gpram", memory: "sram", lbEn: True, gpram_part: 0 }
- { code: "gpram", memory: "sram", lbEn: True, gpram_part: 1 }
- { code: "gpram", memory: "flash", lbEn: True, gpram_part: 0 }
- { code: "gpram", memory: "flash", lbEn: True, gpram_part: 1 }
- { code: "gpram", memory: "flash", lbEn: False, gpram_part: 0 }
- { code: "gpram", memory: "flash", lbEn: False, gpram_part: 1 }
- { code: "sram", memory: "gpram", lbEn: True }
- { code: "sram", memory: "sram", lbEn: True }
- { code: "sram", memory: "flash", lbEn: True }
- { code: "sram", memory: "flash", lbEn: False }
- { code: "flash", memory: "gpram", lbEn: True }
- { code: "flash", memory: "sram", lbEn: True }
- { code: "flash", memory: "flash", lbEn: True }
- { code: "flash", memory: "gpram", lbEn: False }
- { code: "flash", memory: "sram", lbEn: False }
- { code: "flash", memory: "flash", lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 24 %}
{% set encodings = ["T1", "T2", "T3", "T4"] %}
{% set save_func_reg = "r9" %}
{% set counter_reg = "r1" %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set encodings = encodings[:2] %}
    {% elif gpram_part == 1 %}
        {% set encodings = encodings[2:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r0, dwt

    @ Save original SP
    mov.w r11, sp

    {% for counter, save_func in [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}         
        mov.w {{counter_reg}}, {{counter}}
        ldr.w {{save_func_reg}}, ={{save_func}}
        
        bl.w tested_code
    {% endfor %}
.thumb_func
end_label:
    @ Restore original SP
    mov.w  sp, r11
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov r10, lr
{% for encoding in encodings %}
    {% set base_address_register = 'sp' if encoding == 'T2' else 'r6' %}
    {% set ldr_instruction = 'ldr.n' if encoding in ('T1', 'T2') else 'ldr.w' %}
{% for reps in range(repetitions) %}
    @ Prepare LDR test register.
    mov.w r5, #0
    @ Prepare ldr input values and reset flash line buffer
    ldr.w {{base_address_register}}, =rep_{{reps}}_memory{{ '_end' if encoding == 'T4' else '' }}
    mov.w r7, #0
    ldr.w r2, [r7]

    @ Clear flags
    mov.w r8, #0
    msr.w apsr_nzcvq, r8

    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% for i in range(reps) %}
        {{ldr_instruction}} r5, [{{base_address_register}}, {{'-' if encoding == 'T4' else ''}}{{ (i % 4) * 4 + (4 if encoding == 'T4' else 0) }}] 
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, {{counter_reg}}]

    blx.n {{save_func_reg}}
{% endfor %}
    @ Jump to avoid executing code that is a result of .ltorg
    bl.w after_ltorg_{{loop.index}}

.ltorg

after_ltorg_{{loop.index}}:
{% endfor %}
    @ Return to counters loop.
    bx.n r10

.align 2 
.thumb_func
save_times_results_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    {{saveValue("flags", r8, r3, r4)}}
    
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

{{ section(memory) }}
.align 4
{% for reps in range(repetitions) %}
rep_{{reps}}_memory:
    .word {{ reps*4 }}
    .word {{ reps*4 + 1 }}
    .word {{ reps*4 + 2 }}
    .word {{ reps*4 + 3 }}
rep_{{reps}}_memory_end:
{% endfor %}
{% endblock %}
