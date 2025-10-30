---
name: NOP-like instructions test
description: >-
    Test timing and counters of selected instructions that do nothing.
    Checks the following effects:
    - Pipelining (LDR; "NOP")
    - Folding (LDR; "NOP"; IT)
    - Register dependencies ("NOP"; LDR) (postponing AGU)
    Additionally, provides positive and negative control cases for above scenarios.
dumped_symbols:
    times: auto
    flags: auto
    cpicnts: auto
    lsucnts: auto
    foldcnts: auto
configurations:
- { code: gpram, data: sram, lbEn: true, gpram_part: 0 }
- { code: gpram, data: sram, lbEn: true, gpram_part: 1 }
- { code: gpram, data: sram, lbEn: true, gpram_part: 2 }
- { code: gpram, data: sram, lbEn: true, gpram_part: 3 }
- { code: sram, data: gpram, lbEn: true, sram_part: 0 }
- { code: sram, data: gpram, lbEn: true, sram_part: 1 }
- { code: sram, data: gpram, lbEn: true, sram_part: 2 }
- { code: flash, data: sram, lbEn: true }
- { code: flash, data: sram, lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 3 %}

@ note: instructions that change flags are not true NOPs, some of them are tested "just in case"
{% set nop_like_instrs = [
    "mov.n <reg>, <reg>",
    "mov.w <reg>, <reg>",
    "movs.n <reg>, <reg>",
    "movs.w <reg>, <reg>",
    "add.w <reg>, #0",
    "adds.n <reg>, #0",
    "adds.w <reg>, #0",
    "cmn.n <reg>, <reg>",
    "cmp.n <reg>, #1",
    "cmp.n <reg>, <reg>",
    "tst.n <reg>, <reg>",
] %}

{% set scenario_names = ["pipelining", "folding", "reg_dep"] %}
{% set control_scenario_cases = ["positive", "negative"] %} @ independent of instr
{% set test_scenario_cases = ["nop-like"] %}

@ there is no list comprehension in jinja2, so fill it manually:
{% set ns = namespace(total_cases = []) %}
{% for name in scenario_names %}
    {% for case in control_scenario_cases %}
        {% set ns.total_cases = ns.total_cases + [(name, case, "<no-instr>")] %}
    {% endfor %}

    {% for case in test_scenario_cases %}
        {% for instr in nop_like_instrs %}
            {% set ns.total_cases = ns.total_cases + [(name, case, instr)] %}
        {% endfor %}
    {% endfor %}
{% endfor %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set ns.total_cases = ns.total_cases[:9] %}
    {% elif gpram_part == 1 %}
        {% set ns.total_cases = ns.total_cases[9:18] %}
    {% elif gpram_part == 2 %}
        {% set ns.total_cases = ns.total_cases[18:27] %}
    {% elif gpram_part == 3 %}
        {% set ns.total_cases = ns.total_cases[27:] %}
    {% else %}
        unreachable!("invalid gpram_part")
    {% endif %}
{% elif code == "sram" %}
    {% if sram_part == 0 %}
        {% set ns.total_cases = ns.total_cases[:18] %}
    {% elif sram_part == 1 %}
        {% set ns.total_cases = ns.total_cases[18:36] %}
    {% elif sram_part == 2 %}
        {% set ns.total_cases = ns.total_cases[36:] %}
    {% else %}
        unreachable!("invalid sram_part")
    {% endif %}
{% endif %}

{% macro scenario(name, case, instr, reps) %}
    add.w r7, r7 @ pipelining guard

    {% for _ in range(reps) %}
        {% if name == "pipelining" or name == "folding" %}
            ldr.n r4, [r5]
            {% if case == "positive" %}
                nop.n
            {% elif case == "negative" %}
                adds.n r7, r7
            {% elif case == "nop-like" %}
                {{ instr|replace("<reg>", r7) }}
            {% else %}
                unreachable!("invalid case")
            {% endif %}

            {% if name == "folding" %}
                it.n eq  @ flags does not matter
                addeq.w r7, r7  @ r7 is 0 anyway
            {% endif %}
    
        {% elif name == "reg_dep" %}
            {% if case == "positive" %}
                adds.n r5, r7  @ r7 is 0
            {% elif case == "negative" %}
                adds.n r7, r7
            {% elif case == "nop-like" %}
                {{ instr|replace("<reg>", r5) }}
            {% else %}
                unreachable!("invalid case")
            {% endif %}
            ldr.n r4, [r5]
        {% else %}
            unreachable!("invalid scenario")
        {% endif %}
    {% endfor %}

    add.w r7, r7  @ pipelining guard
{% endmacro %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    ldr.w  r5, =read_memory

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for name, case, instr in ns.total_cases %}
{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"), (FOLDCNT, "save_foldcnt")] %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {{scenario(name, case, instr, reps)}}

    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
    {% if counter == CYCCNT %} {{ inc_auto_syms() }} {% endif %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r8, r3, r4)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r2, r3, r4)}}
    
    bx.n lr

save_foldcnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ FOLDCNT is 8-bit wide
    
    {{saveValue("foldcnts", r2, r3, r4)}}
    
    bx.n lr

{{ section(data) }}
.align 4
read_memory:
    .word 0xFA57F00D

{% endblock %}
