---
name: LDR+STR DWT counters tests
description: Correctness test storing to and reading from DWT counters
dumped_symbols:
  results: 6 words
configurations:
- { code: "gpram", lbEn: true }
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set counters = [CYCCNT, CPICNT, EXCCNT, SLEEPCNT, LSUCNT, FOLDCNT] %}

{% block code %}
    @ Prepare base counter address
    ldr.w  r0, dwt

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for cnt in counters %}
    @ Prepare input value
    mov.w  r2, #78

    @ Store some constant to counter, so that starting value is defined
    mov.w  r1, #42
    str.w  r1, [r0, {{cnt}}]

    @ Align and clear PIQ
    .align 4
    isb.w

    str.w  r2, [r0, {{cnt}}]
    ldr.w  r3, [r0, {{cnt}}]

    bl.w save
{% endfor %}

    b.w end_label

save:
    {{saveResult(r3, r9, r10)}}

    bx.n lr
{% endblock %}
