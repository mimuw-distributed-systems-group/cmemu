---
name: Fred-generated test
description: 'Test flow: (conf. 0) label456 -> label59 -> label564 -> label32 -> label157
  -> label150'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: true
  lb_en: true
  wb_en: false
  jump_start: label456
  jump_label456: label59
  jump_label59: label564
  jump_label564: label32
  jump_label32: label157
  jump_label157: label150
  jump_label150: code_end
  code_end: code_end
  space_mod: 2048
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
	mov.w	r0, #30044
	mov.w	r1, #1464
	mov.w	r2, #56123
	mov.w	r3, #23788
	mov.w	r4, #63130
	mov.w	r5, #36038
	mov.w	r6, #58885
	mov.w	r7, #42661
	mov.w	r8, #54502
	mov.w	r9, #54013
	mov.w	r10, #33021

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
.space 132 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.space 4473 % {{space_mod|default("0x10000000")}}
label32:
.space 4
ldr	r9, =forward_label_57  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
orr	r9, #1  @ 4b  @ 4b  @ 4b
ldr	r2, cell_194  @ 4b  @ 4b  @ 4b
.space 20   @ 20b
ldr	r1, cell_191  @ 2b  @ 2b  @ 2b
.space 72   @ 72b
forward_label_57:
.space 28   @ 28b  @ 28b
end_label32:
	b.w	{{jump_label32}}

.ltorg
.align	2
.space	0, 45
.space 10 % {{space_mod|default("0x10000000")}}
.global	cell_194
cell_194:	.word	safeSpaceGpramSram-67428

.space	2, 45
.global	cell_191
cell_191:	.word	safeSpaceSram-152291

.space	3, 45
.space 22 % {{space_mod|default("0x10000000")}}

.align	1
.space 1630 % {{space_mod|default("0x10000000")}}
label59:
ldr	r14, =post_branch_79  @ 4b  @ 4b  @ 4b
.space 82   @ 82b
post_branch_79:



.space 12   @ 12b  @ 12b
end_label59:
	b.w	{{jump_label59}}

