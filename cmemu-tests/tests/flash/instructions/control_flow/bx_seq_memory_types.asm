---
name: BX sequential execution
description: "Timing test of multiple bx instructions one after another, while jumping to different memory types"
dumped_symbols:
  times: 10 words
configurations:
# Case 0: X Y X Y X . . .
- { codePlaces: ["gpram", "sram"], lbEn: true }
- { codePlaces: ["gpram", "flash"], lbEn: true }
- { codePlaces: ["flash", "gpram"], lbEn: true }
- { codePlaces: ["flash", "sram"], lbEn: true }
- { codePlaces: ["sram", "gpram"], lbEn: true }
- { codePlaces: ["sram", "flash"], lbEn: true }
# Case 1: X Y Z X Y Z . . .
- { codePlaces: ["gpram", "sram", "flash"], lbEn: true }
- { codePlaces: ["flash", "gpram", "sram"], lbEn: true }
- { codePlaces: ["sram", "flash", "gpram"], lbEn: true }
- { codePlaces: ["sram", "gpram", "flash"], lbEn: true }
- { codePlaces: ["gpram", "flash", "sram"], lbEn: true }
# Case 2: Random ones with line_buffer disabled
- { codePlaces: ["gpram", "sram", "gpram", "flash", "gpram", "sram", "flash", "sram"], lbEn: false }
- { codePlaces: ["flash", "flash", "gpram", "flash", "flash", "flash", "flash", "gpram"], lbEn: false }
- { codePlaces: ["flash", "flash", "flash", "sram", "gpram", "gpram", "flash"], lbEn: false }
- { codePlaces: ["gpram", "sram", "gpram", "sram", "gpram", "sram", "flash", "flash"], lbEn: false }
- { codePlaces: ["sram", "sram", "flash", "sram", "gpram", "sram", "flash", "gpram"], lbEn: false }
- { codePlaces: ["sram", "sram", "sram", "sram", "sram", "flash", "gpram", "sram"], lbEn: false }
- { codePlaces: ["gpram", "flash", "flash", "flash", "gpram", "flash", "sram", "sram"], lbEn: false }
- { codePlaces: ["flash", "gpram", "gpram", "flash", "flash", "sram", "sram", "flash"], lbEn: false }
- { codePlaces: ["gpram", "sram", "flash", "gpram", "gpram", "sram", "flash", "flash"], lbEn: false }
- { codePlaces: ["flash", "flash", "sram", "flash", "sram", "sram", "gpram", "sram"], lbEn: false }
- { codePlaces: ["flash", "gpram", "gpram", "sram", "flash", "gpram", "sram", "gpram"], lbEn: false }
# Case 3: Random ones with line_buffer enabled
- { codePlaces: ["gpram", "sram", "gpram", "flash", "gpram", "sram", "flash", "sram"], lbEn: true }
- { codePlaces: ["flash", "flash", "gpram", "flash", "flash", "flash", "flash", "gpram"], lbEn: true }
- { codePlaces: ["flash", "flash", "flash", "sram", "gpram", "gpram", "flash"], lbEn: true }
- { codePlaces: ["gpram", "sram", "gpram", "sram", "gpram", "sram", "flash", "flash"], lbEn: true }
- { codePlaces: ["sram", "sram", "flash", "sram", "gpram", "sram", "flash", "gpram"], lbEn: true }
- { codePlaces: ["sram", "sram", "sram", "sram", "sram", "flash", "gpram", "sram"], lbEn: true }
- { codePlaces: ["gpram", "flash", "flash", "flash", "gpram", "flash", "sram", "sram"], lbEn: true }
- { codePlaces: ["flash", "gpram", "gpram", "flash", "flash", "sram", "sram", "flash"], lbEn: true }
- { codePlaces: ["gpram", "sram", "flash", "gpram", "gpram", "sram", "flash", "flash"], lbEn: true }
- { codePlaces: ["flash", "flash", "sram", "flash", "sram", "sram", "gpram", "sram"], lbEn: true }
- { codePlaces: ["flash", "gpram", "gpram", "sram", "flash", "gpram", "sram", "gpram"], lbEn: true }
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
