name: Test of code and data in flash with branches
description: "A series of loads from flash in flash with branches"
dumped_symbols:
  times: 1 words
configurations:
- { lbEn: True, wbEn: True }
- { lbEn: True, wbEn: False }
- { lbEn: False, wbEn: True }
- { lbEn: False, wbEn: False }
# Configurations generated from
# product:
# - lbEn: [True, False]
#   wbEn: [True, False]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = wbEn %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    
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
    ldr.w r1, =flash_part + 1
    {{callHelper('enable_cache', r2)}}
    {{callHelper('synchronize_rng', r2, '#240')}}
    {{callHelper('disable_cache', r2)}}
    {{callHelper('enable_cache', r2)}}
    blx.n r1
.align 2
    {{callHelper('disable_cache', r1)}}
    sub.w r9, r8
    {{saveValue('times', r9, r1, r2)}}
    b.w end_label

{{ section("flash") }}

@ align to set 0
.align 11
flash_part:
    adr.w r1, ldrs
    add.w r2, r1, #2048
    add.w r3, r2, #2048
    add.w r4, r3, #2048
    add.w r5, r4, #2048
    add.w r6, r5, #2048

    mov.w r10, #50

    .align 3
    isb.w

    ldr.w r8, [r0, {{CYCCNT}}]

ldrs:
    ldr.n r7, [r4, #40]
    ldr.n r7, [r5, #48]
    ldr.n r7, [r2, #56]
    ldr.n r7, [r5, #80]
    ldr.n r7, [r1, #24]
    ldr.n r7, [r1, #0]
    ldr.n r7, [r4, #96]
    ldr.n r7, [r2, #88]
    ldr.n r7, [r6, #88]
    ldr.n r7, [r1, #0]
    ldr.n r7, [r2, #32]
    ldr.n r7, [r2, #32]
    ldr.n r7, [r1, #48]
    ldr.n r7, [r2, #24]
    ldr.n r7, [r2, #48]
    ldr.n r7, [r6, #40]
    ldr.n r7, [r6, #56]
    ldr.n r7, [r6, #32]
    ldr.n r7, [r3, #72]
    ldr.n r7, [r3, #0]
    ldr.n r7, [r1, #72]

    b.w jump_forward
    nop.w
    nop.w
    nop.w
jump_forward:

    ldr.n r7, [r4, #64]
    ldr.n r7, [r6, #16]
    ldr.n r7, [r4, #24]
    ldr.n r7, [r3, #24]
    ldr.n r7, [r3, #8]
    ldr.n r7, [r3, #0]
    ldr.n r7, [r6, #96]
    ldr.n r7, [r6, #64]
    ldr.n r7, [r5, #16]
    ldr.n r7, [r1, #16]
    ldr.n r7, [r2, #48]
    ldr.n r7, [r2, #64]
    ldr.n r7, [r2, #40]
    ldr.n r7, [r3, #48]
    ldr.n r7, [r6, #48]
    ldr.n r7, [r6, #0]
    ldr.n r7, [r2, #88]
    ldr.n r7, [r4, #96]
    ldr.n r7, [r2, #64]
    ldr.n r7, [r6, #40]
    ldr.n r7, [r1, #48]
    ldr.n r7, [r2, #8]
    ldr.n r7, [r4, #56]
    ldr.n r7, [r4, #0]
    ldr.n r7, [r6, #80]
    ldr.n r7, [r3, #24]
    ldr.n r7, [r6, #16]
    ldr.n r7, [r6, #80]
    ldr.n r7, [r5, #48]

    subs.w r10, #1
    bne.w ldrs

    ldr.w r9, [r0, {{CYCCNT}}]

    bx.n lr

@ align to set 0 (next tag)
.align 11
.skip ldrs - flash_part
next_set:
.space 200

{% endblock %}
