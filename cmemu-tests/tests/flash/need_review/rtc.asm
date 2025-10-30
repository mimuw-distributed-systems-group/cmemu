---
name: Test RTC timings.
description: Observe basic RTC timings.
dumped_symbols:
  times: 3 words
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
    # RTC base address
    ldr.w r7, rtc_base

{% for reg_offset in ["0x08", "0x0c", "0x2C"] %}
    @ RTC register's address
    mov.w r8, r7
    add.w r8, #{{reg_offset}}

    @ Get start time
    ldr.w r2, [r0, {{CYCCNT}}]

    ldr.w r9, [r8]

    @ Get finish time
    ldr.w r3, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
    b.w end_label

.align 2
save:
    sub.w r2, r3, r2
    {{saveValue("times", r2, r3, r4)}}
    bx.n lr

rtc_base:
    .word 0x40092000

{% endblock %}
