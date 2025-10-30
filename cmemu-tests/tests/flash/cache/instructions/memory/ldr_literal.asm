---
name: LDR literal instruction tests
description: "Timing and correctness test of LDR literal instruction"
dumped_symbols:
  results: 64 words # 2 (LDR instructions) * 32 (repetitions)
  times: 64 words
  flags: 64 words
  cpicnts: 64 words
  lsucnts: 64 words
configurations:
- { code: "sram", lbEn: True, cache_enabled: True }
- { code: "flash", lbEn: True, cache_enabled: True }
- { code: "flash", lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 32 %}
{% set save_func_reg = "r6" %}
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
    mov r7, lr
{% for ldr_instr in ["ldr.w", "ldr.n"] %}
    {% set ldr_instr_loop_idx = loop.index %}
{% for reps in range(repetitions) %}
    @ Set register to deterministic state when reps == 0 and prepare register
    @ for resetting line buffer and clearing flags
    mov.w r5, #0

    @ Reset flash line buffer
    ldr.w r2, [r5]

    @ Clear flags
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% for i in range(reps) %}
        {{ldr_instr}} r5, rep_{{reps}}_{{ldr_instr_loop_idx}}_memory
    {% endfor %}

    @ Get finish counter value
    ldr.w r3, [r0, {{counter_reg}}]

    blx.n {{save_func_reg}}
    b.w after_loaded_value_{{reps}}_{{ldr_instr_loop_idx}}

@ Place loaded value close to the request
.align 4
rep_{{reps}}_{{ldr_instr_loop_idx}}_memory: .word {{2**reps}}

after_loaded_value_{{reps}}_{{ldr_instr_loop_idx}}:
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r7

.thumb_func
save_times_results_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
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

{% endblock %}
