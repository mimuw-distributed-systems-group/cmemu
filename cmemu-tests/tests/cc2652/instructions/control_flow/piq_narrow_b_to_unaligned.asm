---
name: Test narrow decode-time branches to unaligned addresses during waitstates
description: >
    We observed that a decode-time branch after a multi-cycle instruction is not filling the PIQ with 3 full words
    after jump -> it seems to be just 2 words. This is a small test showing that.
    This test should prove whether PIQ uses word- or short- accounting for the "3 words long queue"
    This seems to be the only place where it matters: i.e. we can clearly test when fetch is stalled during multi-cycle
    instructions.
dumped_symbols:
  times: auto
  results: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations: []
product:
    - code: [flash, ]
      lbEn: [True, False]
      branch_instr: ['b.n', 'b.w']
      dest_instr: ['add.w r5, #1', 'adds.n r5, #1']
      distance: [10, 0, 2, 4, 6, 8, ]
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
{% for alignment in range(0, 5) %}
{% for stall_cyc in range(5, 10) %}
{% for pre_stall_cyc in [0, 3] %}
{% for pad in range(5) %}
    {% set jump_label = uniq_label("jump_target") %}
  {% set x_loader1, x_word_exec1 = n_x_cycles(pre_stall_cyc, "r7", "r8") %}
  {% set x_loader2, x_word_exec2 = n_x_cycles(stall_cyc, "r3", "r4") %}
    @ Prepare add arguments
    mov.w  r5, #42
    {{ x_loader1 }}
    {{ x_loader2 }}

    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Test various alignemnts
    {% for _ in range(alignment) %}
    adds.n r5, #1
    {% endfor %}

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {{ x_word_exec1 }}
    {{ x_word_exec2 }}


    @ Decode time branch
    {{ branch_instr }} {{ jump_label }}

    @ These should not execute
    {% for _ in range(distance//2) %}
    adds.n r5, #1
    {% endfor %}

    @ Test various distances
{{ jump_label }}:
    @ Check if landing on a narrow instruction would free a slot in PIQ
    {{ dest_instr }}
    {% for _ in range(pad) %}
    add.w  r5, #1
    {% endfor %}
    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}

    {% if counter == CYCCNT %} {{ inc_auto_syms() }} {% endif %}
{% endfor %}
{% endfor %}
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

{% endblock %}
