---
name: Interrupts Priority Boosting
description:
    Tests if functionality of priority boosting is correctly handled.
    Interrupt and NMI are used for tests.
    Used interrupt should be not handled if its priority is not high enough.
    NMI should be always handled.
dumped_symbols:
  results: 42 words # 2 (values saved) * 21 (priority boosters and their values)
  faultmasks: 42 words
  times: 42 words
  exccnts: 42 words
configurations:
- { use_nmi: False }
- { use_nmi: True }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ The first interrupt is used for tests.
{% set tested_interrupt_bit = "0b1" %}

@ [ARM-ARM] B3.2.4 this bit sets NMI to pending state.
{% set nmipendset_bit = 2 ** 31 %}

@ [ARM-ARM] Table B3-4
@ ICSR stands for Interrupt Control and State Register.
{% set icsr_addr = 'E000ED04'|int(base=16) %}

@ Priority of used interrupt.
@ Interrupt might be handled depending on used booster.
{% set priority = '0b00100000' %}

@ Stores tuples: (
@   priority boosting register,
@   list of values to initialize priority boosting register,
@ )
{% set priority_boosters = [
    (None, [None]),
    ("primask", ['0x1']),
    ("basepri", ['0x0', '0x1', '0x2', '0x4', '0x8', '0x10', '0x20', '0x40', '0x80']),
    ("basepri_max", ['0x0', '0x1', '0x2', '0x4', '0x8', '0x10', '0x20', '0x40', '0x80']),
    ("faultmask", ['0x1']),
] %}

{% block code %}
    @ Save PRIMASK
    mrs.w r3, primask
    ldr.w r4, =old_primask
    str.w r3, [r4]

    @ Save BASEPRI
    mrs.w r3, basepri
    ldr.w r4, =old_basepri
    str.w r3, [r4]

    @ Save FAULTMASK
    mrs.w r3, faultmask
    ldr.w r4, =old_faultmask
    str.w r3, [r4]

    @ Setup interrupt #0 and NMI handlers
    {{setExceptionHandler(16, "Irq_Handler", r3, r4)}}
    {{setExceptionHandler(2, "Irq_Handler", r3, r4)}}

    @ Set interrupt priority
    mov.w r3, #{{priority}}
    ldr.w r4, ={{NVIC.IPR0}}
    str.w r3, [r4]

    {{enableException(16, r3, r4)}}

    @ Prepare dwt for tests
    ldr.w r3, dwt

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Revert PRIMASK
    ldr.w r8, =old_primask
    ldr.w r8, [r8]
    msr.w primask, r8

    @ Revert BASEPRI
    ldr.w r8, =old_basepri
    ldr.w r8, [r8]
    msr.w basepri, r8

    @ Revert FAULTMASK
    ldr.w r8, =old_faultmask
    ldr.w r8, [r8]
    msr.w faultmask, r8

    @ Commits changes to PRIMASK, BASEPRI and FAULTMASK
    isb.w

    @ Reset SP
    mov.w sp, r11

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts")] %}
{% for (priority_boosting_reg, values) in priority_boosters %}
{% for val in values %}
    @ Used by test to save currently dumped counter.
    @ r12 is used to store faultmask at the end of the loop, but fortunately by
    @ the time r12 is overridden by the faultmask, the counter offset is not
    @ required
    mov r12, {{counter}}

{% if priority_boosting_reg %}
    mov.w r1, #{{val}}
    msr.w {{priority_boosting_reg}}, r1
{% endif %}
    @ Prepare correct registers to set exception into pending state
    {% if use_nmi %}
        movw.w r4, #:lower16:{{icsr_addr}}
        movt.w r4, #:upper16:{{icsr_addr}}
        mov.w r5, #{{nmipendset_bit}}
    {% else %}
        movw.w r4, #:lower16:{{NVIC.ISPR0}}
        movt.w r4, #:upper16:{{NVIC.ISPR0}}
        mov.w r5, #{{tested_interrupt_bit}}
    {% endif %}

    bl.w initialize

    @ Align, clear PIQ and commit changes to priority boosting registers (if used).
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

    @ If priority booster is used, handler might not be executed, so r5, r6 and r10 won't
    @ store correct values.
    cmp.w r9, #1
    ittt.n ne
    movne.w r5, #0
    movne.w r6, r0
    movne.w r10, r8

    @ Save FAULTMASK value because it should be always cleared after exception
    @ handling (except for NMI) [ARM-ARM] B1.5.8 - DeActivate() pseudocode
    mrs.w r12, faultmask

    bl.w {{save_func}}

{% endfor %}
{% if priority_boosting_reg %}
    @ Clear pending state of tested interrupt. Because of that any pending
    @ interrupt won't be taken after clearing priority booster.
    {% if not use_nmi %}
        movw.w r1, #:lower16:{{NVIC.ICPR0}}
        movt.w r1, #:upper16:{{NVIC.ICPR0}}
        mov.w r5, #{{tested_interrupt_bit}}
        str.w r5, [r1]
    {% endif %}

    @ Clear priority boosting reg
    mov.w r1, #0x0
    msr.w {{priority_boosting_reg}}, r1
    isb.w
{% endif %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Init ADD value and registers that will save that handler has been called
    mov.w r7, #0
    mov.w r9, #0

    bx.n lr

save:
    @ Compute entry latency
    sub.w r0, r6, r0
    @ Compute exit latency
    sub.w r8, r8, r10

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r8, r2, r1)}}
    {{saveValue("faultmasks", r5, r2, r1)}}
    {{saveValue("faultmasks", r12, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute entry latency
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF
    @ Compute exit latency
    sub.w r8, r8, r10
    and.w r8, r8, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r8, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq_Handler, %function
Irq_Handler:
    ldr.w r6, [r3, r12]

    @ Set that handler has been called
    mov.w r9, #1

    @ Set faultmask to check if it will be cleared after exit from handler
    mov.w r5, #1
    msr.w faultmask, r5
    isb.w

    mrs.w r5, faultmask

    ldr.w r10, [r3, r12]

    bx.n lr

.align 2
old_primask: .word 0x0
old_basepri: .word 0x0
old_faultmask: .word 0x0

{% endblock %}
