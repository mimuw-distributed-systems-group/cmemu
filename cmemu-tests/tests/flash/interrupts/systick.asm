---
name: SysTick logic timings and correctness
description: >
  Tests timings and correctness of SysTick logic.
dumped_symbols:
  cvrs: 112 words # 7 (possible cycles) * 4 (additional actions) * 2 (saved values) * 2 (writes to CVR)
  csrs: 112 words
  times: 112 words
configurations:
- { code: gpram, lbEn: True }
- { code: sram, lbEn: True }
- { code: flash, lbEn: True }
- { code: flash, lbEn: False}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = False %}

{% extends "asm.s.tpl" %}

@ [ARM-ARM] B3.3.2
{% set syst_csr = 'E000E010'|int(base=16) %}
{% set syst_rvr = 'E000E014'|int(base=16) %}
{% set syst_cvr = 'E000E018'|int(base=16) %}

{% set possible_cycles = range(0, 7) %}

{% set registers = [
    syst_csr,
    syst_rvr,
    syst_cvr,
] %}

{% set no_additional_action = 0 %}
{% set disable_ticking = 1 %}
{% set clear_rvr = 2 %}
{% set restart_ticking = 3 %}
{% set additional_actions = [
    no_additional_action,
    disable_ticking,
    clear_rvr,
    restart_ticking,
] %}

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

    @ Prepare registers for tests
    ldr.w r0, dwt
    ldr.w r5, ={{syst_cvr}}
    ldr.w r8, ={{syst_rvr}}

    b.w tested_code

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
{% for cycles in possible_cycles %}
{% for additional_action in additional_actions %}
{% for write_to_cvr in [False, True] %}
    mov.w r1, #{{cycles}}
    mov.w r6, #0
    mov.w r7, #0
    
    bl.w initialize

    @ Prepare register to enable ticking
    mov.w r3, #1

    @ Align and clear PIQ
    .align 4
    isb.w

    ldr.w r1, [r0, {{CYCCNT}}]

    @ Start ticking
    str.w r3, [r2]
    
    {% for cycle in range(cycles) %}
        adds.n r3, #0
        @ #1 cycle was chosen in arbitrary way
        {% if cycle == 1 and additional_action != no_additional_action %}
            {% if additional_action in [disable_ticking, restart_ticking] %}
                @ Clear ticking enabled
                str.w r6, [r2]
            {% elif additional_action == clear_rvr %}
                @ Zero RVR
                str.w r6, [r8]
            {% else %}
                panic!("Uknown additional action.")
            {% endif %}
            
            @ Save CVR
            ldr.w r6, [r5]
            
            {% if additional_action == restart_ticking %}
                @ Enable ticking
                str.w r3, [r2]
            {% endif %}
        {% endif %}
    {% endfor %}

    {% if additional_action != no_additional_action %}
        @ Save CVR to see impact of additional actions on this register
        ldr.w r7, [r5]
    {% endif %}

    @ Writing to CVR clears COUNTFLAG
    @ [ARM-ARM] B3.3.3 "COUNTFLAG is cleared to 0 ... by any write to the Current Value register"
    {% if write_to_cvr %}
        str.w r4, [r5]
    {% endif %}

    @ Save COUNTFLAG
    ldr.w r3, [r2]
    @ Check if COUNTFLAG has been cleared
    @ [ARM-ARM] B3.3.3 "COUNTFLAG is cleared to 0 by a software read of this register"
    ldr.w r4, [r2]

    ldr.w r2, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Clear CSR
    mov.w r2, #0
    ldr.w r3, ={{syst_csr}}
    str.w r2, [r3]

    @ Write cycle to reload
    ldr.w r2, ={{syst_rvr}}
    str.w r1, [r2]

    @ Clear current counter
    ldr.w r2, ={{syst_cvr}}
    str.w r1, [r2]

    ldr.w r2, ={{syst_csr}}

    bx.n lr

save:
    sub.w r1, r2, r1

    {{saveValue("times", r1, r10, r11)}}
    {{saveValue("csrs", r3, r10, r11)}}
    {{saveValue("csrs", r4, r10, r11)}}
    {{saveValue("cvrs", r6, r10, r11)}}
    {{saveValue("cvrs", r7, r10, r11)}}

    bx.n lr

{{ section('sram') }}
.align 3
regs:
.rept 3
    .word 0x0
.endr

{% endblock %}
