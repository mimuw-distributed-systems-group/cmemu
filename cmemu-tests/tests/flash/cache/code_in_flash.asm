name: Cache with code in flash
description: "Check how cache behaves when code is in flash"
dumped_symbols:
  times: 1 words
  evics: 14 words
configurations:
- {}
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set cache_sets = 256 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
    
    b.w    tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section("sram") }}

.align 4
.thumb_func
.type tested_code, %function

tested_code:
    {{callHelper('enable_cache', r2)}}
    {{callHelper('synchronize_rng', r2, '#240')}}
    {{callHelper('disable_cache', r2)}}
    {{callHelper('enable_cache', r2)}}

    ldr.w r9, =tag0
    @ Prepare 14 full sets
    {% for _ in range(4) %}
        add.w r9, #{{14 * 8}}
        {% for _ in range(14) %}
            sub.w r9, #8
            ldr.w r10, [r9]
        {% endfor %}
        add.w r9, #{{256 * 8}}
    {% endfor %}

    @ Evict 7 of them
    add.w r9, #{{7 * 8}}
    {% for _ in range(7) %}
        sub.w r9, #8
        ldr.w r10, [r9]
    {% endfor %}

    @ Prepare the address of the test
    ldr.w r11, =flash_code + 1

    @ Prepare addresses to ldr from
    sub.w r0, r9, #{{4 * 256 * 8}}
    add.w r0, #{{14 * 8}}

    isb.w

    ldr.w r9, [r8, {{CYCCNT}}]

    @ Jump to a test in flash (with code in addresses in sets other than those 14)
    blx.n r11
    nop.n

    ldr.w r10, [r8, {{CYCCNT}}]
    sub.w r9, r10, r9

    {{saveValue('times', r9, r10, r11)}}

    @ Evict the other 7
    ldr.w r9, =tag4
    add.w r9, #{{14 * 8}}
    {% for _ in range(7) %}
        sub.w r9, #8
        ldr.w r10, [r9]
    {% endfor %}

    @ Check evicted ways
    @ Prepare DWT
    mov.w r0, r8
    sub.w r1, r9, #{{7 * 8}}
    sub.w r1, #{{4 * 8 * 256}}
    {% for _ in range(14) %}
        mov.w r8, #4
        mov.w r9, #4
        {% for i in range(4) %}
            isb.w
            ldr.n r2, [r0, {{CYCCNT}}]
            ldr.n r4, [r1]
            ldr.n r3, [r0, {{CYCCNT}}]
            nop.n
            sub.w r2, r3, r2
            cmp.w r2, #5
            nop.n
            it.n gt
            movgt.w r8, #{{i}}
            cmp.w r8, r9
            nop.n
            it.n lt
            movlt.w r9, r8
            add.w r1, #{{8 * 256}}
        {% endfor %}
        bl.w save_evic
        sub.w r1, #{{4 * 8 * 256}}
        add.w r1, #8
    {% endfor %}

    {{callHelper('disable_cache', r2)}}

    b.w end_label

save_evic:
    {{saveValue('evics', r9, r10, r11)}}
    bx.n lr

{{ section("flash") }}

.equ n_addrs_for_ldrs, 5

@ Layout of flash:
@ sets 0..14: for checking RNG shift
@ sets 14..14+n_addrs_for_ldrs: for ldrs in the test
@ sets 14+n_addrs_for_ldrs..256: for test code

.align 11
flash_code_tag:
.skip 14 * 8
.skip n_addrs_for_ldrs * 8

flash_code:
    ldr.w r2, [r0]
    ldr.w r2, [r0, #8]
    bx.n lr
flash_code_end:

.if flash_code_end > flash_code_tag + 256 * 8
.err
.endif

.align 11
tag0:
.space 256 * 8

tag1:
.space 256 * 8

tag2:
.space 256 * 8

tag3:
.space 256 * 8

tag4:
.space 256 * 8

{% endblock %}
