---
name: Array of linked lists iteration test
description: "Timing of iteration over array of linked lists"
dumped_symbols:
  times: 192 words # 2 (iter) * 2 (width) * 4 (adds) * 12 (repetitions)
  results: 192 words
  flags: 192 words
  cpicnts: 192 words
  lsucnts: 192 words
configurations:
- { code: gpram, data: gpram, lbEn: True, gpram_part: 0 }
- { code: gpram, data: gpram, lbEn: True, gpram_part: 1 }
- { code: gpram, data: sram, lbEn: True, gpram_part: 0 }
- { code: gpram, data: sram, lbEn: True, gpram_part: 1 }
- { code: gpram, data: flash, lbEn: True, gpram_part: 0 }
- { code: gpram, data: flash, lbEn: True, gpram_part: 1 }
- { code: gpram, data: flash, lbEn: False, gpram_part: 0 }
- { code: gpram, data: flash, lbEn: False, gpram_part: 1 }
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

{% set widths = ['n', 'w'] %}
{% if gpram_part is defined %}
    {% set widths = widths[gpram_part:gpram_part+1] if gpram_part in [0, 1] else unreachable("Invalid gpram part") %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

{% for counter, save_func in [(CYCCNT, "save_times_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w  r1, {{counter}}
    ldr.w  r10, ={{save_func}}

{% for iter in [True, False] %}
    mov.w  r8, {{ 4 if iter else 0 }}
    mov.w  r9, {{ 2 if iter else 0 }}
    mov.w  r11, {{ 1 if iter else 0 }}

    bl.w   tested_code
{% endfor %}
{% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r7, lr
{% for width in widths %}
{% for adds in range(1, 5) %}
{% for reps in range(12) %}
    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

{% for i in range(reps) %}
    ldr.{{width}} r5, [r5]

    {% if adds == 1 %}
        adds.w r5, r5, r8
    {% elif adds == 2 %}
        adds.w r5, r5, r9
        adds.w r5, r5, r9
    {% elif adds == 3 %}
        adds.w r5, r5, r9
        adds.w r5, r5, r11
        adds.w r5, r5, r11
    {% elif adds == 4 %}
        adds.w r5, r5, r11
        adds.w r5, r5, r11
        adds.w r5, r5, r11
        adds.w r5, r5, r11
    {% else %}
        {{unreachable("invalid configuration, `adds` should be in range [1, 4]")}}
    {% endif %}
{% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
{% endfor %}
{% endfor %}
    @ Return to counters loop
    bx.n r7

initialize:
    @ Clear line buffer
    mov.w  r6, #0
    ldr.w  r5, [r6]

    @ Prepare LDR input value
    ldr.w  r5, =cell_0

    @ Clear flags
    msr.w apsr_nzcvq, r6

    bx.n lr

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
    and.w r2, r2, 0xFF

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{{ section(data) }}
.align 4
{% for i in range(12) %}
cell_{{i}}: .word cell_{{i}}
{% endfor %}

{% endblock %}
