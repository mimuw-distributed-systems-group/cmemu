---
name: Fred-generated test
description: 'Test flow: (conf. 0) label569'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: false
  lb_en: true
  wb_en: false
  jump_start: label569
  jump_label569: code_end
  code_end: code_end
  space_mod: 4
...

{% device:cache_enabled = cache_en %}
{% device:line_buffer_enabled = lb_en %}
{% device:write_buffer_enabled = wb_en %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Save original sp
    ldr.w  r0, =original_sp
    str.w  sp, [r0]

    b.w    tested_code
.thumb_func
end_label:
    @ Restore original sp
    ldr.w  r0, =original_sp
    ldr.w  sp, [r0]
{% endblock %}
{% block after %}
{{section(code_memory)}}


.align  4
.thumb_func
tested_code:
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Reset line buffer
    mov.w  r7, #0
    ldr.w  r1, [r7]

    @ Randomize values of registers
	mov.w	r0, #55788
	mov.w	r1, #18450
	mov.w	r2, #55477
	mov.w	r3, #45528
	mov.w	r4, #44918
	mov.w	r5, #23874
	mov.w	r6, #54004
	mov.w	r7, #22887
	mov.w	r8, #15606
	mov.w	r11, #13697
	mov.w	r12, #26785
	mov.w	r14, #28147

    @ Start the test
    b.w    start_test

.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r9, =stack
    add.w  r9, r9, #328
    mov.w  sp, r9

    @ Get counter address
    ldr.w  r9, =counter_idx
    ldr.w  r9, [r9]
    ldr.w  r10, =counters_to_test
    ldr.w  r9, [r10, r9]
    @ Get counter start value
    ldr.w  r10, [r9]
        @ r9 – counter address
        @ r10 – counter start value

    @ Jump to the 1st block

    b.w    {{jump_start}}

.ltorg


.align	1
.space 26664 % {{space_mod|default("0x10000000")}}
label569:
	sub	r13, #76  @ 2b  @ 2b  @ 2b
mov	r1, r13  @ 2b  @ 2b  @ 2b
.space 4  @ 4b
mov	r11, #92  @ 4b  @ 4b  @ 4b
.space 38   @ 38b
	add	r5, r13, r0, LSL #2           @ A7.7.6  @ 4b  @ 4b  @ 4b
.space 32   @ 32b

ldr	r13, =cell_2171  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r13, [r13]  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
jump_from_23:
	add	r15, r13, r15                 @ A7.7.46  @ 2b  @ 2b  @ 2b

.space 6   @ 6b  @ 6b
forward_label_886:
mov	r13, r1  @ 2b  @ 2b  @ 2b
	subs	r13, r13, r11                 @ A7.7.177  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	strb	r7, [r13], #168               @ A7.7.163  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
end_label569:
	b.w	{{jump_label569}}

.ltorg
.align	2
.space	1, 0xbf
.space 23 % {{space_mod|default("0x10000000")}}
.global	cell_2171
cell_2171:	.word	(forward_label_886 - jump_from_23 - 4)

.space	3, 0xbf
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 1736 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:
    @ Get counter finish value
    ldr.w  r14, [r9]
    @ Calculate counter difference
    sub.w  r14, r14, r10
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r10, cyccnt_addr
    cmp.w  r9, r10
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{ saveValue("counters", r14, r9, r10) }}
    @ Save values of registers
	{{saveValue("registers", r0, r9, r10)}}
	{{saveValue("registers", r1, r9, r10)}}
	{{saveValue("registers", r2, r9, r10)}}
	{{saveValue("registers", r3, r9, r10)}}
	{{saveValue("registers", r4, r9, r10)}}
	{{saveValue("registers", r5, r9, r10)}}
	{{saveValue("registers", r6, r9, r10)}}
	{{saveValue("registers", r7, r9, r10)}}
	{{saveValue("registers", r8, r9, r10)}}
	{{saveValue("registers", r11, r9, r10)}}
	{{saveValue("registers", r12, r9, r10)}}
    @ Advance counter_idx and repeat or end the test
    ldr.w  r9, =counter_idx
    ldr.w  r10, [r9]
    add.w  r10, r10, #4
    str.w  r10, [r9]
    cmp.w  r10, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2

cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	0, 0xbf
.space 213 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 0xbf
.space 209 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 0xbf
.space 264 % {{space_mod|default("0x10000000")}}



@ safeSpaces:
{{section('flash')}}
.align  4
.global safeSpaceFlash
safeSpaceFlash:      .space  1024, 0xbf       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceFlash, .-safeSpaceFlash

{{section('sram')}}
.align  4
.global safeSpaceSram
safeSpaceSram:      .space  1024, 0xbf       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceSram, .-safeSpaceSram

{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  4
.global safeSpaceGpramSram
safeSpaceGpramSram: .space  1024, 0xbf       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceGpramSram, .-safeSpaceGpramSram


@ Stack:
{{section('sram')}}
.align  4
.global stack
stack:  .space  400, 0xbf    @ 256B of stack + upper and lower safety offsets for ldm/stm
.size   stack, .-stack


@ Test's data:

{{section('flash')}}
.align 2
.global counters_to_test
counters_to_test:    .word {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CPICNT_ADDR}}, {{LSUCNT_ADDR}}, {{FOLDCNT_ADDR}}
end_counters_to_test:



{{section('sram')}}

.align  2
.global original_sp
original_sp:        .word   0x00000000

.align  2
.global counter_idx
counter_idx:     .word   0


{% endblock %}