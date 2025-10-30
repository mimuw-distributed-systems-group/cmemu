---
name: Test the behavior of speculative branches if the address phase was not presented on the bus.
description: |
    In theory, the speculative branch would attempt to cancel the existing address phase.
    We need to check if with AHB_CONST_CTRL it changes during a DENIED response to Fetch Transfers.
dumped_symbols:
  cyccnt: auto
configurations:
- {code: gpram, lbEn: true}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}


{% block code %}
    ldr.w r10, dwt
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section("flash") }}
.align 3
flash_cell:
udf 0x42
.word 0

{{ section("sram") }}
.align 3
sram_cell:
udf 0x42
.word 0

{{ section("gpram") }}

.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for br_prefetch in ('label', 'literal') %}

    mov.w r0, 4
    mov.w r1, 0
    {{mov_const_2w(r3, "literal")}}

    .align 3
    isb.w
    ldr.w r5, [r10, {{CYCCNT}}]

@ Reload pipeline for the example to work
    {{ 'isb.w' }} @ not a test
    adds.w r0, r0
    umull.w r1, r0, r0, r1
    ldr.w r2, [r3]
    nop.w
    @ We cannot simply get sram/flash because of 19 bit reloc
    beq.w {{br_prefetch}}
    nop.w
    nop.w
    {% if br_prefetch == 'label' %}label:{% endif %}
    ldr.w r6, [r10, {{CYCCNT}}]

    ldr.w r11, =save_results
    blx r11
    {{inc_auto_syms()}}
{% endfor %}
    @ Continue the test somewhere else
    ldr.w pc, =tested_code2


.align 3
literal:
udf 0x42
.word 0
.ltorg

{{ section("flash") }}
.align 4
.thumb_func
.type tested_code2, %function
tested_code2:

{% set x_load, x_exec = n_x_cycles(12, r1, r2) %}
{% for variant in range(4) %}
{% for pad in ('', 'add.w r1, r1', 'nop.w;\n nop.w;') %}
{% for br_prefetch in ('label', 'self', 'literal2') %}
    mov.w r0, 4
    mov.w r1, 0
    {{x_load}}
    {% set last_fetch_label = uniq_label("fetched") %}
    {% set after_label = uniq_label("after") %}
    {{mov_const_2w(r3, last_fetch_label)}}

    .align 4
    isb.w
    {{pad}}
    ldr.w r5, [r10, {{CYCCNT}}]

@ Reload pipeline for the example to work
    {{ 'isb.w' }} @ not a test
    adds.w r0, r0
    {{x_exec}}
    {% if variant == 0 %}
        @ br will start normal fetching, but conflicts
        ldr.w r2, [r3] @ This should be in line buffer!
        nop.w
    {% elif variant == 1 %}
        add.n r0, r0
        @ fetching should be speculative
        ldr.n r2, [r3] @ This should be in line buffer!
        nop.w
    {% elif variant == 2 %}
        ldr.w r2, [r3] @ This should be in line buffer!
        ldr.w r2, [r3] @ This should be in line buffer!
    {% elif variant == 3 %}
        @ One word shorter
        ldr.w r2, [r3]
    {% endif %}
    @ We cannot simply get sram/flash because of 19 bit reloc
{{last_fetch_label}}:
    beq.w {{{'self': '.', 'label': after_label, 'literal2': 'literal2'}[br_prefetch]}}
    nop.w
    nop.w
    {{after_label}}:
    ldr.w r6, [r10, {{CYCCNT}}]

    ldr.w r11, =save_results
    blx r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endfor %}

    ldr.w pc, =end_label

.align 3
literal2:
udf 0x43
.word 0
.ltorg

.align 3
.thumb_func
save_results:
    sub.w r6, r5
    {{saveValue('cyccnt', r6, r7, r8)}}

    bx.n lr


{% endblock %}
