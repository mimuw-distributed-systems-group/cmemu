---
name: Tool C helpers mechanism test
description: "Verify that the tool's mechanism for calling C helpers is working correctly"
dumped_symbols:
  offset: 1 words
configurations:
- {}
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section("sram") }}

{% set checked_registers = [r0, r1, r3, r4, r5, r6, r7, r8, r9, r11, r12, sp, lr] %}
registers: .space 52

.align 4
.thumb_func
.type tested_code, %function
tested_code:
    {% for register in checked_registers %}
        adr.w  r2, registers + {{4 * loop.index0}}
        str.w  {{register}}, [r2]
    {% endfor %}
    {{callHelper('enable_cache', r10)}}
    {{callHelper('synchronize_rng', r2, '#240')}}
    {{callHelper('shift_rng', r2, '#33', '#200')}}
    {{callHelper('check_rng', r10, '#160')}}
    {{callHelper('disable_cache', r2)}}
    {% for register in checked_registers %}
        ldr.w  r2, registers + {{4 * loop.index0}}
        cmp.w  {{register}}, r2
        bne.w  fail
    {% endfor %}

    cmp.w  r10, #33
    bne.w  fail
    {{saveValue('offset', r10, r2, r3)}}

    b.w end_label

fail:
    udf.w  #0

{% endblock %}
