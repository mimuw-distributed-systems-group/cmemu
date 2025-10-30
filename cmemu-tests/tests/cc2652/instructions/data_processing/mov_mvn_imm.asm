---
name: MOV and MVN (immediate) instructions tests
description: "Timing and correctness test of immediate MOV and MVN instructions"
dumped_symbols:
  results: 32 words
  times: 32 words
  flags: 32 words
configurations:
# MOV tests:
- { repetitions: 8, code: sram, lbEn: True, testedInstr: "movs.n" }
- { repetitions: 16, code: sram, lbEn: True, testedInstr: "movw.w" }
- { repetitions: 32, code: sram, lbEn: True, testedInstr: "mov.w" }
- { repetitions: 8, code: flash, lbEn: True, testedInstr: "movs.n" }
- { repetitions: 16, code: flash, lbEn: True, testedInstr: "movw.w" }
- { repetitions: 32, code: flash, lbEn: True, testedInstr: "mov.w" }
- { repetitions: 8, code: flash, lbEn: False, testedInstr: "movs.n" }
- { repetitions: 16, code: flash, lbEn: False, testedInstr: "movw.w" }
- { repetitions: 32, code: flash, lbEn: False, testedInstr: "mov.w" }
# MVN tests:
- { repetitions: 32, code: sram, lbEn: True, testedInstr: "mvns.w" }
- { repetitions: 32, code: sram, lbEn: True, testedInstr: "mvn.w" }
- { repetitions: 32, code: flash, lbEn: True, testedInstr: "mvns.w" }
- { repetitions: 32, code: flash, lbEn: True, testedInstr: "mvn.w" }
- { repetitions: 32, code: flash, lbEn: False, testedInstr: "mvns.w" }
- { repetitions: 32, code: flash, lbEn: False, testedInstr: "mvn.w" }
...
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
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{testedInstr}} r6, 0b1{% for i in range(reps) %}0{% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
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
