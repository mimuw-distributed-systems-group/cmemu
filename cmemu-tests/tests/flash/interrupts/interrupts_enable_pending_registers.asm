---
name: Test usage of SETENA, CLRENA, SETPEND and CLRPEND registers
description: "Test for timings and correctness of setting values for NVIC SETENA, CLRENA, SETPEND and CLRPEND registers."
dumped_symbols:
    write_times: 54 words # 6 combinations * 9 interrupts
    read_times: 108 words # 6 combinations * 9 interrupts * 2 registers
    results: 108 words # 6 combinations * 9 interrupts * 2 registers
configurations:
# Ranges of interrupts are splitted to 4 parts to fit the code in gpram memory.
- { registers_set: "enable", irqs_start: 0, irqs_end: 8 }
- { registers_set: "enable", irqs_start: 9, irqs_end: 17 }
- { registers_set: "enable", irqs_start: 18, irqs_end: 26 }
- { registers_set: "enable", irqs_start: 27, irqs_end: 34 }
- { registers_set: "pending", irqs_start: 0, irqs_end: 8 }
- { registers_set: "pending", irqs_start: 9, irqs_end: 17 }
- { registers_set: "pending", irqs_start: 18, irqs_end: 26 }
- { registers_set: "pending", irqs_start: 27, irqs_end: 34 }
...

{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}

{#
    Combinations of types of registers to be set. 
    True means that SETENA or SETPEND should be set.
    False means that CLRENA or CLRPEND should be set. 
#}
{% set registers_combinations = [
    [True], 
    [False], 
    [True, True], 
    [True, False], 
    [False, True],
    [False, False],  
] %}

{% set interrupts_per_register = 32 %}
{% if registers_set == "enable" %}
    {# Used registers: SETENA0, SETENA1, CLRENA0, CLRENA1 #}
    {% set registers = { 
        "set0": "0xE000E100",
        "set1": "0xE000E104",
        "clear0": "0xE000E180",
        "clear1": "0xE000E184",
    } %}
{% elif registers_set == "pending" %}
    {# Used registers: SETPEND0, SETPEND1, CLRPEND0, CLRPEND1 #}
    {% set registers = { 
        "set0": "0xE000E200",
        "set1": "0xE000E204",
        "clear0": "0xE000E280",
        "clear1": "0xE000E284",
    } %}
{% else %}
    {{ unreachable() }}
{% endif %}
{% set clear_enable_address0 = "0xE000E180" %}
{% set clear_enable_address1 = "0xE000E184" %}
{% set clear_pending_address0 = "0xE000E280" %}
{% set clear_pending_address1 = "0xE000E284" %}
{% set all_interrupts = "0xFFFFFFFF" %}
{% set two_interrupts = "0x00000003" %}

{% extends "asm.s.tpl" %}
{% block code %}
    ldr.w r0, dwt

    bl.w clear_registers

    b.w tested_code

clear_registers:
    @ Clear enable registers
    ldr.w r1, ={{clear_enable_address0}}
    ldr.w r2, ={{all_interrupts}}
    str.w r2, [r1]

    ldr.w r1, ={{clear_enable_address1}}
    ldr.w r2, ={{two_interrupts}}
    str.w r2, [r1]

    @ Clear pending registers
    ldr.w r1, ={{clear_pending_address0}}
    ldr.w r2, ={{all_interrupts}}
    str.w r2, [r1]

    ldr.w r1, ={{clear_pending_address1}}
    ldr.w r2, ={{two_interrupts}}
    str.w r2, [r1]

    bx.n lr

.thumb_func
end_label:
    @ Cleanup after test

    bl.w clear_registers

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for combination in registers_combinations %}
{% for irq_num in range(irqs_start, irqs_end) %}
    ldr.w r3, ={{2 ** (irq_num % interrupts_per_register)}} 
    
    @ Prepare correct registers    
    {% if irq_num < interrupts_per_register %}   
        ldr.w r1, ={{registers["set0"]}}
        ldr.w r2, ={{registers["clear0"]}}
    {% else %}
        ldr.w r1, ={{registers["set1"]}}
        ldr.w r2, ={{registers["clear1"]}}
    {% endif %}

    .align 4
    isb.w
    
    @ Get write start time
    ldr.w r4, [r0, {{CYCCNT}}]

    {% for set_reg in combination %}
        {% if set_reg %}
            str.w r3, [r1]
        {% else %}
            str.w r3, [r2]
        {% endif %}
    {% endfor %}
    
    @ Get write end time
    ldr.w r5, [r0, {{CYCCNT}}]

    @ Get read start time
    ldr.w r3, [r0, {{CYCCNT}}]

    @ Read the first register
    ldr.w r6, [r1]

    @ Get write end time
    ldr.w r8, [r0, {{CYCCNT}}]

    @ Read the second register
    ldr.w r7, [r2]

    @ Get write end time
    ldr.w r9, [r0, {{CYCCNT}}]

    bl.w save

{% endfor %}
{% endfor %}

    b.w end_label
save:
    @ Write diff
    sub.w r4, r5, r4
    @ The first register read time diff
    sub.w r3, r8, r3
    @ The second register read time diff
    sub.w r8, r9, r8

    {{saveValue("write_times", r4, r10, r11)}}
    {{saveValue("read_times", r3, r10, r11)}}
    {{saveValue("read_times", r8, r10, r11)}}
    {{saveValue("results", r6, r10, r11)}}
    {{saveValue("results", r7, r10, r11)}}

    bx.n lr

{% endblock %}
