---
name: TBB/TBH sequential instructions tests
description: "Timing test of TBB and TBH instructions, when executing consecutive jumps"
dumped_symbols:
  # 8 (repetitions) * 2 (tbInstructions)
  times: 16 words
  results: 16 words
  flags: 16 words
  cpicnts: 16 words
  lsucnts: 16 words
configurations:
- { code: "sram", memory: "sram", lbEn: True, cache_enabled: True }
- { code: "sram", memory: "flash", lbEn: True, cache_enabled: True }
- { code: "flash", memory: "sram", lbEn: True, cache_enabled: True }
- { code: "flash", memory: "flash", lbEn: True, cache_enabled: True }
- { code: "sram", memory: "flash", lbEn: False, cache_enabled: True }
- { code: "flash", memory: "sram", lbEn: False, cache_enabled: True }
- { code: "flash", memory: "flash", lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set tbInstructions = ["tbb.w", "tbh.w"] %}
{% set jumpCounts = 8 %}
{% set jumpOffsets = 8 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

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
{% for tbInstr in tbInstructions %}
    {% if tbInstr[:3] == "tbb" %}
        {% set shiftSuffix = "" %}
    {% elif tbInstr[:3] == "tbh" %}
        {% set shiftSuffix = ", LSL #1" %}
    {% else %}
        panic("Unsupported tbInstr!")
    {% endif %}
    @ Prepare tb arguments
    ldr.w  r5, =jump_offset_table_{{tbInstr[:3]}}

    @ Store helper addresses
    b.w target_{{tbInstr[:3]}}
.ltorg
target_{{tbInstr[:3]}}:
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for jumpOffset in range(jumpOffsets) %}
    @ Prepare input values
    mov.w  r6, #{{jumpOffset}}
    mov.w  r7, #0

    @ Clear flags
    msr.w apsr_nzcvq, r7

    @ Clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{counter}}]

    {% for _ in range(jumpCounts) %} 
        @ Jump using jump table
        {{tbInstr}} [r5, r6{{shiftSuffix}}]

        @ Those adds, shouldn't execute
        {% for _ in range(jumpOffset) %} 
            adds.n r7, #1 
        {% endfor %}
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
    sub.w r2, r3, r2
    and.w r2, r2, #0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 3
@ Halfword offsets table to use in TBH
jump_offset_table_tbh:
{% for num in range(jumpOffsets) %}
    .hword {{num}}
{% endfor %}

.align 3
@ Halfword offsets table to use in TBB
jump_offset_table_tbb:
{% for num in range(jumpOffsets) %}
    .byte {{num}}
{% endfor %}

{% endblock %}
