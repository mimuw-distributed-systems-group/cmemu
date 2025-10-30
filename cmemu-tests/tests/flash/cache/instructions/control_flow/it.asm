---
name: IT correctness
description: >-
  Check if it behaves correctly: timing and conditional execution.
  Not focusing on folding.
dumped_symbols:
  times: 100 words
  results: 100 words
configurations:
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'pl', neg_cond: 'mi', replaceSecond: null, cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'mi', neg_cond: 'pl', replaceSecond: null, cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'pl', neg_cond: 'mi', replaceSecond: null, cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'pl', neg_cond: 'mi', replaceSecond: null, cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'mi', neg_cond: 'pl', replaceSecond: null, cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'mi', neg_cond: 'pl', replaceSecond: null, cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'cs', neg_cond: 'cc', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'cc', neg_cond: 'cs', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'cs', neg_cond: 'cc', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'cs', neg_cond: 'cc', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'cc', neg_cond: 'cs', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'cc', neg_cond: 'cs', replaceSecond: 'mla{cond}.w r0, r0, r0, r0', cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'eq', neg_cond: 'ne', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'ne', neg_cond: 'eq', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'eq', neg_cond: 'ne', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'eq', neg_cond: 'ne', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'movs.n r0, #0', cond: 'ne', neg_cond: 'eq', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'movs.n r0, #0', cond: 'ne', neg_cond: 'eq', replaceSecond: 'cmp{cond}.n r0, #1', cache_enabled: True }
- { code: "sram", lbEn: True, setFlags: 'b.n inside_block_{mask}', cond: 'eq', neg_cond: 'ne', replaceSecond: ".thumb_func \n inside_block_{mask}: \n add{cond}.w r5, r5, r3, LSL #2", cache_enabled: True }
- { code: "flash", lbEn: True, setFlags: 'b.n inside_block_{mask}', cond: 'eq', neg_cond: 'ne', replaceSecond: ".thumb_func \n inside_block_{mask}: \n add{cond}.w r5, r5, r3, LSL #2", cache_enabled: True }
- { code: "flash", lbEn: False, setFlags: 'b.n inside_block_{mask}', cond: 'eq', neg_cond: 'ne', replaceSecond: ".thumb_func \n inside_block_{mask}: \n add{cond}.w r5, r5, r3, LSL #2", cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
    mov.w  r9, {{CYCCNT}}

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set masks = ["teee", "tee", "teet", "te", "tete", "tet", "tett", "t",
                "ttee", "tte", "ttet", "tt", "ttte", "ttt", "tttt"] %}
{% set condRegs = ["r4", "r5", "r6", "r7"] %}
{% set te2cond = {"e": neg_cond, "t": cond} %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for mask in masks %}
    @ r10 is trash register
    @ Flush flash line buffer
    mov.w  r10, #0
    ldr.w  r10, [r10, r10]

    @ Reset important registers
    {% for reg in condRegs %}
        mov.w {{reg}}, #0
    {% endfor %}
    mov.w  r3, #1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r8, r9]

    {{setFlags.format(mask = mask)}}
    i{{mask}}.n {{cond}}
    {% for i in range(mask|length) %}
        {% if replaceSecond and i == 1 %}
            {{replaceSecond.format(cond = te2cond[mask[i]], mask = mask)}}
        {% else %}
            add{{te2cond[mask[i]]}}.n {{condRegs[i]}}, r3
        {% endif %}
    {% endfor %}

    {% if mask|length < 2 %}
        @ Hack for scenario "Jump into middle of it block"
        .thumb_func
        inside_block_t:
    {% endif %}

    @ Get finish time
    ldr.w  r11, [r8, r9]
    subs.w r2, r11, r2

    @ Save the times and results
    bl.w save
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    {{saveTime(r2, r11, r10)}}
    {% for reg in condRegs %}
        {{saveResult(reg, r11, r10)}}
    {% endfor %}
    bx.n lr

{% endblock %}
