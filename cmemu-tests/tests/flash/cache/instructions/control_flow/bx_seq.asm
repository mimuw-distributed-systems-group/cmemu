---
name: BX sequential execution
description: "Timing test of multiple bx instructions one after another"
dumped_symbols:
  times: 10 words
  is_failed: user-defined
configurations:
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.n', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.n', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.n', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 1, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 1, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 1, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 1, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 1, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 1, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 1, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 1, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 1, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 1, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 2, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 2, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 2, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 2, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 2, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 2, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 2, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 2, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 2, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 2, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 3, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 3, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 3, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 3, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 3, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 3, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 3, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 3, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 3, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 4, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 4, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 4, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 4, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 4, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 5, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 5, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 5, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 6, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 6, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 6, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 7, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 7, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 7, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 8, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 8, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 8, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 9, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 9, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 9, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 10, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 10, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 10, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 11, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 11, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 11, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: False, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "sram", lbEn: True, fillingInstrsCount: 0, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 0, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 1, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 2, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: True, fillingInstrsCount: 0, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
- { code: "flash", lbEn: False, fillingInstrsCount: 0, testPadding: 3, ldrDWTEInstr: 'ldr.w', bxPadded: True, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% set free_registers = ['r4', 'r5', 'r6', 'r7', 'r8', 'r9'] %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Load address of trap function
    @ This is to detect whether we do not execute any instruction
    @ between branch instruction and branch target
    ldr.w  r11, =failed

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
{% for reps in range(free_registers|length) %}
    {% for i in range(reps) %}
    ldr.w {{free_registers[i]}}, =jump_target_{{reps}}_{{i}}
    {% endfor %}

    @ Align and clear PIQ
    b.w start_{{reps}}
.align 3
    {% for i in range(testPadding) %}
    nop.n
    {% endfor %}
start_{{reps}}:
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]
    {% if bxPadded %}
        adds.n  r3, r1
    {% endif %}

    {% for i in range(reps) %}
        bx.n {{free_registers[i]}}
        @ Following instructions should not execute
        {% for i in range(fillingInstrsCount) %}
        bx.n r11
        {% endfor %}
    .thumb_func
    jump_target_{{reps}}_{{i}}:
    {% endfor %}

    @ Get finish time
    {{ldrDWTEInstr}} r3, [r0, r1]
    bl.w save
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    bx.n lr

.align 4
.thumb_func
failed:
    mov.w r1, 0
    mov.w r2, 1
    ldr.w r3, =is_failed
    str.w r2, [r3, r1]

    b.w end_label

{{ section("sram") }}
.align 2
.global is_failed
is_failed: .word 0
.size is_failed, .-is_failed

{% endblock %}
