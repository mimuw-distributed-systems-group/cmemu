---
name: Test XOsc switch.
description: Observe XOsc timings.
dumped_symbols:
  times: 1 words
configurations:
- { code: "gpram", lbEn: true }
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
.extern NOROM_OSCHF_TurnOnXosc
.extern NOROM_OSCHF_AttemptToSwitchToXosc

.align 4
.thumb_func
.type tested_code, %function
tested_code:

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    @ Turn on XOsc
    bl.w NOROM_OSCHF_TurnOnXosc

    @ Switch to XOsc
.align 2
xosc_loop:
    @ Try to switch oscillator
    bl.w NOROM_OSCHF_AttemptToSwitchToXosc
    @ Check if switch successful, if not - jump back
    cmp r0, #0
    beq.n xosc_loop

    @ Get finish time
    ldr.w r3, [r0, {{CYCCNT}}]

    bl.w save

    b.w end_label

.align 2
save:
    sub.w r2, r3, r2
    {{saveValue("times", r2, r3, r4)}}
    bx.n lr
{% endblock %}
