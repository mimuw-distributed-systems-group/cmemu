---
name: NMI exception latency
description: "Measures NMI exception latencies"
dumped_symbols:
  results: 4 words
  times:   4 words
  exccnts: 4 words
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
@ [ARM-ARM] Table B3-4
@ ICSR stands for Interrupt Control and State Register.
{% set icsr_addr = 'E000ED04'|int(base=16) %}
@ [ARM-ARM] B3.2.4 this bit sets NMI to pending state.
{% set nmipendset_bit = 2 ** 31 %}

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

    @ Setup NMI handler
    ldr.w r3, =NMI_Handler
    ldr.w r4, ={{vector_table_offset_register}}
    ldr.w r4, [r4]
    @ [ARM-TDG] Table 7.6 - location of NMI handler in vector table.
    add.w r4, r4, #0x00000008
    str.w r3, [r4]

    @ Prepare all registers for tests
    ldr.w r3, dwt
    ldr.w r4, ={{icsr_addr}}
    mov.w r5, #{{nmipendset_bit}}

    b.w tested_code

.thumb_func
end_label:
    @ Revert old vetor table value
    ldr.w    r8, =old_vector_table_value
    ldr.w    r8, [r8]
    ldr.w    r7, ={{vector_table_offset_register}}
    str.w    r8, [r7]

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts")] %}
{% for instr in ["add.w", "adds.n"] %}
    @ Set tested counter
    mov.w r10, {{counter}}
    @ Init ADD value.
    mov.w  r7, #0
    @ Read clock counter
    ldr.w  r0, [r3, r10]
    @ Set pending interrupt
    str.w r5, [r4]

    @ Give processor some time to process interrupt
    .rept 8
        {{instr}} r7, #1
    .endr

    @ Read clock counter
    ldr.w  r8, [r3, r10]

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
    and.w r0, r0, 0xFF
    @ Compute unstacking latency
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}

    bx.n lr

.align 4
.thumb_func
.type	NMI_Handler, %function
NMI_Handler:
    ldr.w  r6, [r3, r10]

    @ Save number of ADDs that have been executed before NMI had been raised.
    mov.w  r9, r7

    bx.n   lr

.align 4
old_vector_table_value:  .word 0x0

.align 8
new_vector_table_value:  .skip {{vector_table_entries}} * 4
{% endblock %}
