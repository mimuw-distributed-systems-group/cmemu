---
name: Modifies CONTROL with MSR
description: Timing and correctness test of "msr CONTROL, rX" and "mrs rX, CONTROL" instruction.
dumped_symbols:
  write_times: 5 words
  read_times: 5 words
  results: 5 words
  control_before_execution: 5 words
  control_mutable_after_execution: 5 words
configurations:
# After setting control[0] = 1 processor enters unprivileged mode and block sucessive writes to control register,
# thus each configuration with such bit is handled as separate case.
# Assumes that intial CONTROL value is 0x2 with privileged access and thread stack pointer.
- { code: "gpram", lbEn: true, new_control_values: [0,2,4,8,16] }
- { code: "sram", lbEn: true, new_control_values: [0,2,4,8,16] }
- { code: "flash", lbEn: true, new_control_values: [0,2,4,8,16] }
- { code: "flash", lbEn: false, new_control_values: [0,2,4,8,16] }

- { code: "gpram", lbEn: true, new_control_values: [1] }
- { code: "sram", lbEn: true, new_control_values: [1] }
- { code: "flash", lbEn: true, new_control_values: [1] }
- { code: "flash", lbEn: false, new_control_values: [1] }

- { code: "gpram", lbEn: true, new_control_values: [3] }
- { code: "sram", lbEn: true, new_control_values: [3] }
- { code: "flash", lbEn: true, new_control_values: [3] }
- { code: "flash", lbEn: false, new_control_values: [3] }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}

@ After ROM initialization user code is executed in privileged mode with thread stack pointer (CONTROL = 0x2)
{% set defaultControlValue = 2 %}
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
{% for val in new_control_values %}
    mov.w r8, #{{val}}
    mov.w r7, #0

    @ Get initial value
    mrs.w r5, CONTROL

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Privileged mode
    {% if val % 2 == 0 %}
        @ Get start time
        ldr.w  r1, [r0, {{CYCCNT}}]

        msr.w CONTROL, r8

        @ Get write time
        ldr.w  r2, [r0, {{CYCCNT}}]

        mrs.w r7, CONTROL

        @ Get read time
        ldr.w  r3, [r0, {{CYCCNT}}]

    @ In unprivileged mode, DWT cannot be accessed to measure time.
    {% else %}
        mov.w r1, #0
        mov.w r2, #0
        mov.w r3, #0

        msr.w CONTROL, r8

        @ [ARM-ARM] B5.2.3 Note: downgrading privilege requires synchronisation barrier
        {{ 'isb.w' }} @ literal, not to confuse test evaluator

        mrs.w r7, CONTROL

    {% endif %}
    bl.w save

    @ Set default control values again to check if it is mutable in current mode
    mov.w r6, {{defaultControlValue}}
    msr.w CONTROL, r6
    isb.w
    mrs.w r6, CONTROL

    bl.w save_before_after_values

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
    subs.n r1, r2, r1
    subs.n r2, r3, r2

    {{ saveValue('write_times', r1, r3, r4) }}
    {{ saveValue('read_times', r2, r3, r4) }}
    {{ saveValue('results', r7, r3, r4) }}

    bx.n lr

.thumb_func
save_before_after_values:
    {{ saveValue('control_before_execution', r5, r3, r4) }}
    {{ saveValue('control_mutable_after_execution', r6, r3, r4) }}

    bx.n lr

.align 4
.thumb_func
.type   Irq30_Handler, %function
Irq30_Handler:
    @ Restore PRIVILEGED mode
    mov.w   r6, #2
    msr.w   CONTROL, r6
    bx.n lr

{% endblock %}
