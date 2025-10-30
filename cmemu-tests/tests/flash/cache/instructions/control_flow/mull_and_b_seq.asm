---
name: smull/umull + branch combination tests
description: >
  Timing test of multiple smull/umull + branch combinations one after another.
  Such configuration tests when decode phase is run, if multicycle instruction is executing.
  Results indicate that it is run as soon as possible.
dumped_symbols:
  times: 100 words
configurations:
- { "code": "sram", lbEn: True, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "sram", lbEn: True, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "sram", lbEn: True, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "smull.w", cache_enabled: True }
- { "code": "sram", lbEn: True, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "sram", lbEn: True, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "sram", lbEn: True, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: True, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
- { "code": "flash", lbEn: False, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "repetitions": 15, "testedInstr": "umull.w", cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% set branchInstrs = ["b.w", "b.n"] %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Prepare input values
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Clean result registers
    mov.w  r8, #0
    mov.w  r9, #0

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
{% for bInstr in branchInstrs %}
{% set bIndex = loop.index %}
{% for reps in range(repetitions) %}
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r8, r9, r6, r7
        {{bInstr}} jump_target_{{bIndex}}_{{reps}}_{{i}}
        @ This padding with nops ensures that we jump to address, that wasn't prefetched.
        nop.w; nop.w; nop.w; nop.w
    jump_target_{{bIndex}}_{{reps}}_{{i}}:
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}

    bx.n lr
{% endblock %}
