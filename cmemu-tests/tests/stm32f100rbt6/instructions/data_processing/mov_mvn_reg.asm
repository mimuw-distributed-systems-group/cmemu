---
name: MOV and MVN from register instructions tests
description: "Timing and correctness test of MOV and MVN from register instructions"
dumped_symbols:
  results: 45 words
  times: 45 words
  flags: 45 words
configurations:
# MOV tests:
- { code: sram, lbEn: True, testedInstr: "movs.n" }
- { code: sram, lbEn: True, testedInstr: "mov.n" }
- { code: sram, lbEn: True, testedInstr: "mov.w" }
- { code: flash, lbEn: True, testedInstr: "movs.n" }
- { code: flash, lbEn: True, testedInstr: "mov.n" }
- { code: flash, lbEn: True, testedInstr: "mov.w" }
- { code: flash, lbEn: False, testedInstr: "movs.n" }
- { code: flash, lbEn: False, testedInstr: "mov.n" }
- { code: flash, lbEn: False, testedInstr: "mov.w" }
# MVN tests:
- { code: sram, lbEn: True, testedInstr: "mvns.n" }
- { code: sram, lbEn: True, testedInstr: "mvn.w" }
- { code: sram, lbEn: True, testedInstr: "mvn.w" }
- { code: flash, lbEn: True, testedInstr: "mvns.n" }
- { code: flash, lbEn: True, testedInstr: "mvn.w" }
- { code: flash, lbEn: True, testedInstr: "mvn.w" }
- { code: flash, lbEn: False, testedInstr: "mvns.n" }
- { code: flash, lbEn: False, testedInstr: "mvn.w" }
- { code: flash, lbEn: False, testedInstr: "mvn.w" }
...
{% set repetitions = 5 %}
{% set tested_registers = ["r5", "r6", "r7"] %}

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
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
{% for reps in range(repetitions) %}
{% for regA in tested_registers %}
{% for regB in tested_registers %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare mov inputs
    mov.w  {{regA}}, 0x0
    mov.w  {{regB}}, 0x2A

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} {{regA}}, {{regB}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    mov.w r6, {{regA}}
    bl.w save
{% endfor %}
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
