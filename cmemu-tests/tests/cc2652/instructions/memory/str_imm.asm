---
name: STR (immediate) instruction tests (without writeback)
description: "Timing and correctness test"
dumped_symbols:
  times: 31 words
  written_memory: user-defined
configurations:
- { code: sram, memory: sram, lbEn: true, repetitions: 31, encoding: T1 }
- { code: sram, memory: sram, lbEn: true, repetitions: 31, encoding: T2 }
- { code: sram, memory: sram, lbEn: true, repetitions: 24, encoding: T3 }
- { code: sram, memory: sram, lbEn: true, repetitions: 24, encoding: T4 }
- { code: flash, memory: sram, lbEn: false, repetitions: 31, encoding: T1 }
- { code: flash, memory: sram, lbEn: false, repetitions: 31, encoding: T2 }
- { code: flash, memory: sram, lbEn: false, repetitions: 24, encoding: T3 }
- { code: flash, memory: sram, lbEn: false, repetitions: 24, encoding: T4 }
- { code: flash, memory: sram, lbEn: true, repetitions: 31, encoding: T1 }
- { code: flash, memory: sram, lbEn: true, repetitions: 31, encoding: T2 }
- { code: flash, memory: sram, lbEn: true, repetitions: 24, encoding: T3 }
- { code: flash, memory: sram, lbEn: true, repetitions: 24, encoding: T4 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set base_address_register = 'sp' if encoding == 'T2' else 'r6' %}
{% set str_instruction = 'str.n' if encoding in ('T1', 'T2') else 'str.w' %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Save original SP
    mov.w  r11, sp

    b.w    tested_code
.thumb_func
end_label:
    @ Restore original SP
    mov.w  sp, r11
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for reps in range(repetitions) %}
    @ Prepare input values and reset flash line buffer
    ldr.w  {{base_address_register}}, =rep_{{reps}}_memory{{ '_end' if encoding == 'T4' else '' }}
    mov.w  r7, #0
    ldr.w  r2, [r7, r7]
    mov.w  r5, {{reps + 128}}

    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{str_instruction}} r5, [{{base_address_register}}, {{'-' if encoding == 'T4' else ''}}{{ (i % 4) * 4 + (4 if encoding == 'T4' else 0) }}] 
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.global written_memory
.align 4
written_memory:
{% for reps in range(repetitions) %}
rep_{{reps}}_memory:
    .word {{ reps*4 }}
    .word {{ reps*4 + 1 }}
    .word {{ reps*4 + 2 }}
    .word {{ reps*4 + 3 }}
rep_{{reps}}_memory_end:
{% endfor %}
.size written_memory, .-written_memory
{% endblock %}
