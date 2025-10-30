---
name: Interrupts Priority Handling
description: "Tests if interrupts are handled based on their priority."
dumped_symbols:
  results: 9 words # 3 (values saved) * 3 (priorities)
  times: 6 words # 2 (times saved) * 3 (priorities)
  exccnts: 6 words # 2 (times saved) * 3 (priorities)
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ [ARM-ARM] Table B3-4
{% set vector_table_offset_register = 'E000ED08'|int(base=16) %}
@ [TI-TRM] Table 4-2
{% set vector_table_entries = 64 %}
@ [ARM-ARM] Table B3-8
{% set clear_enable_register = 'E000E180'|int(base=16) %}
{% set set_pending_register = 'E000E200'|int(base=16) %}
{% set clear_pending_register = 'E000E280'|int(base=16) %}
{% set interrupt_priority_register = 'E000E400'|int(base=16) %}
@ The first and second interrupts are used for tests.
{% set tested_interrupts_bits = "0b11" %}

@ Priorities of tested interrupts.
@ Lower priority value means higher priority level.
@ Order of interrupts that should be handled: #0, #1, #0.
@ Note: Only one interrupt will be handled, because implemented handlers clear
@ pending states of both interrupts.
{% set priorities = [
    ('0b00100000', '0b01100000'),
    ('0b01100000', '0b00100000'),
    ('0b00100000', '0b00100000'),
] %}

{% block code %}
    @ Save old vector table value
    ldr.w r7, ={{vector_table_offset_register}}
    ldr.w r7, [r7]
    ldr.w r8, =old_vector_table_value
    str.w r7, [r8]

    @ Copy old interrupt table
    ldr.w r7, ={{vector_table_offset_register}}
    ldr.w r7, [r7]
    ldr.w r6, =new_vector_table_value
    mov.w r1, #{{vector_table_entries}}

.align 2
loop_copy_memory:
    @ End loop on zero value
    cbz.n r1, end_copy_memory
    @ Decrease counter
    sub.w r1, #1

    @ Copy one word from r7(orginal table) to r6(new copy) and increase addresses.
    ldmia.w r7!, {r2}
    stmia.w r6!, {r2}

    b.w loop_copy_memory
end_copy_memory:

    @ Setup new vector table
    ldr.w r6, ={{vector_table_offset_register}}
    ldr.w r7, =new_vector_table_value
    str.w r7, [r6]

    @ Prepare vector table address
    ldr.w r4, ={{vector_table_offset_register}}
    ldr.w r4, [r4]

    @ Setup interrupts #0 and #1 handlers
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{setExceptionHandler(17, "Irq1_Handler", r3, r4)}}

    @ Save priority register
    ldr.w r3, ={{interrupt_priority_register}}
    ldr.w r3, [r3]
    ldr.w r4, =old_interrupt_priority_register
    str.w r3, [r4]

    {{enableException(16, r3, r4)}}
    {{enableException(17, r3, r4)}}

    @ Prepare dwt for tests
    ldr.w r3, dwt

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Clear enable interrupt
    ldr.w r3, ={{clear_enable_register}}
    mov.w r2, #{{tested_interrupts_bits}}
    str.w r2, [r3]

    @ Revert old vetor table value
    ldr.w r8, =old_vector_table_value
    ldr.w r8, [r8]
    ldr.w r7, ={{vector_table_offset_register}}
    str.w r8, [r7]

    @ Revert priority register value
    ldr.w r8, =old_interrupt_priority_register
    ldr.w r8, [r8]
    ldr.w r7, ={{interrupt_priority_register}}
    str.w r8, [r7]

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
    mov.w r12, {{counter}}
    mov.w r1, #{{interrupt_0_pri}}
    mov.w r2, #{{interrupt_1_pri}}

    bl.w initialize

    @ Get time before interrupt handling
    ldr.w r0, [r3, r12]

    @ Set pending interrupts
    str.w r5, [r4]

    @ Give processor some time to process interrupts
    .rept 8
        adds.n r7, #1
    .endr

    @ Get time after interrupt handling
    ldr.w r8, [r3, r12]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Set interrupts #0 and #1 priorities
    ldr.w r0, ={{interrupt_priority_register}}
    add.w r1, r1, r2, LSL #8
    str.w r1, [r0]

    @ Init ADD value and registers that will save which handler has been called
    mov.w r7, #0
    mov.w r9, #0
    mov.w r10, #0

    @ Prepare registers
    ldr.w r1, ={{clear_pending_register}}
    ldr.w r4, ={{set_pending_register}}
    mov.w r5, #{{tested_interrupts_bits}}

    bx.n lr

save:
    @ Compute entry latency
    sub.w r0, r6, r0
    @ Compute exit latency
    sub.w r8, r8, r4

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r8, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}
    {{saveValue("results", r10, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ EXCCNT is 8-bit wide
    @ Compute unstacking latency
    sub.w r8, r8, r4
    and.w r8, r8, 0xFF

    {{saveValue("exccnts", r0, r7, r10)}}
    {{saveValue("exccnts", r6, r7, r10)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    ldr.w r6, [r3, r12]

    @ Set that interrupt 0 has been called
    mov.w r9, #1

    @ Clear pending states of tested interrupts
    str.w r5, [r1]

    ldr.w r4, [r3, r12]

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    ldr.w r6, [r3, r12]

    @ Set that interrupt 1 has been called
    mov.w r10, #1

    @ Clear pending states of tested interrupts
    str.w r5, [r1]

    ldr.w r4, [r3, r12]

    bx.n lr

.align 4
old_vector_table_value:  .word 0x0

.align 8
new_vector_table_value:  .skip {{vector_table_entries}} * 4

old_interrupt_priority_register: .word 0x0

{% endblock %}
