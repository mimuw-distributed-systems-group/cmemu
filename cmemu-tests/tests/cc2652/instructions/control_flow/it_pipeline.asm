---
name: LDR and STR pipelining inside IT block
description: >-
    Checks LDR and STR pipelining inside IT block.
dumped_symbols:
  times: 168 words
  results: 168 words
configurations:
- { code: "sram", lbEn: true, zeroFlagValue: True }
- { code: "sram", lbEn: true, zeroFlagValue: False }
- { code: "flash", lbEn: true, zeroFlagValue: True }
- { code: "flash", lbEn: true, zeroFlagValue: False }
- { code: "flash", lbEn: false, zeroFlagValue: True }
- { code: "flash", lbEn: false, zeroFlagValue: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt

    @ Prepare memory cell address
    ldr.w  r9, =memory_cell

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{# Without "t" case: the code couldn't fit in GPRAM, and there's no pipelining there anyway #}
{% set masks = ["teee", "tee", "teet", "te", "tete", "tet", "tett",
                "ttee", "tte", "ttet", "tt", "ttte", "ttt", "tttt"] %}
{% set te2cond = {"e": "ne", "t": "eq"} %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for mask in masks %}
{% for ldrStrMask in range(2 ** (mask|length)) %}
    @ r10 is trash register
    @ Flush flash line buffer
    mov.w  r10, #0
    ldr.w  r10, [r10, r10]

    @ Set zero flag value
    movs.w r10, #{{0 if zeroFlagValue else 1}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r8, {{CYCCNT}}]

    i{{mask}}.n {{te2cond["t"]}}
    {% for i in range(mask|length) %}
        {% if (ldrStrMask // (2 ** i)) % 2 == 1 %} {# jinja2 doesn't support bitwise operators :( #}
            ldr{{te2cond[mask[i]]}}.w r10, [r9]
        {% else %}
            str{{te2cond[mask[i]]}}.w r10, [r9]
        {% endif %}
    {% endfor %}

    @ Get finish time
    ldr.w  r11, [r8, {{CYCCNT}}]

    @ Save the times and results
    bl.w save
{% endfor %}
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    subs.w r2, r11, r2

    {{saveTime(r2, r11, r10)}}
    bx.n lr

{{ section("sram") }}

.align 2
memory_cell: .word 0

{% endblock %}
