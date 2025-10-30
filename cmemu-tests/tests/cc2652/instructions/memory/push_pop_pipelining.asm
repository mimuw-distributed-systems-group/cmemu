---
name: PUSH and POP pipelining test
description: >
    Prooves that PUSH and POP are STR/STMDB and LDR/LDM respectively
    by showing that they pipeline depending on encoding.

    Encodings T1 and T2 are treated as LDM/STMDB encodings and should not be pipelined.
    Encoding T3 is treated as LDR/STR (immediate) encoding and should be pipelined.
dumped_symbols:
  results: 72 words    # 9 register sets * 8 repetitions
  push_times: 72 words
  pop_times: 72 words
configurations:
- { code: sram, lbEn: true }
- { code: flash, lbEn: false }
- { code: flash, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{@ Second element of tuple denotes to which encoding PUSH and POP will be assembled @}
{% set register_sets = [
        (["r0"],                "T1"),
        (["r7"],                "T1"),
        (["r8"],                "T3"),
        (["lr"],                "T3"),
        (["r0", "r7"],          "T1"),
        (["r0", "r8"],          "T2"),
        (["r0", "lr"],          "T2"),
        (["r8", "lr"],          "T2"),
        (["r0", "r8", "lr"],    "T2"),
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
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for (registers, encoding) in register_sets %}
    @ Decide the width of PUSH and POP instructions, prefer narrow encodings.
    @ Note that narrow POP instruction does not support reading to LR
    {% set width = 'w' if "r8" in registers or 'lr' in registers else 'n' %}
{% for reps in range(1, 9) %}
    @ Prepare input values
    {% for i in range(registers|length) %}
        mov.w {{ registers[i] }}, {{i+1}}
    {% endfor %}

    @ Store initial SP
    mov.w r11, sp

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time - PUSH
    ldr.w  r4, [r6, {{CYCCNT}}]

    {% for i in range(reps) %}
        push.{{width}} { {{ registers|join(", ") }} }
    {% endfor %}

    @ Get finish time - PUSH
    ldr.w  r5, [r6, {{CYCCNT}}]
    @ r1 <- PUSH execution time
    sub.w r1, r5, r4
    
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time - POP
    ldr.w  r4, [r6, {{CYCCNT}}]

    {% for i in range(reps) %}
        pop.{{width}} { {{ registers|join(", ") }} }
    {% endfor %}

    @ Get finish time - POP
    ldr.w  r5, [r6, {{CYCCNT}}]
    @ r2 <- POP execution time
    sub.w r2, r5, r4
    
    @ r5 <- initial SP - current SP
    mov.w r4, sp
    sub.w r5, r11, r4

    @ Save measurements
    bl.w save
    
    @ Restore initial SP
    mov.w sp, r11

{% endfor %}
    b.n after_pool_{{loop.index}}
    .ltorg
after_pool_{{loop.index}}:
{% endfor %}

    b.w end_label

.align 4
.thumb_func
save:
    {{ saveValue("push_times", r1, r3, r4) }}
    {{ saveValue("pop_times", r2, r3, r4) }}
    {{ saveValue("results", r5, r3, r4) }}

    bx.n lr
{% endblock %}
