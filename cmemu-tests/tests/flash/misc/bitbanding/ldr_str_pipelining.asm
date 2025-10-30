---
name: LDR/STR bitband access pipelining test
description: "Correctness test of pipelined LDR/STR from sram using bitband and regular addresses"
dumped_symbols:
  times: 24 words
  results: 24 words
  cpicnts: 24 words
  lsucnts: 24 words
configurations:
 - { code: "gpram", lbEn: True, instrsPart: 0 }
 - { code: "gpram", lbEn: True, instrsPart: 1 }
 - { code: "gpram", lbEn: True, instrsPart: 2 }
 - { code: "gpram", lbEn: True, instrsPart: 3 }
 - { code: "sram", lbEn: True, instrsPart: 0 }
 - { code: "sram", lbEn: True, instrsPart: 1 }
 - { code: "sram", lbEn: True, instrsPart: 2 }
 - { code: "sram", lbEn: True, instrsPart: 3 }
 - { code: "flash", lbEn: True, instrsPart: 0 }
 - { code: "flash", lbEn: True, instrsPart: 1 }
 - { code: "flash", lbEn: True, instrsPart: 2 }
 - { code: "flash", lbEn: True, instrsPart: 3 }
 - { code: "flash", lbEn: False, instrsPart: 0 }
 - { code: "flash", lbEn: False, instrsPart: 1 }
 - { code: "flash", lbEn: False, instrsPart: 2 }
 - { code: "flash", lbEn: False, instrsPart: 3 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ It is good to have two diffrent starting instruction, hence two last configurations
{% set accessInstrsConfigs = [
        ["str.w"],
        ["ldr.w"],
        ["ldr.w", "str.w"],
        ["str.w", "ldr.w"],
    ]
%}

{% set accessInstrs = accessInstrsConfigs[instrsPart] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr arguments (calculate `cell` bitband address)
    @ r4 - cell address
    @ r5 - cell's 0 bit bitband address
    ldr.w r4, =cell
    lsl.w r5, r4, #5
    add.w r5, 0x22000000

    b.w tested_code
.ltorg
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for guardBefore in [True, False] %}
{% for guardAfter in [True, False] %}
{% for reps in range(1, 4) %}
{% for useMixedAccesses in [True, False] %}
    {% if "str.w" in accessInstrs %}
        @ Prepare arguments
        mov.w r7, #1
    {% endif %}

    .align 4
    isb.w

    @ Get start time
    ldr.n r2, [r0, {{counter}}]

    {% if guardBefore %} add.n r1, r1, r1 {% endif %}

    {% for i in range(reps) %}
    {% for instr in accessInstrs %}
        {{instr}} {{ "r7" if instr == "str.w" else "r6" }}, [{{ "r4, 0" if useMixedAccesses and i % 2 == 1 else "r5, #" + (reps * 4)|string }}]
    {% endfor %}
    {% endfor %}

    {% if guardAfter %} add.n r1, r1, r1 {% endif %}

    @ Get finish time
    ldr.n r3, [r0, {{counter}}]

    @ Save test results
    bl.w {{save_func}}

    @ Reset cell value
    mov.w r6, #0x00000000
    str.n r6, [r4]
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_and_result:
    sub.w r2, r3, r2

    {{saveValue("times", r2, r1, r3)}}

    {% if accessInstrs[-1] == "str.w" %}
        @ As a result for STR we use an accessed cell value
        ldr.w r6, [r4]
    {% elif accessInstrs[-1] != "ldr.w" %}
        {{ unreachable("Invalid instruction") }}
    {% endif %}

    {{saveValue("results", r6, r1, r3)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r1, r3)}}
    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r1, r3)}}
    bx.n lr

{{ section("sram") }}
.align 4
cell: .word 0x00000000
{% endblock %}
