---
name: NOP instruction tests
description: "Timing test of nop instruction"
dumped_symbols:
  times: 100 words
configurations:
- { code: "gpram", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.n" }
- { code: "gpram", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.w" }
- { code: "sram", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.n" }
- { code: "sram", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.w" }
- { code: "flash", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.n" }
- { code: "flash", lbEn: true, repetitions: 10, pad: "", nopInstr: "nop.w" }
- { code: "flash", lbEn: true, repetitions: 10, pad: "add.w r7, #0", nopInstr: "nop.n" }
- { code: "flash", lbEn: true, repetitions: 10, pad: "add.w r7, #0", nopInstr: "nop.w" }
- { code: "flash", lbEn: false, repetitions: 10, pad: "", nopInstr: "nop.n" }
- { code: "flash", lbEn: false, repetitions: 10, pad: "", nopInstr: "nop.w" }
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

{% for piq_fill in range(9) %}
{% set loader, word_exec_stall = n_x_cycles(piq_fill, "r8", "r9") %}
{% for reps in range(repetitions) %}
    {{ loader }}
    @ Align and clear PIQ
    .align 3
    isb.w

    {{ pad }}

    @ Get start time
    ldr.w  r2, [r0, r1]

    {{ word_exec_stall }}
    {% for i in range(reps) %}
        {{nopInstr}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}

    b.w end_label

.align 2
save:
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}

    bx.n lr
{% endblock %}
