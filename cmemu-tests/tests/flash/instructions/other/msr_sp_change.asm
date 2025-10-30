---
name: Modifies CONTROL with MSR
description: >-
    Detecting and measuring time of change of Stack Pointer value after writing to CONTROL register.
    [TI-TRM] 2.5.2.21 claims that ISB must be executed to properly propagate this change.
    Assumes that intial CONTROL value is 0x2 with privileged access and thread stack pointer.
dumped_symbols:
  control_after_execution: 32 words
  control_before_execution: 32 words
  has_sp_changed: 32 words
configurations:
# Control values are thread (0x2) and main (0x0) stack pointer.
- { code: "gpram", lbEn: true, new_control_values: [0,2] }
- { code: "sram", lbEn: true, new_control_values: [0,2] }
- { code: "flash", lbEn: true, new_control_values: [0,2] }
- { code: "flash", lbEn: false, new_control_values: [0,2] }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}

@ Interrupt #30 has (16 + 30) = 46 exception number.
{% set interrupt_30_exc_number = 16 + 30 %}
{% set software_trigger_interrupt = '0xE000EF00' %}
@ For allowing unprivileged code to trigger software interrupts
{% set configuration_control_register = '0xE000ED14' %}
{% set user_set_m_pend_mask = '0x000002' %}


{% extends "asm.s.tpl" %}

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
{% for nop_count in range(16) %}
    {% for val in new_control_values %}
        mov.w r8, #{{val}}
        mov.w r7, #0

        @ Align and clear PIQ
        .align 4
        isb.w

        @ Get initial values
        mrs.w r5, CONTROL
        mov.w r6, sp

        msr.w CONTROL, r8

        {% for _ in range(nop_count) %}
            nop.n
        {% endfor %}

        @ Get changed values
        mov.w r9, sp
        mrs.w r7, CONTROL

        bl.w save
    {% endfor %}
{% endfor %}
    @ Raise interrupt to restore PRIVILEGE
    mov.w r5, #{{interrupt_30_exc_number}}-16
    ldr.w r4, ={{software_trigger_interrupt}}
    str.w r5, [r4]

    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr

    b.w end_label

.thumb_func
save:
    cmp.n  r6, r9
    ite ne
    movne.w r2, #1
    moveq.w r2, #0
    {{ saveValue('control_before_execution', r5, r3, r4) }}
    {{ saveValue('control_after_execution', r7, r3, r4) }}
    {{ saveValue('has_sp_changed', r2, r3, r4) }}

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
