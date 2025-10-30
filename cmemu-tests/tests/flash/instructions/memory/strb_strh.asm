---
name: STRB/STRH instructions tests
description: "Timing and correctness test"
dumped_symbols:
  stored_memory: user-defined
  times: 96 words # 2 (values) * 4 (offsets) * 12 (repetitions)
configurations:
- { code: gpram, memory: gpram, lbEn: false, target: imm, size: b, width: w }
- { code: gpram, memory: gpram, lbEn: false, target: imm, size: b, width: n }
- { code: gpram, memory: gpram, lbEn: false, target: reg, size: b, width: w }
- { code: gpram, memory: gpram, lbEn: false, target: reg, size: b, width: n }

- { code: gpram, memory: gpram, lbEn: false, target: imm, size: h, width: w }
- { code: gpram, memory: gpram, lbEn: false, target: imm, size: h, width: n }
- { code: gpram, memory: gpram, lbEn: false, target: reg, size: h, width: w }
- { code: gpram, memory: gpram, lbEn: false, target: reg, size: h, width: n }


- { code: sram, memory: sram, lbEn: false, target: imm, size: b, width: w }
- { code: sram, memory: sram, lbEn: false, target: imm, size: b, width: n }
- { code: sram, memory: sram, lbEn: false, target: reg, size: b, width: w }
- { code: sram, memory: sram, lbEn: false, target: reg, size: b, width: n }

- { code: sram, memory: sram, lbEn: false, target: imm, size: h, width: w }
- { code: sram, memory: sram, lbEn: false, target: imm, size: h, width: n }
- { code: sram, memory: sram, lbEn: false, target: reg, size: h, width: w }
- { code: sram, memory: sram, lbEn: false, target: reg, size: h, width: n }


- { code: gpram, memory: sram, lbEn: false, target: imm, size: b, width: w }
- { code: gpram, memory: sram, lbEn: false, target: imm, size: b, width: n }
- { code: gpram, memory: sram, lbEn: false, target: reg, size: b, width: w }
- { code: gpram, memory: sram, lbEn: false, target: reg, size: b, width: n }

- { code: gpram, memory: sram, lbEn: false, target: imm, size: h, width: w }
- { code: gpram, memory: sram, lbEn: false, target: imm, size: h, width: n }
- { code: gpram, memory: sram, lbEn: false, target: reg, size: h, width: w }
- { code: gpram, memory: sram, lbEn: false, target: reg, size: h, width: n }


- { code: sram, memory: gpram, lbEn: false, target: imm, size: b, width: w }
- { code: sram, memory: gpram, lbEn: false, target: imm, size: b, width: n }
- { code: sram, memory: gpram, lbEn: false, target: reg, size: b, width: w }
- { code: sram, memory: gpram, lbEn: false, target: reg, size: b, width: n }

- { code: sram, memory: gpram, lbEn: false, target: imm, size: h, width: w }
- { code: sram, memory: gpram, lbEn: false, target: imm, size: h, width: n }
- { code: sram, memory: gpram, lbEn: false, target: reg, size: h, width: w }
- { code: sram, memory: gpram, lbEn: false, target: reg, size: h, width: n }


- { code: flash, memory: gpram, lbEn: false, target: imm, size: b, width: w }
- { code: flash, memory: gpram, lbEn: false, target: imm, size: b, width: n }
- { code: flash, memory: gpram, lbEn: false, target: reg, size: b, width: w }
- { code: flash, memory: gpram, lbEn: false, target: reg, size: b, width: n }

- { code: flash, memory: gpram, lbEn: false, target: imm, size: h, width: w }
- { code: flash, memory: gpram, lbEn: false, target: imm, size: h, width: n }
- { code: flash, memory: gpram, lbEn: false, target: reg, size: h, width: w }
- { code: flash, memory: gpram, lbEn: false, target: reg, size: h, width: n }


- { code: flash, memory: sram, lbEn: false, target: imm, size: b, width: w }
- { code: flash, memory: sram, lbEn: false, target: imm, size: b, width: n }
- { code: flash, memory: sram, lbEn: false, target: reg, size: b, width: w }
- { code: flash, memory: sram, lbEn: false, target: reg, size: b, width: n }

- { code: flash, memory: sram, lbEn: false, target: imm, size: h, width: w }
- { code: flash, memory: sram, lbEn: false, target: imm, size: h, width: n }
- { code: flash, memory: sram, lbEn: false, target: reg, size: h, width: w }
- { code: flash, memory: sram, lbEn: false, target: reg, size: h, width: n }

# Additional tests with flash line buffer turned on
- { code: flash, memory: gpram, lbEn: true, target: imm, size: b, width: w }
- { code: flash, memory: gpram, lbEn: true, target: imm, size: b, width: n }
- { code: flash, memory: gpram, lbEn: true, target: reg, size: b, width: w }
- { code: flash, memory: gpram, lbEn: true, target: reg, size: b, width: n }

- { code: flash, memory: gpram, lbEn: true, target: imm, size: h, width: w }
- { code: flash, memory: gpram, lbEn: true, target: imm, size: h, width: n }
- { code: flash, memory: gpram, lbEn: true, target: reg, size: h, width: w }
- { code: flash, memory: gpram, lbEn: true, target: reg, size: h, width: n }


- { code: flash, memory: sram, lbEn: true, target: imm, size: b, width: w }
- { code: flash, memory: sram, lbEn: true, target: imm, size: b, width: n }
- { code: flash, memory: sram, lbEn: true, target: reg, size: b, width: w }
- { code: flash, memory: sram, lbEn: true, target: reg, size: b, width: n }

- { code: flash, memory: sram, lbEn: true, target: imm, size: h, width: w }
- { code: flash, memory: sram, lbEn: true, target: imm, size: h, width: n }
- { code: flash, memory: sram, lbEn: true, target: reg, size: h, width: w }
- { code: flash, memory: sram, lbEn: true, target: reg, size: h, width: n }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set values = ["0x01234567", "0x89ABCDEF"] %}
{% set offsetRange = 4 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

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

{% for val_idx in range(values|length) %}
{% for offset_cnt in range(offsetRange) %}
    {% set offset = offset_cnt * (2 if size == 'h' else 1) %}
    @ Prepare value to store
    ldr.w  r5, ={{values[val_idx]}}
    @ Prepare address to store to
    ldr.w  r6, =rep_{{val_idx}}_{{offset_cnt}}_memory
    {% if target == "reg" %}
        mov.w  r7, {{offset}}
    {% endif %}

    b.n label_{{val_idx}}_{{offset_cnt}}

@ We store constants close to their usage, to allow longer program.
.ltorg

label_{{val_idx}}_{{offset_cnt}}:
{% for reps in range(1, 12) %}
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {% if target == "imm" %}
            str{{size}}.{{width}} r5, [r6, {{offset}}]
        {% elif target == "reg" %}
            str{{size}}.{{width}} r5, [r6, r7]
        {% else %}
            panic!("Invalid configuration")
        {% endif %} 
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    @ Get stored values
    ldr.w  r8, [r6, #0]
    ldr.w  r9, [r6, #4]

    bl.w save

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.ltorg

.align 2
save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
.global	stored_memory
stored_memory:
{% for val_idx in range(values|length) %}
{% for offset_cnt in range(offsetRange) %}
    rep_{{val_idx}}_{{offset_cnt}}_memory: 
        .word 0
        .word 0
{% endfor %}
{% endfor %}
.size	stored_memory, .-stored_memory

{% endblock %}
