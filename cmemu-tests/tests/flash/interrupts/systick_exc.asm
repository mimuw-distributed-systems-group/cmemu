---
name: SysTick exception timings and correctness
description: >
    Tests timings and correctness of SysTick exception.
dumped_symbols:
  results: 224 words # 14 (cycles) * 4 (additional actions) * 2 (clear pending systick) * 2 (saved values)
  times: 224 words
  exccnts: 224 words
  lsucnts: 224 words
  cpicnts: 224 words
configurations:
- { code: gpram, lbEn: True, part: 0 }
- { code: gpram, lbEn: True, part: 1 }
- { code: sram, lbEn: True, part: 0 }
- { code: sram, lbEn: True, part: 1 }
- { code: flash, lbEn: True, part: 0 }
- { code: flash, lbEn: False, part: 0 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}

{% extends "asm.s.tpl" %}

@ [ARM-ARM] B3.3.2
{% set syst_csr = 'E000E010'|int(base=16) %}
{% set syst_rvr = 'E000E014'|int(base=16) %}
{% set syst_cvr = 'E000E018'|int(base=16) %}
@ [AMR-ARM] Table B3-4
{% set icsr = 'E000ED04'|int(base=16) %}
@ [ARM-ARM] B3.2.4
{% set clear_systick_pending_value = 2 ** 25 %}

{% set registers = [
    syst_csr,
    syst_rvr,
    syst_cvr,
] %}

{% set ticking_cycles = [
    1, 2, 3, 4, 
    10, 11, 12, 13, 14,
    50, 51, 52, 53, 54
] %}

@ Handling interrupt does not start in the same cycle as SysTick finishes ticking, so
@ some cycles have to be added to give processor time to start handling interrupt.
{% set delay_before_interrupt_handling = 8 %}
@ #1 cycle was chosen in arbitrary way.
{% set cycle_to_execute_additional_action = 1 %}
@ Because action preparation requires writing to memory, it takes few cycles to
@ propagate changes. Executing additional action with the buffer of 3 cycles
@ ensures that interrupt won't be raised during action preparation.
{% set minimal_cycles_to_execute_additional_action = cycle_to_execute_additional_action + 3 %}

@ Actions which are executed before interrupt is raised to check if they influence this process.
{% set no_additional_action = 0 %}
{% set disable_ticking = 1 %}
{% set clear_rvr = 2 %}
{% set clear_cvr = 3 %}
{% set additional_actions = [
    no_additional_action,
    disable_ticking,
    clear_rvr,
    clear_cvr,
] %}

{% set ns = namespace(test_cases = []) %}
{% for clear_second_systick_pending_state in [False, True] %}
{% for additional_action in additional_actions %}
{% for cycles in ticking_cycles %}
    {% set ns.test_cases = ns.test_cases + [(clear_second_systick_pending_state, additional_action, cycles)] %}
{% endfor %}
{% endfor %}
{% endfor %}

