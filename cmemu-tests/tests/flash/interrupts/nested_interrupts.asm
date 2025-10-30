---
name: Nested interrupts (preempt)
description:
    Tests if nesting interrupts are properly handled.
    Used interrupts are nested or tail chained, depending on their priorities.
dumped_symbols:
  results: 96 words # 8 (priorities) * 3 (values) * 8 (prigroups) / 2
  times: 128 words # 8 (priorities) * 4 (times) * 8 (prigroups) / 2
  exccnts: 128 words # 8 (priorities) * 4 (times) * 8 (prigroups) / 2
configurations:
- { part: 0 }
- { part: 1 }
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

@ [ARM-ARM] B1.5.4 Priority grouping - PRIGROUP decides how many bits of
@ priority are used for subpriority.
{% set prigroup_range = range(0, 8) %}
@ [ARM-ARM] Table B3-4 Summary of SCB registers.
{% set aircr = 'E000ED0C'|int(base=16) %}
@ [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register, AIRCR.
{% set prigroup_bits = 2**10+2**9+2**8 %}
@ [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register, AIRCR.
{% set aircr_vectkey_write_value = '05FA'|int(base=16) %}

@ If #1 interrupt priority group < #0 interrupt priority group, nesting
@ interrupts happens. Otherwise tail chaining is executed.
{% set interrupt_0_priority = '0b01100000' %}
{% set interrupt_1_priorities = [
    '0b11100000',
    '0b11000000',
    '0b10100000',
    '0b10000000',
    '0b01100000',
    '0b01000000',
    '0b00100000',
    '0b00000000',
] %}

{% set ns = namespace(test_cases = []) %}
{% for prigroup in prigroup_range %}
{% for priority in interrupt_1_priorities %}
    {% set ns.test_cases = ns.test_cases + [(prigroup, priority)] %}
{% endfor %}
{% endfor %}

{% set test_cases_len = ns.test_cases | length %}
{% set test_parts = 2 %}
{% if 0 <= part < test_parts %} 
    {% set part_len = test_cases_len // test_parts if test_cases_len % test_parts == 0 else test_cases_len // test_parts + 1 %}
    @ None for last element to include the remaining few elements.
    {% set ns.test_cases = ns.test_cases[part_len * part : (part_len * (part+1) if part < test_parts - 1 else none)] %}
{% else %}
    {% set ns.test_cases = panic("invalid part") %}
{% endif %}

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

    @ Prepare dwt for tests
    ldr.w r3, dwt

    @ Save AIRCR
    ldr.w r0, ={{aircr}}
    ldr.w r0, [r0]
    ldr.w r1, =old_aircr
    str.w r0, [r1]

    @ Save and init msp/psp
    mrs.w r0, psp
    ldr.w r1, =old_psp
    str.w r0, [r1]

    mrs.w r0, msp
    ldr.w r1, =old_msp
    str.w r0, [r1]

    ldr.w r1, =process_stack_begin
    msr.w psp, r1

    ldr.w r0, =main_stack_begin
    msr.w msp, r0

    isb.w

    b.w tested_code

.thumb_func
end_label:
    @ Restore AIRCR
    ldr.w r0, ={{aircr}}
    ldr.w r1, =old_aircr
    ldr.w r1, [r1]
    {{prepare_aircr_vectkey_write_value(r1, r2)}}
    @ Save new AIRCR
    str.w r1, [r0]

    @ Restore psp/msp
    ldr.w r0, =old_psp
    ldr.w r0, [r0]
    msr.w psp, r0

    ldr.w r1, =old_msp
    ldr.w r1, [r1]
    msr.w msp, r1

    isb.w

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:

{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts")] %}
{% for prigroup, pri in ns.test_cases %}
    @ Set tested counter
    mov.w r11, {{counter}}
    @ Set registers to store interrupts priorities
    mov.w r0, #{{pri}}
    mov.w r2, #{{interrupt_0_priority}}
    mov.w r1, #{{prigroup}}

    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Read clock counter
    ldr.w r0, [r3, r11]

    @ Set pending interrupt
    str.w r2, [r1]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r4, #1
    .endr

    @ Read clock counter
    ldr.w r1, [r3, r11]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

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

    @ Prepare ADD register
    mov.w r4, #0

    @ Prepare registers for pending interrupt #0
    ldr.w r1, ={{NVIC.ISPR0}}
    mov.w r2, #{{interrupt_0_bit}}

    bx.n lr

save:
    @ Compute entry latency to the first interrupt
    sub.w r0, r5, r0
    @ Compute entry latency to the second interrupt
    sub.w r5, r6, r5
    @ Compute exit latency from the second interrupt
    sub.w r7, r8, r7
    @ Compute exit latency from the first interrupt
    sub.w r8, r1, r8

    {{saveValue("times", r0, r1, r2)}}
    {{saveValue("times", r5, r1, r2)}}
    {{saveValue("times", r7, r1, r2)}}
    {{saveValue("times", r8, r1, r2)}}
    {{saveValue("results", r4, r1, r2)}}
    {{saveValue("results", r9, r1, r2)}}
    {{saveValue("results", r10, r1, r2)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency to the first interrupt
    sub.w r0, r5, r0
    and.w r0, r0, 0xFF
    @ Compute stacking latency to the second interrupt
    sub.w r5, r6, r5
    and.w r5, r5, 0xFF
    @ Compute unstacking latency from the second interrupt
    sub.w r7, r8, r7
    and.w r7, r7, 0xFF
    @ Compute unstacking latency from the first interrupt
    sub.w r8, r1, r8
    and.w r8, r8, 0xFF

    {{saveValue("exccnts", r0, r1, r2)}}
    {{saveValue("exccnts", r5, r1, r2)}}
    {{saveValue("exccnts", r7, r1, r2)}}
    {{saveValue("exccnts", r8, r1, r2)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    ldr.w r5, [r3, r11]

    @ Save ADDs executed
    mov.w r9, r4

    @ Set pending interrupt 1
    mov.w r6, #{{interrupt_1_bit}}
    str.w r6, [r1]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r4, #1
    .endr

    ldr.w r8, [r3, r11]

    @ Without this ADD there is an unknown issue that causes wrong time of entry
    @ to the next handler. Maybe it's because of bad implementation of tail
    @ chaining or LDR + BX or something else. Until it's not solved let it be here.
    adds.n r4, #0

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    ldr.w r6, [r3, r11]

    @ Save number of ADDs that have been executed in previous exception
    mov.w r10, r4

    ldr.w r7, [r3, r11]

    bx.n lr

{{ section("sram") }}
.align 4
.global main_stack
main_stack:
@ Own stack is used because handlers use Main Stack.
.rept 20
    .word 0
.endr
main_stack_begin:
.size main_stack, .-main_stack

.align 4
.global process_stack
process_stack:
@ Own stack is used because the first stack frame is pushed onto process stack.
.rept 20
    .word 0
.endr
process_stack_begin:
.size process_stack, .-process_stack

old_msp: .word 0x0
old_psp: .word 0x0
old_aircr: .word 0x0

{% endblock %}
