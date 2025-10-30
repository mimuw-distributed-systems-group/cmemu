---
name: Tail chaining
description:
    Tests feature of tail chaining. It is optimization when the first interrupt
    handling is finished and some other is pended, unstacking and stacking are
    not executed.
dumped_symbols:
  results: 12 words # 2 (values saved) * 3 (priorities) * 2 (two and three tail chained interrupts)
  times: 24 words # 4 (times saved) * 3 (priorities) * 2 (two and three tail chained interrupts)
  exccnts: 24 words
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ Priorities of tested interrupts.
@ Interrupt with lower or equal priority should be executed after already
@ executed interrupt without stacking/unstacking phase.
{% set priorities = [
    ('0b00100000', '0b01100000', '0b11100000'),
    ('0b00100000', '0b01100000', '0b01100000'),
    ('0b00100000', '0b00100000', '0b00100000'),
] %}

{% block code %}
    @ Setup interrupts #0, #1 and #2 handlers
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{setExceptionHandler(17, "Irq1_Handler", r3, r4)}}
    {{setExceptionHandler(18, "Irq2_Handler", r3, r4)}}

    {{enableException(16, r3, r4)}}
    {{enableException(17, r3, r4)}}
    {{enableException(18, r3, r4)}}

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
{% for interrupt_0_pri, interrupt_1_pri, interrupt_2_pri in priorities %}
{% for use_three_interrupts in [False, True] %}
    mov.w r12, {{counter}}
    mov.w r0, #{{interrupt_0_pri}}
    mov.w r1, #{{interrupt_1_pri}}
    mov.w r2, #{{interrupt_2_pri}}

    bl.w initialize

    @ Prepare register with value setting interrupts pending state
    {% if use_three_interrupts %}
        mov.w r5, #0b111
    {% else %}
        mov.w r5, #0b11
    {% endif %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get time before interrupt handling
    ldr.w r0, [r3, r12]

    @ Set pending interrupts
    str.w r5, [r4]

    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr

    @ Get time after interrupt handling
    ldr.w r10, [r3, r12]

    @ If two interrupts are used, r8 value won't be set, so initialize it with r10
    {% if not use_three_interrupts %}
        mov.w r8, r10
    {% endif %}

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Set #0, #1 and #2 interrupts priorities
    ldr.w r4, ={{NVIC.IPR0}}
    add.w r0, r0, r1, LSL #8
    add.w r0, r0, r2, LSL #16
    str.w r0, [r4]

    @ Init ADD value and registers that will save which handler has been called
    mov.w r7, #0
    mov.w r9, #0

    @ Prepare register for setting interrupts into pending state
    ldr.w r4, ={{NVIC.ISPR0}}

    bx.n lr

save:
    @ Compute entry latency to the first handler
    sub.w r0, r4, r0
    @ Compute latency between the first and second handler
    sub.w r4, r6, r4
    @ Compute latency between the second and third handler
    sub.w r6, r8, r6
    @ Compute exit latency
    sub.w r8, r10, r8

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r4, r2, r1)}}
    {{saveValue("times", r6, r2, r1)}}
    {{saveValue("times", r8, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency to the first handler
    sub.w r0, r4, r0
    and.w r0, r0, 0xFF
    @ Compute exccnt between the first and second handler
    sub.w r4, r6, r4
    and.w r4, r4, 0xFF
    @ Compute exccnt between the second and third handler
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF
    @ Compute unstacking latency
    sub.w r8, r10, r8
    and.w r8, r8, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r4, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}
    {{saveValue("exccnts", r8, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    ldr.w r4, [r3, r12]

    @ Update counter of handled interrupts
    add.w r9, #1

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    ldr.w r6, [r3, r12]

    @ Update counter of handled interrupts
    add.w r9, #1

    bx.n lr

.align 4
.thumb_func
.type	Irq2_Handler, %function
Irq2_Handler:
    ldr.w r8, [r3, r12]

    @ Update counter of handled interrupts
    add.w r9, #1

    bx.n lr

{% endblock %}