.ltorg
.align	2
.space	1, 45
.space 113 % {{space_mod|default("0x10000000")}}
func_11:
ldr	r1, cell_380  @ 2b  @ 2b  @ 2b
ldr	r2, cell_379  @ 4b  @ 4b  @ 4b
ldr	r0, cell_378  @ 4b  @ 4b  @ 4b
mov	r3, #13297  @ 4b  @ 4b  @ 4b
ldr	r10, cell_377  @ 4b  @ 4b  @ 4b
	strd	r1, r5, [r13, #52]            @ A7.7.166  @ 4b  @ 4b  @ 4b
.space 64   @ 64b  @ 64b
	ldr	r0, cell_366                  @ A7.7.44 @ 4b  @ 4b  @ 4b  @ 4b
mov	r10, #27426  @ 4b  @ 4b  @ 4b
ldr	r0, cell_365  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
	ldrh	r9, [r2, r10, LSL #2]         @ A7.7.57 @ 4b  @ 4b  @ 4b  @ 4b
	isb	                              @ A7.7.37 @ 4b  @ 4b  @ 4b  @ 4b
	ite	ne  @ 2b  @ 2b  @ 2b
	ldrhne	r2, [r13]                     @ A7.7.55 @ 4b @ 4b @ 4b
	moveq	r2, r15                       @ A7.7.77 @ 2b @ 2b @ 2b


.space 14   @ 14b
ldr	r2, cell_353  @ 4b  @ 4b  @ 4b
	str	r2, [r13]                     @ A7.7.161  @ 2b  @ 2b  @ 2b
.space 30   @ 30b
	ldm	r13, {r0-r3,r9-r10}           @ A7.7.41  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b  @ 16b
end_func_11:
	bx	r14

.ltorg
.align	2
.space 17 % {{space_mod|default("0x10000000")}}
.global	cell_378
cell_378:	.word	safeSpaceSram+84

.align	2
.space 33 % {{space_mod|default("0x10000000")}}
.global	cell_379
cell_379:	.word	safeSpaceSram-109082

.align	2
.space 9 % {{space_mod|default("0x10000000")}}
.global	cell_377
cell_377:	.word	safeSpaceSram+322

.space	2, 45
.global	cell_366
cell_366:	.word	0x63f972a1

.space	0, 45
.global	cell_353
cell_353:	.word	safeSpaceGpramSram+795

.align	2
.space 36 % {{space_mod|default("0x10000000")}}
.global	cell_380
cell_380:	.word	safeSpaceFlash-105547

.align	2
.space 42 % {{space_mod|default("0x10000000")}}
.global	cell_365
cell_365:	.word	safeSpaceSram-27351

.align	2
.space 9794 % {{space_mod|default("0x10000000")}}
func_19:
.space 50   @ 50b
ldr	r9, cell_766  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b  @ 12b
ldr	r1, cell_765  @ 4b  @ 4b  @ 4b
.space 92   @ 92b
end_func_19:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_765
cell_765:	.word	safeSpaceSram-5663

.space	1, 45
.space 23 % {{space_mod|default("0x10000000")}}
.global	cell_766
cell_766:	.word	safeSpaceSram-24051

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 3094 % {{space_mod|default("0x10000000")}}
label150:
.space 8   @ 8b
ldr	r14, =post_branch_193  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b
ldr	r6, cell_991  @ 2b  @ 2b  @ 2b
.space 66   @ 66b  @ 66b
	bne	func_11                       @ A7.7.12 @ 4b  @ 4b  @ 4b  @ 4b
post_branch_193:


.space 4  @ 4b  @ 4b  @ 4b
end_label150:
	b.w	{{jump_label150}}

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_991
cell_991:	.word	safeSpaceSram-111848

.space	1, 45
.space 737 % {{space_mod|default("0x10000000")}}
label157:
.space 132 
end_label157:
	b.w	{{jump_label157}}

.ltorg
.align	2
.space	3, 45
.space 19 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 31 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 8009 % {{space_mod|default("0x10000000")}}

.align	1
.space 20318 % {{space_mod|default("0x10000000")}}
label456:
.space 2  @ 2b  @ 2b
ldr	r1, =func_19  @ 2b  @ 2b  @ 2b
orr	r1, #1  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	blx	r1                            @ A7.7.19  @ 2b  @ 2b  @ 2b


.space 30   @ 30b  @ 30b
end_label456:
	b.w	{{jump_label456}}

.ltorg
.align	2
.space	0, 45
.space 1040 % {{space_mod|default("0x10000000")}}

.align	1
.space 11304 % {{space_mod|default("0x10000000")}}
label564:
	sub	r13, #208  @ 2b  @ 2b  @ 2b
.space 4  @ 4b  @ 4b
mov	r2, #63762  @ 4b  @ 4b  @ 4b
ldr	r5, cell_3577  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r6, cell_3575  @ 4b  @ 4b  @ 4b
ldr	r0, =table_110  @ 2b  @ 2b  @ 2b
mov	r8, #15  @ 4b  @ 4b  @ 4b
	itee	eq  @ 2b  @ 2b  @ 2b
	addeq	r0, r13, r5                   @ A7.7.6  @ 4b  @ 4b  @ 4b
movne	r7, #0  @ 2b  @ 2b  @ 2b
	tbbne	[r0, r7]                      @ A7.7.185  @ 4b  @ 4b  @ 4b
label564_switch_1_case_1:
.space 10   @ 10b  @ 10b  @ 10b
label564_switch_1_case_2:
.space 20   @ 20b  @ 20b
mov	r4, #20088  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b
	ldrb	r6, [r13, #-11]               @ A7.7.46  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
label564_switch_1_case_3:
.space 8   @ 8b  @ 8b
	ldrb	r4, [r13, #36]!               @ A7.7.46  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
	ldrb	r4, [r13, r8]                 @ A7.7.48  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b  @ 16b
	str	r2, [r13, #43]                @ A7.7.161  @ 4b  @ 4b  @ 4b
	add	r13, r13, #188                @ A7.7.5  @ 2b  @ 2b  @ 2b
ldr	r4, cell_3570  @ 4b  @ 4b  @ 4b
.space 48   @ 48b  @ 48b
end_label564:
	b.w	{{jump_label564}}

.ltorg
.align	2
.space	3, 45
.space 38 % {{space_mod|default("0x10000000")}}
.global	cell_3570
cell_3570:	.word	safeSpaceSram-63395

.space	1, 45
.global	cell_3577
cell_3577:	.word	safeSpaceSram+93

.space	0, 45
.global	cell_3575
cell_3575:	.word	safeSpaceSram+98

.align	1
.space 8213 % {{space_mod|default("0x10000000")}}



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
.space	3, 46
.space 155 % {{space_mod|default("0x10000000")}}
.global	table_110
table_110:
.byte	0
.byte	((label564_switch_1_case_2-label564_switch_1_case_1)/2)
.byte	((label564_switch_1_case_3-label564_switch_1_case_1)/2)

.space	0, 46
.space 76 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 471 % {{space_mod|default("0x10000000")}}



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