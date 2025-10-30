---
name: ADR instruction tests
description: "Timing and correctness test"
dumped_symbols:
  results: 15 words
  times: 15 words
  flags: 15 words
configurations:
- { code: sram, lbEn: True, instr: "adr.n" }
- { code: sram, lbEn: True, instr: "adr.w" }
- { code: flash, lbEn: True, instr: "adr.n" }
- { code: flash, lbEn: True, instr: "adr.w" }
- { code: flash, lbEn: False, instr: "adr.n" }
- { code: flash, lbEn: False, instr: "adr.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ `add == FALSE` is available only to wide `adr` instruction
{% set addFlagValues = ([True, False] if instr == 'adr.w' else [True]) %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

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
{% for add in addFlagValues %}
{% for reps in range(1, 8) %}
    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

{% if not add %}
.align 2
label_{{reps}}_0:
{% endif %}
    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{instr}} r6, label_{{reps}}_{{1 if add else 0}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
{% if add %}
.align 2
label_{{reps}}_1:
{% endif %}
    bl.w save


{% endfor %}
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(r6, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

{% endblock %}
