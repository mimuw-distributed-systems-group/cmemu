---
name: Modifies and reads MSP and PSP with MSR/MRS
description: >-
    Timing and correctness test of "MSR MSP/PSP, rX" and "MRS rX, MSP/PSP" instruction.
    Uses different values, targets and execution privilege modes.
dumped_symbols:
  write_times: 32 words
  read_times: 32 words
  results: 32 words
configurations:
- { code: "gpram", lbEn: true }
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}

{% extends "asm.s.tpl" %}

{% set values = ["0x1000000", "0x2000000", "0x4000000", "0x6000000", "0x8000000", "0xA000000", "0xC000000", "0xE000000"] %}
{% set interrupt_30_exc_number = 16 + 30 %}
{% set software_trigger_interrupt = '0xE000EF00' %}
@ For allowing unprivileged code to trigger software interrupts
{% set configuration_control_register = '0xE000ED14' %}
{% set user_set_m_pend_mask = '0x000002' %}


{% block code %}
    {{setExceptionHandler(interrupt_30_exc_number, "Irq30_Handler", r4, r5)}}
    {{enableException(interrupt_30_exc_number, r4, r5)}}
    @ Enable our way of resetting privileges
    @ [ARM-ARM] B3.2.8 Configuration and Control Register, CCR
    @ Allow unprivileged code to write to the STIR register
    ldr.w r0, ={{configuration_control_register}}
    ldr.w r1, [r0]
    orr.w r1, {{user_set_m_pend_mask}}
    str.w r1, [r0]
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

@ First test in privileged mode (CONTROL[0] == 0) then in unprivileged mode, both using PSP
{% for privilege in [2, 3] %}
    mov.w r8, #{{privilege}}
    msr.w CONTROL, r8
    isb.w

{% for target in ["MSP", "PSP"] %}
    @ Save initial value
    mrs.w r11, {{target}}

{% for val in values %}
    mov.w r8, #{{val}}
    mov.w r7, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Privileged mode
    {% if privilege % 2 == 0 %}
        @ Get start time
        ldr.w  r1, [r0, {{CYCCNT}}]

        msr.w {{target}}, r8

        @ Get write time
        ldr.w  r2, [r0, {{CYCCNT}}]

        mrs.w r7, {{target}}

        @ Get read time
        ldr.w  r3, [r0, {{CYCCNT}}]

    @ In unprivileged mode, DWT cannot be accessed to measure time.
    {% else %}
        mov.w r1, #0
        mov.w r2, #0
        mov.w r3, #0

        msr.w {{target}}, r8

        @ [ARM-ARM] B5.2.3 Note: downgrading privilege requires synchronisation barrier
        {{ 'isb.w' }} @ literal, not to confuse test evaluator

        mrs.w r7, {{target}}

    {% endif %}
    bl.w save

    @ Restore initial value
    msr.w {{target}}, r11

    @ Raise interrupt to restore PRIVILEGE
    mov.w r6, #{{interrupt_30_exc_number}}-16
    ldr.w r4, ={{software_trigger_interrupt}}
    str.w r6, [r4]

    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr

{% endfor %}

{% endfor %}
{% endfor %}
    b.w end_label

.thumb_func
save:
    sub.w r1, r2, r1
    sub.w r2, r3, r2

    {{ saveValue('write_times', r1, r3, r4) }}
    {{ saveValue('read_times', r2, r3, r4) }}
    {{ saveValue('results', r7, r3, r4) }}
    bx.n lr

.align 4
.thumb_func
.type	Irq30_Handler, %function
Irq30_Handler:
    @ Restore PRIVILEGED mode
    mov.w   r6, #2
    msr.w   CONTROL, r6
    bx.n lr

{% endblock %}
