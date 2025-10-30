---
name: Fred-generated test
description: 'Test flow: (conf. 0) label577'
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
  jump_start: label577
  jump_label577: code_end
  code_end: code_end
...

{% device:cache_enabled = cache_en %}
{% device:line_buffer_enabled = lb_en %}
{% device:write_buffer_enabled = wb_en %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Save original sp
    ldr.w  r11, =original_sp
    str.w  sp, [r11]

    b.w    tested_code
.thumb_func
end_label:
    @ Restore original sp
    ldr.w  r11, =original_sp
    ldr.w  sp, [r11]
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
    ldr.w  r2, [r7]

    @ Randomize values of registers
	mov.w	r0, #861
	mov.w	r1, #18105
	mov.w	r2, #31628
	mov.w	r3, #27031
	mov.w	r4, #9226
	mov.w	r5, #39504
	mov.w	r6, #63070
	mov.w	r7, #9472
	mov.w	r8, #48495
	mov.w	r9, #10820
	mov.w	r10, #63155

    @ Start the test
    b.w    start_test


.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r11, =stack
    add.w  r11, r11, #328
    mov.w  sp, r11

    @ Get counter address
    ldr.w  r11, =counter_idx
    ldr.w  r11, [r11]
    ldr.w  r12, =counters_to_test
    ldr.w  r11, [r12, r11]
    @ Get counter start value
    ldr.w  r12, [r11]
        @ r11 – counter address
        @ r12 – counter start value

    @ Jump to the 1st block
    b.w    {{jump_start}}



.align	1
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 620 % {{space_mod|default("0x10000000")}}
func_2:
.space 2  @ 2b  @ 2b
ldr	r0, cell_46  @ 4b  @ 4b  @ 4b
mov	r3, #55  @ 4b  @ 4b  @ 4b
.space 14   @ 14b  @ 14b
mov	r3, #61875  @ 4b  @ 4b  @ 4b
.space 4   @ 4b

mov	r2, #38  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
mov	r10, #5  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
ldr	r9, cell_43  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r3, cell_42  @ 4b  @ 4b  @ 4b
.space 68   @ 68b
ldr	r9, cell_38  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b
ldr	r10, cell_37  @ 4b  @ 4b  @ 4b
.space 10   @ 10b  @ 10b
mov	r0, #18180  @ 4b  @ 4b  @ 4b
.space 26   @ 26b  @ 26b
	umull	r0, r3, r2, r5                @ A7.7.204  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	mla	r9, r9, r10, r3               @ A7.7.74  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	mla	r2, r2, r5, r9                @ A7.7.74  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
.space 8   @ 8b  @ 8b @ looks important!
end_func_2:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_46
cell_46:	.word	safeSpaceSram+846

.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_38
cell_38:	.word	safeSpaceGpramSram+228

.space	1, 45
.global	cell_37
cell_37:	.word	safeSpaceFlash-35634

.space	0, 45
.space 15 % {{space_mod|default("0x10000000")}}
.global	cell_42
cell_42:	.word	safeSpaceFlash+752

.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_43
cell_43:	.word	safeSpaceSram+568

.space	1, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 58892 % {{space_mod|default("0x10000000")}}
label577:
ldr	r14, =post_branch_727  @ 4b  @ 4b  @ 4b
ldr	r3, =func_2  @ 2b  @ 2b  @ 2b
ldr	r4, cell_3451  @ 4b  @ 4b  @ 4b
ldr	r2, cell_3450  @ 4b  @ 4b  @ 4b
mov	r8, #1  @ 4b  @ 4b  @ 4b
.space 4  @ 4b
orr	r3, #1  @ 4b  @ 4b  @ 4b
.space 28   @ 28b
orr	r14, #1  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	ldmdb	r4!, {r1-r2,r8-r9}            @ A7.7.42  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	bx	r3                            @ A7.7.20  @ 2b @ looks important!  @ 2b  @ 2b @ looks important!
post_branch_727:


.space 32   @ 32b
ldr	r5, cell_3447  @ 4b  @ 4b  @ 4b @ looks important!
.space 20   @ 20b @ looks important!
end_label577:
	b.w	{{jump_label577}}

.ltorg
.align	2
.space	2, 46
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_3451
cell_3451:	.word	safeSpaceFlash+812

.space	0, 45
.global	cell_3450
cell_3450:	.word	safeSpaceGpramSram+270

.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_3447
cell_3447:	.word	safeSpaceFlash+75

.space	1, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 7223 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:

    @ Get counter finish value
    ldr.w  r14, [r11]
    @ Calculate counter difference
    sub.w  r14, r14, r12
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r12, cyccnt_addr
    cmp.w  r11, r12
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{saveValue("counters", r14, r11, r12)}}

    @ Save values of registers
	{{saveValue("registers", r0, r11, r12)}}
	{{saveValue("registers", r1, r11, r12)}}
	{{saveValue("registers", r2, r11, r12)}}
	{{saveValue("registers", r3, r11, r12)}}
	{{saveValue("registers", r4, r11, r12)}}
	{{saveValue("registers", r5, r11, r12)}}
	{{saveValue("registers", r6, r11, r12)}}
	{{saveValue("registers", r7, r11, r12)}}
	{{saveValue("registers", r8, r11, r12)}}
	{{saveValue("registers", r9, r11, r12)}}
	{{saveValue("registers", r10, r11, r12)}}

    @ Advance counter_idx and repeat or end the test
    ldr.w  r11, =counter_idx
    ldr.w  r12, [r11]
    add.w  r12, r12, #4
    str.w  r12, [r11]
    cmp.w  r12, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2
cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	1, 46
.space 131 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 121 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 8 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	2, 46
.space 223 % {{space_mod|default("0x10000000")}}



@ safeSpaces:
{{section('flash')}}
.align  4
.global safeSpaceFlash
safeSpaceFlash:      .space  1024, 41       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceFlash, .-safeSpaceFlash

{{section('sram')}}
.align  4
.global safeSpaceSram
safeSpaceSram:      .space  1024, 42       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceSram, .-safeSpaceSram

{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  4
.global safeSpaceGpramSram
safeSpaceGpramSram: .space  1024, 43       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceGpramSram, .-safeSpaceGpramSram


@ Stack:
{{section('sram')}}
.align  4
.global stack
stack:  .space  400, 44    @ 256B of stack + upper and lower safety offsets for ldm/stm
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