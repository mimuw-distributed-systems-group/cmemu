---
name: Raising interrupt by writing to STIR.
description: >
  Measures interrupt latencies and stacking/unstacking timings for interrupt
  raised by writing to STIR.
dumped_symbols:
  results: 6 words # 3 (tested exception numbers) * 2 (saved values)
  times:   6 words
  exccnts: 6 words
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ Interrupt #3 has (16 + 3) = 19 exception number.
{% set interrupt_3_exc_number = 19 %}
@ Interrupt #30 has (16 + 30) = 46 exception number.
{% set interrupt_30_exc_number = 46 %}
{% set interrupt_30_number_with_reserved_bits = 286 %}
@ Interrupt should be handled only for #3 and #30.
{% set tested_interrupts_numbers = [
    3,
    30,
    interrupt_30_number_with_reserved_bits
] %}
{% set stir = "E000EF00"|int(base=16) %}

{% block code %}
    {{setExceptionHandler(interrupt_3_exc_number, "Irq_Handler", r3, r4)}}
    {{enableException(interrupt_3_exc_number, r3, r4)}}
    {{setExceptionHandler(interrupt_30_exc_number, "Irq_Handler", r3, r4)}}
    {{enableException(interrupt_30_exc_number, r3, r4)}}

    @ Prepare DWT for tests
    ldr.w r0, dwt

    b.w tested_code

.thumb_func
end_label:

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts")] %}
{% for int_num in tested_interrupts_numbers %}
    @ Used by interrupt handler to save currently dumped counter
    mov r10, {{counter}}
    @ Init ADD value.
    mov.w r7, #0
    @ Prepare registers for test
    mov.w r9, #0
    mov.w r1, #{{int_num}}
    ldr.w r2, ={{stir}}

    @ Read counter
    ldr.w r3, [r0, r10]
    @ Set pending interrupt
    str.w r1, [r2]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r7, #1
    .endr

    @ Read counter
    ldr.w r4, [r0, r10]

    @ If exception hasn't been raised, then r5 stores invalid value
    cmp.w r9, #0
    it.n eq
    moveq.w r5, r4

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    @ Compute entry latency
    sub.w r3, r5, r3
    @ Compute exit latency
    sub.w r5, r4, r5

    {{saveValue("times", r3, r2, r1)}}
    {{saveValue("times", r5, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency
    sub.w r3, r5, r3
    and.w r3, r3, 0xFF  @ EXCCNT is 8-bit wide
    @ Compute unstacking latency
    sub.w r5, r4, r5
    and.w r5, r5, 0xFF

    {{saveValue("exccnts", r3, r2, r1)}}
    {{saveValue("exccnts", r5, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq_Handler, %function
Irq_Handler:
    ldr.w r5, [r0, r10]

    @ Save number of ADDs that have been executed before interrupt.
    mov.w r9, r7

    bx.n lr

{% endblock %}
