---
name: Tail chaining of the same interrupt
description:
    Tests feature of tail chaining the same interrupt. When the same interrupt
    is pended while being active, tail chain should be executed instead of
    unstacking and stacking.
dumped_symbols:
  results: 2 words
  times: 3 words
  exccnts: 3 words
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ The first interrupt is used for test.
{% set tested_interrupt_bits = "0b1" %}

{% block code %}
    @ Setup interrupt #0 handler
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{enableException(16, r3, r4)}}

    @ Prepare dwt for tests
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
    mov.w r12, {{counter}}

    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get time before interrupt handling
    ldr.w r0, [r3, r12]

    @ Set pending interrupt
    str.w r5, [r4]

    @ Give processor some time to process interrupt
    .rept 8
        adds.n r7, #1
    .endr

    @ Get time after interrupt handling
    ldr.w r8, [r3, r12]

    bl.w {{save_func}}
{% endfor %}

    b.w end_label

initialize:
    @ Init ADD value and register that will save the number of handler executions
    mov.w r7, #0
    mov.w r9, #0

    @ Prepare registers for setting interrupt into pending state
    ldr.w r4, ={{NVIC.ISPR0}}
    mov.w r5, #{{tested_interrupt_bits}}

    bx.n lr

save:
    @ Compute entry latency to the first handler
    sub.w r0, r4, r0
    @ Compute latency between the first and second handler executions
    sub.w r4, r6, r4
    @ Compute exit latency
    sub.w r6, r8, r6

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r4, r2, r1)}}
    {{saveValue("times", r6, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency to the first handler
    sub.w r0, r4, r0
    and.w r0, r0, 0xFF
    @ Compute exccnts latency between the first and second handler executions
    sub.w r4, r6, r4
    and.w r4, r4, 0xFF
    @ Compute unstacking latency
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r4, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    @ Check if interrupt has executed
    cmp.w r9, #0
    itte.n eq

    @ If not, set interrupt to pending state again and read counter.
    streq.w r5, [r4]
    ldreq.w r4, [r3, r12]

    @ Otherwise, only read counter.
    ldrne.w r6, [r3, r12]

    @ Increase number of executions
    add.w r9, #1

    bx.n lr

{% endblock %}
