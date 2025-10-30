---
name: B and LDR interaction tests
description: "Timing test for LDR+B combination, LDR should give prefetch queue more time to fill."
dumped_symbols:
  results: 10 words
  times: 10 words
  flags: 10 words
  cpicnts: 10 words
  lsucnts: 10 words
configurations:
- { code: "sram", memory: "sram", lbEn: True }
- { code: "sram", memory: "flash", lbEn: True }
- { code: "sram", memory: "flash", lbEn: False }
- { code: "flash", memory: "sram", lbEn: True }
- { code: "flash", memory: "flash", lbEn: True }
- { code: "flash", memory: "sram", lbEn: False }
- { code: "flash", memory: "flash", lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
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
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for bInstr in ["b.n", "b.w"] %}
{% for pad in range(5) %}
    {% set jump_label = uniq_label("jump_target") %}
    {% set skip_label = uniq_label("skip_target") %}
    @ Prepare add arguments
    mov.w  r5, #42
    @ Prepare ldr arguments
    ldr.w  r7, =memory_cell

    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    @ ldr gives prefetch queue time to fill
    ldr.w  r6, [r7]

    @ Execute branch
    {{bInstr}} {{ jump_label }}

    @ Branch not taken
    {% for _ in range(pad) %}
    add.w  r5, #1
    {% endfor %}
    @ Get finish time
    ldr.n  r3, [r0, {{counter}}]
    b.n {{ skip_label }}

.align 4
{{ jump_label }}:
    {% for _ in range(pad) %}
    add.w  r5, #1
    {% endfor %}
    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

{{ skip_label }}:
    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_flags_and_result:
    mrs.w r6, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    {{saveValue("flags", r6, r3, r4)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
memory_cell: .word 0x0

{% endblock %}
