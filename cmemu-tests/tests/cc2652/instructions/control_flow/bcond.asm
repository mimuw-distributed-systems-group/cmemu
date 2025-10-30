---
name: B<c> timing and correctness
description: "Timing and correctness of conditional branch (outside IT block)."
dumped_symbols:
  results: 12 words
  times: 12 words
  flags: 12 words
  cpicnts: 12 words
  lsucnts: 12 words
configurations:
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: false }
- { code: "flash", lbEn: true }
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

{% for width in ["n", "w"] %}
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for makeAPSRDirty in [False, True] %}
{% for cond in ["", "eq", "ne"] %}
    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Clear jump guard value
    mov.w r5, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {% if makeAPSRDirty %}
        @ Clear zero flag making APSR kinda "dirty",
        @ to check if b<cond> always executes in decode phase
        movs.n r7, #1
    {% endif %}

    @ Execute branch
    b{{cond}}.{{width}} jump_target_{{makeAPSRDirty}}_{{cond}}_{{counter_idx}}_{{width}}

    @ Executes if not jumped
    add.w  r5, 1
    add.w  r5, 1
    add.w  r5, 1
    add.w  r5, 1

.align 4
jump_target_{{makeAPSRDirty}}_{{cond}}_{{counter_idx}}_{{width}}:
    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

.align 2
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
{% endblock %}
