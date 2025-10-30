---
name: MOV+LDR/STR instructions tests
description: "Timing test of MOV+LDR/STR register dependencies"
dumped_symbols:
  results: 240 words # 20 (repetitions) * 4 (test_cases) * 3 (registers combinations)
  times: 240 words
  flags: 240 words
  cpicnts: 240 words
  lsucnts: 240 words
configurations:
- { code: "flash", lbEn: True, test_cases_part: 0 }
- { code: "flash", lbEn: True, test_cases_part: 1 }
- { code: "flash", lbEn: False, test_cases_part: 0 }
- { code: "flash", lbEn: False, test_cases_part: 1 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 20 %}
{% set save_func_reg = "r10" %}
{% set counter_reg = "r1" %}
{% set regA = "r2" %}
{% set test_cases = [ 
    ("str.n", "mov.n"),
    ("str.w", "mov.n"),
    ("str.n", "mov.w"),
    ("str.w", "mov.w"),
    ("ldr.n", "mov.n"),
    ("ldr.w", "mov.n"),
    ("ldr.n", "mov.w"),
    ("ldr.w", "mov.w"),
] %}
{% set test_cases = test_cases[test_cases_part * 4:test_cases_part * 4 + 4] if test_cases_part in range(2) else unreachable("invalid test cases part") %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r0, dwt

    {% for counter, save_func in [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}         
        mov.w {{counter_reg}}, {{counter}}
        ldr.w {{save_func_reg}}, ={{save_func}}
        
        bl.w tested_code
    {% endfor %}

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{section(code)}}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov.w r11, lr
{% for regB, regC in [("r2", "r3"), ("r3", "r2"), ("r3", "r4")] %}
    @ Prepare LDR/STR arguments
    movw.w {{regC}}, #:lower16:cell_0
    movt.w {{regC}}, #:upper16:cell_0
{% for instr, mov in test_cases %}
{% if instr[:3] == "str" %}
    mov.w r5, {{regC}}
{% else %}
    movw.w r5, #:lower16:cell_1
    movt.w r5, #:upper16:cell_1
{% endif %}
{% for reps in range(repetitions) %}
    mov.w {{regB}}, #42

    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r8, [r0, {{counter_reg}}]

    @ Those adds are to prohibit pipelining of LDR/STRs
    add.w r6, #1
    .rept {{reps}}
        {{mov}} {{regA}}, r5
        {{instr}} {{regB}}, [{{regC}}]
    .endr
    add.w r6, #1

    @ Get finish counter value
    ldr.w r9, [r0, {{counter_reg}}]

    mov.w r7, {{regB}}
    
    blx.n {{save_func_reg}}

{% endfor %}
{% endfor %}
{% endfor %}

    @ Return to counters loop.
    bx.n r11

.thumb_func
save_times_results_and_flags:
    sub.w r8, r9, r8
    
    {{saveValue("times", r8, r9, r6)}}
    {{saveValue("results", r7, r9, r6)}}
    
    mrs.w r7, apsr
    {{saveValue("flags", r7, r9, r6)}}
    
    bx.n lr

.thumb_func
save_cpicnt:
    sub.w r8, r9, r8
    and.w r8, r8, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r8, r9, r6)}}
    
    bx.n lr

.thumb_func
save_lsucnt:
    sub.w r8, r9, r8
    and.w r8, r8, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r8, r9, r6)}}
    
    bx.n lr

{{section("sram")}}
.align 4
cell_0: .word cell_0
cell_1: .word cell_1
{% endblock %}
