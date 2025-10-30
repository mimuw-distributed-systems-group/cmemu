---
name: STM instruction tests
description: Timing of STM preceded by register dependant instruction
dumped_symbols:
# 3 (registers) * 2 (writeback) * 2 (preinstr) * 3 (dep)
  times: 36 words
  memory: user-defined
configurations:
- { code: gpram, data: sram, lbEn: true }
- { code: sram, data: sram, lbEn: true }
- { code: flash, data: sram, lbEn: false }
- { code: flash, data: sram, lbEn: true }
- { code: gpram, data: gpram, lbEn: true }
- { code: sram, data: gpram, lbEn: true }
- { code: flash, data: gpram, lbEn: false }
- { code: flash, data: gpram, lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set register_sets = [
        ["r0", "r8", "r9"],
        ["r0", "r8", "lr"],
        ["r0", "r8", "r9", "lr"],
    ]
%}

{% set memory_cell_size = 4 %}

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
{% for wback in (False, True) %}
{% for preinstr in ["add.w", "ldr.w"] %}
{% for dep in ["source", "target", "none"] %}
    @ Prepare input values
    {% for reg in registers %}
        mov.w {{ reg }}, #1
    {% endfor %}

    ldr.w r1, =memory
    ldr.w r11, =memory_adr
    @ zero memory cell
    mov.w r10, #0
    {% for offset in range(memory_cell_size) %}
        str.w r10, [r1, {{ offset * 4 }}]
    {% endfor %}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r6, {{CYCCNT}}]

    {% if dep == "source" %}
        {% if preinstr == "add.w" %}
            add.w r1, r1, 0
        {% elif preinstr == "ldr.w" %}
            ldr.w r1, [r11, 0]
        {% else %}
            panic!
        {% endif %}
    {% elif dep == "target" %}
        {% set reg = registers[0] %}

        {% if preinstr == "add.w" %}
            add.w {{ reg }}, {{ reg }}, 0
        {% elif preinstr == "ldr.w" %}
            ldr.w {{ reg }}, [r11, 4]
        {% else %}
            panic!
        {% endif %}
    {% elif dep != "none" %}
        panic!
    {% endif %}

    stm.w r1 {{ "!" if wback }}, { {{ registers|join(", ") }} }

    @ Finish time
    ldr.w  r5, [r6, {{CYCCNT}}]

    @ Save times
    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.ltorg

.thumb_func
save:
    @ Finish time - Start time
    sub.w r5, r5, r4

    {{ saveValue("times", r5, r3, r4) }}
    bx.n lr

.ltorg

{{ section(data) }}
.align 4
.global memory
memory:
{% for _ in range(memory_cell_size) %}
    .word 0
{% endfor %}
.size memory, .-memory
.align 4
.global memory_adr
memory_adr:
    .word memory
{% for _ in range(7) %}
    .word 0
{% endfor %}
.size memory_adr, .-memory_adr
{% endblock %}
