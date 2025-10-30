---
name: Interrupt latency and stacking/unstacking timings
description: "Measures interrupt latencies and stacking/unstacking timings"
dumped_symbols:
  results: 4 words
  times:   4 words
  exccnts: 4 words
configurations:
# All ways to exit from interrupt described in [ARM-ARM] B1.5.8 Exception return behavior.
- { handler_exit_instr: "push.w {lr}; pop.w {pc}" }
- { handler_exit_instr: "mov.w r1, lr; push.w {r1, lr}; pop.w {r1, pc}" }
- { handler_exit_instr: "mov.w r2, lr; ldr.w r1, =cells; stm.w r1, {r2, lr}; ldm.w r1, {r2, pc}" }
- { handler_exit_instr: "ldr.w r1, =cells; str.w lr, [r1]; ldr.w pc, [r1]" }
- { handler_exit_instr: "bx.n lr" }
- { handler_exit_instr: "mov.n r1, lr; bx.n r1" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ Interrupt #30 has (16 + 30) = 46 exception number.
{% set interrupt_30_exc_number = 46 %}

{% block code %}
    {{setExceptionHandler(interrupt_30_exc_number, "Irq30_Handler", r3, r4)}}
    {{enableException(interrupt_30_exc_number, r3, r4)}}
    {{calculateAddressAndValueForSettingExceptionPending(interrupt_30_exc_number, r5, r4)}}

    @ Prepare all registers for tests
    ldr.w r3, dwt

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Reset SP
    mov.w sp, r11

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts")] %}
{% for instr in ["add.w", "adds.n"] %}
    @ Used by interrupt handler to save currently dumped counter
    mov r10, {{counter}}
    @ Init ADD value.
    mov.w r7, #0
    @ Read counter
    ldr.w r0, [r3, r10]
    @ Set pending interrupt
    str.w r5, [r4]

    @ Give processor some time to process interrupt
    .rept 8
        {{instr}} r7, #1
    .endr

    @ Read counter
    ldr.w r8, [r3, r10]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    @ Compute entry latency
    sub.w r0, r6, r0
    @ Compute exit latency
    sub.w r6, r8, r6

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r6, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ EXCCNT is 8-bit wide
    @ Compute unstacking latency
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq30_Handler, %function
Irq30_Handler:
    ldr.w r6, [r3, r10]

    @ Save number of ADDs that have been executed before interrupt.
    mov.w r9, r7

    {{handler_exit_instr}}

.align 4
cells: .word 0x0, 0x0
{% endblock %}
