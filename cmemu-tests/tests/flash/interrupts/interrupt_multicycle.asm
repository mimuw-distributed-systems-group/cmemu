---
name: Interrupt multicycle instructions.
description: "Tests interrupting multicycle instructions."
dumped_symbols:
  results: 8 words # 2 (instructions) * 2 (saved values) * 2 (delays)
  interrupted_state: 8 words
  times: 8 words
  exccnts: 8 words
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
@ The 30th interrupt is used in test.
{% set value_to_set_for_tested_interrupt = 2 ** 30 %}

@ Each test case is a tuple: (
@    multicycle instruction to interrupt,
@    list of used registers and values to initialize them,
@ ).
@ Note: chosen values for registers don't matter.
@ ASSUMPTION: r1 and r2 registers are used to save instruction result, so at
@ least one of them should be used
{% set test_cases = [
    ("mla.w r1, r1, r2, r7", [(r1, '0x2'), (r2, '0x3'), (r7, '0x4')]),
    ("mls.w r1, r1, r2, r7", [(r1, '0x2'), (r2, '0x3'), (r7, '0x4')]),
] %}

@ To test all moments when interrupt can occur in multicycle instruction,
@ executing additonal ADDs between pending interrupt and tested instruction is used.
@ It's not the best solution because there are max. 2 cycles between these two.
@ TODO: Replace delays with SysTick (once it is supported by the emulator).
{% set delays = [0, 1] %}

{% block code %}
    @ Save old vector table value
    ldr r7, ={{vector_table_offset_register}}
    ldr r7, [r7]
    ldr r8, =old_vector_table_value
    str r7, [r8]

    @ Copy old interrupt table
    ldr r7, ={{vector_table_offset_register}}
    ldr r7, [r7]
    ldr r6, =new_vector_table_value
    mov r1, #{{vector_table_entries}}

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

    @ Interrupt #30 has (16 + 30) = 46 exception number.
    {{setExceptionHandler(46, "Irq30_Handler", r3, r4)}}
    {{enableException(46, r3, r4)}}

    @ Prepare all registers for tests
    ldr.w r3, dwt
    ldr.w r4, ={{set_pending_register}}

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Clear enable interrupt
    ldr.w r3, ={{clear_enable_register}}
    ldr.w r2, ={{value_to_set_for_tested_interrupt}}
    str.w r2, [r3]

    @ Revert old vetor table value
    ldr.w r8, =old_vector_table_value
    ldr.w r8, [r8]
    ldr.w r7, ={{vector_table_offset_register}}
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
{% for (instr, registers_and_values) in test_cases %}
{% for delay in delays %}
    @ Prepare registers for test.
    mov.w r8, #0
    mov.w r5, #{{value_to_set_for_tested_interrupt}}
    mov.w r9, #0
    @ Init r1 and r2, because they are saved
    mov.w r1, #0
    mov.w r2, #0

    {% for (reg, val) in registers_and_values %}
        mov.w {{reg}}, #{{val}}
    {% endfor %}

    @ Prepare currently dumped counter offset
    mov r10, {{counter}}
    @ Read clock counter
    ldr.w r0, [r3, r10]
    @ Set pending interrupt
    str.w r5, [r4]

    @ Delay instruction execution to give time for interrupt to be risen during
    @ instruction.
    .rept {{delay}}
        add.w r8, #42
    .endr

    {{instr}}

    @ Read clock counter
    ldr.w r8, [r3, r10]

    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    @ Compute entry counter
    sub.w r0, r6, r0
    @ Compute exit counter
    sub.w r6, r8, r6

    {{saveValue("times", r0, r7, r10)}}
    {{saveValue("times", r6, r7, r10)}}
    {{saveValue("results", r1, r7, r10)}}
    {{saveValue("results", r2, r7, r10)}}
    {{saveValue("interrupted_state", r5, r7, r10)}}
    {{saveValue("interrupted_state", r9, r7, r10)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ EXCCNT is 8-bit wide
    @ Compute unstacking latency
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("exccnts", r0, r7, r10)}}
    {{saveValue("exccnts", r6, r7, r10)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq30_Handler, %function
Irq30_Handler:
    ldr.w r6, [r3, r10]

    @ Save registers to check if instruction has been interrupted during execution.
    mov.w r5, r1
    mov.w r9, r2

    bx.n lr

.align 4
old_vector_table_value:  .word 0x0

.align 8
new_vector_table_value:  .skip {{vector_table_entries}} * 4

.align 4
cells: .word 0x0, 0x0
{% endblock %}
