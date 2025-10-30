---
name: Measures access latency to doorbell and radio ram
description: >
    Timing tests for access to Radio Doorbell and Radio Ram.
dumped_symbols:
  times: 126 words
configurations:
- { code: gpram, base: ram, ldrInstr: ldr.w }
- { code: gpram, base: ram, ldrInstr: ldr.n }
- { code: gpram, base: dbell, ldrInstr: ldr.w }
- { code: gpram, base: dbell, ldrInstr: ldr.n }
- { code: sram, base: ram, ldrInstr: ldr.w }
- { code: sram, base: ram, ldrInstr: ldr.n }
- { code: sram, base: dbell, ldrInstr: ldr.w }
- { code: sram, base: dbell, ldrInstr: ldr.n }
- { code: flash, base: ram, ldrInstr: ldr.w }
- { code: flash, base: ram, ldrInstr: ldr.n }
- { code: flash, base: dbell, ldrInstr: ldr.w }
- { code: flash, base: dbell, ldrInstr: ldr.n }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:radio_mode = "full" %}
{% extends "asm.s.tpl" %}

{% set RFC_RAM_BASE = '0x21000000' %}
{% set RFC_DBELL_BASE = '0x40041004' %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt

    @ Prepare ldr input value
    {% if base == "ram" %}
      ldr.w  r5, ={{RFC_DBELL_BASE}}
    {% elif base == "dbell" %}
      ldr.w  r5, ={{RFC_RAM_BASE}}
    {% else %}
      unreachable()
    {% endif %}

    mov.w  r7, #0

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
{% for reps in range(5) %}
{% for delays in itertools.combinations_with_replacement(range(5), reps) %}
    @ Clear flags
    mov.w r9, #0
    msr.w apsr_nzcvq, r9

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Determinize state of Doorbell memory
    ldr.w r2, [r5]

    @ Get start counter value
    ldr.w  r2, [r8, {{CYCCNT}}]

    {% for delay in delays %}
    .rept {{delay}}
    add.n r1, r1
    .endr
    {{ ldrInstr }} r0, [r5]
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r8, {{CYCCNT}}]
    
    bl.w save_times

{% endfor %}
{% endfor %}

    b.w end_label

save_times:
    mrs.w r9, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r11)}}

    bx.n lr

{% endblock %}
