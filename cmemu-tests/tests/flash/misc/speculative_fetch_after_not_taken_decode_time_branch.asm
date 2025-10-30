name: Test of speculative fetch after not taken decode time branch
description: "Test of speculative fetch after not taken decode time branch. It uses an execute time branch after a not taken decode time branch. The line after the execute time branch is cached."
dumped_symbols:
  times: 1 words
configurations:
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 0 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 4 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 8 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 12 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 16 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 20 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 24 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 28 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 32 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 36 }
- { lbEn: True, wbEn: True, bytes_between_cbz_and_line_1: 40 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 0 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 4 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 8 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 12 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 16 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 20 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 24 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 28 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 32 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 36 }
- { lbEn: True, wbEn: False, bytes_between_cbz_and_line_1: 40 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 0 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 4 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 8 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 12 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 16 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 20 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 24 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 28 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 32 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 36 }
- { lbEn: False, wbEn: True, bytes_between_cbz_and_line_1: 40 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 0 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 4 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 8 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 12 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 16 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 20 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 24 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 28 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 32 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 36 }
- { lbEn: False, wbEn: False, bytes_between_cbz_and_line_1: 40 }
# Configurations generated from:
# product:
# - lbEn: [True, False]
#   wbEn: [True, False]
#   bytes_between_cbz_and_line_1: [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40]
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
    movs.n r7, #1
    adr.n r4, label1
    adds.n r4, #1
    ldr.n r2, line_after_cbz

    isb.w

    ldr.w r8, [r0, {{CYCCNT}}]

    adds.n r2, #1
    cmp.n r7, #1
    bne.n label2
    bx.n r4 @ or cbz - an execute-time branch (but cbz can't be used if bytes_between_cbz_and_line_1 == 0)

line_after_cbz:
    .skip {{bytes_between_cbz_and_line_1}}

label1:
    ldr.w r9, [r0, {{CYCCNT}}]

    bx.n lr

.skip 24

.align 3
label2:
    adds.n r2, #1

{% endblock %}
