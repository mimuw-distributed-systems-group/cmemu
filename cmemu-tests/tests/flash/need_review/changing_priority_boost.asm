---
name: Changing Priority Boosting during 
description:
    Tests if changing priority boosting registers during one interrupt's
    handling causes tail chaining.
dumped_symbols:
  results: 20 words
  times: 40 words
  exccnts: 40 words
configurations: # TODO: is this doable with one configuration. does it need to?
# - { priority_boosting_reg: ~, values: [~] }
# - { priority_boosting_reg: "primask", values: ['0x0', '0x1'] }
# TODO: do we need all the basepris?
- { priority_boosting_reg: "basepri", values: ['0x0', '0x0', '0x1', '0x2', '0x4', '0x8', '0x10', '0x20', '0x40', '0x80'] }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ Priorities of tested interrupts.
@ TODO: do we need more than one?
{% set priorities = [
    ('0b00100000', '0b00100000'),
] %}

{% block code %}
{% if priority_boosting_reg is not none %}
    @ Save tested register
    mrs.w r3, {{priority_boosting_reg}}
    ldr.w r4, =old_booster_value
    str.w r3, [r4]
{% endif %}

    @ Setup interrupts #0, #1 and #2 handlers
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{setExceptionHandler(17, "Irq1_Handler", r3, r4)}}

    {{enableException(16, r3, r4)}}
    {{enableException(17, r3, r4)}}

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
{% for interrupt_0_pri, interrupt_1_pri in priorities %}
{% for value in values %}
    mov.w r12, {{counter}}
    mov.w r0, #{{interrupt_0_pri}}
    mov.w r1, #{{interrupt_1_pri}}

    bl.w initialize

    @ Prepare register with value setting interrupts pending state
    mov.w r2, #0b11

{% if value is not none %}
    @ Prepare register with intended value
    mov.w r5, #{{value}}
{% endif %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get time before interrupt handling
    ldr.w r0, [r3, r12]

    @ Set pending interrupts
    str.w r2, [r4]

    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr

    @ Get time after processing interrupts
    ldr.w r10, [r3, r12]

{% if priority_boosting_reg is not none %}
    @ Revert tested register
    ldr.w r8, =old_booster_value
    ldr.w r8, [r8]
    msr.w {{priority_boosting_reg}}, r8
    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr
{% endif %}

    @ Get time after reverting register value
    ldr.w r8, [r3, r12]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Set #0 and #1 interrupts priorities
    ldr.w r4, ={{NVIC.IPR0}}
    add.w r0, r0, r1, LSL #8
    str.w r0, [r4]

    @ Init ADD value and registers that will save latencies and which handler has been called
    mov.w r7, #0
    mov.w r9, #0
    mov.w r0, #0
    mov.w r4, #0
    mov.w r6, #0
    mov.w r10, #0

    @ Prepare register for setting interrupts into pending state
    ldr.w r4, ={{NVIC.ISPR0}}

    bx.n lr

save:
    @ Compute entry latency to the first handler
    sub.w r4, r4, r0
    @ Compute entry latency to the second handler
    sub.w r6, r6, r0
    @ Compute latency to right before reverting the register
    sub.w r10, r10, r0
    @ Compute test finish latency
    sub.w r8, r8, r0

    {{saveValue("times", r4, r2, r1)}}
    {{saveValue("times", r6, r2, r1)}}
    {{saveValue("times", r10, r2, r1)}}
    {{saveValue("times", r8, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency to the first handler
    sub.w r4, r4, r0
    and.w r4, r4, 0xFF
    @ Compute stacking latency to the second handler
    sub.w r6, r6, r0
    and.w r6, r6, 0xFF
    @ Compute latency to right before reverting the register
    sub.w r10, r10, r0
    and.w r10, r10, 0xFF
    @ Compute total stacking latency
    sub.w r8, r8, r0
    and.w r8, r8, 0xFF

    {{saveValue("exccnts", r4, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}
    {{saveValue("exccnts", r8, r2, r1)}}
    {{saveValue("exccnts", r10, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    ldr.w r4, [r3, r12]

    @ Update counter of handled interrupts
    add.w r9, #1

    {% if priority_boosting_reg is not none %}
    @ Write value to priority boosting register
    msr.w {{priority_boosting_reg}}, r5
    {% endif %}

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    ldr.w r6, [r3, r12]

    @ Update counter of handled interrupts
    add.w r9, #1

    bx.n lr

{% if priority_boosting_reg is not none %}
.align 2
old_booster_value: .word 0x0
{% endif %}

{% endblock %}
