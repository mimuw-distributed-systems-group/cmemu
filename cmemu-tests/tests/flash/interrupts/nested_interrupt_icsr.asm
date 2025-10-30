---
name: ICSR in nested interrupts
description:
    Tests value of ICSR::RETTOBASE in nested interrupts.
dumped_symbols:
  results: 4 words
configurations: 
- {}
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ #0 and #1 interrupts are used in test.
{% set interrupt_0_bit = '0b1' %}
{% set interrupt_1_bit = '0b10' %}
@ Interrupt #n has (16 + n) exception number.
{% set interrupt_0_exc_number = 16 %}
{% set interrupt_1_exc_number = 17 %}

@ [ARM-ARM] B3.2.2
{% set icsr_addr = 'E000ED04'|int(base=16) %}
@ [ARM-ARM] B1.5.4 Priority grouping - PRIGROUP decides how many bits of
@ priority are used for subpriority.
{% set prigroup = '0b00000001' %}
@ [ARM-ARM] Table B3-4 Summary of SCB registers.
{% set aircr = 'E000ED0C'|int(base=16) %}
@ [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register, AIRCR.
{% set prigroup_bits = 2**10+2**9+2**8 %}
@ [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register, AIRCR.
{% set aircr_vectkey_write_value = '05FA'|int(base=16) %}

@ If #1 interrupt priority group < #0 interrupt priority group, nesting
@ interrupts happens. Otherwise tail chaining is executed.
{% set interrupt_0_priority = '0b01100000' %}
{% set interrupt_1_priority = '0b00000000' %}

@ [TI-TRM] Table 2-125 - "Writing to this register (AIRCR) requires 0x05FA in VECTKEY."
@ Updates value in `register_with_value` to store correct value in VECTKEY field.
{% macro prepare_aircr_vectkey_write_value(register_with_value, tmp_register) %}
    @ Clear AIRCR.VECTKEY
    ldr.w {{tmp_register}}, =0xFFFF0000
    bic.w {{register_with_value}}, {{tmp_register}}
    @ Set ARICR.VECTKEY
    mov.w {{tmp_register}}, #{{aircr_vectkey_write_value}}
    orr.w {{register_with_value}}, {{register_with_value}}, {{tmp_register}}, LSL #16
{% endmacro %}

{% block code %}
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{setExceptionHandler(17, "Irq1_Handler", r3, r4)}}

    {{enableException(interrupt_0_exc_number, r3, r4)}}
    {{enableException(interrupt_1_exc_number, r3, r4)}}

    b.w tested_code

.thumb_func
end_label:

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
    @ Store ICSR address for further usage
    ldr.w r7, ={{icsr_addr}}

    @ Store ICSR value
    ldr.w r8, [r7]

    @ Set registers to store interrupts priorities
    mov.w r0, #{{interrupt_1_priority}}
    mov.w r2, #{{interrupt_0_priority}}
    mov.w r1, #{{prigroup}}

    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Set pending interrupt
    str.w r2, [r1]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r4, #1
    .endr

    @ Store ICSR value
    ldr.w r11, [r7]

    bl.w save
    b.w end_label

initialize:
    @ Read value of AIRCR
    ldr.w r4, ={{aircr}}
    ldr.w r4, [r4]
    @ Clear AIRCR.PRIGROUP
    mov.w r5, #{{prigroup_bits}}
    bic.w r4, r5
    @ Write new AIRCR.PRIGROUP
    orr.w r4, r4, r1, LSL #8
    {{prepare_aircr_vectkey_write_value(r4, r5)}}
    ldr.w r1, ={{aircr}}
    @ Save new AIRCR
    str.w r4, [r1]

    @ Set interrupts priorities
    ldr.w r1, ={{NVIC.IPR0}}
    add.w r2, r2, r0, LSL #8
    str.w r2, [r1]

    @ Prepare registers for pending interrupt #0
    ldr.w r1, ={{NVIC.ISPR0}}
    mov.w r2, #{{interrupt_0_bit}}

    bx.n lr

save:
    {{saveValue("results", r8, r2, r3)}}
    {{saveValue("results", r9, r2, r3)}}
    {{saveValue("results", r10, r2, r3)}}
    {{saveValue("results", r11, r2, r3)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    @ Store ICSR value
    ldr.w r9, [r7]

    @ Set pending interrupt 1
    mov.w r6, #{{interrupt_1_bit}}
    str.w r6, [r1]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r4, #1
    .endr

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    @ Store ICSR value
    ldr.w r10, [r7]

    bx.n lr

{% endblock %}
