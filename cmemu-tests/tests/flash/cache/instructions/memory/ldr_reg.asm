---
name: LDR (register) instruction tests
description: "Timing and correctness test"
dumped_symbols:
  results: 48 words # 24 (repetitions) * 2 (LDR widths)
  times: 48 words
  flags: 48 words
  cpicnts: 48 words
  lsucnts: 48 words
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

{% set repetitions = 24 %}
{% set save_func_reg = "r9" %}
{% set counter_reg = "r1" %}

{% block code %}
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
{% for width in ["n", "w"] %}
    {% set width_idx = loop.index %}
{% for reps in range(repetitions) %}
    @ Prepare LDR input values
    ldr.w r6, =rep_{{reps}}_memory
    mov.w r5, #0
    mov.w r7, #0

    @ Reset flash line buffer
    ldr.w r2, [r5]

    @ Clear flags
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r0, {{counter_reg}}]

    {% for _ in range(reps) %}
        ldr.{{width}} r5, [r6, r7]
    {% endfor %}

    @ Get finish counter value
    ldr.w r3, [r0, {{counter_reg}}]

    blx.n {{save_func_reg}}
{% endfor %}
    b.w after_pool_{{width_idx}}
.ltorg
after_pool_{{width_idx}}:
{% endfor %}
    @ Return to counters loop.
    bx.n r10

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

{{ section(memory) }}
.align 4
{% for reps in range(repetitions) %}
rep_{{reps}}_memory: .word 0b1{% for i in range(reps) %}0{% endfor %}
{% endfor %}
{% endblock %}