{% set test_cases_len = ns.test_cases | length %}
{% set test_parts = {"gpram": 2, "sram": 2, "flash": 1}[code] %}
{% if 0 <= part < test_parts %} 
    {% set part_len = test_cases_len // test_parts if test_cases_len % test_parts == 0 else test_cases_len // test_parts + 1 %}
    @ None for last element to include the remaining few elements.
    {% set ns.test_cases = ns.test_cases[part_len * part : (part_len * (part+1) if part < test_parts - 1 else none)] %}
{% else %}
    {% set ns.test_cases = panic("invalid part") %}
{% endif %}

{% block code %}
    @ Save all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r1, [r1]
        str.w r1, [r0, #{{offset}}]
    {% endfor %}

    @ Zero all registers
    {% for reg in registers %}
        ldr.w r0, ={{reg}}
        mov.w r1, #0
        str.w r1, [r0]
    {% endfor %}

    {{setExceptionHandler(15, "SysTick_Handler", r0, r1)}}

    @ Prepare registers for tests
    ldr.w r0, dwt

    {% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts"), (LSUCNT, "save_lsucnts"), (CPICNT, "save_cpicnts")] %}
        mov.w r10, {{counter}}
        ldr.w r11, ={{save_func}}

        bl.w tested_code
    {% endfor %}

.thumb_func
end_label:
    @ Restore all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r2, [r0, #{{offset}}]
        str.w r2, [r1]
    {% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
tested_code:
    @ Save where to return after test.
    mov r12, lr
{% for (clear_second_systick_pending_state, additional_action, cycles) in ns.test_cases %}
    mov.w r9, #{{ 1 if clear_second_systick_pending_state else 0 }}
    mov.w r1, #{{cycles}}

    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Read start counter value
    ldr.w r1, [r0, r10]

    @ Start ticking
    str.w r3, [r2]
    
    @ Give time to process interrupt
    {% for cycle in range(cycles + delay_before_interrupt_handling) %}
        adds.n r6, #1
        {% set ready_to_execute_additional_action =
            cycle == cycle_to_execute_additional_action and
            minimal_cycles_to_execute_additional_action <= cycles
        %}
        {% if ready_to_execute_additional_action and additional_action == disable_ticking %}
            @ Clear ticking enabled
            str.w r5, [r2]
        {% elif ready_to_execute_additional_action and additional_action == clear_rvr %}
            @ Zero RVR
            str.w r5, [r8]
        {% elif ready_to_execute_additional_action and additional_action == clear_cvr %}
            @ Zero CVR
            str.w r5, [r7]
        {% endif %}
    {% endfor %}

    @ Read end counter value
    ldr.w r2, [r0, r10]

    @ If interrupt hasn't been raised, set r4 to r2, otherwise its value is invalid
    cmp.w r5, #0
    it.n eq
    moveq.w r4, r2

    blx.n r11
{% endfor %}

    @ Return to counters loop.
    bx.n r12

initialize:
    ldr.w r2, ={{syst_csr}}
    ldr.w r8, ={{syst_cvr}}
    ldr.w r7, ={{syst_rvr}}

    @ Clear CSR
    mov.w r3, #0
    str.w r3, [r2]

    @ Write cycle to reload
    str.w r1, [r7]

    @ Clear current counter
    str.w r1, [r8]

    @ Prepare registers for tests
    mov.w r6, #0
    mov.w r5, #0

    @ Prepare register to enable ticking and pend interrupt
    mov.w r3, #3

    bx.n lr

.thumb_func
save:
    sub.w r1, r4, r1
    sub.w r2, r2, r4

    {{saveValue("times", r1, r8, r9)}}
    {{saveValue("times", r2, r8, r9)}}
    {{saveValue("results", r5, r8, r9)}}
    {{saveValue("results", r7, r8, r9)}}

    bx.n lr

.thumb_func
save_exccnts:
    sub.w r1, r4, r1
    and.w r1, r1, 0xFF

    sub.w r2, r2, r4
    and.w r2, r2, 0xFF

    {{saveValue("exccnts", r1, r8, r9)}}
    {{saveValue("exccnts", r2, r8, r9)}}

    bx.n lr

.thumb_func
save_lsucnts:
    sub.w r1, r4, r1
    and.w r1, r1, 0xFF

    sub.w r2, r2, r4
    and.w r2, r2, 0xFF

    {{saveValue("lsucnts", r1, r8, r9)}}
    {{saveValue("lsucnts", r2, r8, r9)}}

    bx.n lr

.thumb_func
save_cpicnts:
    sub.w r1, r4, r1
    and.w r1, r1, 0xFF

    sub.w r2, r2, r4
    and.w r2, r2, 0xFF

    {{saveValue("cpicnts", r1, r8, r9)}}
    {{saveValue("cpicnts", r2, r8, r9)}}

    bx.n lr

.align 4
.thumb_func
.type	SysTick_Handler, %function
SysTick_Handler:
    ldr.w r4, [r0, r10]

    @ Count exceptions
    adds.n r5, #1
    @ Save number of ADDs that have been executed before interrupt
    mov.w r7, r6
    
    @ Clear CSR
    ldr.w r3, ={{syst_csr}}
    mov.w r2, #0
    str.w r2, [r3]

    @ Clear pending interrupt if necessary
    cmp.w r9, #1
    ittt.n eq
    ldreq.w r2, ={{icsr}}
    moveq.w r3, #{{clear_systick_pending_value}}
    streq.w r3, [r2]

    bx.n lr

{{ section('sram') }}
.align 3
regs:
.rept 3
    .word 0x0
.endr

{% endblock %}
