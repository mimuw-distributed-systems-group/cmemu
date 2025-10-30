---
name: MLA and MLS instructions tests
description: "Timing and correctness test of MLA and MLS instructions"
dumped_symbols:
  results: 33 words
  times: 33 words
  flags: 33 words
configurations:
# MLA tests
- { code: gpram, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mla.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mla.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000001", r8Value: "#0x50000000", repetitions: 20, testedInstr: mla.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mla.w }
- { code: sram, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mla.w }
- { code: sram, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mla.w }
- { code: sram, lbEn: True, r7Value: "#0x00000001", r8Value: "#0x50000000", repetitions: 20, testedInstr: mla.w }
- { code: sram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mla.w }
- { code: flash, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mla.w }
- { code: flash, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mla.w }
- { code: flash, lbEn: True, r7Value: "#0x00000001", r8Value: "#0x50000000", repetitions: 20, testedInstr: mla.w }
- { code: flash, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mla.w }
- { code: flash, lbEn: False, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mla.w }
- { code: flash, lbEn: False, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mla.w }
- { code: flash, lbEn: False, r7Value: "#0x00000001", r8Value: "#0x50000000", repetitions: 20, testedInstr: mla.w }
- { code: flash, lbEn: False, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mla.w }
# MLS tests
- { code: gpram, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mls.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mls.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x30000000", repetitions: 20, testedInstr: mls.w }
- { code: gpram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mls.w }
- { code: sram, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mls.w }
- { code: sram, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mls.w }
- { code: sram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x30000000", repetitions: 20, testedInstr: mls.w }
- { code: sram, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mls.w }
- { code: flash, lbEn: True, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mls.w }
- { code: flash, lbEn: True, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mls.w }
- { code: flash, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x30000000", repetitions: 20, testedInstr: mls.w }
- { code: flash, lbEn: True, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mls.w }
- { code: flash, lbEn: False, r7Value: "#0x00000002", r8Value: "#0x00000001", repetitions: 33, testedInstr: mls.w }
- { code: flash, lbEn: False, r7Value: "#0x00000010", r8Value: "#0x00000003", repetitions: 15, testedInstr: mls.w }
- { code: flash, lbEn: False, r7Value: "#0x00000003", r8Value: "#0x30000000", repetitions: 20, testedInstr: mls.w }
- { code: flash, lbEn: False, r7Value: "#0x00000003", r8Value: "#0x00050005", repetitions: 25, testedInstr: mls.w }
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

    @ Prepare input values
    mov.w  r6, #0
    mov.w  r7, {{r7Value}}
    mov.w  r8, {{r8Value}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r6, r6, r7, r8
    {% endfor %}

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
