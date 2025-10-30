---
name: LDR with writeback pipelining
description: >
  [ARM-TRM-G] 18.3 Load-store timings states that:
  "Base update load is generally not pipelined."
  However, [...] it may with non-register writing instructions.
dumped_symbols:
  results6: auto
  results7: auto
  results8: auto
  results9: auto
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations:
- { code: sram, memory: sram, lbEn: True, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
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

{% set after_instrs = [
    "",
    "nop.n",

    "cmn.n r7, r7",
    "cmp.n r7, #1",
    "cmp.n r7, r8",
    "tst.n r6, r6",

    "it.n vs; addvs.n r7, #1",
    "str.w r8, [r4]",
    "str.w r8, [r4, 4]!",
] %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov r10, lr
{% for after_instr in after_instrs %}
{% for preindex in [True, False] %}
{% for reps in range(1, 4) %}
    @ Prepare LDR input values and reset flash line buffer
    ldr.w r4, =write_mem
    ldr.w r6, =rep_{{reps}}_memory
    mov.w r7, #0
    mov.w r8, #0
    mov.w r9, #0
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r8

    .align 3
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% for i in range(reps) %}
        ldr.w {{data_regs[i]}}, {% if preindex %} [r6, 4]! {% else %} [r6], 4 {% endif %}
        {{ after_instr }}
    {% endfor %}

    @ Get finish counter value
    ldr.n r3, [r0, {{counter_reg}}]
    
    blx.n {{save_func_reg}}
    {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r10

.thumb_func
save_times_results_and_flags:
    mrs.w r11, apsr
    sub.w r2, r3, r2
    
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results6", r6, r3, r4)}}
    {{saveValue("results7", r7, r3, r4)}}
    {{saveValue("results8", r8, r3, r4)}}
    {{saveValue("results9", r9, r3, r4)}}
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

{{ section(memory if memory != "flash" else "sram") }}
write_mem:
.rept 16
    .word 0
.endr
{% endblock %}
