---
name: ADD+LDR instructions tests
description: "Timing test of ADD+LDR register dependencies"
dumped_symbols: 
    results: 80 words # 20 repetitions * 4 instructions combinations
    times: 80 words # 20 repetitions * 4 instructions combinations
    flags: 80 words # 20 repetitions * 4 instructions combinations
    cpicnts: 80 words # 20 repetitions * 4 instructions combinations
    lsucnts: 80 words # 20 repetitions * 4 instructions combinations
configurations:
# lbEn = true
- { code: "flash", lbEn: true, addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "flash", lbEn: true, addr: "sram", regA: "r2", regB: "r3", regC: "r2" }
- { code: "flash", lbEn: true, addr: "sram", regA: "r2", regB: "r3", regC: "r4" }

# lbEn = false
- { code: "flash", lbEn: false, addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "flash", lbEn: false, addr: "sram", regA: "r2", regB: "r3", regC: "r2" }
- { code: "flash", lbEn: false, addr: "sram", regA: "r2", regB: "r3", regC: "r4" }
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 20 %}
{% set instructions = [
    ("ldr.w", "add.w"),
    ("ldr.w", "adds.n"),
    ("ldr.n", "add.w"),
    ("ldr.n", "adds.n"), 
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set instructions = instructions[:1] %}
    {% elif gpram_part == 1 %}
        {% set instructions = instructions[1:2] %}
    {% elif gpram_part == 2 %}
        {% set instructions = instructions[2:3] %}
    {% elif gpram_part == 3 %}
        {% set instructions = instructions[3:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr arguments
    ldr.w  r5, =cell_1
    ldr.w  {{regC}}, =cell_0
    mov.w  r7, #0
    mov.w  {{regB}}, #42

    @ Prepare arguments
    mov.w  r6, #1

    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{section(code)}}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for ldrInstr, addInstr in instructions %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r8, [r0, {{counter}}]

    @ Those ADDs are to prohibit pipelining of LDRs
    add.w r6, r6
    {% for i in range(reps) %}
        {{addInstr}}  {{regA}}, r5, r7
        {{ldrInstr}}  {{regB}}, [{{regC}}, r7]
    {% endfor %}
    add.w r6, r6

    @ Get end counter value
    ldr.w  r9, [r0, {{counter}}]

    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_results_and_flags:
    mrs.w r1, apsr
    sub.w r8, r9, r8

    {{saveValue("times", r8, r10, r11)}}
    {{saveValue("flags", r1, r10, r11)}}
    {{saveValue("results", regB, r10, r11)}}

    bx.n lr

save_cpicnt:
    sub.w r8, r9, r8
    ands.w r8, r8, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r8, r10, r11)}}

    bx.n lr

save_lsucnt:
    sub.w r8, r9, r8
    ands.w r8, r8, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r8, r10, r11)}}
    
    bx.n lr

{{section(addr)}}
.align 4
cell_0: .word cell_0
cell_1: .word cell_1
{% endblock %}
