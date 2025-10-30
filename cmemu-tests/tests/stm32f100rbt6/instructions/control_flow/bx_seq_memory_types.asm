---
name: BX sequential execution
description: "Timing test of multiple bx instructions one after another, while jumping to different memory types"
dumped_symbols:
  times: 10 words
configurations:
# Case 0: X Y X Y X . . .
- { codePlaces: ["flash", "sram"], lbEn: true }
- { codePlaces: ["sram", "flash"], lbEn: true }
# Case 1: X Y Z X Y Z . . .
# Case 2: Random ones with line_buffer disabled
# Case 3: Random ones with line_buffer enabled
...
{% set free_registers = ["r3", "r4", "r5", "r6", "r7", "r8", "r9", "r11"] %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
.thumb_func
tested_code:
    b.w tested_code_1
{% for jumps in range(1, free_registers|length + 1) %}
{{ section(codePlaces[0]) }}
.thumb_func
tested_code_{{jumps}}:
.align 4
    {% for i in range(jumps) %}
    ldr.w {{free_registers[i]}}, =jump_target_{{jumps}}_{{i+1}}
    {% endfor %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    bx.n {{free_registers[0]}}

{% for rep in range(1, jumps) %}
{{ section(codePlaces[rep % codePlaces|length]) }}
.align 4
.thumb_func
jump_target_{{jumps}}_{{rep}}:
    bx.n {{free_registers[rep]}}
{% endfor %}

# This padding with nops ensures that we jump to address, that wasn't prefetched.
    nop.w
    nop.w
    nop.w
    nop.w
.align 4
.thumb_func
jump_target_{{jumps}}_{{jumps}}:
    @ Get finish time
    ldr.w r3, [r0, r1]
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}

    b.w tested_code_{{jumps+1}}

{% endfor %}

tested_code_{{free_registers|length+1}}:
    b.w end_label

{% endblock %}
