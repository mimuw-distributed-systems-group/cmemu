---
name: LDRB/LDRH/LDRSB/LDRSH instructions tests
description: "Timing and correctness test"
dumped_symbols:
  results: 96 words  # 2 (values) * 4 (offsets) * 12 (repetitions)
  times: 96 words
configurations:


- { code: sram, memory: sram, lbEn: False, target: lit, size: b, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: b, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: b, width: n, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: b, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: b, width: n, cache_enabled: True }

- { code: sram, memory: sram, lbEn: False, target: lit, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: sb, width: n, cache_enabled: True }

- { code: sram, memory: sram, lbEn: False, target: lit, size: h, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: h, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: h, width: n, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: h, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: h, width: n, cache_enabled: True }

- { code: sram, memory: sram, lbEn: False, target: lit, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: imm, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: sram, lbEn: False, target: reg, size: sh, width: n, cache_enabled: True }


- { code: flash, memory: flash, lbEn: False, target: lit, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: b, width: n, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: b, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: False, target: lit, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: sb, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: False, target: lit, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: h, width: n, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: h, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: False, target: lit, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: imm, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, target: reg, size: sh, width: n, cache_enabled: True }


- { code: sram, memory: flash, lbEn: False, target: imm, size: b, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: imm, size: b, width: n, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: b, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: b, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: False, target: imm, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: sb, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: False, target: imm, size: h, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: imm, size: h, width: n, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: h, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: h, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: False, target: imm, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, target: reg, size: sh, width: n, cache_enabled: True }


- { code: flash, memory: sram, lbEn: False, target: imm, size: b, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: imm, size: b, width: n, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: b, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: b, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: False, target: imm, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: sb, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: False, target: imm, size: h, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: imm, size: h, width: n, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: h, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: h, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: False, target: imm, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, target: reg, size: sh, width: n, cache_enabled: True }

# Additional tests with flash line buffer turned on.

- { code: flash, memory: flash, lbEn: True, target: lit, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: b, width: n, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: b, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: b, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: True, target: lit, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: sb, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: True, target: lit, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: h, width: n, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: h, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: h, width: n, cache_enabled: True }

- { code: flash, memory: flash, lbEn: True, target: lit, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: imm, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, target: reg, size: sh, width: n, cache_enabled: True }


- { code: sram, memory: flash, lbEn: True, target: imm, size: b, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: imm, size: b, width: n, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: b, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: b, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: True, target: imm, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: sb, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: sb, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: True, target: imm, size: h, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: imm, size: h, width: n, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: h, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: h, width: n, cache_enabled: True }

- { code: sram, memory: flash, lbEn: True, target: imm, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: sh, width: w, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, target: reg, size: sh, width: n, cache_enabled: True }


- { code: flash, memory: sram, lbEn: True, target: imm, size: b, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: imm, size: b, width: n, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: b, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: b, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: True, target: imm, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: sb, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: sb, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: True, target: imm, size: h, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: imm, size: h, width: n, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: h, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: h, width: n, cache_enabled: True }

- { code: flash, memory: sram, lbEn: True, target: imm, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: sh, width: w, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, target: reg, size: sh, width: n, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set values = ["0x01234567", "0x89ABCDEF"] %}
{% set half = size in ['h', 'sh'] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    mov.w  r8, 0

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
{% for offset_cnt in range(4) %}
    {% set offset = offset_cnt * (2 if half else 1) %}
    @ Prepare input values
    ldr.w  r6, =rep_{{val_idx}}_memory
    mov.w  r7, {{offset}}

{% for reps in range(1, 12) %}
    @ Reset flash line buffer
    ldr.w  r2, [r8]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {% if target == "lit" %}
            ldr{{size}}.{{width}} r5, rep_{{reps}}_{{offset_cnt}}_{{val_idx}}_memory
        {% elif target == "imm" %}
            ldr{{size}}.{{width}} r5, [r6, {{offset}}]
        {% elif target == "reg" %}
            ldr{{size}}.{{width}} r5, [r6, r7]
        {% else %}
            panic!("Invalid configuration")
        {% endif %} 
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]
    bl.w save

    b.n literal_pool_jump_{{reps}}_{{offset_cnt}}_{{val_idx}}

{% if target == "lit" %}
.align 3
    {% for _ in range(offset) %}
    .byte 0
    {% endfor %}
rep_{{reps}}_{{offset_cnt}}_{{val_idx}}_memory: .word {{values[val_idx]}}
{% endif %}

.ltorg

.align 2
literal_pool_jump_{{reps}}_{{offset_cnt}}_{{val_idx}}:
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.ltorg

.align 2
save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r5, r3, r4)}}
    bx.n lr

{{ section(memory) }}
.align 4
{% for val_idx in range(values|length) %}
    rep_{{val_idx}}_memory: 
        .word {{values[val_idx]}}
        .word {{values[val_idx]}}
{% endfor %}

{% endblock %}
