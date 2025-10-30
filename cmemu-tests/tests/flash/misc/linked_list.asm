---
name: Linked list iteration test
description: "Timing of iteration over linked list"
dumped_symbols:
  times: 64 words # 16 (repetitions) * 2 (widths) * 2 ((non-)consecutive)
  results: 64 words
  flags: 64 words
  cpicnts: 64 words
  lsucnts: 64 words
configurations:
- { code: gpram, data: gpram, lbEn: True }
- { code: gpram, data: sram, lbEn: True }
- { code: gpram, data: flash, lbEn: True }
- { code: gpram, data: flash, lbEn: False }
- { code: sram, data: gpram, lbEn: True }
- { code: sram, data: sram, lbEn: True }
- { code: sram, data: flash, lbEn: True }
- { code: sram, data: flash, lbEn: False }
- { code: flash, data: gpram, lbEn: True }
- { code: flash, data: gpram, lbEn: False }
- { code: flash, data: sram, lbEn: True }
- { code: flash, data: sram, lbEn: False }
- { code: flash, data: flash, lbEn: True }
- { code: flash, data: flash, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 16 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

{% for counter, save_func in [(CYCCNT, "save_times_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r1, {{counter}}
    ldr.w r10, ={{save_func}}

    bl.w    tested_code
{% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r11, lr
{% for consecutive in ["consecutive", "non_consecutive"] %}
{% for width in ["w", "n"] %}
{% for reps in range(repetitions) %}
    @ Prepare LDR input values
    mov.w  r6, #0
    ldr.w  r5, [r6] @ reset line buffer
    movw.w r5, #:lower16:cell_{{consecutive}}_0
    movt.w r5, #:upper16:cell_{{consecutive}}_0

    @ Clear flags
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        ldr.{{width}} r5, [r5]
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
{% endfor %}
{% endfor %}
    @ Return to counters loop
    bx.n r11

.align 2
.thumb_func
save_times_flags_and_result:
    sub.w r2, r3, r2
    mrs.w r6, apsr

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    {{saveValue("flags", r6, r3, r4)}}

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


{{ section(data) }}
.align 9
{% for i in range(15) %}
cell_consecutive_{{i}}: .word cell_consecutive_{{i+1}}
{% endfor %}
cell_consecutive_15: .word 0xCAFE

.align 9
{% for i in range(8) %}
cell_non_consecutive_{{2*i}}: .word cell_non_consecutive_{{2*i+1}}
{% endfor %}
.align 9
{% for i in range(7) %}
cell_non_consecutive_{{2*i+1}}: .word cell_non_consecutive_{{2*i+2}}
{% endfor %}
cell_non_consecutive_15: .word 0xCAFE

{% endblock %}
