---
name: STR instruction tests
description: "Timing and correctness test of str instruction"
dumped_symbols:
  times: 72 words
  written_memory: user-defined
configurations:
- { code: "sram", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 0, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 1, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 2, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 3, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 4, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 0, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 1, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 2, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 3, cache_enabled: True }
- { code: "sram", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 4, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 0, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 1, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 2, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 3, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 24, strInstr: "str.n", addCount: 4, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 0, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 1, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 2, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 3, cache_enabled: True }
- { code: "flash", memory: "sram", repetitions: 16, strInstr: "str.w", addCount: 4, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
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
{% for cntr in [CYCCNT, CPICNT, LSUCNT] %}
{% set cntr_idx = loop.index %}
    mov.w r1, {{cntr}}
{% for reps in range(repetitions) %}
    @ Prepare str arguments
    ldr.w  r5, rep_{{cntr_idx}}_{{reps}}_data
    ldr.w  r6, =rep_{{cntr_idx}}_{{reps}}_memory
    mov.w  r7, #0

    @ Branch to test case and clean prefetch queue
    b.n rep_{{cntr_idx}}_{{reps}}_label

.align 2
rep_{{cntr_idx}}_{{reps}}_data: .word 0b1{% for i in range(reps) %}0{% endfor %}
.ltorg
.align 4
rep_{{cntr_idx}}_{{reps}}_label:
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(addCount) %}
        adds.n r1, r7
    {% endfor %}
    

    {% for i in range(reps) %}
        {{strInstr}} r5, [r6, r7] 
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w {{ 'save32' if cntr == CYCCNT else 'save8' }}

{% endfor %}
{% endfor %}

    b.w end_label


save32:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    bx.n lr

save8:
    subs.n r2, r3, r2
    and.w r2, r2, 0xFF
    {{saveTime(r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
.global	written_memory
written_memory:
    {% for cntr_idx in [1, 2, 3] %}
    {% for reps in range(repetitions) %}
    rep_{{cntr_idx}}_{{reps}}_memory: .word 0x0
    {% endfor %}
    {% endfor %}
.size	written_memory, .-written_memory
{% endblock %}
