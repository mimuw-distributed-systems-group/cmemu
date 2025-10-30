---
name: TBB/TBH instructions tests
description: "Timing test of TBB and TBH instructions"
dumped_symbols:
  times: 20 words
  results: 20 words
  flags: 20 words
  cpicnts: 20 words
  lsucnts: 20 words
configurations:
# TBB
- { code: "sram", memory: "sram", lbEn: True, tbInstr: "tbb.w" }
- { code: "sram", memory: "flash", lbEn: True, tbInstr: "tbb.w" }
- { code: "flash", memory: "sram", lbEn: True, tbInstr: "tbb.w" }
- { code: "flash", memory: "flash", lbEn: True, tbInstr: "tbb.w" }
- { code: "sram", memory: "flash", lbEn: False, tbInstr: "tbb.w" }
- { code: "flash", memory: "sram", lbEn: False, tbInstr: "tbb.w" }
- { code: "flash", memory: "flash", lbEn: False, tbInstr: "tbb.w" }
# TBH
- { code: "sram", memory: "sram", lbEn: True, tbInstr: "tbh.w" }
- { code: "sram", memory: "flash", lbEn: True, tbInstr: "tbh.w" }
- { code: "flash", memory: "sram", lbEn: True, tbInstr: "tbh.w" }
- { code: "flash", memory: "flash", lbEn: True, tbInstr: "tbh.w" }
- { code: "sram", memory: "flash", lbEn: False, tbInstr: "tbh.w" }
- { code: "flash", memory: "sram", lbEn: False, tbInstr: "tbh.w" }
- { code: "flash", memory: "flash", lbEn: False, tbInstr: "tbh.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{# NOTE: JOT = Jump Offset Table #}

{% if tbInstr == "tbb.w" %}
    {% set shiftSuffix = "" %}
    {% set sizeSpec = "byte" %}
    {% set jotCellSize = 1 %}
{% elif tbInstr == "tbh.w" %}
    {% set shiftSuffix = ", LSL #1" %}
    {% set sizeSpec = "hword" %}
    {% set jotCellSize = 2 %}
{% else %}
    {{ unreachable() }}
{% endif %}

{% set sourceRegisters = ["r5", "pc"] if code == memory else ["r5"] %}
{% set jotLength = 10 %}
{% set jotHalfSize = (1 + jotLength * jotCellSize) // 2 %}

{% macro makeJot(globalJumpOffset=0) %}
    @ It's halfword offsets table to use in TBB/TBH
    @ Table size just a little bit larger than PIQ size
    {% for num in range(jotLength) %}
    .{{sizeSpec}} {{num+globalJumpOffset}}
    {% endfor %}
{% endmacro %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare tb arguments
    ldr.w  r5, =jump_offset_table

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
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for tableIdx in range(jotLength) %}
{% for reg in sourceRegisters %}
    @ Prepare input values
    mov.w  r6, #{{tableIdx}}
    mov.w  r7, #0

    @ Clear flags
    msr.w apsr_nzcvq, r7

    @ Clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    @ Jump using jump table
    {{tbInstr}} [{{reg}}, r6{{shiftSuffix}}]

    {{ makeJot(jotHalfSize) if reg == 'pc' else '' }}

    @ result = amount of executed `ADD`s
    {% for _ in range(jotLength) %}
        adds.n  r7, #1
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_flags_and_result:
    mrs.w r6, apsr
    subs.n r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r7, r3, r4)}}
    {{saveValue('flags', r6, r3, r4)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
@ used by cases `reg != PC` in main loop
jump_offset_table:
{{ makeJot() }}
{% endblock %}
