---
name: STM/STMDB instruction tests
description: Timing and correctness test
dumped_symbols:
  results: 120 words
  times: 120 words
  memory: user-defined
configurations:
- { code: sram, data: sram, lbEn: True, repetitions: 7, instr: stm, cache_enabled: True }
- { code: flash, data: sram, lbEn: False, repetitions: 7, instr: stm, cache_enabled: True }
- { code: flash, data: sram, lbEn: True, repetitions: 7, instr: stm, cache_enabled: True }
- { code: sram, data: sram, lbEn: True, repetitions: 7, instr: stmdb, cache_enabled: True }
- { code: flash, data: sram, lbEn: False, repetitions: 7, instr: stmdb, cache_enabled: True }
- { code: flash, data: sram, lbEn: True, repetitions: 7, instr: stmdb, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set register_sets = [
        ["r0"],
        ["r8"],
        ["lr"],
        ["r0", "r8"],
        ["r0", "lr"],
        ["r8", "r9"],
        ["r8", "lr"],
        ["r0", "r8", "r9"],
        ["r0", "r8", "lr"],
        ["r0", "r8", "r9", "lr"],
    ]
%}

{% block code %}
    @ Prepare cycle counter address
    ldr.w  r6, dwt

    b.w    tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.thumb_func
.type tested_code, %function
tested_code:

{% for registers in register_sets %}
    {% set regset_idx = loop.index0 %}
{% for wback in (False, True) %}
    {% set width = 'n' if instr == 'stm' and wback and registers == ["r0"] else 'w' %}
{% for reps in range(1, repetitions) %}
    @ Prepare input values
    bl.w initialize

    ldr.w r1, =memory_{{wback}}_{{regset_idx}}_{{reps}}
    mov.w r2, r1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r6, {{CYCCNT}}]

    {% for i in range(reps) %}
        {{instr}}.{{width}} r1 {{ "!" if wback }}, { {{ registers|join(", ") }} }
    {% endfor %}
    
    @ Finish time
    ldr.w  r5, [r6, {{CYCCNT}}]

    @ Save measurements
    bl.w save
{% endfor %}
{% endfor %}
    b.n after_pool_{{regset_idx}}
.ltorg
after_pool_{{regset_idx}}:
{% endfor %}

    b.w end_label

.thumb_func
initialize:
    @ Store LR since it's overwritten by this function
    mov.w r7, lr

    @ After each initialization registers will have different values
    @ so we can distinguish writes done by different loop iterations.
    ldr.w r4, =counter
    ldr.w r5, [r4]
    {% for reg in ["r0", "r8", "r9", "lr"] %}
        mov.w {{ reg }}, r5
        add.w r5, 1
    {% endfor %}
    str.w r5, [r4]
    bx.n r7

.ltorg

.thumb_func
save:
    @ Finish time - Start time
    sub.w r5, r5, r4
    @ Current r1 - Initial r1
    sub.w r1, r1, r2

    {{ saveValue("times", r5, r3, r4) }}
    {{ saveValue("results", r1, r3, r4) }}
    bx.n lr

.ltorg

{{ section(data) }}
.align 4
.global memory
memory:
{% for regset_idx in range(register_sets|length) %}
{% for wback in (False, True) %}
{% for reps in range(1, repetitions) %}

{% if instr == 'stm' %}memory_{{wback}}_{{regset_idx}}_{{reps}}:{% endif %}
{% for j in range(4 * (reps if wback else 1)) %}  @ At most 4 written words per STMDB
    .word 0
{% endfor %}
{% if instr == 'stmdb' %}memory_{{wback}}_{{regset_idx}}_{{reps}}:{% endif %}

{% endfor %}
{% endfor %}
{% endfor %}
.size memory, .-memory

.align 2
counter: .word 1
{% endblock %}
