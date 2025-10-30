---
name: PUSH instruction tests
description: Timing and correctness test
dumped_symbols: 
  results: 120 words
  times: 120 words
  stack: user-defined
configurations:
# - { code: sram, lbEn: true }
- { code: flash, lbEn: false }
- { code: flash, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set register_sets = [
        ["r0"],
        ["r1"],
        ["r2"],
        ["r8"],
        ["r9"],
        ["lr"],
        ["r0", "r1"],
        ["r0", "r2"],
        ["r1", "r2"],
        ["r8", "r9"],
        ["r0", "r8"],
        ["r0", "lr"],
        ["r8", "lr"],
        ["r0", "r1", "r2"],
        ["r0", "r8", "lr"],
        ["r8", "r9", "lr"],
        ["r0", "r1", "r2", "r8"],
        ["r0", "r1", "r2", "r8", "r9"],
        ["r0", "r1", "r2", "r8", "r9", "lr"],
    ]
%}

{% block code %}
    @ Prepare cycle counter address
    ldr.w  r6, dwt

    @ Switch to our own stack
    mov.w r11, sp
    ldr.w sp, =stack_begin

    b.w    tested_code
.thumb_func
end_label:
    @ Recover the original stack pointer
    mov.w sp, r11
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for registers in register_sets %}
{% for pad in ["", "adds.n r7, #0", "add.w r7, #0"] %}
{% for call_dest in (True, False) %} @ Simulate conditions on function entry
    @ Decide the width of PUSH instruction, prefer narrow encodings
    {% set width = 'w' if 'r8' in registers or 'r9' in registers else 'n' %}

    @ Prepare input values
    {% for i in range(registers|length) %}
        mov.w {{ registers[i] }}, {{i+1}}
    {% endfor %}

    @ Store initial SP value
    mov.w r10, sp

    @ Align and clear PIQ
    .align 4
    isb.w
    {{ pad }}

    @ Get start time
    ldr.w  r4, [r6, {{CYCCNT}}]

    {% if call_dest %}
        bl.w .+12
        nop.w
        nop.w
    {% endif %}

    push.{{width}} { {{ registers|join(", ") }} }
    
    @ Finish time
    ldr.w  r5, [r6, {{CYCCNT}}]
    
    @ Finish time - Start time
    sub.w r5, r5, r4
    @ Initial SP - Current SP
    mov.w r4, sp
    sub.w r10, r10, r4

    @ Save measurements
    bl.w save

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.thumb_func
save:
    {{ saveValue("times", r5, r3, r4) }}
    {{ saveValue("results", r10, r3, r4) }}
    bx.n lr

{{ section("sram") }}
.align 4
.global stack
stack:
@ The stack will finally contain values written by all PUSH instructions from the test.
@ You can distinguish which PUSH written each value by inspecting values saved in `results` array
{% for i in range(50*2*3) %}
    .word 0
{% endfor %}
stack_begin:
.size stack, .-stack
{% endblock %}
