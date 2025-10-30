---
name: MOV + STR (immediate) instruction tests
description: Check for dependencies between these instructions
dumped_symbols:
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
  exccnts: auto
  sleepcnts: auto
  foldcnts: auto
configurations:
- { code: flash, data: sram, lbEn: True, cache_enabled: True }
- { code: flash, data: sram, lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set save_func_reg = "r10" %}
{% set counter_reg = "r6" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r0, dwt
    ldr.w r5, =cell

    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"),
                                  (EXCCNT, "save_exccnt"), (SLEEPCNT, "save_sleepcnt"), (FOLDCNT, "save_foldcnt")] %}
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
    mov.w r9, lr

{% for reps in range(16) %}
    @ Prepare input values and reset flash line buffer
    mov.w r8, {{reps + 128}}
    mov.w r7, #0
    ldr.w r2, [r7]

    @ Clear flags
    msr.w apsr_nzcvq, r7
    
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    @ TODO: THIS TESTS IT INCOMPATIBLE WITH ITS NAME
    {% for i in range(reps) %}
        mov.w r7, r{{i}}
        str.w r8, [r5]
    {% endfor %}

    @ Get finish time
    ldr.w r3, [r0, {{counter_reg}}]
    
    blx.n {{save_func_reg}}
    {{ inc_auto_syms() }}
{% endfor %}

    @ Return to counters loop.
    bx.n r9

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

.thumb_func
save_exccnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ EXCCNT is 8-bit wide

    {{saveValue("exccnts", r2, r3, r4)}}

    bx.n lr

.thumb_func
save_sleepcnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ SLEEPCNT is 8-bit wide

    {{saveValue("sleepcnts", r2, r3, r4)}}

    bx.n lr

.thumb_func
save_foldcnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ FOLDCNT is 8-bit wide

    {{saveValue("foldcnts", r2, r3, r4)}}

    bx.n lr

{{ section(data) }}
.align 4
cell: .word 0xCAFE
{% endblock %}
