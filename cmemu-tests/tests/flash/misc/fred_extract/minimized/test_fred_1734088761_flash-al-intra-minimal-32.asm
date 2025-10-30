---
name: Fred-generated test
description: 'Test flow: (conf. 0) label296 -> label439 -> label382 -> label193 ->
  label526 -> label22 -> label273 -> label572 -> label561 -> label541 -> label291
  -> label37 -> label231 -> label115'
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
  jump_start: label296
  jump_label296: label439
  jump_label439: label382
  jump_label382: label193
  jump_label193: label526
  jump_label526: label22
  jump_label22: label273
  jump_label273: label572
  jump_label572: label561
  jump_label561: label541
  jump_label541: label291
  jump_label291: label37
  jump_label37: label231
  jump_label231: label115
  jump_label115: code_end
  code_end: code_end
  space_mod: 8
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
	mov.w	r0, #46902
	mov.w	r1, #27578
	mov.w	r2, #51194
	mov.w	r3, #27693
	mov.w	r4, #53753
	mov.w	r5, #36664
	mov.w	r6, #13802
	mov.w	r7, #11915
	mov.w	r8, #19062
	mov.w	r9, #43584
	mov.w	r10, #18252

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
.space 92 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 160 % {{space_mod|default("0x10000000")}}
label3:
	sub	r13, #40
	bl	forward_label_5               @ A7.7.18

	mov	r5, r8, LSL #3                @ A7.7.78
ldr	r7, cell_13
	ldr	r5, [r7, #206]                @ A7.7.43
ldr	r6, cell_12
	ldmdb	r6!, {r0-r5,r7-r10}           @ A7.7.42
	movw	r3, #22098                    @ A7.7.76
	smull	r0, r6, r6, r7                @ A7.7.149
mov	r6, #4
	ldrd	r8, r10, [r13, #-48]          @ A7.7.50
	mul	r8, r10                       @ A7.7.84
	mla	r10, r10, r4, r5              @ A7.7.74
	sdiv	r4, r5                        @ A7.7.127
ldr	r14, =post_branch_4
orr	r14, #1
	bvs	func_75                       @ A7.7.12
post_branch_4:


	strb	r8, [r13, r6, LSL #2]         @ A7.7.164

forward_label_5:
	adds	r13, #40                      @ A7.7.5
	tst	r4, #20                       @ A7.7.188
end_label3:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_12
cell_12:	.word	safeSpaceGpramSram+280

.space	1, 45
.global	cell_13
cell_13:	.word	safeSpaceFlash-21

.align	1
.space 408 % {{space_mod|default("0x10000000")}}
label8:
ldr	r7, cell_33
ldr	r2, cell_32
	strb	r4, [r7, #-108]!              @ A7.7.163
	stm	r13, {r0-r1,r3-r10}           @ A7.7.159
	cmn	r10, r6, LSL #2               @ A7.7.26
	cmn	r13, r1, LSL #3               @ A7.7.26
	ldrsb	r10, [r2, #-151]!             @ A7.7.59
ldr	r9, cell_31
	ldrd	r6, r4, [r9]                  @ A7.7.50
mov	r9, #26
	bic	r1, r6                        @ A7.7.16
	strh	r10, [r13, r9]                @ A7.7.171
	strd	r1, r6, [r13]                 @ A7.7.166
	bics	r10, r1, r4, LSL #3           @ A7.7.16
	mov	r3, r0, LSL #2                @ A7.7.78
	adcs	r6, r9, r9, LSL #2            @ A7.7.2
end_label8:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_33
cell_33:	.word	safeSpaceSram+307

.space	1, 45
.global	cell_32
cell_32:	.word	safeSpaceSram+285

.space	2, 45
.global	cell_31
cell_31:	.word	safeSpaceFlash+584

.align	1
label9:
ldr	r0, cell_41
ldr	r2, cell_40
mov	r3, #54404
ldr	r9, cell_39
ldr	r10, cell_38
	bne	forward_label_15              @ A7.7.12

	bics	r5, #96                       @ A7.7.15
ldr	r8, cell_37
	addw	r1, r13, #91                  @ A7.7.5
	str	r2, [r9], #-39                @ A7.7.161
	bic	r1, r3, r10                   @ A7.7.16
	ldmdb	r8, {r6,r9}                   @ A7.7.42
ldr	r1, cell_36
	ldrsh	r6, [r13, #45]                @ A7.7.63
	strh	r9, [r1, r3, LSL #3]          @ A7.7.171
	mls	r7, r9, r2, r6                @ A7.7.75
	ands	r3, r3, r10                   @ A7.7.9
	adds	r7, r4, #98                   @ A7.7.3
	tst	r3, r6, LSL #1                @ A7.7.189
	ands	r5, r10, r6                   @ A7.7.9
	mla	r6, r4, r10, r2               @ A7.7.74

forward_label_15:
	ldr	r3, cell_35                   @ A7.7.44
	ldrb	r7, [r13]                     @ A7.7.46
	ldrsh	r7, [r0, #-236]!              @ A7.7.63
	umlal	r0, r1, r0, r9                @ A7.7.203
ldr	r3, cell_34
	addw	r0, r6, #3872                 @ A7.7.3
	adc	r6, r6, #48                   @ A7.7.1
	mov	r5, r6                        @ A7.7.77
	ldrsb	r0, [r2], #219                @ A7.7.59
	stmdb	r3, {r3-r4,r6,r10}            @ A7.7.160
	strh	r6, [r10]                     @ A7.7.170
	and	r0, r10                       @ A7.7.9
end_label9:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_39
cell_39:	.word	safeSpaceSram+626

.space	0, 45
.global	cell_35
cell_35:	.word	0x0545ab90

.space	2, 45
.global	cell_41
cell_41:	.word	safeSpaceGpramSram+1180

.space	3, 45
.global	cell_40
cell_40:	.word	safeSpaceFlash+133

.space	3, 45
.global	cell_34
cell_34:	.word	safeSpaceGpramSram+268

.space	1, 45
.global	cell_36
cell_36:	.word	safeSpaceSram-434683

.space	3, 45
.global	cell_38
cell_38:	.word	safeSpaceGpramSram+656

.space	0, 45
.global	cell_37
cell_37:	.word	safeSpaceFlash+216

.align	1
.space 460 % {{space_mod|default("0x10000000")}}
func_1:
	sub	r13, #16
ldr	r0, =table_1
mov	r10, #0
	tbh	[r0, r10, LSL #1]             @ A7.7.185
func_1_switch_1_case_1:
	ldrb	r2, cell_66                   @ A7.7.47
	mls	r0, r7, r0, r7                @ A7.7.75
mov	r3, #44
	ldrsh	r1, [r13, r3]                 @ A7.7.65
ldr	r9, cell_65
	bfc	r3, #6, #7                    @ A7.7.13
ldr	r10, cell_64
	ldrsh	r2, cell_63                   @ A7.7.64
mov	r2, #3
	ldrb	r3, [r10, r2]                 @ A7.7.48
	ands	r10, r1, r7                   @ A7.7.9
	ldrh	r10, [r13, r2, LSL #2]        @ A7.7.57
	strh	r9, [r9, #26]                 @ A7.7.170
func_1_switch_1_case_2:
	add	r1, r13, r7, LSL #3           @ A7.7.6
func_1_switch_1_case_3:
	tst	r8, #248                      @ A7.7.188
	bic	r0, r7, r7                    @ A7.7.16
	cmn	r8, r2, LSL #3                @ A7.7.26
	bics	r10, r2, r8, LSL #3           @ A7.7.16
ldr	r1, cell_62
ldr	r9, cell_61
	bfi	r2, r2, #14, #2               @ A7.7.14
	adds	r2, r8, r4, LSL #3            @ A7.7.4
	bfc	r3, #3, #25                   @ A7.7.13
mov	r10, #60
	strh	r7, [r13, #41]                @ A7.7.170
	strh	r5, [r1]                      @ A7.7.170
	ldrsh	r3, cell_60                   @ A7.7.64
	ldm	r9, {r0-r2}                   @ A7.7.41
	ldm	r9!, {r0,r3}                  @ A7.7.41
	str	r4, [r13, r10]                @ A7.7.162
	pop	{r0-r1,r9-r10}                @ A7.7.99
	mul	r10, r4                       @ A7.7.84
end_func_1:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_66
cell_66:	.byte	0x0d

.space	0, 45
.global	cell_60
cell_60:	.short	0xed95

.space	1, 45
.global	cell_63
cell_63:	.short	0xb4fb

.space	1, 45
.global	cell_64
cell_64:	.word	safeSpaceFlash+925

.space	0, 45
.global	cell_65
cell_65:	.word	safeSpaceGpramSram+747

.space	0, 45
.global	cell_61
cell_61:	.word	safeSpaceSram+348

.space	2, 45
.global	cell_62
cell_62:	.word	safeSpaceSram+143

.align	1
.space 370 % {{space_mod|default("0x10000000")}}
label20:
	sub	r13, #88
ldr	r7, =forward_label_33
orr	r7, #1
ldr	r6, cell_83
mov	r10, #51433
ldr	r9, cell_82
	blx	r7                            @ A7.7.19
ldr	r14, =post_branch_23
orr	r14, #1

	stmdb	r9, {r0-r1,r4-r6,r10}         @ A7.7.160
	bic	r0, r3                        @ A7.7.16
mov	r0, #23
	ldr	r7, [r13, r0]                 @ A7.7.45
	ldrsh	r1, cell_81                   @ A7.7.64
	ldmdb	r13, {r0-r5,r7-r9}            @ A7.7.42
	adc	r1, r7, r8, LSL #1            @ A7.7.2
	mls	r3, r6, r7, r4                @ A7.7.75
	ldrh	r1, [r6, r10, LSL #1]         @ A7.7.57
	bhi	func_24                       @ A7.7.12
post_branch_23:


	bic	r4, #178                      @ A7.7.15
ldr	r10, cell_80
	ldrd	r3, r4, [r10, #-160]!         @ A7.7.50

forward_label_33:
	tst	r4, #120                      @ A7.7.188
	ldrh	r10, [r13, #-2]               @ A7.7.55
	nop	                              @ A7.7.88
	add	r8, #187                      @ A7.7.1
	umull	r10, r0, r7, r1               @ A7.7.204
	smlal	r4, r6, r3, r7                @ A7.7.138
ldr	r4, cell_79
	stm	r4!, {r0-r1,r3,r5-r9}         @ A7.7.159
	mov	r7, r2, LSL #3                @ A7.7.78
	strd	r7, r0, [r13, #88]!           @ A7.7.166
	cbz	r1, forward_label_32          @ A7.7.21

	mrs	r8, apsr                      @ A7.7.82
	bics	r3, r6, r6, LSL #1            @ A7.7.16

forward_label_32:
	mls	r8, r6, r3, r4                @ A7.7.75
end_label20:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_82
cell_82:	.word	safeSpaceGpramSram+472

.space	0, 45
.global	cell_79
cell_79:	.word	safeSpaceSram+424

.space	0, 45
.global	cell_81
cell_81:	.short	0x48a0

.space	2, 45
.global	cell_83
cell_83:	.word	safeSpaceFlash-102685

.space	0, 45
.global	cell_80
cell_80:	.word	safeSpaceFlash+1072

.align	1
.space 170 % {{space_mod|default("0x10000000")}}
label22:
mov	r5, #1  @ 4b
ldr	r10, cell_94  @ 4b
ldr	r9, =table_2  @ 4b
mov	r6, #2  @ 4b
ldr	r1, cell_93  @ 2b
.space 4
label22_switch_1_case_1:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
label22_switch_1_case_2:
.space 4
label22_switch_1_case_3:
.space 4
.space 4
.space 4
label22_switch_1_case_4:
.space 4
.space 2
end_label22:
	b.w	{{jump_label22}}

.ltorg
.align	2
.space	0, 46
.global	table_2
table_2:
.hword	0
.hword	((label22_switch_1_case_2-label22_switch_1_case_1)/2)
.hword	((label22_switch_1_case_3-label22_switch_1_case_1)/2)
.hword	((label22_switch_1_case_4-label22_switch_1_case_1)/2)

.space	2, 45
.global	cell_94
cell_94:	.word	safeSpaceSram+945

.space	2, 45
.global	cell_93
cell_93:	.word	safeSpaceSram+958

.space	0, 45
.global	cell_92
cell_92:	.short	0xed8e

.align	1
.space 1392 % {{space_mod|default("0x10000000")}}
label35:
	sub	r13, #168
ldr	r8, =forward_label_59
orr	r8, #1
	tst	r10, r10                      @ A7.7.189
	pop	{r3,r7}                       @ A7.7.99
	strd	r7, r10, [r13, #160]!         @ A7.7.166
	ldmdb	r13, {r0-r1,r4,r9}            @ A7.7.42
	bx	r8                            @ A7.7.20

	strd	r10, r3, [r13]                @ A7.7.166

forward_label_59:
end_label35:
	b.w	{{code_end}}

.ltorg
.align	1
.space 110 % {{space_mod|default("0x10000000")}}
label37:
mov	r8, #32933  @ 4b
ldr	r2, cell_160  @ 4b
mov	r7, #9311  @ 4b
	cbnz	r7, forward_label_63          @ A7.7.21  @ 2b

	mls	r6, r5, r0, r7                @ A7.7.75  @ 4b

forward_label_63:
	bic	r3, r8, r0                    @ A7.7.16  @ 4b
	ldrb	r5, cell_159                  @ A7.7.47  @ 4b
ldr	r5, cell_158  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
ldr	r8, cell_156  @ 4b
	bic	r4, r6                        @ A7.7.16  @ 4b
	nop	                              @ A7.7.88  @ 2b
	ldm	r8!, {r1-r3}                  @ A7.7.41  @ 4b
end_label37:
	b.w	{{jump_label37}}

.ltorg
.align	2
.space	3, 45
.global	cell_156
cell_156:	.word	safeSpaceFlash+600

.space	0, 45
.global	cell_158
cell_158:	.word	safeSpaceFlash-32661

.space	3, 45
.global	cell_159
cell_159:	.byte	0x39

.space	2, 45
.global	cell_157
cell_157:	.byte	0xa0

.space	1, 45
.global	cell_160
cell_160:	.word	safeSpaceGpramSram-36327

.align	1
.space 808 % {{space_mod|default("0x10000000")}}
label45:
ldr	r9, =table_4
mov	r4, #17
mov	r3, #33372
ldr	r6, cell_193
ldr	r14, =post_branch_53
ldr	r7, cell_192
ldr	r5, cell_191
mov	r10, #1
ldr	r1, =func_35
orr	r1, #1
orr	r14, #1
mov	r2, #47
	tbb	[r9, r10]                     @ A7.7.185
label45_switch_1_case_1:
	strd	r5, r2, [r7], #140            @ A7.7.166
	cmn	r13, #186                     @ A7.7.25
	strb	r4, [r13, r4]                 @ A7.7.164
	asrs	r9, r2, #30                   @ A7.7.10
	ldr	r10, [r5, #2817]              @ A7.7.43
	mls	r5, r6, r5, r4                @ A7.7.75
	ldrh	r0, cell_190                  @ A7.7.56
label45_switch_1_case_2:
ldr	r5, cell_189
	.align	2
	ldrd	r7, r9, cell_188              @ A7.7.51
	strb	r5, [r13, r2]                 @ A7.7.164
	mov	r9, r13                       @ A7.7.77
ldr	r9, cell_187
	ldrd	r8, r4, [r9, #-120]           @ A7.7.50
	cmn	r3, #157                      @ A7.7.25
	ldrb	r2, [r5, #250]                @ A7.7.46
label45_switch_1_case_3:
	str	r5, [r6, r3]                  @ A7.7.162
	bic	r0, r10, #235                 @ A7.7.15
	bx	r1                            @ A7.7.20
post_branch_53:


	umull	r0, r2, r1, r5                @ A7.7.204
mov	r9, #30
	ldr	r8, [r13, r9]                 @ A7.7.45
	adcs	r4, r9, r3                    @ A7.7.2
end_label45:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_190
cell_190:	.short	0xdde1

.space	3, 45
.global	cell_192
cell_192:	.word	safeSpaceGpramSram+484

.space	1, 45
.global	cell_193
cell_193:	.word	safeSpaceSram-33094

.align	2
.global	cell_188
cell_188:	.quad	0x62a9c5d538e09db0

.space	1, 45
.global	cell_191
cell_191:	.word	safeSpaceSram-1878

.space	0, 45
.global	cell_189
cell_189:	.word	safeSpaceFlash+218

.space	0, 45
.global	cell_187
cell_187:	.word	safeSpaceSram+704

.align	1
.space 980 % {{space_mod|default("0x10000000")}}
label52:
ldr	r1, cell_237
mov	r2, #2586
ldr	r4, cell_236
ldr	r7, cell_235
	add	r9, r9, #127                  @ A7.7.3
	cmn	r3, #1                        @ A7.7.25
	add	r9, r9                        @ A7.7.4
	mov	r3, r7, LSL #2                @ A7.7.78
	str	r7, [r7, r2]                  @ A7.7.162
	sdiv	r3, r3                        @ A7.7.127
	smull	r2, r5, r1, r2                @ A7.7.149
mov	r9, #25545
	ldr	r10, [r1, r9]                 @ A7.7.45
	ldrsh	r3, [r4], #119                @ A7.7.63
	umull	r2, r1, r1, r1                @ A7.7.204
	ldrsh	r0, cell_234                  @ A7.7.64
end_label52:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_235
cell_235:	.word	safeSpaceGpramSram-2057

.space	3, 45
.global	cell_234
cell_234:	.short	0xe950

.space	0, 45
.global	cell_236
cell_236:	.word	safeSpaceGpramSram+761

.space	0, 45
.global	cell_237
cell_237:	.word	safeSpaceSram-24992

.align	1
.space 556 % {{space_mod|default("0x10000000")}}
label59:
ldr	r5, cell_263
mov	r9, #62
	ldrsh	r6, [r5], #-249               @ A7.7.63
	ldrh	r6, [r13, r9]                 @ A7.7.57
	mla	r1, r10, r3, r7               @ A7.7.74
	ldr	r6, [r13, #33]                @ A7.7.43
	mrs	r8, apsr                      @ A7.7.82
	it	ne
	andsne	r5, r4, r8, LSL #2            @ A7.7.9
ldr	r3, cell_262
	movs	r8, r1                        @ A7.7.77
	add	r9, r15                       @ A7.7.4
	bics	r10, r6, #230                 @ A7.7.15
	mls	r2, r9, r4, r9                @ A7.7.75
	ldrsb	r2, [r3]                      @ A7.7.59
ldr	r3, cell_261
mov	r9, #6
	strb	r2, [r13, r9, LSL #2]         @ A7.7.164
mov	r1, #3795
	ldrh	r6, [r3, r1, LSL #3]          @ A7.7.57
	add	r9, r15                       @ A7.7.4
	add	r9, r9, r2                    @ A7.7.4
	mul	r10, r8                       @ A7.7.84
end_label59:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_263
cell_263:	.word	safeSpaceFlash+817

.space	3, 45
.global	cell_261
cell_261:	.word	safeSpaceGpramSram-29840

.space	3, 45
.global	cell_262
cell_262:	.word	safeSpaceFlash+179

.align	1
.space 556 % {{space_mod|default("0x10000000")}}
label66:
	sub	r13, #36
ldr	r7, cell_286
	ldrd	r10, r5, [r7, #-200]!         @ A7.7.50
	mov	r8, #174                      @ A7.7.76
	bfc	r8, #11, #9                   @ A7.7.13
	.align	2
	ldrd	r9, r5, cell_285              @ A7.7.51
mov	r5, #30
	pop	{r0-r2,r4,r6-r10}             @ A7.7.99
	ldrb	r7, cell_284                  @ A7.7.47
	strh	r1, [r13, r5]                 @ A7.7.171
	smlal	r0, r3, r10, r10              @ A7.7.138
	mul	r0, r0                        @ A7.7.84
end_label66:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_284
cell_284:	.byte	0x70

.align	2
.global	cell_285
cell_285:	.quad	0x17e82e9d17dc8b1e

.space	1, 45
.global	cell_286
cell_286:	.word	safeSpaceSram+688

.align	1
label67:
ldr	r4, cell_290
ldr	r9, cell_289
ldr	r8, =forward_label_110
orr	r8, #1
mov	r5, #5
ldr	r7, cell_288
	bx	r8                            @ A7.7.20

	adds	r0, r4, r8, LSL #1            @ A7.7.4
	cmn	r13, r8                       @ A7.7.26
	ldrb	r0, [r9], #41                 @ A7.7.46

forward_label_110:
	stm	r4!, {r0-r3,r5,r8,r10}        @ A7.7.159
	stmdb	r7, {r0-r9}                   @ A7.7.160
	.align	2
	ldrd	r9, r8, cell_287              @ A7.7.51
	strh	r8, [r13, r5]                 @ A7.7.171
	cmp	r2, #181                      @ A7.7.27
	mls	r7, r9, r9, r7                @ A7.7.75
	isb	                              @ A7.7.37
	str	r1, [r13, r5, LSL #1]         @ A7.7.162
	bfi	r8, r0, #10, #22              @ A7.7.14
	adc	r0, r2, r5                    @ A7.7.2
end_label67:
	b.w	{{code_end}}

.ltorg
.align	2
.global	cell_287
cell_287:	.quad	0x43139d8e36fae8d3

.space	2, 45
.global	cell_288
cell_288:	.word	safeSpaceSram+176

.space	3, 45
.global	cell_289
cell_289:	.word	safeSpaceGpramSram+385

.space	0, 45
.global	cell_290
cell_290:	.word	safeSpaceGpramSram+328

.align	1
.space 196 % {{space_mod|default("0x10000000")}}
label70:
	sub	r13, #24
ldr	r7, =forward_label_116
orr	r7, #1
mov	r10, #22140
ldr	r0, cell_302
ldr	r3, cell_301
	blx	r7                            @ A7.7.19

	adds	r9, r9, r10                   @ A7.7.4
	addw	r8, r13, #1991                @ A7.7.5
ldr	r8, cell_300
	ldrd	r4, r5, [r8], #-104           @ A7.7.50
	smull	r9, r1, r9, r1                @ A7.7.149
	add	r7, r14, r3, LSL #2           @ A7.7.4
	mov	r1, r3, LSL #3                @ A7.7.78
	strb	r1, [r13, #44]                @ A7.7.163
	nop	                              @ A7.7.88
	smlal	r4, r2, r7, r0                @ A7.7.138
	mla	r2, r0, r2, r10               @ A7.7.74
	ldrsb	r6, cell_299                  @ A7.7.60
	smlal	r5, r6, r0, r5                @ A7.7.138
	bics	r2, #246                      @ A7.7.15
	cmn	r14, #146                     @ A7.7.25

forward_label_116:
	ldrb	r4, cell_298                  @ A7.7.47
	ldrsh	r5, [r0, #755]                @ A7.7.63
	pop	{r0-r1,r5-r8}                 @ A7.7.99
	ldrb	r2, cell_297                  @ A7.7.47
	ldrsh	r1, [r3, r10]                 @ A7.7.65
	ldrh	r10, cell_296                 @ A7.7.56
	cmn	r10, r3                       @ A7.7.26
end_label70:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_302
cell_302:	.word	safeSpaceGpramSram-317

.space	1, 45
.global	cell_296
cell_296:	.short	0xd42d

.space	2, 45
.global	cell_301
cell_301:	.word	safeSpaceFlash-21772

.space	2, 45
.global	cell_297
cell_297:	.byte	0xa2

.space	1, 45
.global	cell_299
cell_299:	.byte	0x56

.space	1, 45
.global	cell_300
cell_300:	.word	safeSpaceGpramSram+920

.space	2, 45
.global	cell_298
cell_298:	.byte	0xd2

.align	1
.space 304 % {{space_mod|default("0x10000000")}}
label73:
ldr	r8, =table_7
ldr	r4, cell_316
mov	r1, #2
	tbh	[r8, r1, LSL #1]              @ A7.7.185
label73_switch_1_case_1:
	ldr	r1, [r13]                     @ A7.7.43
ldr	r1, cell_315
	asrs	r7, r2, #19                   @ A7.7.10
	strd	r8, r1, [r1, #180]            @ A7.7.166
label73_switch_1_case_2:
	adcs	r5, r8, r1                    @ A7.7.2
	bics	r7, r1, #152                  @ A7.7.15
	tst	r4, r1, LSL #2                @ A7.7.189
	addw	r2, r0, #691                  @ A7.7.3
	add	r9, r7                        @ A7.7.4
label73_switch_1_case_3:
	bl	func_16                       @ A7.7.18


	smlal	r5, r3, r8, r5                @ A7.7.138
	and	r10, r4, #107                 @ A7.7.8
	stm	r4!, {r0-r3,r6-r10}           @ A7.7.159
label73_switch_1_case_4:
end_label73:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 46
.global	table_7
table_7:
.hword	0
.hword	((label73_switch_1_case_2-label73_switch_1_case_1)/2)
.hword	((label73_switch_1_case_3-label73_switch_1_case_1)/2)
.hword	((label73_switch_1_case_4-label73_switch_1_case_1)/2)

.space	0, 45
.global	cell_316
cell_316:	.word	safeSpaceGpramSram+616

.space	3, 45
.global	cell_315
cell_315:	.word	safeSpaceSram+172

.align	1
.space 798 % {{space_mod|default("0x10000000")}}
func_4:
.space 2
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
mov	r0, #1  @ 4b
.space 4
.space 4
mov	r0, #8  @ 4b
.space 4
ldr	r10, cell_391  @ 4b
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r3, #29909  @ 4b
ldr	r1, cell_389  @ 4b
.space 2
.space 4
ldr	r9, =forward_label_134  @ 4b
ldr	r10, cell_387  @ 4b
orr	r9, #1  @ 4b
.space 4
.space 4
ldr	r0, cell_386  @ 2b
.space 4
.space 2
.space 4
.space 2
.space 4
.space 2

.space 4
ldr	r0, cell_345  @ 4b
.space 4
.space 4

forward_label_134:
mov	r0, #6815  @ 4b
.space 4
.space 4
ldr	r3, cell_343  @ 4b
.space 4
mov	r3, #26400  @ 4b
.space 4
.space 4
.space 4
.space 4
end_func_4:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_386
cell_386:	.word	safeSpaceGpramSram-119220

.space	2, 45
.global	cell_387
cell_387:	.word	safeSpaceGpramSram+604

.space	0, 45
.global	cell_343
cell_343:	.word	safeSpaceGpramSram-12734

.space	2, 45
.space 15 % {{space_mod|default("0x10000000")}}
.global	cell_342
cell_342:	.byte	0x70

.space	2, 45
.global	cell_392
cell_392:	.word	0x80c687ca

.space	1, 45
.space 29 % {{space_mod|default("0x10000000")}}
.global	cell_390
cell_390:	.word	0xe317ac79

.align	2
.space 22 % {{space_mod|default("0x10000000")}}
.global	cell_391
cell_391:	.word	safeSpaceSram+618

.align	2
.space 61 % {{space_mod|default("0x10000000")}}
.global	cell_389
cell_389:	.word	safeSpaceSram-25748

.space	3, 45
.space 40 % {{space_mod|default("0x10000000")}}
.global	cell_388
cell_388:	.quad	0x1198744fb7ceabae

.space	2, 45
.space 31 % {{space_mod|default("0x10000000")}}
.global	cell_344
cell_344:	.word	0x5176216e

.space	1, 45
.space 24 % {{space_mod|default("0x10000000")}}
.global	cell_345
cell_345:	.word	safeSpaceFlash+365

.align	2
.space 20 % {{space_mod|default("0x10000000")}}
label82:
	sub	r13, #48
ldr	r0, =forward_label_136
ldr	r2, cell_396
orr	r0, #1
mov	r7, #30
	bx	r0                            @ A7.7.20

	ldr	r10, [r13, r7]                @ A7.7.45

forward_label_136:
	strh	r2, [r13]                     @ A7.7.170
mov	r0, #52097
ldr	r7, cell_395
	cmp	r14, r2, LSL #2               @ A7.7.28
	ldrb	r4, cell_394                  @ A7.7.47
	ldrsh	r8, [r7, #-62]!               @ A7.7.63
	push	{r1,r5}                       @ A7.7.101
	ldrb	r4, cell_393                  @ A7.7.47
	cmn	r10, #184                     @ A7.7.25
	strb	r2, [r2, r0]                  @ A7.7.164
	pop	{r3,r5,r7}                    @ A7.7.99
	umlal	r3, r4, r0, r7                @ A7.7.203
	smlal	r10, r4, r3, r0               @ A7.7.138
	pop	{r0-r10}                      @ A7.7.99
	mrs	r5, apsr                      @ A7.7.82
	add	r3, r15                       @ A7.7.4
	ands	r4, r4, #221                  @ A7.7.8
	mrs	r4, apsr                      @ A7.7.82
	cmn	r9, r4, LSL #2                @ A7.7.26
	asrs	r7, r4, #5                    @ A7.7.10
end_label82:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_395
cell_395:	.word	safeSpaceSram+429

.space	0, 45
.global	cell_394
cell_394:	.byte	0x5f

.space	0, 45
.global	cell_393
cell_393:	.byte	0x5a

.space	1, 45
.global	cell_396
cell_396:	.word	safeSpaceGpramSram-51734

.align	1
.space 184 % {{space_mod|default("0x10000000")}}
label85:
ldr	r7, cell_410
ldr	r9, cell_409
	cbz	r3, forward_label_141         @ A7.7.21

	ldrsh	r8, [r13, #-44]               @ A7.7.63
	str	r9, [r13, #6]                 @ A7.7.161
	umlal	r8, r2, r1, r0                @ A7.7.203
ldr	r1, cell_408
	ldrsh	r3, cell_407                  @ A7.7.64
	ldrsh	r0, cell_406                  @ A7.7.64
	smull	r4, r0, r7, r4                @ A7.7.149
	ldrsb	r0, [r1]                      @ A7.7.59
	smlal	r10, r6, r3, r9               @ A7.7.138
	ldrb	r0, [r7, #145]!               @ A7.7.46
	ldm	r9, {r0-r1,r3-r8,r10}         @ A7.7.41
	adds	r3, #172                      @ A7.7.1
	cmn	r0, #0                        @ A7.7.25

forward_label_141:
	tst	r7, r4, LSL #1                @ A7.7.189
	cmp	r6, #222                      @ A7.7.27
	stm	r13, {r0,r6-r7,r9-r10}        @ A7.7.159
	movt	r7, #7081                     @ A7.7.79
	isb	                              @ A7.7.37
	ldrsb	r8, cell_405                  @ A7.7.60
end_label85:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_407
cell_407:	.short	0x8282

.space	2, 45
.global	cell_409
cell_409:	.word	safeSpaceFlash+872

.space	1, 45
.global	cell_410
cell_410:	.word	safeSpaceSram+618

.space	2, 45
.global	cell_406
cell_406:	.short	0x8b55

.space	3, 45
.global	cell_405
cell_405:	.byte	0x91

.space	0, 45
.global	cell_408
cell_408:	.word	safeSpaceGpramSram+334

.align	1
.space 164 % {{space_mod|default("0x10000000")}}
label87:
mov	r3, #43
ldr	r8, cell_422
ldr	r9, cell_421
mov	r1, #52597
	bfi	r5, r9, #5, #27               @ A7.7.14
	ldrsb	r4, cell_420                  @ A7.7.60
	bl	forward_label_146             @ A7.7.18

	tst	r8, #156                      @ A7.7.188
	ldr	r7, [r9, #-120]               @ A7.7.43
	cmn	r1, r4                        @ A7.7.26
	asr	r5, r9, r9                    @ A7.7.11
	mov	r4, r15                       @ A7.7.77
	cmn	r4, r7                        @ A7.7.26
	ldr	r7, [r8, r1]                  @ A7.7.45
	asr	r4, r4, #32                   @ A7.7.10
	and	r4, r7, #23                   @ A7.7.8
	addw	r1, r2, #994                  @ A7.7.3
	ldrsh	r0, cell_419                  @ A7.7.64
	add	r1, r7, #185                  @ A7.7.3

forward_label_146:
	str	r2, [r13, r3]                 @ A7.7.162
	asr	r1, r3, r3                    @ A7.7.11
end_label87:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_421
cell_421:	.word	safeSpaceSram+406

.space	0, 45
.global	cell_419
cell_419:	.short	0x5a47

.space	1, 45
.global	cell_420
cell_420:	.byte	0x09

.space	3, 45
.global	cell_422
cell_422:	.word	safeSpaceSram-52013

.align	1
.space 570 % {{space_mod|default("0x10000000")}}
label90:
mov	r4, #1
ldr	r8, cell_473
mov	r6, #38
ldr	r2, =func_38
ldr	r9, =table_9
ldr	r0, cell_472
	tbb	[r9, r4]                      @ A7.7.185
label90_switch_1_case_1:
	ldrh	r10, [r8, #-227]              @ A7.7.55
	.align	2
	ldrd	r3, r5, cell_471              @ A7.7.51
	isb	                              @ A7.7.37
	nop	                              @ A7.7.88
mov	r3, #5
	mrs	r9, apsr                      @ A7.7.82
	ldrsb	r1, cell_470                  @ A7.7.60
	umlal	r9, r5, r9, r7                @ A7.7.203
	movs	r7, r3                        @ A7.7.77
	ldrb	r5, [r13, r3, LSL #1]         @ A7.7.48
	cmp	r7, #59                       @ A7.7.27
	cmp	r7, #94                       @ A7.7.27
label90_switch_1_case_2:
	ldrsb	r5, cell_469                  @ A7.7.60
label90_switch_1_case_3:
	ldr	r1, [r8], #-127               @ A7.7.43
orr	r2, #1
	cmn	r14, #211                     @ A7.7.25
	ldrd	r1, r3, [r0]                  @ A7.7.50
	asrs	r5, r7, #22                   @ A7.7.10
	blx	r2                            @ A7.7.19


	ldrsh	r5, cell_468                  @ A7.7.64
	ldr	r3, [r13, r6]                 @ A7.7.45
	mls	r0, r5, r6, r0                @ A7.7.75
	mls	r7, r8, r7, r5                @ A7.7.75
	movs	r3, #14                       @ A7.7.76
	ldrb	r1, cell_467                  @ A7.7.47
end_label90:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 46
.global	table_9
table_9:
.byte	0
.byte	((label90_switch_1_case_2-label90_switch_1_case_1)/2)
.byte	((label90_switch_1_case_3-label90_switch_1_case_1)/2)

.space	1, 45
.global	cell_470
cell_470:	.byte	0xb2

.space	3, 45
.global	cell_473
cell_473:	.word	safeSpaceSram+766

.align	2
.global	cell_471
cell_471:	.quad	0x6ccb268072da87fa

.space	2, 45
.global	cell_468
cell_468:	.short	0x7b0b

.space	3, 45
.global	cell_472
cell_472:	.word	safeSpaceSram+760

.space	0, 45
.global	cell_469
cell_469:	.byte	0x76

.space	2, 45
.global	cell_467
cell_467:	.byte	0x3a

.align	1
func_6:
	sub	r13, #56  @ 2b
ldr	r9, cell_484  @ 4b
mov	r10, #2  @ 4b
ldr	r3, cell_483  @ 2b
ldr	r0, cell_482  @ 4b
ldr	r2, cell_481  @ 4b
ldr	r1, =table_10  @ 2b
.space 4
func_6_switch_1_case_1:
.space 2
mov	r10, #60  @ 4b
.space 4
.space 2
	ldrh	r1, [r13, r10]                @ A7.7.57  @ 4b
	str	r9, [r13, #53]                @ A7.7.161  @ 4b
.space 4
.space 4
.space 4
mov	r10, #38673  @ 4b
.space 4
.space 4
.space 4
func_6_switch_1_case_2:
.space 4
.space 4
.space 4
.space 2
func_6_switch_1_case_3:
	msr	apsr_nzcvq, r4                @ A7.7.83  @ 4b
	str	r0, [r2]                      @ A7.7.161  @ 2b
ldr	r0, cell_478  @ 4b
	ldrd	r2, r3, [r13]                 @ A7.7.50  @ 4b
	pop	{r2-r3}                       @ A7.7.99  @ 2b
	ldrsh	r1, [r0]                      @ A7.7.63  @ 4b
	movt	r2, #7908                     @ A7.7.79  @ 4b
	cmn	r13, #237                     @ A7.7.25  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r3, cell_476  @ 4b
	adds	r2, r13, r7, LSL #3           @ A7.7.6  @ 4b
.space 4
.space 4
	stm	r13, {r0-r8}                  @ A7.7.159  @ 4b
mov	r0, #60  @ 4b
ldr	r2, cell_475  @ 4b
.space 4
.space 4
.space 4
	push	{r1,r3-r6,r8-r10}             @ A7.7.101  @ 4b
.space 4
.space 4
	strh	r6, [r13], #72                @ A7.7.170  @ 4b
	ldrb	r1, [r13, r0]                 @ A7.7.48  @ 4b
	adds	r0, r13, r5                   @ A7.7.6  @ 4b
.space 4
	pop	{r1,r3}                       @ A7.7.99  @ 2b
ldr	r0, cell_474  @ 2b
.space 2
.space 4
	ldrh	r3, [r13]                     @ A7.7.55  @ 4b
end_func_6:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_484
cell_484:	.word	safeSpaceGpramSram+448

.space	0, 45
.global	cell_479
cell_479:	.short	0xc9fb

.space	0, 45
.global	cell_474
cell_474:	.word	safeSpaceGpramSram+720

.space	0, 45
.global	cell_483
cell_483:	.word	safeSpaceSram-77077

.space	0, 45
.global	cell_480
cell_480:	.byte	0x8d

.space	2, 45
.global	cell_481
cell_481:	.word	safeSpaceGpramSram+596

.space	0, 45
.global	cell_475
cell_475:	.word	safeSpaceSram+804

.space	3, 45
.global	cell_476
cell_476:	.word	safeSpaceFlash-65

.space	1, 45
.global	cell_478
cell_478:	.word	safeSpaceGpramSram+427

.space	0, 45
.global	cell_482
cell_482:	.word	safeSpaceSram+124

.space	2, 45
.global	cell_477
cell_477:	.byte	0x41

.align	1
.space 3244 % {{space_mod|default("0x10000000")}}
label113:
	sub	r13, #100
ldr	r4, cell_667
ldr	r0, cell_666
ldr	r6, cell_665
ldr	r3, cell_664
ldr	r5, =table_15
mov	r7, #3
mov	r1, #5776
mov	r2, #17087
ldr	r8, cell_663
	tbb	[r5, r7]                      @ A7.7.185
label113_switch_1_case_1:
	str	r5, [r4, r2, LSL #2]          @ A7.7.162
mov	r5, #22512
	udiv	r2, r3                        @ A7.7.195
	cmp	r3, #169                      @ A7.7.27
	strb	r7, [r8, r5]                  @ A7.7.164
	adds	r4, r6, #3                    @ A7.7.3
	ldrb	r7, cell_662                  @ A7.7.47
	bfc	r8, #2, #30                   @ A7.7.13
label113_switch_1_case_2:
	ldrb	r10, cell_661                 @ A7.7.47
	asrs	r7, r7, #17                   @ A7.7.10
	tst	r3, r3, LSL #2                @ A7.7.189
label113_switch_1_case_3:
	.align	2
	ldrd	r9, r2, cell_660              @ A7.7.51
	ldrsh	r4, cell_659                  @ A7.7.64
	ldrd	r9, r8, [r0, #-64]            @ A7.7.50
ldr	r8, cell_658
	ldm	r8!, {r2,r5,r7,r10}           @ A7.7.41
label113_switch_1_case_4:
	udiv	r0, r1, r5                    @ A7.7.195
	nop	                              @ A7.7.88
	smlal	r5, r2, r2, r1                @ A7.7.138
	ldrb	r8, [r6, r1, LSL #2]          @ A7.7.48
	and	r5, r1, r3, LSL #2            @ A7.7.9
	ldrd	r7, r10, [r13, #84]!          @ A7.7.50
	smlal	r2, r5, r7, r1                @ A7.7.138
	ldrsh	r7, cell_657                  @ A7.7.64
	adc	r2, r5, #140                  @ A7.7.1
	ldrd	r1, r4, [r3, #200]!           @ A7.7.50
	pop	{r2-r4,r8}                    @ A7.7.99
	ldrh	r2, cell_656                  @ A7.7.56
	cmp	r3, r1                        @ A7.7.28
end_label113:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_663
cell_663:	.word	safeSpaceGpramSram-21936

.space	2, 45
.global	cell_661
cell_661:	.byte	0xd7

.space	1, 45
.global	cell_666
cell_666:	.word	safeSpaceFlash+344

.space	0, 45
.global	cell_665
cell_665:	.word	safeSpaceGpramSram-22269

.space	3, 45
.global	cell_658
cell_658:	.word	safeSpaceSram+312

.space	0, 45
.global	cell_657
cell_657:	.short	0x4b36

.space	2, 45
.global	cell_667
cell_667:	.word	safeSpaceGpramSram-68030

.space	3, 45
.global	cell_656
cell_656:	.short	0xd446

.space	2, 45
.global	cell_662
cell_662:	.byte	0x59

.align	2
.global	cell_660
cell_660:	.quad	0x59f9e76963d08f2b

.space	1, 45
.global	cell_659
cell_659:	.short	0x6c79

.space	0, 45
.global	cell_664
cell_664:	.word	safeSpaceGpramSram+132

.align	1
.space 80 % {{space_mod|default("0x10000000")}}
label115:
.space 2
ldr	r9, =forward_label_205  @ 4b
orr	r9, #1  @ 4b
ldr	r10, cell_674  @ 4b
	bx	r9                            @ A7.7.20  @ 2b

	tst	r2, r4                        @ A7.7.189  @ 2b
	adc	r1, r5                        @ A7.7.2  @ 4b
	lsrs	r2, r3, r2                    @ A7.7.71  @ 4b
	tst	r5, r4                        @ A7.7.189  @ 2b
ldr	r3, cell_673  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
ldr	r3, cell_672  @ 4b
.space 4
.space 4
.space 2
.space 4

forward_label_205:
ldr	r5, =forward_label_204  @ 2b
.space 2
ldr	r1, cell_671  @ 4b
.space 4
mov	r4, #12558  @ 4b
orr	r5, #1  @ 4b
	it	ge  @ 2b
	bxge	r5                            @ A7.7.20 @ 2b


.space 2

forward_label_204:
	str	r3, [r10, #44]                @ A7.7.161  @ 4b
	nop	                              @ A7.7.88  @ 2b
	adds	r10, #60                      @ A7.7.1  @ 4b
.space 4
	cmn	r3, r10, LSL #3               @ A7.7.26  @ 4b
	movt	r7, #6157                     @ A7.7.79  @ 4b
	ldrb	r6, cell_670                  @ A7.7.47  @ 4b
end_label115:
	b.w	{{jump_label115}}

.ltorg
.align	2
.space	2, 45
.global	cell_670
cell_670:	.byte	0x5a

.space	2, 45
.global	cell_673
cell_673:	.word	safeSpaceGpramSram+248

.space	2, 45
.global	cell_674
cell_674:	.word	safeSpaceGpramSram+348

.space	2, 45
.global	cell_672
cell_672:	.word	safeSpaceSram+579

.space	2, 45
.global	cell_671
cell_671:	.word	safeSpaceSram-11833

.align	1
.space 354 % {{space_mod|default("0x10000000")}}
label118:
mov	r9, #8
ldr	r1, =forward_label_212
	ldrb	r4, [r13]                     @ A7.7.46
orr	r1, #1
	bx	r1                            @ A7.7.20

	addw	r7, r13, #3376                @ A7.7.5
	udiv	r5, r0                        @ A7.7.195

forward_label_212:
	itttt	ls
	ldrsbls	r10, cell_691                 @ A7.7.60
	tstls	r1, #155                      @ A7.7.188
movls	r1, #0
	ldrbls	r0, [r13, r1, LSL #3]         @ A7.7.48
	bfi	r1, r8, #3, #29               @ A7.7.14
	ldrh	r6, [r13, r9]                 @ A7.7.57
	ldrsh	r8, [r13]                     @ A7.7.63
mov	r4, #5
	sdiv	r1, r8                        @ A7.7.127
	.align	2
	ldrd	r5, r6, cell_690              @ A7.7.51
	str	r1, [r13, r4, LSL #1]         @ A7.7.162
	strh	r9, [r13, #-58]               @ A7.7.170
	sdiv	r2, r4, r4                    @ A7.7.127
	ldrh	r4, [r13, #-59]               @ A7.7.55
	cmn	r14, #2                       @ A7.7.25
	ldrb	r8, cell_689                  @ A7.7.47
end_label118:
	b.w	{{code_end}}

.ltorg
.align	2
.global	cell_690
cell_690:	.quad	0x302361bfdaa956e5

.space	2, 45
.global	cell_689
cell_689:	.byte	0x15

.space	0, 45
.global	cell_691
cell_691:	.byte	0x92

.align	1
.space 552 % {{space_mod|default("0x10000000")}}
label122:
	sub	r13, #120
ldr	r1, cell_728
mov	r6, #35
ldr	r10, cell_727
	ldrb	r8, cell_726                  @ A7.7.47
ldr	r7, cell_725
	ldrh	r3, [r13, r6]                 @ A7.7.57
	ldrb	r6, [r13], #80                @ A7.7.46
	smlal	r0, r2, r0, r5                @ A7.7.138
	strh	r1, [r7], #-230               @ A7.7.170
	ldm	r1!, {r0,r2-r8}               @ A7.7.41
	strd	r3, r7, [r13, #40]!           @ A7.7.166
	ldrsh	r7, [r10]                     @ A7.7.63
	adcs	r6, r2, #132                  @ A7.7.1
	lsrs	r10, r6, r2                   @ A7.7.71
	adc	r2, r2                        @ A7.7.2
	ldrh	r2, cell_724                  @ A7.7.56
end_label122:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_726
cell_726:	.byte	0x2e

.space	3, 45
.global	cell_727
cell_727:	.word	safeSpaceGpramSram+547

.space	1, 45
.global	cell_724
cell_724:	.short	0x1b60

.space	2, 45
.global	cell_725
cell_725:	.word	safeSpaceGpramSram+853

.space	0, 45
.global	cell_728
cell_728:	.word	safeSpaceGpramSram+540

.align	1
.space 3488 % {{space_mod|default("0x10000000")}}
label148:
mov	r9, #51
ldr	r3, cell_895
mov	r7, #6
ldr	r8, cell_894
ldr	r5, cell_893
	ldr	r2, [r13, r7, LSL #3]         @ A7.7.45
	ldrsb	r0, [r8], #177                @ A7.7.59
	strb	r5, [r5, #-210]               @ A7.7.163
	ldrsh	r5, cell_892                  @ A7.7.64
	ldrsh	r8, [r13, r9]                 @ A7.7.65
	ldrb	r5, cell_891                  @ A7.7.47
	ldr	r5, cell_890                  @ A7.7.44
	ldm	r3, {r1,r3-r6,r9}             @ A7.7.41
	ldmdb	r13, {r0-r2,r7}               @ A7.7.42
	.align	2
	ldrd	r9, r3, cell_889              @ A7.7.51
	nop	                              @ A7.7.88
ldr	r3, cell_888
	ldrh	r5, [r3, #-231]!              @ A7.7.55
end_label148:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_895
cell_895:	.word	safeSpaceGpramSram+516

.space	1, 45
.global	cell_890
cell_890:	.word	0x6014200c

.space	0, 45
.global	cell_892
cell_892:	.short	0x94ab

.align	2
.global	cell_889
cell_889:	.quad	0x52f79b3e22d606d8

.space	2, 45
.global	cell_894
cell_894:	.word	safeSpaceSram+306

.space	2, 45
.global	cell_891
cell_891:	.byte	0x24

.space	3, 45
.global	cell_888
cell_888:	.word	safeSpaceSram+634

.space	0, 45
.global	cell_893
cell_893:	.word	safeSpaceSram+1034

.align	1
.space 290 % {{space_mod|default("0x10000000")}}
label152:
ldr	r7, =forward_label_272
orr	r7, #1
	bx	r7                            @ A7.7.20

	bics	r0, r7, #64                   @ A7.7.15
	ldrsb	r7, [r13]                     @ A7.7.59
	smlal	r10, r4, r0, r5               @ A7.7.138
	udiv	r3, r0                        @ A7.7.195
mov	r7, #18
	ldr	r5, [r13, r7]                 @ A7.7.45
	ldrsh	r0, cell_903                  @ A7.7.64
	addw	r0, r13, #45                  @ A7.7.5

forward_label_272:
end_label152:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_903
cell_903:	.short	0x9054

.align	1
label153:
	sub	r13, #8
ldr	r0, cell_908
mov	r5, #56528
	str	r5, [r13], #-164              @ A7.7.161
mov	r10, #20
	ldrsb	r8, cell_907                  @ A7.7.60
	strb	r5, [r13, #84]!               @ A7.7.163
	mla	r6, r6, r8, r1                @ A7.7.74
	ldrsh	r9, [r13], #84                @ A7.7.63
	push	{r5,r7,r9-r10}                @ A7.7.101
	strb	r9, [r0, r5]                  @ A7.7.164
mov	r7, #8
	add	r13, r13, r10                 @ A7.7.6
	strb	r2, [r13, r7]                 @ A7.7.164
ldr	r9, cell_906
	strd	r10, r1, [r9]                 @ A7.7.166
	cbz	r0, forward_label_274         @ A7.7.21

	ldrsb	r0, cell_905                  @ A7.7.60
	bfi	r2, r7, #3, #27               @ A7.7.14
ldr	r9, cell_904
	umlal	r0, r10, r5, r9               @ A7.7.203
	sdiv	r6, r5, r2                    @ A7.7.127
	ldmdb	r9!, {r0,r2-r8,r10}           @ A7.7.42
	str	r6, [r13, #-48]               @ A7.7.161
	movs	r2, r7                        @ A7.7.77
	movt	r3, #53979                    @ A7.7.79
	sdiv	r6, r2, r7                    @ A7.7.127
	umlal	r5, r0, r5, r0                @ A7.7.203
	nop	                              @ A7.7.88
	smlal	r6, r1, r5, r5                @ A7.7.138

forward_label_274:
end_label153:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_908
cell_908:	.word	safeSpaceSram-56015

.space	2, 45
.global	cell_905
cell_905:	.byte	0xc5

.space	3, 45
.global	cell_906
cell_906:	.word	safeSpaceSram+456

.space	0, 45
.global	cell_904
cell_904:	.word	safeSpaceSram+368

.space	1, 45
.global	cell_907
cell_907:	.byte	0x35

.align	1
.space 628 % {{space_mod|default("0x10000000")}}
func_13:
	sub	r13, #160
ldr	r10, cell_940
	ldrd	r9, r3, [r13]                 @ A7.7.50
	strb	r7, [r10]                     @ A7.7.163
	add	r2, #234                      @ A7.7.1
	stm	r13!, {r1-r4,r7-r9}           @ A7.7.159
	movs	r2, #211                      @ A7.7.76
	bvc	forward_label_289             @ A7.7.12


forward_label_289:
ldr	r9, cell_939
	ldrd	r2, r10, [r9]                 @ A7.7.50
	add	r0, r5, #179                  @ A7.7.3
	bfc	r9, #4, #28                   @ A7.7.13
ldr	r0, cell_938
	adc	r2, r0, #226                  @ A7.7.1
	cmp	r4, r0                        @ A7.7.28
	lsr	r2, r0                        @ A7.7.71
ldr	r9, cell_937
	adds	r1, r13, #190                 @ A7.7.5
	mul	r10, r3, r5                   @ A7.7.84
	ldrsh	r10, cell_936                 @ A7.7.64
ldr	r3, cell_935
ldr	r1, cell_934
	ands	r10, r3, #134                 @ A7.7.8
	ldrh	r2, cell_933                  @ A7.7.56
	ldrb	r10, [r1, #-252]!             @ A7.7.46
	ldrd	r2, r10, [r13]                @ A7.7.50
	ldrsb	r2, [r9]                      @ A7.7.59
	ldrsh	r10, [r0, #3728]              @ A7.7.63
ldr	r2, cell_932
mov	r9, #2
	tst	r9, #251                      @ A7.7.188
ldr	r0, =forward_label_288
	asrs	r10, r0, #14                  @ A7.7.10
	ldrb	r1, [r3, #125]!               @ A7.7.46
orr	r0, #1
	ands	r3, #225                      @ A7.7.8
	cmp	r0, r13                       @ A7.7.28
	smull	r3, r10, r0, r2               @ A7.7.149
	bx	r0                            @ A7.7.20

	sdiv	r1, r2, r0                    @ A7.7.127
	ldr	r10, cell_931                 @ A7.7.44
	asrs	r0, r7, r7                    @ A7.7.11
	ldrh	r10, cell_930                 @ A7.7.56

forward_label_288:
	ldrb	r10, [r13, r9, LSL #1]        @ A7.7.48
	stmdb	r13, {r0,r3,r6-r9}            @ A7.7.160
	ldrb	r1, [r2], #81                 @ A7.7.46
	add	r10, r15                      @ A7.7.4
	ldrsh	r0, [r13], #132               @ A7.7.63
end_func_13:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_931
cell_931:	.word	0x62b8691e

.space	2, 45
.global	cell_933
cell_933:	.short	0xaee1

.space	1, 45
.global	cell_932
cell_932:	.word	safeSpaceSram+281

.space	0, 45
.global	cell_937
cell_937:	.word	safeSpaceSram+786

.space	1, 45
.global	cell_940
cell_940:	.word	safeSpaceSram+219

.space	1, 45
.global	cell_936
cell_936:	.short	0xb447

.space	3, 45
.global	cell_935
cell_935:	.word	safeSpaceGpramSram+348

.space	1, 45
.global	cell_939
cell_939:	.word	safeSpaceGpramSram+68

.space	1, 45
.global	cell_938
cell_938:	.word	safeSpaceFlash-3151

.space	2, 45
.global	cell_934
cell_934:	.word	safeSpaceGpramSram+1064

.space	1, 45
.global	cell_930
cell_930:	.short	0x2f64

.align	1
.space 920 % {{space_mod|default("0x10000000")}}
label164:
	sub	r13, #88
ldr	r7, cell_985
ldr	r0, cell_984
mov	r8, #35566
	bl	forward_label_301             @ A7.7.18

	strb	r3, [r0, r8, LSL #1]          @ A7.7.164
	cmp	r0, r4                        @ A7.7.28
	.align	2
	ldrd	r10, r1, cell_983             @ A7.7.51
	asrs	r0, r3, #4                    @ A7.7.10
	and	r10, r2                       @ A7.7.9
	adds	r8, r13, r1                   @ A7.7.6
	cmp	r3, #4                        @ A7.7.27
	ands	r8, #45                       @ A7.7.8
	adr	r6, cell_982                  @ A7.7.7

forward_label_301:
	mla	r6, r8, r5, r4                @ A7.7.74
	str	r6, [r13, #-10]               @ A7.7.161
mov	r0, #21755
	cmn	r3, r7                        @ A7.7.26
	strb	r6, [r7, r0]                  @ A7.7.164
ldr	r5, cell_981
	pop	{r0-r4,r6-r10}                @ A7.7.99
	cmp	r0, #7                        @ A7.7.27
ldr	r3, cell_980
	bic	r1, r6, #72                   @ A7.7.15
	it	mi
	strhmi	r6, [r3, #-242]!              @ A7.7.170
	bics	r10, r10, r8                  @ A7.7.16
	ldm	r5, {r1,r3,r7-r8,r10}         @ A7.7.41
	mrs	r5, apsr                      @ A7.7.82
ldr	r10, cell_979
	movt	r6, #62179                    @ A7.7.79
	pop	{r0-r8}                       @ A7.7.99
	pop	{r1,r4,r7}                    @ A7.7.99
	ldrb	r3, [r10], #-174              @ A7.7.46
end_label164:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_979
cell_979:	.word	safeSpaceFlash+253

.space	1, 45
.global	cell_982
cell_982:	.byte	0x66

.space	2, 45
.global	cell_984
cell_984:	.word	safeSpaceGpramSram-70334

.align	2
.global	cell_983
cell_983:	.quad	0x5ff0dd4d3321eb5f

.space	3, 45
.global	cell_985
cell_985:	.word	safeSpaceGpramSram-21534

.space	2, 45
.global	cell_980
cell_980:	.word	safeSpaceGpramSram+687

.space	2, 45
.global	cell_981
cell_981:	.word	safeSpaceSram+884

.align	1
.space 776 % {{space_mod|default("0x10000000")}}
func_16:
	sub	r13, #244
mov	r0, #56
ldr	r9, cell_1047
ldr	r1, cell_1046
mov	r10, #144
ldr	r2, cell_1045
	cbnz	r2, forward_label_313         @ A7.7.21

	ldrsb	r3, [r1, #41]                 @ A7.7.59
	ldr	r3, [r13, r0]                 @ A7.7.45
	cmp	r7, #167                      @ A7.7.27
	stmdb	r2, {r0-r1,r8,r10}            @ A7.7.160
ldr	r3, cell_1044
	strd	r0, r7, [r2]                  @ A7.7.166
	cmp	r14, #149                     @ A7.7.27
	strh	r5, [r3]                      @ A7.7.170
	strh	r7, [r2]                      @ A7.7.170
	ands	r3, r2, #12                   @ A7.7.8
	adr	r3, cell_1043                 @ A7.7.7

forward_label_313:
	sdiv	r3, r1, r7                    @ A7.7.127
ldr	r3, cell_1042
	msr	apsr_nzcvq, r5                @ A7.7.83
	stm	r2, {r0-r10}                  @ A7.7.159
	str	r8, [r3, r10, LSL #2]         @ A7.7.162
	cmp	r6, #95                       @ A7.7.27
	bfc	r3, #1, #27                   @ A7.7.13
	strb	r4, [r13, r0]                 @ A7.7.164
mov	r3, #8
	ldrb	r0, cell_1041                 @ A7.7.47
	ands	r0, r3, #42                   @ A7.7.8
	bic	r0, r0, r9                    @ A7.7.16
	cmp	r7, #92                       @ A7.7.27
ldr	r0, cell_1040
	strb	r9, [r13, r3, LSL #1]         @ A7.7.164
	isb	                              @ A7.7.37
	tst	r2, r9, LSL #3                @ A7.7.189
	adds	r13, r10                      @ A7.7.6
	umull	r10, r3, r2, r2               @ A7.7.204
	mov	r3, r13                       @ A7.7.77
	ldr	r3, cell_1039                 @ A7.7.44
	adds	r10, r6, r7                   @ A7.7.4
	ldrh	r10, [r13]                    @ A7.7.55
	cmp	r3, r1                        @ A7.7.28
	nop	                              @ A7.7.88
mov	r3, #60620
	itett	pl
	ldrshpl	r1, [r0, #3017]               @ A7.7.63
	ldrdmi	r10, r9, [r1]                 @ A7.7.50
	ldrbpl	r1, [r9, r3]                  @ A7.7.48
	addpl	r1, r1, #157                  @ A7.7.3
	ldrb	r3, cell_1017                 @ A7.7.47
	stmdb	r2!, {r0,r3,r5-r7,r9-r10}     @ A7.7.160
	ldrsh	r3, [r13, #100]!              @ A7.7.63
	ldrb	r3, [r13]                     @ A7.7.46
	umlal	r2, r0, r3, r3                @ A7.7.203
	lsr	r2, r2                        @ A7.7.71
end_func_16:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 17 % {{space_mod|default("0x10000000")}}
.global	cell_1039
cell_1039:	.word	0xb036e38e

.space	1, 45
.global	cell_1045
cell_1045:	.word	safeSpaceSram+884

.space	3, 45
.space 14 % {{space_mod|default("0x10000000")}}
.global	cell_1043
cell_1043:	.byte	0x5b

.space	2, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_1042
cell_1042:	.word	safeSpaceSram+264

.space	1, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_1017
cell_1017:	.byte	0xe2

.space	3, 45
.space 14 % {{space_mod|default("0x10000000")}}
.global	cell_1044
cell_1044:	.word	safeSpaceSram+935

.space	3, 45
.space 10 % {{space_mod|default("0x10000000")}}
.global	cell_1047
cell_1047:	.word	safeSpaceGpramSram-59687

.space	2, 45
.space 19 % {{space_mod|default("0x10000000")}}
.global	cell_1046
cell_1046:	.word	safeSpaceFlash+656

.space	3, 45
.global	cell_1040
cell_1040:	.word	safeSpaceFlash-2525

.space	2, 45
.global	cell_1041
cell_1041:	.byte	0x85

.align	1
.space 2936 % {{space_mod|default("0x10000000")}}
func_20:
	sub	r13, #252  @ 2b
ldr	r3, =table_25  @ 2b
mov	r2, #58170  @ 4b
mov	r9, #8  @ 4b
mov	r0, #0  @ 4b
mov	r1, #5  @ 4b
	tbh	[r3, r0, LSL #1]              @ A7.7.185  @ 4b
func_20_switch_2_case_1:
	ldrsb	r0, [r13, r9]                 @ A7.7.61  @ 4b
ldr	r3, cell_1170  @ 4b
	mov	r9, r13                       @ A7.7.77  @ 2b
.space 4
func_20_switch_2_case_2:
ldr	r0, cell_1169  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 4
func_20_switch_2_case_3:
.space 4
func_20_switch_2_case_4:
	ldr	r9, [r13, #37]                @ A7.7.43  @ 4b
	ldrsb	r3, [r13, r1, LSL #3]         @ A7.7.61  @ 4b
ldr	r0, cell_1168  @ 4b
.space 4
.space 4
	stm	r13, {r0-r10}                 @ A7.7.159  @ 4b
.space 4
func_20_switch_2_case_5:
	umlal	r0, r1, r3, r0                @ A7.7.203  @ 4b
mov	r1, #12427  @ 4b
	asrs	r3, r9, #22                   @ A7.7.10  @ 4b
	mla	r3, r0, r7, r2                @ A7.7.74  @ 4b
	itett	pl  @ 2b
	lsrspl	r0, r1, r0                    @ A7.7.71 @ 4b
	movwmi	r9, #28551                    @ A7.7.76 @ 4b
	cmnpl	r5, r0, LSL #2                @ A7.7.26 @ 4b
	strbpl	r3, [r13]                     @ A7.7.163 @ 4b




ldr	r3, cell_1164  @ 2b
	mul	r9, r5, r6                    @ A7.7.84  @ 4b
	umull	r9, r0, r7, r6                @ A7.7.204  @ 4b
	adds	r10, r13, #56                 @ A7.7.5  @ 4b
ldr	r9, cell_1163  @ 4b
	nop	                              @ A7.7.88  @ 2b
	tst	r2, r0                        @ A7.7.189  @ 2b
	bfc	r0, #27, #1                   @ A7.7.13  @ 4b
ldr	r0, cell_1162  @ 4b
	bic	r10, #46                      @ A7.7.15  @ 4b
	ldrb	r10, cell_1161                @ A7.7.47  @ 4b
	ldmdb	r0, {r0,r10}                  @ A7.7.42  @ 4b
	bic	r0, #61                       @ A7.7.15  @ 4b
	msr	apsr_nzcvq, r1                @ A7.7.83  @ 4b
	mov	r10, r15                      @ A7.7.77  @ 2b
	str	r1, [r3, r2, LSL #3]          @ A7.7.162  @ 4b
	strh	r3, [r9, r1]                  @ A7.7.171  @ 4b
	ldrb	r0, cell_1160                 @ A7.7.47  @ 4b
	ldrh	r9, [r13], #252               @ A7.7.55  @ 4b
	mls	r3, r3, r2, r5                @ A7.7.75  @ 4b
	ldr	r3, cell_1159                 @ A7.7.44  @ 2b
	ldrh	r9, [r13]                     @ A7.7.55  @ 4b
	bfi	r9, r3, #9, #14               @ A7.7.14  @ 4b
	stmdb	r13, {r1,r3,r7-r10}           @ A7.7.160  @ 4b
end_func_20:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_1169
cell_1169:	.word	safeSpaceGpramSram+78

.space	3, 45
.global	cell_1162
cell_1162:	.word	safeSpaceGpramSram+564

.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1168
cell_1168:	.word	safeSpaceGpramSram+128

.space	1, 45
.global	cell_1164
cell_1164:	.word	safeSpaceSram-464760

.space	2, 45
.global	cell_1160
cell_1160:	.byte	0x2a

.space	1, 45
.global	cell_1159
cell_1159:	.word	0x54de51e7

.space	0, 45
.global	cell_1161
cell_1161:	.byte	0xce

.space	2, 45
.space 9 % {{space_mod|default("0x10000000")}}
.global	cell_1163
cell_1163:	.word	safeSpaceSram-11998

.space	1, 45
.global	cell_1170
cell_1170:	.word	safeSpaceSram+412

.align	1
.space 942 % {{space_mod|default("0x10000000")}}
label193:
	sub	r13, #44  @ 2b
.space 4
.space 4
	strd	r5, r3, [r13]                 @ A7.7.166  @ 4b
mov	r2, #1  @ 4b
	str	r2, [r13]                     @ A7.7.161  @ 2b
	strh	r9, [r13, r2, LSL #2]         @ A7.7.171  @ 4b
ldr	r10, cell_1210  @ 4b
.space 4
	ldr	r3, [r13], #8                 @ A7.7.43  @ 4b
.space 2
	pop	{r0-r3,r5-r7,r9-r10}          @ A7.7.99  @ 4b
ldr	r7, cell_1209  @ 2b
.space 4
ldr	r1, =forward_label_361  @ 2b
.space 4
.space 4
.space 4
orr	r1, #1  @ 4b
.space 4
.space 2

.space 4
.space 4

forward_label_361:
.space 4
.space 4
.space 4
end_label193:
	b.w	{{jump_label193}}

.ltorg
.align	2
.space	0, 45
.global	cell_1209
cell_1209:	.word	safeSpaceGpramSram+625

.space	0, 45
.global	cell_1208
cell_1208:	.short	0xf714

.space	0, 45
.global	cell_1210
cell_1210:	.word	safeSpaceGpramSram+320

.align	1
.space 3374 % {{space_mod|default("0x10000000")}}
label219:
mov	r4, #54551
ldr	r1, cell_1352
	ldrb	r5, [r1, r4, LSL #1]          @ A7.7.48
ldr	r1, cell_1351
	strb	r5, [r1, #-196]!              @ A7.7.163
ldr	r4, cell_1350
	cmn	r10, #40                      @ A7.7.25
mov	r8, #65255
	add	r2, r2, r4                    @ A7.7.4
	ldrh	r10, [r4, r8]                 @ A7.7.57
	strh	r10, [r13, #14]               @ A7.7.170
	stm	r13, {r0,r3-r6,r8-r10}        @ A7.7.159
	mla	r0, r0, r0, r10               @ A7.7.74
end_label219:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_1351
cell_1351:	.word	safeSpaceSram+292

.space	0, 45
.global	cell_1352
cell_1352:	.word	safeSpaceGpramSram-108580

.space	3, 45
.global	cell_1350
cell_1350:	.word	safeSpaceSram-64696

.align	1
.space 150 % {{space_mod|default("0x10000000")}}
label221:
	sub	r13, #84
ldr	r3, cell_1361
ldr	r5, =table_32
mov	r2, #3
mov	r10, #14
mov	r7, #4
	tbh	[r5, r2, LSL #1]              @ A7.7.185
label221_switch_1_case_1:
	strb	r2, [r3, r7]                  @ A7.7.164
label221_switch_1_case_2:
	ldmdb	r13, {r0-r5,r8}               @ A7.7.42
	ldrh	r0, [r13, r10]                @ A7.7.57
	strb	r8, [r13, #-33]               @ A7.7.163
	bfc	r3, #12, #19                  @ A7.7.13
	str	r9, [r13, #42]                @ A7.7.161
	cmp	r3, r1, LSL #3                @ A7.7.28
	ands	r9, #22                       @ A7.7.8
label221_switch_1_case_3:
	mla	r6, r8, r8, r1                @ A7.7.74
	bics	r9, r8, r3, LSL #2            @ A7.7.16
	ldr	r2, [r13, r7, LSL #2]         @ A7.7.45
label221_switch_1_case_4:
	sdiv	r1, r0, r0                    @ A7.7.127
ldr	r3, cell_1360
	isb	                              @ A7.7.37
	ldrb	r8, cell_1359                 @ A7.7.47
	ldrb	r1, cell_1358                 @ A7.7.47
mov	r10, #6801
	strh	r0, [r3, r10]                 @ A7.7.171
label221_switch_1_case_5:
	and	r1, #78                       @ A7.7.8
	smull	r8, r6, r3, r10               @ A7.7.149
	push	{r4,r7,r10}                   @ A7.7.101
	strd	r5, r8, [r13, #20]!           @ A7.7.166
	mov	r3, r5, LSL #1                @ A7.7.78
	ldr	r1, [r13], #8                 @ A7.7.43
	pop	{r2-r3,r6-r8,r10}             @ A7.7.99
	adds	r7, r13, r5                   @ A7.7.6
	pop	{r0-r10}                      @ A7.7.99
	bics	r2, #143                      @ A7.7.15
	and	r1, r9                        @ A7.7.9
end_label221:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_1359
cell_1359:	.byte	0x03

.space	1, 45
.global	cell_1361
cell_1361:	.word	safeSpaceGpramSram+552

.space	1, 45
.global	cell_1358
cell_1358:	.byte	0x07

.space	1, 45
.global	cell_1360
cell_1360:	.word	safeSpaceSram-6322

.align	1
.space 822 % {{space_mod|default("0x10000000")}}
label229:
ldr	r7, =forward_label_416
orr	r7, #1
ldr	r2, cell_1398
	blx	r7                            @ A7.7.19

	mla	r0, r4, r5, r3                @ A7.7.74
	bfi	r9, r10, #10, #15             @ A7.7.14
	ands	r1, r3, r5                    @ A7.7.9
	movt	r1, #52655                    @ A7.7.79
ldr	r7, cell_1397
	ldrd	r4, r1, [r7]                  @ A7.7.50
	.align	2
	ldrd	r9, r4, cell_1396             @ A7.7.51
	cmn	r10, r2, LSL #2               @ A7.7.26
	umlal	r1, r4, r8, r1                @ A7.7.203
	asr	r7, r5, #10                   @ A7.7.10
	adds	r4, r13, #249                 @ A7.7.5
	smull	r1, r7, r5, r1                @ A7.7.149
	mov	r6, r13                       @ A7.7.77
	mrs	r4, apsr                      @ A7.7.82
	nop	                              @ A7.7.88

forward_label_416:
ldr	r5, cell_1395
	ldr	r6, [r13]                     @ A7.7.43
	msr	apsr_nzcvq, r1                @ A7.7.83
	ldr	r3, [r2]                      @ A7.7.43
	ldrsh	r6, [r5], #250                @ A7.7.63
	bl	func_52                       @ A7.7.18


end_label229:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_1395
cell_1395:	.word	safeSpaceGpramSram+807

.space	2, 45
.global	cell_1397
cell_1397:	.word	safeSpaceGpramSram+776

.space	3, 45
.global	cell_1398
cell_1398:	.word	safeSpaceFlash+529

.align	2
.global	cell_1396
cell_1396:	.quad	0x22aa511114888df5

.align	1
.space 54 % {{space_mod|default("0x10000000")}}
label231:
.space 2
mov	r2, #15  @ 4b
ldr	r9, =forward_label_419  @ 4b
orr	r9, #1  @ 4b
	cmp	r6, r9, LSL #3                @ A7.7.28  @ 4b
	blx	r9                            @ A7.7.19  @ 2b

	cmp	r2, r0, LSL #2                @ A7.7.28  @ 4b
	ldr	r0, cell_1406                 @ A7.7.44  @ 4b
	mrs	r5, apsr                      @ A7.7.82  @ 4b
	mla	r3, r8, r8, r9                @ A7.7.74  @ 4b
	asr	r6, r5                        @ A7.7.11  @ 4b
	ldrsh	r5, cell_1405                 @ A7.7.64  @ 4b
	bfi	r6, r9, #7, #21               @ A7.7.14  @ 4b

forward_label_419:
mov	r9, #17984  @ 4b
	adcs	r0, r6, #241                  @ A7.7.1  @ 4b
	bfi	r3, r3, #13, #14              @ A7.7.14  @ 4b
ldr	r6, cell_1404  @ 4b
	adr	r10, cell_1403                @ A7.7.7  @ 4b
.space 4
	umlal	r10, r3, r0, r0               @ A7.7.203  @ 4b
	umull	r1, r2, r3, r2                @ A7.7.204  @ 4b
	movw	r10, #36681                   @ A7.7.76  @ 4b
.space 4
	cmn	r6, r9, LSL #2                @ A7.7.26  @ 4b
.space 4
	str	r6, [r6, r9]                  @ A7.7.162  @ 4b
	bfi	r9, r6, #11, #15              @ A7.7.14  @ 4b
ldr	r10, cell_1402  @ 4b
	str	r9, [r10], #115               @ A7.7.161  @ 4b
.space 4
end_label231:
	b.w	{{jump_label231}}

.ltorg
.align	2
.space	3, 45
.global	cell_1405
cell_1405:	.short	0xd048

.space	0, 45
.global	cell_1402
cell_1402:	.word	safeSpaceSram+454

.space	2, 45
.global	cell_1406
cell_1406:	.word	0xc1743f8f

.space	3, 45
.global	cell_1403
cell_1403:	.byte	0x99

.space	2, 45
.global	cell_1404
cell_1404:	.word	safeSpaceSram-17593

.align	1
.space 1890 % {{space_mod|default("0x10000000")}}
func_24:
mov	r2, #6
ldr	r10, cell_1487
	ldm	r10!, {r0-r1,r3,r9}           @ A7.7.41
	ldrsh	r3, cell_1486                 @ A7.7.64
	ldrsh	r10, [r13, r2, LSL #1]        @ A7.7.65
	sdiv	r1, r7, r3                    @ A7.7.127
	bic	r0, #99                       @ A7.7.15
mov	r2, #49661
	add	r1, r13, #43                  @ A7.7.5
	mul	r9, r3                        @ A7.7.84
mov	r3, #58
	mul	r9, r6, r5                    @ A7.7.84
	ldrb	r10, [r13, #34]               @ A7.7.46
	add	r1, r13, #25                  @ A7.7.5
	ldrsb	r9, cell_1485                 @ A7.7.60
	mrs	r9, apsr                      @ A7.7.82
	adr	r10, cell_1484                @ A7.7.7
	strb	r2, [r13]                     @ A7.7.163
	cmn	r13, #30                      @ A7.7.25
ldr	r9, cell_1483
	ldrsh	r10, [r9, #205]               @ A7.7.63
ldr	r10, cell_1482
ldr	r1, cell_1481
	ldrsh	r9, [r1, r2, LSL #1]          @ A7.7.65
	strb	r9, [r13, r3]                 @ A7.7.164
	ldrsb	r1, [r10, #-161]              @ A7.7.59
ldr	r10, cell_1480
	ldrh	r9, cell_1479                 @ A7.7.56
ldr	r9, cell_1478
	strb	r6, [r10]                     @ A7.7.163
	movw	r0, #14398                    @ A7.7.76
	and	r2, r5                        @ A7.7.9
	tst	r2, r0, LSL #2                @ A7.7.189
	strh	r5, [r9]                      @ A7.7.170
	cmp	r4, r6, LSL #3                @ A7.7.28
	adcs	r1, r2, r6, LSL #1            @ A7.7.2
	sdiv	r2, r10                       @ A7.7.127
	lsrs	r9, r9, r10                   @ A7.7.71
mov	r10, #3
	ldrh	r2, [r13, r10, LSL #1]        @ A7.7.57
end_func_24:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1484
cell_1484:	.byte	0x09

.space	1, 45
.global	cell_1481
cell_1481:	.word	safeSpaceSram-99103

.space	3, 45
.global	cell_1482
cell_1482:	.word	safeSpaceGpramSram+260

.space	0, 45
.global	cell_1480
cell_1480:	.word	safeSpaceGpramSram+661

.space	2, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1483
cell_1483:	.word	safeSpaceSram+94

.space	0, 45
.global	cell_1478
cell_1478:	.word	safeSpaceSram+484

.space	3, 45
.global	cell_1487
cell_1487:	.word	safeSpaceSram+468

.space	0, 45
.global	cell_1479
cell_1479:	.short	0x9af6

.space	3, 45
.global	cell_1486
cell_1486:	.short	0x3ad3

.space	2, 45
.global	cell_1485
cell_1485:	.byte	0x30

.align	1
func_25:
	sub	r13, #204
mov	r1, #4
mov	r0, #56215
ldr	r9, cell_1496
ldr	r3, cell_1495
ldr	r2, cell_1494
	it	ls
	asrsls	r10, r6, #18                  @ A7.7.10
	isb	                              @ A7.7.37
	ldrsb	r10, [r9]                     @ A7.7.59
mov	r9, #4
	bics	r10, r1, r6                   @ A7.7.16
	tst	r9, r9, LSL #2                @ A7.7.189
	ldrb	r10, [r13, #204]!             @ A7.7.46
	tst	r1, #134                      @ A7.7.188
	strd	r3, r5, [r3, #-200]           @ A7.7.166
	push	{r0,r8-r9}                    @ A7.7.101
	str	r5, [r13, r1, LSL #3]         @ A7.7.162
	movw	r3, #35929                    @ A7.7.76
	pop	{r1,r3,r10}                   @ A7.7.99
	bvc	forward_label_447             @ A7.7.12

	lsr	r1, r8                        @ A7.7.71
	ldrb	r1, [r2, r0, LSL #3]          @ A7.7.48
	mov	r0, r5, LSL #3                @ A7.7.78
	adc	r3, r10, r10, LSL #3          @ A7.7.2
	sdiv	r10, r8                       @ A7.7.127
	msr	apsr_nzcvq, r9                @ A7.7.83
	strb	r10, [r13, r9, LSL #3]        @ A7.7.164

forward_label_447:
	add	r2, r2, r5, LSL #1            @ A7.7.4
	umlal	r0, r2, r5, r2                @ A7.7.203
	ldrb	r2, cell_1493                 @ A7.7.47
	ldr	r10, cell_1492                @ A7.7.44
ldr	r0, cell_1491
	cmp	r1, r2, LSL #1                @ A7.7.28
mov	r3, #24922
	mov	r10, r15                      @ A7.7.77
	adcs	r9, r6, #36                   @ A7.7.1
	str	r6, [r0, r3]                  @ A7.7.162
	asr	r9, r6                        @ A7.7.11
	movs	r10, #13                      @ A7.7.76
	lsr	r0, r5                        @ A7.7.71
mov	r2, #3
	bics	r3, r2, r10                   @ A7.7.16
	cmn	r6, r7                        @ A7.7.26
	ldrsh	r9, [r13, r2, LSL #3]         @ A7.7.65
ldr	r2, cell_1490
	sdiv	r3, r7, r1                    @ A7.7.127
mov	r3, #18993
	ldm	r13, {r0-r1,r9-r10}           @ A7.7.41
	addw	r10, r5, #455                 @ A7.7.3
	ldrsh	r9, [r2, r3]                  @ A7.7.65
	tst	r2, #10                       @ A7.7.188
ldr	r10, cell_1489
ldr	r1, cell_1488
	strd	r3, r6, [r10, #236]!          @ A7.7.166
	umlal	r0, r2, r7, r3                @ A7.7.203
	ldrsb	r0, [r1, #73]                 @ A7.7.59
	sdiv	r2, r7, r2                    @ A7.7.127
end_func_25:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_1492
cell_1492:	.word	0x7e455445

.space	1, 45
.global	cell_1491
cell_1491:	.word	safeSpaceSram-24693

.space	0, 45
.global	cell_1495
cell_1495:	.word	safeSpaceSram+992

.space	0, 45
.global	cell_1496
cell_1496:	.word	safeSpaceGpramSram+715

.space	1, 45
.global	cell_1493
cell_1493:	.byte	0x70

.space	0, 45
.global	cell_1488
cell_1488:	.word	safeSpaceGpramSram+235

.space	1, 45
.global	cell_1494
cell_1494:	.word	safeSpaceSram-448908

.space	0, 45
.global	cell_1489
cell_1489:	.word	safeSpaceSram+140

.space	1, 45
.global	cell_1490
cell_1490:	.word	safeSpaceFlash-18714

.align	1
.space 220 % {{space_mod|default("0x10000000")}}
label252:
	sub	r13, #128
ldr	r2, cell_1509
ldr	r4, =forward_label_453
orr	r4, #1
ldr	r6, cell_1508
ldr	r7, cell_1507
	blx	r4                            @ A7.7.19

mov	r5, #33922
	.align	2
	ldrd	r10, r1, cell_1506            @ A7.7.51
	ldrsb	r0, [r2, #76]!                @ A7.7.59
	add	r10, r13, #253                @ A7.7.5
	ldrsb	r10, [r6, r5, LSL #3]         @ A7.7.61
mov	r10, #10915
	stm	r13, {r0-r2,r5,r7}            @ A7.7.159
	ldrsh	r5, cell_1505                 @ A7.7.64
	strb	r5, [r7, r10]                 @ A7.7.164
	mov	r5, r5, LSL #1                @ A7.7.78
mov	r2, #23
	ldr	r7, [r13, r2]                 @ A7.7.45
ldr	r9, cell_1504
	movw	r8, #17546                    @ A7.7.76
	ldm	r9!, {r0-r8,r10}              @ A7.7.41
	udiv	r4, r10, r1                   @ A7.7.195

forward_label_453:
	add	r5, r15                       @ A7.7.4
	pop	{r2,r5}                       @ A7.7.99
	smlal	r5, r1, r1, r5                @ A7.7.138
	add	r13, r13, #120                @ A7.7.5
	ands	r10, r2, r1                   @ A7.7.9
end_label252:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_1505
cell_1505:	.short	0x4afa

.space	0, 45
.global	cell_1508
cell_1508:	.word	safeSpaceFlash-270710

.space	1, 45
.global	cell_1507
cell_1507:	.word	safeSpaceSram-10414

.space	2, 45
.global	cell_1504
cell_1504:	.word	safeSpaceSram+832

.space	2, 45
.global	cell_1509
cell_1509:	.word	safeSpaceGpramSram+100

.align	2
.global	cell_1506
cell_1506:	.quad	0x061acd29a16e0750

.align	1
.space 76 % {{space_mod|default("0x10000000")}}
label254:
	sub	r13, #208
ldr	r7, cell_1518
ldr	r6, cell_1517
ldr	r9, cell_1516
ldr	r4, cell_1515
ldr	r10, cell_1514
mov	r2, #136
mov	r0, #6
mov	r5, #33873
mov	r3, #22161
	lsr	r1, r1                        @ A7.7.71
	ldrsh	r1, [r9, r3]                  @ A7.7.65
	ldrb	r9, [r10], #-145              @ A7.7.46
	str	r5, [r13, r0, LSL #2]         @ A7.7.162
	adds	r13, r2                       @ A7.7.6
	movt	r0, #25928                    @ A7.7.79
	ldrsb	r10, [r7, r5]                 @ A7.7.61
	add	r0, #0                        @ A7.7.1
	movt	r9, #55862                    @ A7.7.79
	ldrsb	r7, [r6]                      @ A7.7.59
	adr	r7, cell_1513                 @ A7.7.7
	strb	r7, [r4, #-204]!              @ A7.7.163
	ldrb	r4, [r13, #72]!               @ A7.7.46
mov	r4, #23
	bic	r7, r5, r7, LSL #2            @ A7.7.16
mov	r9, #0
mov	r7, #4
	bics	r5, #167                      @ A7.7.15
	ldrsh	r6, [r13, r7, LSL #2]         @ A7.7.65
	ldrsb	r0, [r13, r4]                 @ A7.7.61
ldr	r4, cell_1512
	ldm	r13, {r1-r2,r7,r10}           @ A7.7.41
	ldrsb	r6, [r4, #181]                @ A7.7.59
	strb	r9, [r13, r9]                 @ A7.7.164
	adr	r5, cell_1511                 @ A7.7.7
end_label254:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_1511
cell_1511:	.byte	0xf5

.space	3, 45
.global	cell_1514
cell_1514:	.word	safeSpaceFlash+130

.space	1, 45
.global	cell_1518
cell_1518:	.word	safeSpaceFlash-33587

.space	2, 45
.global	cell_1512
cell_1512:	.word	safeSpaceGpramSram+198

.space	3, 45
.global	cell_1513
cell_1513:	.byte	0x3b

.space	0, 45
.global	cell_1516
cell_1516:	.word	safeSpaceSram-21835

.space	2, 45
.global	cell_1515
cell_1515:	.word	safeSpaceSram+937

.space	3, 45
.global	cell_1517
cell_1517:	.word	safeSpaceFlash+178

.align	1
.space 1282 % {{space_mod|default("0x10000000")}}
label265:
	beq	forward_label_474             @ A7.7.12

	cmp	r8, r7, LSL #2                @ A7.7.28
	mov	r4, #117                      @ A7.7.76
ldr	r10, cell_1565
	cmp	r5, #37                       @ A7.7.27
	strd	r10, r4, [r10, #56]           @ A7.7.166
	bic	r5, r9                        @ A7.7.16
	umlal	r5, r3, r10, r9               @ A7.7.203
mov	r10, #6
	str	r5, [r13, r10, LSL #1]        @ A7.7.162

forward_label_474:
ldr	r9, cell_1564
	movt	r7, #39427                    @ A7.7.79
mov	r10, #5606
	ldr	r4, [r9, r10, LSL #2]         @ A7.7.45
mov	r1, #2
	mrs	r3, apsr                      @ A7.7.82
	ldrb	r5, [r13, r1, LSL #1]         @ A7.7.48
	mrs	r7, apsr                      @ A7.7.82
end_label265:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_1565
cell_1565:	.word	safeSpaceSram+128

.space	0, 45
.global	cell_1564
cell_1564:	.word	safeSpaceFlash-22185

.align	1
.space 70 % {{space_mod|default("0x10000000")}}
label267:
	sub	r13, #228
	stmdb	r13, {r2-r10}                 @ A7.7.160
mov	r2, #192
	sdiv	r4, r5, r1                    @ A7.7.127
	ldrsb	r8, cell_1572                 @ A7.7.60
	adds	r13, r2                       @ A7.7.6
	smlal	r9, r3, r7, r7                @ A7.7.138
ldr	r5, cell_1571
	ldrh	r8, [r5, #3806]               @ A7.7.55
	movt	r2, #41313                    @ A7.7.79
	mls	r1, r8, r5, r5                @ A7.7.75
	ldrsh	r4, cell_1570                 @ A7.7.64
	isb	                              @ A7.7.37
	ldrsh	r1, [r13], #36                @ A7.7.63
	umull	r8, r1, r8, r3                @ A7.7.204
end_label267:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_1571
cell_1571:	.word	safeSpaceGpramSram-3056

.space	1, 45
.global	cell_1570
cell_1570:	.short	0x5dae

.space	2, 45
.global	cell_1572
cell_1572:	.byte	0xc3

.align	1
.space 144 % {{space_mod|default("0x10000000")}}
label270:
	sub	r13, #68
	ands	r10, #167                     @ A7.7.8
ldr	r8, =func_31
	str	r7, [r13, #68]!               @ A7.7.161
orr	r8, #1
	ldrsb	r6, [r13]                     @ A7.7.59
	blx	r8                            @ A7.7.19


end_label270:
	b.w	{{code_end}}

.ltorg
.align	1
.space 232 % {{space_mod|default("0x10000000")}}
label273:
mov	r10, #17  @ 4b
	strb	r9, [r13, r10]                @ A7.7.164  @ 4b
	bfc	r6, #18, #8                   @ A7.7.13  @ 4b
ldr	r4, cell_1587  @ 4b
	ldrh	r0, cell_1586                 @ A7.7.56  @ 4b
mov	r6, #37862  @ 4b
ldr	r0, =func_6  @ 2b
	ldr	r3, cell_1585                 @ A7.7.44  @ 4b
	str	r10, [r4, r6, LSL #3]         @ A7.7.162  @ 4b
	ldrd	r6, r10, [r13, #56]           @ A7.7.50  @ 4b
	umlal	r1, r6, r10, r2               @ A7.7.203  @ 4b
orr	r0, #1  @ 4b
	ldrsb	r3, cell_1584                 @ A7.7.60  @ 4b
	blx	r0                            @ A7.7.19  @ 2b


	cbnz	r6, forward_label_484         @ A7.7.21  @ 2b

	ldrsb	r0, [r13]                     @ A7.7.59  @ 4b
	stm	r13, {r0,r3-r7,r9}            @ A7.7.159  @ 4b
mov	r9, #46  @ 4b
	and	r6, r3                        @ A7.7.9  @ 4b
	ldrsh	r2, [r13, r9]                 @ A7.7.65  @ 4b
	adcs	r9, r9, r3, LSL #1            @ A7.7.2  @ 4b
	and	r1, r0, #103                  @ A7.7.8  @ 4b

forward_label_484:
	adds	r8, r13, #29                  @ A7.7.5  @ 4b
end_label273:
	b.w	{{jump_label273}}

.ltorg
.align	2
.space	3, 45
.global	cell_1584
cell_1584:	.byte	0x58

.space	2, 45
.global	cell_1587
cell_1587:	.word	safeSpaceGpramSram-302386

.space	3, 45
.global	cell_1585
cell_1585:	.word	0x09f5a89a

.space	3, 45
.global	cell_1586
cell_1586:	.short	0x22b1

.align	1
.space 1394 % {{space_mod|default("0x10000000")}}
label281:
	sub	r13, #12
ldr	r10, =func_13
orr	r10, #1
	bic	r2, #46                       @ A7.7.15
	pop	{r2,r4,r9}                    @ A7.7.99
	cmn	r14, #99                      @ A7.7.25
	blx	r10                           @ A7.7.19


	sdiv	r10, r6                       @ A7.7.127
mov	r10, #7
	it	vs
	umlalvs	r6, r4, r6, r2                @ A7.7.203
	ldrsh	r5, [r13, r10, LSL #2]        @ A7.7.65
	ite	ne
	addne	r9, #220                      @ A7.7.1
	nopeq	                              @ A7.7.88
end_label281:
	b.w	{{code_end}}

.ltorg
.align	1
.space 1060 % {{space_mod|default("0x10000000")}}
label287:
ldr	r6, =forward_label_508
ldr	r7, cell_1727
mov	r10, #44
orr	r6, #1
	movw	r1, #40151                    @ A7.7.76
	blx	r6                            @ A7.7.19

	movs	r6, #214                      @ A7.7.76
	ldrsb	r1, cell_1726                 @ A7.7.60
	strd	r2, r5, [r7], #-44            @ A7.7.166

forward_label_508:
mov	r1, #25426
	ldrsh	r2, [r13, r10]                @ A7.7.65
	umull	r5, r10, r7, r10              @ A7.7.204
	strh	r1, [r13, #-49]               @ A7.7.170
	bfi	r5, r5, #11, #12              @ A7.7.14
	movw	r5, #60236                    @ A7.7.76
ldr	r7, cell_1725
	mrs	r2, apsr                      @ A7.7.82
	ldrb	r5, [r7, r1]                  @ A7.7.48
	cmp	r14, r10                      @ A7.7.28
	bfc	r4, #1, #17                   @ A7.7.13
	ldrb	r8, [r13, #20]                @ A7.7.46
	bl	func_72                       @ A7.7.18


	sdiv	r10, r10, r2                  @ A7.7.127
end_label287:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_1725
cell_1725:	.word	safeSpaceFlash-25103

.space	3, 45
.global	cell_1727
cell_1727:	.word	safeSpaceGpramSram+612

.space	0, 45
.global	cell_1726
cell_1726:	.byte	0xdb

.align	1
.space 78 % {{space_mod|default("0x10000000")}}
label289:
ldr	r6, =forward_label_512
ldr	r1, cell_1733
orr	r6, #1
	bx	r6                            @ A7.7.20

ldr	r4, cell_1732
	asr	r7, r9, r6                    @ A7.7.11
mov	r3, #37
	asrs	r8, r8, #14                   @ A7.7.10
	umull	r8, r6, r8, r10               @ A7.7.204
	bfi	r7, r8, #8, #14               @ A7.7.14
	ldrsh	r10, cell_1731                @ A7.7.64
	str	r10, [r4], #37                @ A7.7.161
	cmp	r4, #249                      @ A7.7.27
	mul	r6, r8, r7                    @ A7.7.84
	mul	r0, r3                        @ A7.7.84
	strb	r8, [r13, r3]                 @ A7.7.164

forward_label_512:
	umull	r8, r2, r1, r8                @ A7.7.204
	add	r0, #160                      @ A7.7.3
	ldrh	r0, [r1]                      @ A7.7.55
	ldr	r7, cell_1730                 @ A7.7.44
end_label289:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_1731
cell_1731:	.short	0x676e

.space	3, 45
.global	cell_1730
cell_1730:	.word	0x20bd6f9e

.space	0, 45
.global	cell_1733
cell_1733:	.word	safeSpaceGpramSram+512

.space	3, 45
.global	cell_1732
cell_1732:	.word	safeSpaceGpramSram+301

.align	1
.space 420 % {{space_mod|default("0x10000000")}}
label291:
ldr	r9, =forward_label_519  @ 4b
orr	r9, #1  @ 4b
mov	r2, #2  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r6, cell_1753  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_519:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label291:
	b.w	{{jump_label291}}

.ltorg
.align	2
.space	1, 45
.global	cell_1752
cell_1752:	.byte	0x7d

.space	0, 45
.global	cell_1750
cell_1750:	.short	0x1333

.space	3, 45
.global	cell_1753
cell_1753:	.word	safeSpaceFlash+590

.align	2
.global	cell_1751
cell_1751:	.quad	0x4180978f5ca48955

.align	1
.space 358 % {{space_mod|default("0x10000000")}}
func_31:
	sub	r13, #128
ldr	r9, =table_47
mov	r3, #3
ldr	r2, cell_1775
ldr	r10, cell_1774
mov	r0, #24
	tbb	[r9, r3]                      @ A7.7.185
func_31_switch_1_case_1:
	mov	r9, #219                      @ A7.7.76
func_31_switch_1_case_2:
	smull	r1, r9, r0, r7                @ A7.7.149
func_31_switch_1_case_3:
	ldr	r1, [r2], #213                @ A7.7.43
func_31_switch_1_case_4:
mov	r2, #4163
	cmn	r7, #46                       @ A7.7.25
	str	r5, [r10, r2]                 @ A7.7.162
	adcs	r2, r8, r6                    @ A7.7.2
	ldrsh	r9, cell_1773                 @ A7.7.64
ldr	r10, cell_1772
	ldrb	r1, [r10], #-7                @ A7.7.46
func_31_switch_1_case_5:
	add	r10, r13, r10, LSL #3         @ A7.7.6
	msr	apsr_nzcvq, r3                @ A7.7.83
	and	r1, r9, r10, LSL #1           @ A7.7.9
	umlal	r1, r2, r1, r10               @ A7.7.203
	isb	                              @ A7.7.37
	mov	r10, r2, LSL #1               @ A7.7.78
func_31_switch_1_case_6:
	mul	r10, r7                       @ A7.7.84
	nop	                              @ A7.7.88
ldr	r10, cell_1771
	ldr	r3, [r13, #104]!              @ A7.7.43
	add	r9, #45                       @ A7.7.1
	stm	r10, {r1,r6,r8-r9}            @ A7.7.159
	add	r13, r0                       @ A7.7.6
ldr	r1, cell_1770
mov	r2, #2
	bic	r9, r8, r5                    @ A7.7.16
	ldrsh	r9, [r1, #392]                @ A7.7.63
	ldrh	r10, [r13, #-20]              @ A7.7.55
	ldrsb	r10, [r13, r2, LSL #1]        @ A7.7.61
	asrs	r9, r5, #6                    @ A7.7.10
mov	r10, #0
	ldrh	r0, [r13, r10, LSL #1]        @ A7.7.57
	add	r3, r5                        @ A7.7.4
mov	r3, #47
	tst	r6, r8                        @ A7.7.189
	ldrh	r1, [r13, r3]                 @ A7.7.57
	ldrsb	r2, [r13, r10, LSL #1]        @ A7.7.61
	bfc	r3, #2, #30                   @ A7.7.13
	bfi	r2, r5, #2, #25               @ A7.7.14
	ldr	r2, [r13]                     @ A7.7.43
	tst	r2, r2                        @ A7.7.189
end_func_31:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_1772
cell_1772:	.word	safeSpaceSram+423

.space	0, 45
.global	cell_1771
cell_1771:	.word	safeSpaceGpramSram+468

.space	1, 45
.global	cell_1774
cell_1774:	.word	safeSpaceSram-3211

.space	0, 45
.global	cell_1770
cell_1770:	.word	safeSpaceSram-165

.space	3, 45
.global	cell_1773
cell_1773:	.short	0x25ab

.space	2, 45
.global	cell_1775
cell_1775:	.word	safeSpaceGpramSram+149

.align	1
label295:
ldr	r0, cell_1777
mov	r3, #5
ldr	r14, =post_branch_360
ldr	r9, =func_54
orr	r14, #1
	add	r10, r13, r6                  @ A7.7.6
	stm	r0!, {r1-r10}                 @ A7.7.159
	adc	r0, r1, r8, LSL #3            @ A7.7.2
	mov	r10, r5, LSL #2               @ A7.7.78
	ldr	r1, [r13, r3]                 @ A7.7.45
	adcs	r5, r10, #41                  @ A7.7.1
	bls	forward_label_527             @ A7.7.12


forward_label_527:
orr	r9, #1
	bx	r9                            @ A7.7.20
post_branch_360:


	ldrsb	r5, cell_1776                 @ A7.7.60
	and	r4, #64                       @ A7.7.8
	smlal	r6, r1, r5, r8                @ A7.7.138
	smull	r1, r6, r6, r5                @ A7.7.149
	isb	                              @ A7.7.37
	mov	r8, r1, LSL #1                @ A7.7.78
	tst	r8, #233                      @ A7.7.188
	ands	r0, r1, r5                    @ A7.7.9
	bfi	r2, r8, #2, #21               @ A7.7.14
end_label295:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_1776
cell_1776:	.byte	0xb7

.space	0, 45
.global	cell_1777
cell_1777:	.word	safeSpaceSram+808

.align	1
label296:
.space 2
ldr	r2, =forward_label_529  @ 2b
ldr	r0, cell_1779  @ 4b
orr	r2, #1  @ 4b
mov	r8, #62994  @ 4b
.space 2

ldr	r9, cell_1778  @ 4b
.space 4

forward_label_529:
	ldrb	r9, [r0, r8, LSL #3]          @ A7.7.48  @ 4b
.space 4
	asr	r2, r9                        @ A7.7.11  @ 4b
	adds	r3, r10, r10                  @ A7.7.4  @ 4b
end_label296:
	b.w	{{jump_label296}}

.ltorg
.align	2
.space	0, 45
.global	cell_1778
cell_1778:	.word	safeSpaceGpramSram+72

.space	3, 45
.global	cell_1779
cell_1779:	.word	safeSpaceGpramSram-503799

.align	1
.space 1706 % {{space_mod|default("0x10000000")}}
label309:
	sub	r13, #4
mov	r4, #5910
mov	r1, #49
ldr	r7, cell_1846
mov	r10, #58
ldr	r5, cell_1845
	ldrh	r2, [r13, r10]                @ A7.7.57
	lsr	r8, r2, r8                    @ A7.7.71
ldr	r9, cell_1844
	mul	r8, r10                       @ A7.7.84
	bics	r8, #236                      @ A7.7.15
	stmdb	r7!, {r1,r8,r10}              @ A7.7.160
	bfc	r8, #7, #17                   @ A7.7.13
	str	r0, [r13], #-128              @ A7.7.161
	ldr	r3, [r13, #-52]               @ A7.7.43
	stm	r9, {r1,r5-r6}                @ A7.7.159
	ldrsb	r0, [r5, r4]                  @ A7.7.61
	ldr	r2, [r13, r1]                 @ A7.7.45
	ldr	r0, [r13], #132               @ A7.7.43
	bics	r4, r1, r8, LSL #1            @ A7.7.16
end_label309:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_1845
cell_1845:	.word	safeSpaceFlash-5769

.space	0, 45
.global	cell_1844
cell_1844:	.word	safeSpaceSram+724

.space	2, 45
.global	cell_1846
cell_1846:	.word	safeSpaceSram+480

.align	1
.space 2546 % {{space_mod|default("0x10000000")}}
label333:
	sub	r13, #88
ldr	r5, =func_70
ldr	r14, =post_branch_404
orr	r14, #1
orr	r5, #1
	bx	r5                            @ A7.7.20
post_branch_404:


	bic	r6, r6, r10, LSL #2           @ A7.7.16
	bic	r5, #243                      @ A7.7.15
	asrs	r0, r8, #19                   @ A7.7.10
	ldrh	r8, cell_1945                 @ A7.7.56
	isb	                              @ A7.7.37
ldr	r10, cell_1944
	nop	                              @ A7.7.88
ldr	r2, cell_1943
	tst	r10, #14                      @ A7.7.188
	ldmdb	r2!, {r0-r1,r4-r8}            @ A7.7.42
	adds	r8, #128                      @ A7.7.1
	mul	r0, r8, r4                    @ A7.7.84
	ldrsb	r2, cell_1942                 @ A7.7.60
	stmdb	r10, {r1,r3}                  @ A7.7.160
	adds	r13, #88                      @ A7.7.5
end_label333:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_1945
cell_1945:	.short	0x8890

.space	2, 45
.global	cell_1944
cell_1944:	.word	safeSpaceSram+680

.space	0, 45
.global	cell_1943
cell_1943:	.word	safeSpaceSram+848

.space	3, 45
.global	cell_1942
cell_1942:	.byte	0x2b

.align	1
.space 834 % {{space_mod|default("0x10000000")}}
label339:
mov	r5, #11712
mov	r7, #5
ldr	r6, cell_1984
	bl	forward_label_598             @ A7.7.18
ldr	r14, =post_branch_413
orr	r14, #1

	bvc	func_73                       @ A7.7.12
post_branch_413:


	ldr	r0, [r13, r7, LSL #2]         @ A7.7.45
ldr	r7, cell_1983
mov	r0, #5
	adcs	r1, r4, #228                  @ A7.7.1
ldr	r2, cell_1982
	str	r3, [r13, r0]                 @ A7.7.162
	ldrb	r3, [r7, #1742]               @ A7.7.46
	strh	r2, [r6, #52]                 @ A7.7.170
	msr	apsr_nzcvq, r4                @ A7.7.83
	ldrb	r7, [r2, r5, LSL #2]          @ A7.7.48
	umlal	r4, r9, r7, r0                @ A7.7.203

forward_label_598:
	ldrsb	r7, [r13, #-14]               @ A7.7.59
	mls	r5, r5, r4, r5                @ A7.7.75
end_label339:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_1984
cell_1984:	.word	safeSpaceSram+536

.space	0, 45
.global	cell_1983
cell_1983:	.word	safeSpaceGpramSram-891

.space	1, 45
.global	cell_1982
cell_1982:	.word	safeSpaceSram-46292

.align	1
.space 292 % {{space_mod|default("0x10000000")}}
label342:
ldr	r7, cell_2002
ldr	r6, cell_2001
ldr	r1, cell_2000
ldr	r0, cell_1999
mov	r3, #19993
	bge	forward_label_604             @ A7.7.12

	udiv	r8, r6, r9                    @ A7.7.195
	bic	r4, #120                      @ A7.7.15
	add	r9, r0, #36                   @ A7.7.3
	ldrb	r8, [r6, r3]                  @ A7.7.48
	ldr	r3, [r0]                      @ A7.7.43
	umlal	r10, r9, r7, r10              @ A7.7.203
	ldrh	r0, cell_1998                 @ A7.7.56
	sdiv	r2, r1                        @ A7.7.127
	ldm	r7, {r0,r4}                   @ A7.7.41
	adr	r4, cell_1997                 @ A7.7.7

forward_label_604:
ldr	r4, cell_1996
	bfi	r5, r0, #4, #16               @ A7.7.14
	ldrh	r10, [r1, #-240]              @ A7.7.55
	movs	r2, r5                        @ A7.7.77
	cmp	r5, #209                      @ A7.7.27
	adds	r0, r4, r4                    @ A7.7.4
	ands	r8, r2, #48                   @ A7.7.8
	ldr	r8, [r4]                      @ A7.7.43
	udiv	r3, r5                        @ A7.7.195
	adds	r1, r10, r10, LSL #3          @ A7.7.4
	ldrh	r8, cell_1995                 @ A7.7.56
	tst	r9, #255                      @ A7.7.188
end_label342:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_1996
cell_1996:	.word	safeSpaceFlash+814

.space	1, 45
.global	cell_1997
cell_1997:	.byte	0x22

.space	3, 45
.global	cell_1998
cell_1998:	.short	0xdad1

.space	2, 45
.global	cell_2002
cell_2002:	.word	safeSpaceSram+256

.space	3, 45
.global	cell_2001
cell_2001:	.word	safeSpaceFlash-19484

.space	1, 45
.global	cell_2000
cell_2000:	.word	safeSpaceSram+1156

.space	0, 45
.global	cell_1995
cell_1995:	.short	0x001c

.space	3, 45
.global	cell_1999
cell_1999:	.word	safeSpaceFlash+550

.align	1
.space 434 % {{space_mod|default("0x10000000")}}
func_35:
	sub	r13, #120
ldr	r1, cell_2031
mov	r9, #4
ldr	r0, =table_53
mov	r10, #14625
ldr	r3, cell_2030
mov	r2, #1
	itett	lt
	cmplt	r2, r9, LSL #2                @ A7.7.28
	strge	r8, [r3, #242]                @ A7.7.161
	cmplt	r1, #86                       @ A7.7.27
	tbblt	[r0, r2]                      @ A7.7.185
func_35_switch_2_case_1:
	mul	r0, r3, r3                    @ A7.7.84
func_35_switch_2_case_2:
	tst	r5, r5                        @ A7.7.189
func_35_switch_2_case_3:
	strb	r6, [r13, r9, LSL #3]         @ A7.7.164
	ldrb	r9, cell_2029                 @ A7.7.47
	strd	r8, r7, [r13]                 @ A7.7.166
ldr	r2, cell_2028
	mls	r9, r10, r3, r7               @ A7.7.75
mov	r9, #22
	ldrsb	r0, [r13, r9]                 @ A7.7.61
	ldrsb	r9, [r13, #-7]                @ A7.7.59
	str	r4, [r2, r10]                 @ A7.7.162
func_35_switch_2_case_4:
	isb	                              @ A7.7.37
func_35_switch_2_case_5:
	adcs	r9, r10, r4, LSL #2           @ A7.7.2
ldr	r0, cell_2027
	ldrh	r9, [r1], #-5                 @ A7.7.55
ldr	r9, cell_2026
	ldrb	r2, cell_2025                 @ A7.7.47
	smlal	r1, r2, r8, r6                @ A7.7.138
	add	r1, r13, r3                   @ A7.7.6
	ldrsh	r1, [r13], #120               @ A7.7.63
	cbnz	r1, forward_label_612         @ A7.7.21

	ldrd	r2, r1, [r3]                  @ A7.7.50
	lsrs	r3, r3, r4                    @ A7.7.71
	smlal	r1, r3, r2, r7                @ A7.7.138
ldr	r1, cell_2024
	ldm	r1!, {r2-r3}                  @ A7.7.41
	ldrsb	r2, [r13, #-60]               @ A7.7.59
	mla	r2, r4, r9, r2                @ A7.7.74

forward_label_612:
ldr	r3, cell_2023
	cmn	r1, r5, LSL #1                @ A7.7.26
mov	r2, #42
	ldrsh	r1, cell_2022                 @ A7.7.64
	tst	r5, #150                      @ A7.7.188
	strb	r0, [r13, r2]                 @ A7.7.164
	udiv	r1, r3, r2                    @ A7.7.195
	smull	r2, r1, r0, r9                @ A7.7.149
	ldrd	r1, r2, [r0], #96             @ A7.7.50
	nop	                              @ A7.7.88
	ldrsb	r0, [r9, r10]                 @ A7.7.61
	str	r0, [r3, r10, LSL #1]         @ A7.7.162
	smull	r3, r9, r5, r10               @ A7.7.149
	add	r2, r13, #217                 @ A7.7.3
	cmn	r1, r3, LSL #2                @ A7.7.26
	movw	r3, #61203                    @ A7.7.76
	mov	r1, r0, LSL #1                @ A7.7.78
	udiv	r9, r1, r10                   @ A7.7.195
	stmdb	r13, {r4,r8}                  @ A7.7.160
	smlal	r10, r3, r3, r10              @ A7.7.138
	ldrb	r3, cell_2020                 @ A7.7.47
end_func_35:
	bx	r14

.ltorg
.align	2
.space	2, 46
.global	table_53
table_53:
.byte	0
.byte	((func_35_switch_2_case_2-func_35_switch_2_case_1)/2)
.byte	((func_35_switch_2_case_3-func_35_switch_2_case_1)/2)
.byte	((func_35_switch_2_case_4-func_35_switch_2_case_1)/2)
.byte	((func_35_switch_2_case_5-func_35_switch_2_case_1)/2)

.space	2, 45
.global	cell_2026
cell_2026:	.word	safeSpaceGpramSram-13852

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}
.global	cell_2023
cell_2023:	.word	safeSpaceGpramSram-29162

.space	1, 45
.global	cell_2020
cell_2020:	.byte	0x1c

.space	0, 45
.global	cell_2025
cell_2025:	.byte	0xd9

.space	3, 45
.global	cell_2031
cell_2031:	.word	safeSpaceFlash+692

.space	1, 45
.global	cell_2029
cell_2029:	.byte	0xfa

.space	1, 45
.global	cell_2028
cell_2028:	.word	safeSpaceGpramSram-14121

.space	3, 45
.global	cell_2027
cell_2027:	.word	safeSpaceGpramSram+840

.space	0, 45
.global	cell_2024
cell_2024:	.word	safeSpaceSram+920

.space	2, 45
.global	cell_2022
cell_2022:	.short	0x676b

.space	0, 45
.global	cell_2030
cell_2030:	.word	safeSpaceSram+104

.align	1
.space 124 % {{space_mod|default("0x10000000")}}
label348:
ldr	r10, cell_2042
mov	r5, #17407
ldr	r3, cell_2041
ldr	r1, cell_2040
ldr	r8, cell_2039
ldr	r2, cell_2038
ldr	r0, cell_2037
	bl	forward_label_615             @ A7.7.18

	add	r4, r15                       @ A7.7.4
	strb	r0, [r10, r5]                 @ A7.7.164
	ldr	r9, [r1]                      @ A7.7.43
	asr	r10, r0                       @ A7.7.11
	stmdb	r13, {r0-r1,r3-r6,r10}        @ A7.7.160
	bfi	r4, r8, #23, #3               @ A7.7.14
	ldr	r9, cell_2036                 @ A7.7.44
	lsrs	r4, r10, r8                   @ A7.7.71
mov	r1, #49
	adc	r5, r3, #158                  @ A7.7.1
	strb	r6, [r13]                     @ A7.7.163
	ldmdb	r3!, {r4-r7,r9-r10}           @ A7.7.42
	strb	r5, [r2, #-26]                @ A7.7.163
	bfi	r9, r4, #1, #27               @ A7.7.14
	str	r6, [r13, r1]                 @ A7.7.162

forward_label_615:
	tst	r9, #201                      @ A7.7.188
mov	r10, #20805
	itt	vs
	ldrhvs	r1, [r8]                      @ A7.7.55
	ldrshvs	r7, [r0, r10]                 @ A7.7.65
	smull	r2, r9, r8, r1                @ A7.7.149
	sdiv	r2, r10                       @ A7.7.127
	addw	r1, r3, #928                  @ A7.7.3
	movt	r8, #24964                    @ A7.7.79
	isb	                              @ A7.7.37
mov	r0, #2
ldr	r8, cell_2035
	ldrb	r3, [r13, r0, LSL #2]         @ A7.7.48
	str	r0, [r8]                      @ A7.7.161
	tst	r10, r1, LSL #2               @ A7.7.189
	lsr	r1, r0                        @ A7.7.71
end_label348:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2042
cell_2042:	.word	safeSpaceGpramSram-16548

.space	1, 45
.global	cell_2037
cell_2037:	.word	safeSpaceSram-20342

.space	0, 45
.global	cell_2040
cell_2040:	.word	safeSpaceGpramSram+247

.space	1, 45
.global	cell_2041
cell_2041:	.word	safeSpaceSram+668

.space	1, 45
.global	cell_2038
cell_2038:	.word	safeSpaceSram+547

.space	0, 45
.global	cell_2039
cell_2039:	.word	safeSpaceFlash+494

.space	1, 45
.global	cell_2036
cell_2036:	.word	0x2a542611

.space	0, 45
.global	cell_2035
cell_2035:	.word	safeSpaceSram+203

.align	1
label349:
	sub	r13, #72
ldr	r8, cell_2047
ldr	r2, cell_2046
mov	r3, #10470
	.align	2
	ldrd	r4, r9, cell_2045             @ A7.7.51
	strd	r2, r10, [r13], #72           @ A7.7.166
	strd	r0, r7, [r13, #8]             @ A7.7.166
	cmn	r9, r10                       @ A7.7.26
	str	r10, [r8, r3, LSL #1]         @ A7.7.162
	strh	r1, [r13]                     @ A7.7.170
ldr	r3, cell_2044
	sdiv	r9, r8                        @ A7.7.127
ldr	r1, cell_2043
	ldrd	r5, r10, [r2, #-204]          @ A7.7.50
	bic	r2, #115                      @ A7.7.15
	umull	r0, r7, r10, r8               @ A7.7.204
	ldrsh	r2, [r1, #77]                 @ A7.7.63
	umull	r1, r2, r8, r3                @ A7.7.204
	strd	r1, r8, [r3]                  @ A7.7.166
	adcs	r9, r2, r10, LSL #2           @ A7.7.2
end_label349:
	b.w	{{code_end}}

.ltorg
.align	2
.global	cell_2045
cell_2045:	.quad	0x503fa26c0e8fbc5a

.space	3, 45
.global	cell_2044
cell_2044:	.word	safeSpaceSram+580

.space	3, 45
.global	cell_2047
cell_2047:	.word	safeSpaceGpramSram-20790

.space	1, 45
.global	cell_2046
cell_2046:	.word	safeSpaceFlash+316

.space	2, 45
.global	cell_2043
cell_2043:	.word	safeSpaceGpramSram+756

.align	1
.space 138 % {{space_mod|default("0x10000000")}}
label351:
	sub	r13, #40
mov	r0, #7
	ldrsh	r10, [r13]                    @ A7.7.63
	mls	r7, r7, r8, r0                @ A7.7.75
	ldr	r10, [r13], #40               @ A7.7.43
mov	r7, #2
	ands	r9, #251                      @ A7.7.8
	strb	r7, [r13, r7, LSL #1]         @ A7.7.164
	stmdb	r13, {r0,r2,r5-r6,r8}         @ A7.7.160
	ldrh	r8, cell_2052                 @ A7.7.56
	ldrsb	r9, [r13, r0, LSL #3]         @ A7.7.61
end_label351:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_2052
cell_2052:	.short	0xcfc2

.align	1
.space 378 % {{space_mod|default("0x10000000")}}
label355:
ldr	r8, =func_55
orr	r8, #1
mov	r10, #0
mov	r1, #11246
ldr	r7, cell_2081
ldr	r2, =table_55
ldr	r5, cell_2080
ldr	r6, cell_2079
ldr	r0, cell_2078
mov	r4, #15547
	tbh	[r2, r10, LSL #1]             @ A7.7.185
label355_switch_1_case_1:
	ldrsh	r2, cell_2077                 @ A7.7.64
	stm	r6, {r0,r5,r10}               @ A7.7.159
	ldrsh	r3, [r7, #1040]               @ A7.7.63
label355_switch_1_case_2:
	smull	r3, r2, r0, r4                @ A7.7.149
label355_switch_1_case_3:
	strh	r1, [r0, r1]                  @ A7.7.171
label355_switch_1_case_4:
	mls	r10, r2, r8, r2               @ A7.7.75
	ands	r9, r4, r4                    @ A7.7.9
	cmn	r7, r1                        @ A7.7.26
label355_switch_1_case_5:
	asr	r7, r8                        @ A7.7.11
label355_switch_1_case_6:
	blx	r8                            @ A7.7.19


	ldrsb	r6, cell_2076                 @ A7.7.60
	ldrb	r0, [r5, r4, LSL #1]          @ A7.7.48
	adr	r5, cell_2075                 @ A7.7.7
	adds	r9, r13, r0, LSL #1           @ A7.7.4
mov	r1, #1797
ldr	r5, cell_2074
	adr	r7, cell_2073                 @ A7.7.7
	cmn	r0, r5                        @ A7.7.26
	asr	r0, r5                        @ A7.7.11
	ldrsh	r8, [r5, r1]                  @ A7.7.65
	movt	r9, #19058                    @ A7.7.79
	ldrsh	r7, [r13, #19]                @ A7.7.63
	mls	r2, r4, r4, r4                @ A7.7.75
end_label355:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 46
.global	table_55
table_55:
.hword	0
.hword	((label355_switch_1_case_2-label355_switch_1_case_1)/2)
.hword	((label355_switch_1_case_3-label355_switch_1_case_1)/2)
.hword	((label355_switch_1_case_4-label355_switch_1_case_1)/2)
.hword	((label355_switch_1_case_5-label355_switch_1_case_1)/2)
.hword	((label355_switch_1_case_6-label355_switch_1_case_1)/2)

.space	2, 45
.global	cell_2073
cell_2073:	.byte	0xf8

.space	2, 45
.global	cell_2080
cell_2080:	.word	safeSpaceSram-30921

.space	2, 45
.global	cell_2077
cell_2077:	.short	0xa387

.space	3, 45
.global	cell_2081
cell_2081:	.word	safeSpaceGpramSram-666

.space	2, 45
.global	cell_2078
cell_2078:	.word	safeSpaceGpramSram-10342

.space	1, 45
.global	cell_2079
cell_2079:	.word	safeSpaceSram+432

.space	2, 45
.global	cell_2075
cell_2075:	.byte	0xb2

.space	0, 45
.global	cell_2076
cell_2076:	.byte	0x3f

.space	3, 45
.global	cell_2074
cell_2074:	.word	safeSpaceFlash-1578

.align	1
.space 228 % {{space_mod|default("0x10000000")}}
label358:
ldr	r6, cell_2096
mov	r5, #5
ldr	r0, cell_2095
ldr	r10, =table_56
mov	r8, #2
	tbh	[r10, r8, LSL #1]             @ A7.7.185
label358_switch_1_case_1:
	ldrsh	r8, [r13, r5, LSL #2]         @ A7.7.65
	stmdb	r13, {r0,r2,r7,r9}            @ A7.7.160
	adr	r1, cell_2094                 @ A7.7.7
	bics	r3, #172                      @ A7.7.15
	ldrsb	r3, [r13]                     @ A7.7.59
label358_switch_1_case_2:
	bic	r2, r7, r6, LSL #1            @ A7.7.16
	movw	r4, #28705                    @ A7.7.76
	ldmdb	r6, {r1-r5,r7-r10}            @ A7.7.42
	cmn	r2, r5, LSL #1                @ A7.7.26
mov	r7, #10
	str	r5, [r13, r7]                 @ A7.7.162
	sdiv	r10, r2                       @ A7.7.127
mov	r4, #1
	cmp	r7, r8, LSL #3                @ A7.7.28
	and	r2, r10                       @ A7.7.9
	ldrh	r5, cell_2093                 @ A7.7.56
	ldrh	r3, [r13, r4, LSL #3]         @ A7.7.57
	adc	r10, r0, #26                  @ A7.7.1
	adc	r5, r10, #46                  @ A7.7.1
label358_switch_1_case_3:
	bics	r1, #110                      @ A7.7.15
	bfi	r5, r9, #10, #14              @ A7.7.14
mov	r5, #55541
	strh	r1, [r0, r5, LSL #2]          @ A7.7.171
end_label358:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2095
cell_2095:	.word	safeSpaceGpramSram-221928

.space	0, 45
.global	cell_2096
cell_2096:	.word	safeSpaceSram+744

.space	3, 45
.global	cell_2093
cell_2093:	.short	0x602b

.space	3, 45
.global	cell_2094
cell_2094:	.byte	0x86

.align	1
.space 1822 % {{space_mod|default("0x10000000")}}
label373:
mov	r3, #55937
mov	r4, #0
mov	r8, #8733
mov	r7, #7
ldr	r10, cell_2174
ldr	r2, cell_2173
ldr	r6, cell_2172
mov	r1, #0
ldr	r9, =table_59
ldr	r5, cell_2171
	tbh	[r9, r1, LSL #1]              @ A7.7.185
label373_switch_1_case_1:
	ldr	r9, [r2, #3283]               @ A7.7.43
label373_switch_1_case_2:
	sdiv	r2, r2, r10                   @ A7.7.127
mov	r9, #1472
	cmp	r6, r9                        @ A7.7.28
	ldmdb	r13, {r0,r2}                  @ A7.7.42
ldr	r0, cell_2170
	bfc	r2, #8, #8                    @ A7.7.13
	str	r2, [r0, r9, LSL #3]          @ A7.7.162
	stmdb	r6!, {r0,r4,r8-r10}           @ A7.7.160
	cmp	r0, r9                        @ A7.7.28
	nop	                              @ A7.7.88
	ldrsb	r9, cell_2169                 @ A7.7.60
	ldm	r10, {r0-r2,r6}               @ A7.7.41
	ldrb	r2, [r5, r3, LSL #2]          @ A7.7.48
	stmdb	r10!, {r1-r3,r5-r9}           @ A7.7.160
	ldrsh	r9, [r13, r4]                 @ A7.7.65
	cmn	r7, #251                      @ A7.7.25
	mls	r6, r9, r4, r5                @ A7.7.75
label373_switch_1_case_3:
	bic	r5, r10                       @ A7.7.16
	adr	r2, cell_2168                 @ A7.7.7
	stm	r13, {r1,r4,r7-r8}            @ A7.7.159
	msr	apsr_nzcvq, r2                @ A7.7.83
	ldrh	r3, [r13, r7, LSL #2]         @ A7.7.57
ldr	r10, cell_2167
	bfc	r0, #4, #22                   @ A7.7.13
	strb	r7, [r10, r8]                 @ A7.7.164
end_label373:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_2174
cell_2174:	.word	safeSpaceSram+144

.space	1, 45
.global	cell_2173
cell_2173:	.word	safeSpaceFlash-2327

.space	1, 45
.global	cell_2168
cell_2168:	.byte	0x7b

.space	0, 45
.global	cell_2172
cell_2172:	.word	safeSpaceGpramSram+592

.space	1, 45
.global	cell_2171
cell_2171:	.word	safeSpaceSram-223419

.space	1, 45
.global	cell_2169
cell_2169:	.byte	0xb5

.space	0, 45
.global	cell_2170
cell_2170:	.word	safeSpaceSram-11471

.space	3, 45
.global	cell_2167
cell_2167:	.word	safeSpaceGpramSram-8049

.align	1
.space 916 % {{space_mod|default("0x10000000")}}
func_38:
	sub	r13, #248
ldr	r0, cell_2214
mov	r10, #112
mov	r3, #3
	movt	r9, #9839                     @ A7.7.79
	str	r10, [r13, r3, LSL #3]        @ A7.7.162
	cmn	r6, r9                        @ A7.7.26
	udiv	r2, r9, r9                    @ A7.7.195
	ldrsh	r1, cell_2213                 @ A7.7.64
ldr	r1, cell_2212
	tst	r1, r2                        @ A7.7.189
mov	r2, #126
	ldrsb	r9, cell_2211                 @ A7.7.60
mov	r3, #7
	ldrsb	r9, [r13], #-4                @ A7.7.59
	add	r9, #96                       @ A7.7.3
	msr	apsr_nzcvq, r10               @ A7.7.83
	mul	r9, r4, r0                    @ A7.7.84
	ldrsh	r9, [r13, r3, LSL #3]         @ A7.7.65
	stm	r13!, {r3,r10}                @ A7.7.159
	ldrh	r3, [r1], #-178               @ A7.7.55
	movw	r9, #58744                    @ A7.7.76
	add	r13, r10                      @ A7.7.6
	smlal	r1, r10, r6, r2               @ A7.7.138
	ldrsh	r10, [r0, r2]                 @ A7.7.65
ldr	r3, cell_2210
	umlal	r10, r0, r4, r0               @ A7.7.203
mov	r1, #58
	ldr	r0, cell_2209                 @ A7.7.44
	add	r2, r2                        @ A7.7.4
ldr	r10, cell_2208
	cmn	r8, r3                        @ A7.7.26
	iteet	ge
	ldrge	r2, [r10, #-228]!             @ A7.7.43
	andslt	r0, #197                      @ A7.7.8
	bfilt	r9, r8, #19, #3               @ A7.7.14
	ldrsbge	r10, [r3], #-220              @ A7.7.59
	ldr	r9, [r13, #132]!              @ A7.7.43
	udiv	r0, r6, r2                    @ A7.7.195
mov	r2, #33897
ldr	r0, cell_2207
	mla	r3, r10, r1, r4               @ A7.7.74
	ldr	r9, [r0, r2, LSL #3]          @ A7.7.45
mov	r3, #5
	asr	r10, r8                       @ A7.7.11
	.align	2
	ldrd	r2, r10, cell_2206            @ A7.7.51
	cbz	r3, forward_label_664         @ A7.7.21

ldr	r10, cell_2205
	strh	r0, [r13, r3, LSL #1]         @ A7.7.171
	ldrh	r2, [r13, r1]                 @ A7.7.57
	ldrsb	r9, [r13, r3, LSL #3]         @ A7.7.61
	msr	apsr_nzcvq, r0                @ A7.7.83
	ldrb	r1, [r10, #149]               @ A7.7.46
	umull	r0, r10, r2, r4               @ A7.7.204
	ands	r0, #153                      @ A7.7.8
	bics	r10, r4, #49                  @ A7.7.15
	sdiv	r1, r10                       @ A7.7.127
	umlal	r9, r3, r0, r8                @ A7.7.203

forward_label_664:
end_func_38:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_2210
cell_2210:	.word	safeSpaceSram+750

.space	0, 45
.global	cell_2212
cell_2212:	.word	safeSpaceSram+109

.space	2, 45
.global	cell_2214
cell_2214:	.word	safeSpaceFlash+509

.space	3, 45
.global	cell_2211
cell_2211:	.byte	0x1a

.space	3, 45
.global	cell_2208
cell_2208:	.word	safeSpaceGpramSram+1027

.space	1, 45
.global	cell_2213
cell_2213:	.short	0x1fa7

.space	3, 45
.global	cell_2207
cell_2207:	.word	safeSpaceFlash-270295

.space	3, 45
.global	cell_2209
cell_2209:	.word	0xb900d0a0

.align	2
.global	cell_2206
cell_2206:	.quad	0x5bfb524ecb2205bd

.space	3, 45
.global	cell_2205
cell_2205:	.word	safeSpaceFlash+332

.align	1
.space 180 % {{space_mod|default("0x10000000")}}
label382:
ldr	r4, cell_2225  @ 4b
ldr	r2, cell_2224  @ 2b
ldr	r3, cell_2223  @ 2b
	ldmdb	r3!, {r0,r5,r7,r9-r10}        @ A7.7.42  @ 4b
	strb	r1, [r2, #-237]               @ A7.7.163  @ 4b
	mov	r10, r10, LSL #2              @ A7.7.78  @ 4b
	nop	                              @ A7.7.88  @ 2b
	asr	r10, r7                       @ A7.7.11  @ 4b
mov	r7, #41590  @ 4b
	sdiv	r10, r10                      @ A7.7.127  @ 4b
	adr	r5, cell_2222                 @ A7.7.7  @ 4b
ldr	r2, cell_2221  @ 4b
	movw	r3, #45422                    @ A7.7.76  @ 4b
	ldrh	r8, cell_2220                 @ A7.7.56  @ 4b
	movt	r8, #10974                    @ A7.7.79  @ 4b
	ldrh	r8, [r2, r7]                  @ A7.7.57  @ 4b
ldr	r8, cell_2219  @ 4b
	stm	r4!, {r0,r2,r6}               @ A7.7.159  @ 2b
	and	r2, r2, #190                  @ A7.7.8  @ 4b
	ldrh	r0, [r8], #-89                @ A7.7.55  @ 4b
end_label382:
	b.w	{{jump_label382}}

.ltorg
.align	2
.space	2, 45
.global	cell_2225
cell_2225:	.word	safeSpaceSram+396

.space	3, 45
.global	cell_2222
cell_2222:	.byte	0x2d

.space	0, 45
.global	cell_2221
cell_2221:	.word	safeSpaceSram-40701

.space	2, 45
.global	cell_2219
cell_2219:	.word	safeSpaceFlash+408

.space	1, 45
.global	cell_2220
cell_2220:	.short	0x67a3

.space	1, 45
.global	cell_2224
cell_2224:	.word	safeSpaceGpramSram+1092

.space	0, 45
.global	cell_2223
cell_2223:	.word	safeSpaceSram+540

.align	1
.space 992 % {{space_mod|default("0x10000000")}}
func_41:
ldr	r3, =forward_label_682  @ 2b
orr	r3, #1  @ 4b
ldr	r2, cell_2278  @ 2b
ldr	r0, =forward_label_684  @ 2b
orr	r0, #1  @ 4b
mov	r9, #49507  @ 4b
ldr	r1, cell_2277  @ 2b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r0, cell_2276  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r0, cell_2275  @ 4b
.space 4

forward_label_684:
.space 4
mov	r2, #62  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_683:
	adr	r9, cell_2272                 @ A7.7.7  @ 4b
.space 4
ldr	r1, cell_2271  @ 4b
	ldm	r1, {r1-r2,r9-r10}            @ A7.7.41  @ 4b
	mov	r15, r3                       @ A7.7.77  @ 2b

ldr	r0, cell_2270  @ 4b
	strh	r9, [r0, #-172]!              @ A7.7.170  @ 4b
ldr	r0, cell_2269  @ 4b
	mov	r9, r2                        @ A7.7.77  @ 2b
	ldrh	r3, cell_2268                 @ A7.7.56  @ 4b
.space 4
ldr	r3, cell_2267  @ 4b
	movs	r1, #148                      @ A7.7.76  @ 2b
	ldrh	r9, [r0, #185]!               @ A7.7.55  @ 4b
	nop.n  @ was .align 2  @ 2b
	ldrd	r2, r9, cell_2266             @ A7.7.51  @ 4b
	ldrb	r1, cell_2265                 @ A7.7.47  @ 4b
	strh	r4, [r3], #-79                @ A7.7.170  @ 4b
	cmp	r5, #2                        @ A7.7.27  @ 2b

forward_label_682:
mov	r9, #30  @ 4b
.space 4
ldr	r9, cell_2264  @ 4b
.space 4
.space 4
end_func_41:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_2269
cell_2269:	.word	safeSpaceGpramSram+744

.space	2, 45
.global	cell_2267
cell_2267:	.word	safeSpaceGpramSram+189

.space	2, 45
.global	cell_2275
cell_2275:	.word	safeSpaceSram+983

.align	2
.global	cell_2266
cell_2266:	.quad	0x65fd11a761d2c8cc

.space	3, 45
.global	cell_2272
cell_2272:	.byte	0xee

.space	3, 45
.global	cell_2273
cell_2273:	.byte	0x25

.space	3, 45
.global	cell_2268
cell_2268:	.short	0xa4a9

.space	1, 45
.global	cell_2271
cell_2271:	.word	safeSpaceSram+132

.space	2, 45
.global	cell_2265
cell_2265:	.byte	0xc7

.space	1, 45
.global	cell_2270
cell_2270:	.word	safeSpaceSram+868

.space	0, 45
.global	cell_2276
cell_2276:	.word	safeSpaceSram+1175

.space	2, 45
.global	cell_2278
cell_2278:	.word	safeSpaceSram-98167

.space	0, 45
.global	cell_2264
cell_2264:	.word	safeSpaceSram+319

.align	2
.global	cell_2274
cell_2274:	.quad	0x7719516380a4b154

.space	0, 45
.global	cell_2277
cell_2277:	.word	safeSpaceSram+774

.align	1
.space 1236 % {{space_mod|default("0x10000000")}}
label391:
	sub	r13, #44
ldr	r7, =func_25
orr	r7, #1
ldr	r3, cell_2340
mov	r9, #3
mov	r4, #2
ldr	r0, =table_66
	tbb	[r0, r9]                      @ A7.7.185
label391_switch_1_case_1:
	ldmdb	r3, {r0,r2,r5,r8-r10}         @ A7.7.42
	umull	r10, r8, r2, r9               @ A7.7.204
	ldr	r9, cell_2339                 @ A7.7.44
label391_switch_1_case_2:
	ldrsh	r0, cell_2338                 @ A7.7.64
label391_switch_1_case_3:
	ldrh	r8, [r13, #23]                @ A7.7.55
label391_switch_1_case_4:
	adds	r0, r13, r9, LSL #3           @ A7.7.6
	bfi	r8, r9, #21, #7               @ A7.7.14
ldr	r9, cell_2337
	stm	r9!, {r0,r3,r7-r8,r10}        @ A7.7.159
ldr	r10, cell_2336
	ldrh	r3, [r10], #3                 @ A7.7.55
	movt	r0, #32220                    @ A7.7.79
	.align	2
	ldrd	r0, r8, cell_2335             @ A7.7.51
label391_switch_1_case_5:
	ldrsh	r9, [r13, #-192]!             @ A7.7.63
	movt	r0, #42607                    @ A7.7.79
	ldrsh	r8, cell_2334                 @ A7.7.64
	strh	r7, [r13, r4, LSL #1]         @ A7.7.171
	blx	r7                            @ A7.7.19


mov	r8, #894
ldr	r7, cell_2333
	strb	r7, [r7, r8, LSL #2]          @ A7.7.164
ldr	r9, cell_2332
	strd	r7, r0, [r9, #-4]             @ A7.7.166
	addw	r13, r13, #236                @ A7.7.5
	nop	                              @ A7.7.88
	mov	r7, r13                       @ A7.7.77
end_label391:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2337
cell_2337:	.word	safeSpaceGpramSram+328

.space	0, 45
.global	cell_2332
cell_2332:	.word	safeSpaceSram+376

.space	0, 45
.global	cell_2340
cell_2340:	.word	safeSpaceGpramSram+704

.align	2
.global	cell_2335
cell_2335:	.quad	0x36b0b59ac5b6c614

.space	2, 45
.global	cell_2333
cell_2333:	.word	safeSpaceGpramSram-3320

.space	3, 45
.global	cell_2334
cell_2334:	.short	0x90e4

.space	2, 45
.global	cell_2336
cell_2336:	.word	safeSpaceFlash+447

.space	2, 45
.global	cell_2339
cell_2339:	.word	0x5f83b477

.space	2, 45
.global	cell_2338
cell_2338:	.short	0xc542

.align	1
.space 606 % {{space_mod|default("0x10000000")}}
label397:
	sub	r13, #224
mov	r1, #0
ldr	r2, =func_1
ldr	r9, cell_2367
mov	r6, #3
mov	r7, #7697
ldr	r8, cell_2366
	cbnz	r4, forward_label_709         @ A7.7.21

	nop	                              @ A7.7.88
	ldr	r3, [r8, #525]                @ A7.7.43
	ldrd	r10, r5, [r13]                @ A7.7.50
	str	r5, [r13, r1, LSL #3]         @ A7.7.162
ldr	r5, cell_2365
	strh	r2, [r9, r7, LSL #3]          @ A7.7.171
	ldrh	r9, [r13, r6, LSL #2]         @ A7.7.57
	ldrsb	r1, cell_2364                 @ A7.7.60
	asr	r1, r8                        @ A7.7.11
	ldrsh	r1, [r5]                      @ A7.7.63
	bfi	r4, r2, #17, #6               @ A7.7.14
	asr	r9, r7, r6                    @ A7.7.11
	ldrb	r6, cell_2363                 @ A7.7.47

forward_label_709:
	msr	apsr_nzcvq, r8                @ A7.7.83
	umull	r8, r3, r3, r2                @ A7.7.204
	mrs	r0, apsr                      @ A7.7.82
	ldrd	r9, r4, [r13, #-36]           @ A7.7.50
	strd	r5, r10, [r13, #224]!         @ A7.7.166
orr	r2, #1
	ldr	r4, cell_2362                 @ A7.7.44
	adds	r7, r5, #115                  @ A7.7.3
	ldm	r13, {r0-r1,r3,r5-r6,r8-r10}  @ A7.7.41
	bfi	r4, r6, #20, #2               @ A7.7.14
	cmp	r9, #125                      @ A7.7.27
	adds	r6, #237                      @ A7.7.1
	blx	r2                            @ A7.7.19


end_label397:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_2364
cell_2364:	.byte	0xca

.space	0, 45
.global	cell_2362
cell_2362:	.word	0xfbdd7b1f

.space	3, 45
.global	cell_2367
cell_2367:	.word	safeSpaceGpramSram-60969

.space	0, 45
.global	cell_2365
cell_2365:	.word	safeSpaceGpramSram+628

.space	3, 45
.global	cell_2363
cell_2363:	.byte	0x82

.space	2, 45
.global	cell_2366
cell_2366:	.word	safeSpaceGpramSram-350

.align	1
func_46:
	sub	r13, #196
ldr	r2, =forward_label_712
orr	r2, #1
mov	r3, #10690
ldr	r9, cell_2378
ldr	r0, cell_2377
ldr	r1, cell_2376
ldr	r10, cell_2375
	mov	r15, r2                       @ A7.7.77

	lsr	r2, r6                        @ A7.7.71
	adcs	r2, r7, r8                    @ A7.7.2
	adds	r2, #251                      @ A7.7.1
	mla	r2, r5, r10, r3               @ A7.7.74
ldr	r2, cell_2374
	isb	                              @ A7.7.37
	stmdb	r2!, {r0,r3,r5,r7-r9}         @ A7.7.160
	tst	r2, r0                        @ A7.7.189
	movs	r2, r2                        @ A7.7.77
	ldrsh	r2, cell_2373                 @ A7.7.64
	lsr	r2, r6                        @ A7.7.71
	adds	r2, r13, #251                 @ A7.7.5
	bics	r2, #28                       @ A7.7.15
	cmp	r2, #26                       @ A7.7.27
	bfi	r2, r10, #1, #16              @ A7.7.14

forward_label_712:
	ldrsb	r2, [r13], #152               @ A7.7.59
	adcs	r2, r4, #39                   @ A7.7.1
mov	r2, #59629
	bvc	forward_label_711             @ A7.7.12

	strb	r4, [r13, #22]                @ A7.7.163
	strb	r1, [r1]                      @ A7.7.163
	movt	r1, #42106                    @ A7.7.79
ldr	r1, cell_2372
	stmdb	r10, {r0-r7,r9-r10}           @ A7.7.160
	strh	r9, [r1, r2, LSL #3]          @ A7.7.171
	asrs	r1, r7, r2                    @ A7.7.11

forward_label_711:
	asr	r1, r2                        @ A7.7.11
	strh	r1, [r10, #175]!              @ A7.7.170
	nop	                              @ A7.7.88
ldr	r1, cell_2371
	ldrsh	r10, [r1, r3, LSL #1]         @ A7.7.65
	ldrh	r1, cell_2370                 @ A7.7.56
ldr	r3, cell_2369
	adc	r10, r5, #101                 @ A7.7.1
	mul	r10, r9, r3                   @ A7.7.84
	ldr	r1, [r9, r2, LSL #2]          @ A7.7.45
	itee	pl
	ldrdpl	r1, r9, [r0, #208]!           @ A7.7.50
	sdivmi	r9, r9, r7                    @ A7.7.127
	cmnmi	r6, #180                      @ A7.7.25
	umlal	r2, r9, r4, r0                @ A7.7.203
	stm	r13!, {r0,r3,r5-r7}           @ A7.7.159
mov	r0, #3093
	stm	r13!, {r0,r3-r4,r7-r8,r10}    @ A7.7.159
	asr	r2, r9, #13                   @ A7.7.10
	addw	r10, r3, #277                 @ A7.7.3
	umull	r10, r1, r4, r9               @ A7.7.204
	str	r9, [r3, r0]                  @ A7.7.162
	ldrh	r9, cell_2368                 @ A7.7.56
	ldrsh	r0, [r13]                     @ A7.7.63
	adds	r9, r4, #140                  @ A7.7.3
	lsrs	r3, r6, r4                    @ A7.7.71
	tst	r6, #141                      @ A7.7.188
	add	r9, #156                      @ A7.7.3
end_func_46:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_2376
cell_2376:	.word	safeSpaceGpramSram+676

.space	0, 45
.global	cell_2375
cell_2375:	.word	safeSpaceSram+144

.space	3, 45
.global	cell_2372
cell_2372:	.word	safeSpaceSram-476873

.space	1, 45
.global	cell_2377
cell_2377:	.word	safeSpaceFlash+432

.space	0, 45
.global	cell_2368
cell_2368:	.short	0x9760

.space	3, 45
.global	cell_2370
cell_2370:	.short	0x8a85

.space	0, 45
.global	cell_2369
cell_2369:	.word	safeSpaceSram-2963

.space	0, 45
.global	cell_2373
cell_2373:	.short	0x5ecc

.space	1, 45
.global	cell_2374
cell_2374:	.word	safeSpaceSram+420

.space	0, 45
.global	cell_2378
cell_2378:	.word	safeSpaceGpramSram-238368

.space	2, 45
.global	cell_2371
cell_2371:	.word	safeSpaceSram-21075

.align	1
.space 1358 % {{space_mod|default("0x10000000")}}
label405:
	sub	r13, #8
ldr	r1, cell_2434
ldr	r2, cell_2433
mov	r10, #37537
	asr	r9, r4                        @ A7.7.11
	pop	{r3,r6}                       @ A7.7.99
	ldr	r5, [r2, r10, LSL #2]         @ A7.7.45
	ldm	r1, {r0-r1,r4-r8,r10}         @ A7.7.41
	cmn	r1, r9, LSL #2                @ A7.7.26
	isb	                              @ A7.7.37
	ldrsh	r9, [r13, #17]                @ A7.7.63
	ldr	r9, cell_2432                 @ A7.7.44
end_label405:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2434
cell_2434:	.word	safeSpaceSram+768

.space	0, 45
.global	cell_2432
cell_2432:	.word	0x1ed7cba2

.space	3, 45
.global	cell_2433
cell_2433:	.word	safeSpaceSram-149921

.align	1
label406:
	sub	r13, #64
ldr	r3, cell_2437
ldr	r14, =post_branch_500
ldr	r0, =func_46
	ldr	r4, [r3], #183                @ A7.7.43
	pop	{r1-r2,r5-r8}                 @ A7.7.99
	add	r9, r13, #54                  @ A7.7.3
	mrs	r6, apsr                      @ A7.7.82
	tst	r2, #179                      @ A7.7.188
	cmn	r7, r10, LSL #3               @ A7.7.26
orr	r14, #1
	pop	{r7-r8}                       @ A7.7.99
orr	r0, #1
	ldrsh	r7, cell_2436                 @ A7.7.64
	bics	r5, r7, r6                    @ A7.7.16
	isb	                              @ A7.7.37
	stm	r13!, {r0,r2,r4-r9}           @ A7.7.159
	mov	r15, r0                       @ A7.7.77
post_branch_500:


mov	r5, #53
	isb	                              @ A7.7.37
	strh	r5, [r13, r5]                 @ A7.7.171
	.align	2
	ldrd	r5, r3, cell_2435             @ A7.7.51
	cmp	r9, r7                        @ A7.7.28
	smlal	r6, r5, r6, r8                @ A7.7.138
	add	r0, r6, r0                    @ A7.7.4
end_label406:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_2437
cell_2437:	.word	safeSpaceGpramSram+638

.align	2
.global	cell_2435
cell_2435:	.quad	0x1ab9f950d193e525

.space	2, 45
.global	cell_2436
cell_2436:	.short	0x0e19

.align	1
.space 606 % {{space_mod|default("0x10000000")}}
label412:
ldr	r10, cell_2467
mov	r9, #18885
ldr	r7, cell_2466
	sdiv	r3, r10                       @ A7.7.127
	strd	r2, r5, [r7], #-196           @ A7.7.166
	ldrsh	r3, cell_2465                 @ A7.7.64
mov	r6, #1612
	strb	r9, [r10, r9]                 @ A7.7.164
ldr	r4, cell_2464
	ldrsh	r10, [r4, r6, LSL #2]         @ A7.7.65
	mla	r9, r9, r7, r8                @ A7.7.74
	bic	r4, r9, r2, LSL #1            @ A7.7.16
	mls	r4, r3, r9, r6                @ A7.7.75
	bic	r10, r2, #129                 @ A7.7.15
	ldrb	r10, cell_2463                @ A7.7.47
mov	r10, #52
	ldrsb	r0, [r13, r10]                @ A7.7.61
end_label412:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_2467
cell_2467:	.word	safeSpaceSram-18671

.space	2, 45
.global	cell_2463
cell_2463:	.byte	0x70

.space	0, 45
.global	cell_2466
cell_2466:	.word	safeSpaceSram+688

.space	2, 45
.global	cell_2465
cell_2465:	.short	0xcd59

.space	1, 45
.global	cell_2464
cell_2464:	.word	safeSpaceSram-5543

.align	1
.space 2148 % {{space_mod|default("0x10000000")}}
func_52:
	sub	r13, #24
ldr	r9, cell_2565
ldr	r2, =forward_label_766
orr	r2, #1
	stm	r9, {r0-r6,r8-r10}            @ A7.7.159
	udiv	r0, r8                        @ A7.7.195
ldr	r9, cell_2564
	stmdb	r13, {r1,r3-r5,r7-r8,r10}     @ A7.7.160
	bx	r2                            @ A7.7.20

	smull	r2, r3, r3, r2                @ A7.7.149
	stm	r9, {r2,r4,r8}                @ A7.7.159
	smlal	r1, r9, r0, r2                @ A7.7.138
	cmp	r10, #60                      @ A7.7.27
	ldrh	r10, [r13]                    @ A7.7.55
	tst	r4, r9                        @ A7.7.189
	mrs	r2, apsr                      @ A7.7.82

forward_label_766:
	strd	r10, r9, [r13]                @ A7.7.166
	cmp	r10, #178                     @ A7.7.27
ldr	r1, cell_2563
	ldrsh	r0, [r1, #21]                 @ A7.7.63
	bfi	r2, r3, #6, #26               @ A7.7.14
ldr	r9, cell_2562
	ittee	gt
	tstgt	r9, r2                        @ A7.7.189
	asrgt	r3, r1                        @ A7.7.11
ldrle	r2, cell_2561
	ldrble	r1, [r2]                      @ A7.7.46
ldr	r1, cell_2560
	ldrh	r2, [r13], #24                @ A7.7.55
	ldrb	r3, [r1, #76]!                @ A7.7.46
mov	r2, #0
	cbz	r4, forward_label_765         @ A7.7.21

	ldrb	r0, [r9, #1]                  @ A7.7.46
	umull	r1, r9, r8, r10               @ A7.7.204
	ldr	r9, [r13, #-58]               @ A7.7.43
	movw	r1, #59022                    @ A7.7.76
	ldrsb	r10, [r13, r2, LSL #1]        @ A7.7.61
	cmn	r8, r3, LSL #2                @ A7.7.26
	adc	r9, r8, r9, LSL #3            @ A7.7.2
mov	r3, #19083
	bics	r1, r5, r3, LSL #3            @ A7.7.16
ldr	r1, cell_2559
	ldrsb	r2, [r1, r3, LSL #2]          @ A7.7.61
	lsrs	r10, r4, r1                   @ A7.7.71

forward_label_765:
	mul	r9, r8                        @ A7.7.84
mov	r10, #5
	ldrh	r9, [r13, r10, LSL #3]        @ A7.7.57
end_func_52:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_2563
cell_2563:	.word	safeSpaceSram+330

.space	0, 45
.global	cell_2562
cell_2562:	.word	safeSpaceGpramSram+179

.space	3, 45
.global	cell_2564
cell_2564:	.word	safeSpaceSram+612

.space	1, 45
.global	cell_2565
cell_2565:	.word	safeSpaceSram+312

.space	1, 45
.global	cell_2559
cell_2559:	.word	safeSpaceFlash-76055

.space	2, 45
.global	cell_2561
cell_2561:	.word	safeSpaceFlash+275

.space	0, 45
.global	cell_2560
cell_2560:	.word	safeSpaceSram+134

.align	1
.space 188 % {{space_mod|default("0x10000000")}}
func_53:
.space 2
ldr	r10, =forward_label_769  @ 4b
ldr	r1, cell_2589  @ 2b
mov	r2, #37  @ 4b
ldr	r3, cell_2588  @ 2b
ldr	r9, cell_2587  @ 4b
.space 4
.space 4
ldr	r2, cell_2586  @ 2b
	tst	r3, r10                       @ A7.7.189  @ 4b
	ldm	r9, {r0,r9}                   @ A7.7.41  @ 4b
ldr	r0, cell_2585  @ 4b
	cmn	r8, r8, LSL #3                @ A7.7.26  @ 4b
	nop	                              @ A7.7.88  @ 2b
	and	r9, r7                        @ A7.7.9  @ 4b
	nop	                              @ A7.7.88  @ 2b
.space 4
.space 4
.space 4
mov	r9, #17911  @ 4b
	stm	r1!, {r6,r9-r10}              @ A7.7.159  @ 4b
	strb	r5, [r0, #-122]!              @ A7.7.163  @ 4b
.space 4
	and	r0, #214                      @ A7.7.8  @ 4b
	and	r0, r8                        @ A7.7.9  @ 4b
orr	r10, #1  @ 4b
	ldrb	r1, cell_2574                 @ A7.7.47  @ 4b
	ldm	r3!, {r0-r1}                  @ A7.7.41  @ 2b
	ldr	r0, [r2, r9, LSL #2]          @ A7.7.45  @ 4b
.space 4
	bx	r10                           @ A7.7.20  @ 2b

mov	r10, #45  @ 4b
	mov	r3, r10, LSL #1               @ A7.7.78  @ 4b
ldr	r1, cell_2573  @ 2b
.space 2
	ldrb	r2, [r1, #-246]               @ A7.7.46  @ 4b
.space 4

forward_label_769:
.space 4
.space 4
.space 4
mov	r2, #8  @ 4b
.space 4
.space 4
.space 4
.space 2
mov	r3, #8  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
end_func_53:
	bx	r14

.ltorg
.align	2
.space 24 % {{space_mod|default("0x10000000")}}
.global	cell_2589
cell_2589:	.word	safeSpaceSram+204

.align	2
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_2573
cell_2573:	.word	safeSpaceSram+1199

.align	2
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_2586
cell_2586:	.word	safeSpaceGpramSram-71132

.space	1, 45
.global	cell_2574
cell_2574:	.byte	0x03

.align	2
.space 27 % {{space_mod|default("0x10000000")}}
.global	cell_2587
cell_2587:	.word	safeSpaceFlash+128

.align	2
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_2588
cell_2588:	.word	safeSpaceSram+200

.space	2, 45
.global	cell_2585
cell_2585:	.word	safeSpaceSram+821

.align	2
.space 162 % {{space_mod|default("0x10000000")}}
label427:
ldr	r1, cell_2605
ldr	r3, cell_2604
ldr	r2, =forward_label_773
orr	r2, #1
ldr	r8, cell_2603
	blx	r2                            @ A7.7.19

	addw	r5, r13, #2489                @ A7.7.5
	cmn	r0, r0, LSL #1                @ A7.7.26
	umull	r10, r9, r10, r3              @ A7.7.204
	.align	2
	ldrd	r7, r0, cell_2602             @ A7.7.51
	ldrh	r10, [r13]                    @ A7.7.55
	mla	r9, r3, r7, r2                @ A7.7.74
	ldrsb	r5, cell_2601                 @ A7.7.60
	msr	apsr_nzcvq, r8                @ A7.7.83
	umlal	r4, r7, r7, r4                @ A7.7.203
	add	r10, r15                      @ A7.7.4
	bics	r7, #29                       @ A7.7.15
	nop	                              @ A7.7.88
	smlal	r5, r0, r7, r1                @ A7.7.138
	addw	r0, r5, #4095                 @ A7.7.3

forward_label_773:
	lsrs	r9, r10, r4                   @ A7.7.71
	smlal	r9, r7, r5, r7                @ A7.7.138
	stm	r8!, {r1,r3,r5-r6}            @ A7.7.159
mov	r5, #0
	ldrh	r8, cell_2600                 @ A7.7.56
	mov	r4, #169                      @ A7.7.76
ldr	r2, cell_2599
ldr	r9, cell_2598
	ldrsb	r7, [r13, r5]                 @ A7.7.61
	add	r5, r15                       @ A7.7.4
	adds	r7, r13, #12                  @ A7.7.5
	ldrsh	r5, [r9]                      @ A7.7.63
	ldm	r1!, {r6,r8}                  @ A7.7.41
	bfc	r8, #1, #27                   @ A7.7.13
	ldr	r5, [r2, #203]                @ A7.7.43
	strb	r8, [r3, #17]                 @ A7.7.163
	nop	                              @ A7.7.88
	ite	mi
	mlsmi	r10, r7, r4, r0               @ A7.7.75
	cmnpl	r8, #227                      @ A7.7.25
	smlal	r9, r1, r2, r5                @ A7.7.138
	tst	r7, r5                        @ A7.7.189
	cmn	r9, r5, LSL #1                @ A7.7.26
ldr	r1, cell_2597
	strd	r8, r10, [r1], #-208          @ A7.7.166
end_label427:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2601
cell_2601:	.byte	0xec

.space	3, 45
.global	cell_2599
cell_2599:	.word	safeSpaceSram+330

.space	0, 45
.global	cell_2604
cell_2604:	.word	safeSpaceSram+544

.space	1, 45
.global	cell_2603
cell_2603:	.word	safeSpaceGpramSram+504

.space	0, 45
.global	cell_2598
cell_2598:	.word	safeSpaceFlash+885

.space	1, 45
.global	cell_2605
cell_2605:	.word	safeSpaceGpramSram+576

.space	3, 45
.global	cell_2600
cell_2600:	.short	0x0871

.space	1, 45
.global	cell_2597
cell_2597:	.word	safeSpaceSram+660

.align	2
.global	cell_2602
cell_2602:	.quad	0x159f5ecc6839fd00

.align	1
.space 248 % {{space_mod|default("0x10000000")}}
func_54:
	tst	r0, r4, LSL #1                @ A7.7.189
	movt	r1, #32468                    @ A7.7.79
	ldrsb	r3, cell_2618                 @ A7.7.60
	cbz	r1, forward_label_778         @ A7.7.21

	strd	r6, r3, [r13]                 @ A7.7.166
	umull	r0, r1, r2, r0                @ A7.7.204
	cmn	r8, #229                      @ A7.7.25
	bics	r0, #243                      @ A7.7.15
	ands	r1, #108                      @ A7.7.8
	bic	r3, r1                        @ A7.7.16

forward_label_778:
	tst	r10, #4                       @ A7.7.188
	lsrs	r3, r10, r0                   @ A7.7.71
	adcs	r2, r3, r8                    @ A7.7.2
	asrs	r1, r6, #16                   @ A7.7.10
mov	r2, #19
	ite	cs
	asrscs	r0, r6, #9                    @ A7.7.10
	stmdbcc	r13, {r0,r2,r5,r7,r9}         @ A7.7.160
ldr	r0, cell_2617
	smlal	r10, r1, r9, r6               @ A7.7.138
ldr	r3, cell_2616
	adds	r9, r13, r9, LSL #3           @ A7.7.6
	msr	apsr_nzcvq, r6                @ A7.7.83
	ldr	r1, [r0, r2, LSL #1]          @ A7.7.45
	ldrb	r1, [r3, #3706]               @ A7.7.46
	strb	r6, [r13]                     @ A7.7.163
	ldrsh	r3, [r13, r2]                 @ A7.7.65
	mls	r10, r2, r10, r10             @ A7.7.75
	ldrh	r1, [r0, #-228]!              @ A7.7.55
	msr	apsr_nzcvq, r6                @ A7.7.83
	.align	2
	ldrd	r1, r9, cell_2615             @ A7.7.51
end_func_54:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_2617
cell_2617:	.word	safeSpaceFlash+879

.space	1, 45
.global	cell_2618
cell_2618:	.byte	0x0f

.align	2
.global	cell_2615
cell_2615:	.quad	0x415930129ff4afeb

.space	3, 45
.global	cell_2616
cell_2616:	.word	safeSpaceFlash-3235

.align	1
func_55:
	sub	r13, #252
mov	r0, #208
mov	r1, #7
ldr	r10, cell_2624
mov	r2, #42
mov	r3, #1570
ldr	r9, cell_2623
	strd	r8, r10, [r9, #76]            @ A7.7.166
ldr	r9, cell_2622
	str	r10, [r10, r3, LSL #2]        @ A7.7.162
	addw	r10, r13, #273                @ A7.7.5
	str	r7, [r9, #-220]               @ A7.7.161
	bic	r3, #132                      @ A7.7.15
	strh	r0, [r13]                     @ A7.7.170
	movw	r3, #36400                    @ A7.7.76
	umull	r10, r3, r8, r4               @ A7.7.204
	bhi	forward_label_780             @ A7.7.12

	adc	r3, r2, #130                  @ A7.7.1

forward_label_780:
	ldrd	r10, r3, [r9]                 @ A7.7.50
	movs	r10, r4                       @ A7.7.77
	ldrh	r3, [r13, r1, LSL #1]         @ A7.7.57
	isb	                              @ A7.7.37
	adds	r9, r1, r3, LSL #1            @ A7.7.4
	pop	{r1,r3,r10}                   @ A7.7.99
	strh	r0, [r13, r2]                 @ A7.7.171
mov	r3, #44491
	adds	r13, r0                       @ A7.7.6
	mov	r10, #101                     @ A7.7.76
	and	r10, r2, #136                 @ A7.7.8
	umlal	r10, r2, r7, r3               @ A7.7.203
	adds	r2, #79                       @ A7.7.1
	nop	                              @ A7.7.88
	ldrb	r2, [r13, #16]                @ A7.7.46
	stm	r13, {r0-r1,r3,r6,r10}        @ A7.7.159
ldr	r2, cell_2621
	itte	cc
ldrcc	r0, cell_2620
	strcc	r3, [r0, r3]                  @ A7.7.162
	cmncs	r0, r1, LSL #1                @ A7.7.26
	strb	r1, [r2]                      @ A7.7.163
mov	r1, #2
	ldrh	r2, [r13, r1, LSL #3]         @ A7.7.57
	cmp	r14, #138                     @ A7.7.27
	and	r9, r1, r9                    @ A7.7.9
	ldrsb	r1, cell_2619                 @ A7.7.60
	ldrsh	r0, [r13, #-35]               @ A7.7.63
	str	r9, [r13, #32]!               @ A7.7.161
	mov	r9, r15                       @ A7.7.77
end_func_55:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_2622
cell_2622:	.word	safeSpaceGpramSram+876

.space	2, 45
.global	cell_2624
cell_2624:	.word	safeSpaceGpramSram-5618

.space	0, 45
.global	cell_2619
cell_2619:	.byte	0x34

.space	1, 45
.global	cell_2621
cell_2621:	.word	safeSpaceSram+752

.space	2, 45
.global	cell_2623
cell_2623:	.word	safeSpaceGpramSram+556

.space	2, 45
.global	cell_2620
cell_2620:	.word	safeSpaceGpramSram-43555

.align	1
.space 530 % {{space_mod|default("0x10000000")}}
label433:
mov	r1, #8
mov	r10, #1
ldr	r2, cell_2650
ldr	r7, cell_2649
ldr	r3, =table_74
	tbh	[r3, r10, LSL #1]             @ A7.7.185
label433_switch_1_case_1:
	umlal	r3, r5, r6, r0                @ A7.7.203
label433_switch_1_case_2:
ldr	r4, cell_2648
	ldm	r4!, {r0,r6,r10}              @ A7.7.41
	add	r3, #64                       @ A7.7.3
	ldrd	r10, r4, [r13]                @ A7.7.50
	cmp	r6, #146                      @ A7.7.27
	tst	r0, r2                        @ A7.7.189
label433_switch_1_case_3:
	strb	r2, [r13, r1, LSL #3]         @ A7.7.164
	stm	r7, {r0-r10}                  @ A7.7.159
	stmdb	r13, {r0,r5-r9}               @ A7.7.160
label433_switch_1_case_4:
	adr	r0, cell_2647                 @ A7.7.7
	and	r0, #74                       @ A7.7.8
	movt	r5, #21752                    @ A7.7.79
label433_switch_1_case_5:
	msr	apsr_nzcvq, r2                @ A7.7.83
	ldrh	r0, [r13, #-59]               @ A7.7.55
	strb	r0, [r2], #-196               @ A7.7.163
end_label433:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 46
.global	table_74
table_74:
.hword	0
.hword	((label433_switch_1_case_2-label433_switch_1_case_1)/2)
.hword	((label433_switch_1_case_3-label433_switch_1_case_1)/2)
.hword	((label433_switch_1_case_4-label433_switch_1_case_1)/2)
.hword	((label433_switch_1_case_5-label433_switch_1_case_1)/2)

.space	3, 45
.global	cell_2650
cell_2650:	.word	safeSpaceSram+474

.space	3, 45
.global	cell_2647
cell_2647:	.byte	0x0d

.space	1, 45
.global	cell_2649
cell_2649:	.word	safeSpaceGpramSram+644

.space	3, 45
.global	cell_2648
cell_2648:	.word	safeSpaceGpramSram+388

.align	1
.space 244 % {{space_mod|default("0x10000000")}}
label436:
	sub	r13, #124
ldr	r0, cell_2668
ldr	r4, cell_2667
mov	r2, #31091
	cbz	r0, forward_label_793         @ A7.7.21

	adds	r9, r4, #15                   @ A7.7.3
	mul	r6, r7                        @ A7.7.84
	ldrb	r9, cell_2666                 @ A7.7.47
	cmp	r9, #168                      @ A7.7.27
	add	r8, r13, r0                   @ A7.7.4
	add	r9, r13, r8                   @ A7.7.4
	msr	apsr_nzcvq, r4                @ A7.7.83
	ldrsh	r9, [r4]                      @ A7.7.63
	mla	r3, r9, r6, r9                @ A7.7.74
ldr	r9, cell_2665
	ldrh	r6, [r9, r2]                  @ A7.7.57

forward_label_793:
	adr	r3, cell_2664                 @ A7.7.7
ldr	r3, cell_2663
	str	r9, [r13], #124               @ A7.7.161
	cmp	r4, r6, LSL #2                @ A7.7.28
	strb	r2, [r3], #-148               @ A7.7.163
mov	r3, #18315
ldr	r9, cell_2662
	mls	r8, r0, r0, r3                @ A7.7.75
	strd	r0, r3, [r9, #208]            @ A7.7.166
	strh	r9, [r0, r3]                  @ A7.7.171
	isb	                              @ A7.7.37
	ldrd	r3, r9, [r13]                 @ A7.7.50
	tst	r9, r9, LSL #1                @ A7.7.189
	adc	r3, r9, #84                   @ A7.7.1
	stm	r13, {r0-r2,r4-r10}           @ A7.7.159
end_label436:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_2664
cell_2664:	.byte	0x1f

.space	3, 45
.global	cell_2666
cell_2666:	.byte	0x56

.space	2, 45
.global	cell_2662
cell_2662:	.word	safeSpaceSram+624

.space	0, 45
.global	cell_2667
cell_2667:	.word	safeSpaceSram+795

.space	2, 45
.global	cell_2668
cell_2668:	.word	safeSpaceSram-17850

.space	0, 45
.global	cell_2665
cell_2665:	.word	safeSpaceGpramSram-30508

.space	2, 45
.global	cell_2663
cell_2663:	.word	safeSpaceGpramSram+787

.align	1
.space 142 % {{space_mod|default("0x10000000")}}
label439:
ldr	r7, =forward_label_799  @ 2b
orr	r7, #1  @ 4b
ldr	r9, cell_2678  @ 4b
ldr	r6, cell_2677  @ 4b
mov	r2, #21785  @ 4b
	bx	r7                            @ A7.7.20  @ 2b

	ldrb	r3, [r6, #30]                 @ A7.7.46  @ 2b
	umlal	r0, r7, r6, r4                @ A7.7.203  @ 4b
mov	r7, #34  @ 4b
	smlal	r1, r6, r4, r2                @ A7.7.138  @ 4b
	mul	r4, r2                        @ A7.7.84  @ 4b
.space 4
	and	r1, r6, r2                    @ A7.7.9  @ 4b

forward_label_799:
.space 4
.space 4
.space 2
	ldrb	r1, cell_2676                 @ A7.7.47  @ 4b
.space 4
	tst	r2, r2                        @ A7.7.189  @ 2b
	ldrb	r6, [r9, r2]                  @ A7.7.48  @ 4b
end_label439:
	b.w	{{jump_label439}}

.ltorg
.align	2
.space	3, 45
.global	cell_2677
cell_2677:	.word	safeSpaceSram+664

.space	2, 45
.global	cell_2678
cell_2678:	.word	safeSpaceGpramSram-21618

.space	3, 45
.global	cell_2676
cell_2676:	.byte	0x21

.align	1
.space 404 % {{space_mod|default("0x10000000")}}
func_58:
	sub	r13, #144
mov	r1, #112
mov	r10, #4
ldr	r2, cell_2709
mov	r9, #33
	ldr	r3, cell_2708                 @ A7.7.44
	mla	r3, r5, r7, r6                @ A7.7.74
	adr	r0, cell_2707                 @ A7.7.7
	adds	r0, r13, r9                   @ A7.7.6
mov	r3, #16
	add	r13, r1                       @ A7.7.6
	mrs	r1, apsr                      @ A7.7.82
	cbnz	r6, forward_label_803         @ A7.7.21

	asr	r0, r1                        @ A7.7.11
	smlal	r1, r0, r5, r5                @ A7.7.138
ldr	r1, cell_2706
	ldrsh	r0, [r1]                      @ A7.7.63
	cmp	r14, r7, LSL #1               @ A7.7.28
	mrs	r1, apsr                      @ A7.7.82
	ldrsb	r1, cell_2705                 @ A7.7.60

forward_label_803:
	ldrb	r1, [r13, r10, LSL #2]        @ A7.7.48
	ldr	r10, cell_2704                @ A7.7.44
	mrs	r10, apsr                     @ A7.7.82
mov	r0, #6
	smlal	r1, r10, r8, r1               @ A7.7.138
	strd	r3, r10, [r2, #84]!           @ A7.7.166
	strh	r7, [r13, r0, LSL #3]         @ A7.7.171
	pop	{r0,r2,r10}                   @ A7.7.99
	bics	r10, r10, r5                  @ A7.7.16
	ldrsh	r0, [r13, r9]                 @ A7.7.65
	udiv	r0, r10                       @ A7.7.195
	mrs	r9, apsr                      @ A7.7.82
mov	r1, #9846
ldr	r10, cell_2703
	ldrsh	r2, [r10, r1]                 @ A7.7.65
	ldr	r1, [r13, r3]                 @ A7.7.45
	ldrh	r1, cell_2702                 @ A7.7.56
	ldrh	r10, [r13], #20               @ A7.7.55
	smlal	r10, r3, r5, r3               @ A7.7.138
ldr	r2, cell_2701
	strb	r4, [r2]                      @ A7.7.163
	bfc	r10, #7, #4                   @ A7.7.13
	tst	r2, #1                        @ A7.7.188
	strh	r7, [r13, #-31]               @ A7.7.170
	tst	r7, r3, LSL #1                @ A7.7.189
	add	r1, r13, r1                   @ A7.7.6
	asr	r10, r1                       @ A7.7.11
	ldrh	r10, cell_2700                @ A7.7.56
ldr	r1, cell_2699
	ldrsh	r10, [r1], #24                @ A7.7.63
	asr	r10, r8, #13                  @ A7.7.10
	mov	r3, r15                       @ A7.7.77
	bics	r1, r4, r10, LSL #3           @ A7.7.16
end_func_58:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_2706
cell_2706:	.word	safeSpaceFlash+651

.space	1, 45
.global	cell_2705
cell_2705:	.byte	0x04

.space	2, 45
.global	cell_2701
cell_2701:	.word	safeSpaceSram+739

.space	2, 45
.global	cell_2704
cell_2704:	.word	0xccca363a

.space	0, 45
.global	cell_2708
cell_2708:	.word	0xe4c0a22f

.space	2, 45
.global	cell_2700
cell_2700:	.short	0x5a73

.space	3, 45
.global	cell_2703
cell_2703:	.word	safeSpaceGpramSram-9191

.space	3, 45
.global	cell_2702
cell_2702:	.short	0x498a

.space	0, 45
.global	cell_2699
cell_2699:	.word	safeSpaceSram+860

.space	1, 45
.global	cell_2707
cell_2707:	.byte	0xe6

.space	2, 45
.global	cell_2709
cell_2709:	.word	safeSpaceGpramSram-4

.align	1
label441:
	sub	r13, #48
mov	r4, #29
ldr	r10, =table_79
ldr	r9, cell_2713
	cmn	r13, #138                     @ A7.7.25
	itte	cs
ldrcs	r7, cell_2712
	ldrsbcs	r1, [r7, #-130]               @ A7.7.59
	tstcc	r7, #208                      @ A7.7.188
	strb	r3, [r13, #-160]!             @ A7.7.163
	ldrb	r0, cell_2711                 @ A7.7.47
mov	r6, #0
	tbb	[r10, r6]                     @ A7.7.185
label441_switch_1_case_1:
	ldr	r2, [r13, r4]                 @ A7.7.45
label441_switch_1_case_2:
	bics	r3, r2, #65                   @ A7.7.15
label441_switch_1_case_3:
mov	r10, #31
	tst	r10, #239                     @ A7.7.188
	ldr	r1, [r13, r10]                @ A7.7.45
	ldr	r0, cell_2710                 @ A7.7.44
label441_switch_1_case_4:
	sdiv	r10, r0, r7                   @ A7.7.127
label441_switch_1_case_5:
	ldm	r9!, {r0-r1,r3,r5,r7-r8,r10}  @ A7.7.41
label441_switch_1_case_6:
	add	r6, r13, r10, LSL #2          @ A7.7.6
	ldrh	r0, [r13, #208]!              @ A7.7.55
	ldrd	r7, r9, [r13]                 @ A7.7.50
	cmp	r3, #230                      @ A7.7.27
	isb	                              @ A7.7.37
	bfc	r4, #1, #31                   @ A7.7.13
end_label441:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_2711
cell_2711:	.byte	0x1c

.space	0, 45
.global	cell_2712
cell_2712:	.word	safeSpaceGpramSram+838

.space	2, 45
.global	cell_2713
cell_2713:	.word	safeSpaceFlash+344

.space	2, 45
.global	cell_2710
cell_2710:	.word	0x35608329

.align	1
.space 680 % {{space_mod|default("0x10000000")}}
label446:
ldr	r2, =forward_label_814
orr	r2, #1
mov	r7, #54553
ldr	r10, cell_2745
ldr	r3, cell_2744
ldr	r5, cell_2743
	bx	r2                            @ A7.7.20

	and	r9, r2, r7, LSL #2            @ A7.7.9
	ldrd	r1, r6, [r5, #200]!           @ A7.7.50
	mov	r6, r15                       @ A7.7.77
	mrs	r2, apsr                      @ A7.7.82
	ldrh	r6, [r13]                     @ A7.7.55
	mov	r2, r15                       @ A7.7.77
	ldrh	r9, cell_2742                 @ A7.7.56
	bfc	r8, #16, #5                   @ A7.7.13
ldr	r8, cell_2741
	str	r0, [r8, #81]                 @ A7.7.161
ldr	r6, cell_2740
	asr	r0, r3, r0                    @ A7.7.11
	strb	r10, [r6, #1623]              @ A7.7.163

forward_label_814:
	ite	ls
	ldrls	r9, [r3, r7]                  @ A7.7.45
	tsthi	r3, r0                        @ A7.7.189
	sdiv	r5, r1                        @ A7.7.127
	cmp	r1, r0                        @ A7.7.28
ldr	r3, cell_2739
	cmp	r1, #61                       @ A7.7.27
	strh	r1, [r3, #77]!                @ A7.7.170
	ldrb	r8, cell_2738                 @ A7.7.47
mov	r6, #64077
	smlal	r0, r8, r9, r9                @ A7.7.138
	ldr	r2, cell_2737                 @ A7.7.44
	ldrsh	r3, cell_2736                 @ A7.7.64
	strb	r8, [r10, r6, LSL #1]         @ A7.7.164
	mul	r3, r0                        @ A7.7.84
	ldr	r9, cell_2735                 @ A7.7.44
	adds	r0, #85                       @ A7.7.1
	adc	r8, r0, r8                    @ A7.7.2
end_label446:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_2743
cell_2743:	.word	safeSpaceFlash-12

.space	2, 45
.global	cell_2744
cell_2744:	.word	safeSpaceSram-53846

.space	2, 45
.global	cell_2739
cell_2739:	.word	safeSpaceSram+796

.space	3, 45
.global	cell_2741
cell_2741:	.word	safeSpaceSram+2

.space	1, 45
.global	cell_2735
cell_2735:	.word	0xa9019a8f

.space	3, 45
.global	cell_2740
cell_2740:	.word	safeSpaceSram-1507

.space	3, 45
.global	cell_2736
cell_2736:	.short	0x0579

.space	3, 45
.global	cell_2737
cell_2737:	.word	0xfa181e95

.space	2, 45
.global	cell_2742
cell_2742:	.short	0x6da8

.space	1, 45
.global	cell_2738
cell_2738:	.byte	0xb9

.space	3, 45
.global	cell_2745
cell_2745:	.word	safeSpaceSram-128068

.align	1
.space 1182 % {{space_mod|default("0x10000000")}}
label457:
	sub	r13, #112
ldr	r10, cell_2798
	asrs	r7, r8, #3                    @ A7.7.10
	umlal	r3, r9, r2, r10               @ A7.7.203
	adds	r6, r13, r3                   @ A7.7.6
mov	r8, #50
mov	r5, #96
	udiv	r6, r7, r10                   @ A7.7.195
	cmp	r2, r10, LSL #2               @ A7.7.28
	pop	{r0,r2-r3,r9}                 @ A7.7.99
	ldrsh	r6, [r13, r8]                 @ A7.7.65
ldr	r7, cell_2797
	cmp	r7, r10                       @ A7.7.28
	add	r13, r13, r5                  @ A7.7.6
	bic	r6, r8                        @ A7.7.16
	str	r2, [r13]                     @ A7.7.161
	smull	r0, r5, r7, r10               @ A7.7.149
	ldr	r8, [r7, #84]                 @ A7.7.43
	ldmdb	r10, {r0-r3,r5-r6,r8-r9}      @ A7.7.42
ldr	r5, cell_2796
	ldrsb	r10, [r5, #-241]              @ A7.7.59
end_label457:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_2798
cell_2798:	.word	safeSpaceSram+936

.space	0, 45
.global	cell_2797
cell_2797:	.word	safeSpaceSram+762

.space	1, 45
.global	cell_2796
cell_2796:	.word	safeSpaceGpramSram+332

.align	1
.space 692 % {{space_mod|default("0x10000000")}}
label463:
	sub	r13, #56
ldr	r7, cell_2835
mov	r9, #41976
ldr	r8, cell_2834
ldr	r10, cell_2833
mov	r0, #48
	ldrsh	r6, [r13, r0]                 @ A7.7.65
	smull	r0, r2, r7, r3                @ A7.7.149
	strb	r1, [r7]                      @ A7.7.163
	ldrsh	r5, cell_2832                 @ A7.7.64
	ldr	r5, [r13], #56                @ A7.7.43
	ldrsb	r7, [r10, r9, LSL #3]         @ A7.7.61
	stm	r8!, {r1,r3-r7,r9-r10}        @ A7.7.159
ldr	r1, cell_2831
	ldrh	r6, cell_2830                 @ A7.7.56
	ldrd	r8, r6, [r13, #12]            @ A7.7.50
	ldmdb	r13, {r0,r2,r4-r8,r10}        @ A7.7.42
	mov	r0, r13                       @ A7.7.77
	tst	r8, #27                       @ A7.7.188
	stmdb	r1!, {r2-r3,r5-r10}           @ A7.7.160
	ldrh	r0, cell_2829                 @ A7.7.56
ldr	r10, cell_2828
	ldr	r6, [r10, #-35]               @ A7.7.43
	cmp	r6, r6, LSL #3                @ A7.7.28
	cmn	r7, r10, LSL #2               @ A7.7.26
end_label463:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_2832
cell_2832:	.short	0xdb78

.space	1, 45
.global	cell_2833
cell_2833:	.word	safeSpaceSram-335094

.space	2, 45
.global	cell_2831
cell_2831:	.word	safeSpaceSram+752

.space	2, 45
.global	cell_2835
cell_2835:	.word	safeSpaceSram+375

.space	1, 45
.global	cell_2829
cell_2829:	.short	0xe4ba

.space	2, 45
.global	cell_2828
cell_2828:	.word	safeSpaceGpramSram+714

.space	3, 45
.global	cell_2830
cell_2830:	.short	0x52f0

.space	3, 45
.global	cell_2834
cell_2834:	.word	safeSpaceGpramSram+92

.align	1
.space 2612 % {{space_mod|default("0x10000000")}}
label486:
ldr	r0, =table_85
ldr	r6, cell_2945
mov	r4, #0
	tbb	[r0, r4]                      @ A7.7.185
label486_switch_1_case_1:
mov	r1, #2
	str	r0, [r13, r1]                 @ A7.7.162
mov	r4, #40
	ldrsh	r2, cell_2944                 @ A7.7.64
	ldrsh	r0, cell_2943                 @ A7.7.64
	ldrsh	r3, [r13, r4]                 @ A7.7.65
	ldrb	r0, [r6], #99                 @ A7.7.46
label486_switch_1_case_2:
	adds	r5, r13, #184                 @ A7.7.5
mov	r2, #3
	ldrsb	r3, [r13, r2, LSL #3]         @ A7.7.61
	ldrb	r6, [r13]                     @ A7.7.46
	lsrs	r0, r5, r3                    @ A7.7.71
	cmn	r14, r3, LSL #1               @ A7.7.26
label486_switch_1_case_3:
	umull	r1, r2, r2, r2                @ A7.7.204
	nop	                              @ A7.7.88
end_label486:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 46
.global	table_85
table_85:
.byte	0
.byte	((label486_switch_1_case_2-label486_switch_1_case_1)/2)
.byte	((label486_switch_1_case_3-label486_switch_1_case_1)/2)

.space	3, 45
.global	cell_2944
cell_2944:	.short	0x2a2b

.space	0, 45
.global	cell_2943
cell_2943:	.short	0xeef4

.space	2, 45
.global	cell_2945
cell_2945:	.word	safeSpaceFlash+195

.align	1
.space 3140 % {{space_mod|default("0x10000000")}}
label504:
	sub	r13, #64
ldr	r2, cell_3078
ldr	r3, cell_3077
mov	r5, #10028
ldr	r10, cell_3076
mov	r9, #64
mov	r8, #64266
mov	r7, #4104
ldr	r1, cell_3075
ldr	r0, cell_3074
ldr	r4, cell_3073
	ldr	r6, [r3, r5]                  @ A7.7.45
	str	r1, [r2, r8, LSL #1]          @ A7.7.162
	ldrsb	r3, [r10, #-7]!               @ A7.7.59
	ldrsb	r3, [r0, #131]                @ A7.7.59
ldr	r10, cell_3072
	.align	2
	ldrd	r6, r2, cell_3071             @ A7.7.51
	ldrsb	r6, [r4, #1995]               @ A7.7.59
	cmp	r13, r10, LSL #2              @ A7.7.28
ldr	r6, =func_13
	ldrh	r0, [r10, r5]                 @ A7.7.57
	adds	r13, r9                       @ A7.7.6
	cbz	r1, forward_label_916         @ A7.7.21

	umull	r4, r9, r9, r10               @ A7.7.204
	smlal	r3, r0, r4, r0                @ A7.7.138
	ldrsh	r10, [r1, r7, LSL #2]         @ A7.7.65
	isb	                              @ A7.7.37
orr	r6, #1
	add	r2, r13, r7, LSL #2           @ A7.7.6
	tst	r7, #240                      @ A7.7.188
	blx	r6                            @ A7.7.19


	mul	r7, r7, r8                    @ A7.7.84

forward_label_916:
ldr	r1, cell_3070
	str	r8, [r1, r8]                  @ A7.7.162
	tst	r10, r8                       @ A7.7.189
	movt	r1, #12564                    @ A7.7.79
end_label504:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_3078
cell_3078:	.word	safeSpaceSram-127994

.space	0, 45
.global	cell_3076
cell_3076:	.word	safeSpaceFlash+192

.space	2, 45
.global	cell_3074
cell_3074:	.word	safeSpaceGpramSram+322

.space	1, 45
.global	cell_3072
cell_3072:	.word	safeSpaceFlash-9573

.space	2, 45
.global	cell_3077
cell_3077:	.word	safeSpaceGpramSram-9265

.space	2, 45
.global	cell_3073
cell_3073:	.word	safeSpaceFlash-1911

.space	0, 45
.global	cell_3070
cell_3070:	.word	safeSpaceGpramSram-63755

.space	0, 45
.global	cell_3075
cell_3075:	.word	safeSpaceSram-16165

.align	2
.global	cell_3071
cell_3071:	.quad	0x73115f3980f0dbbd

.align	1
.space 778 % {{space_mod|default("0x10000000")}}
label512:
ldr	r7, cell_3119
mov	r5, #22520
ldr	r10, cell_3118
mov	r3, #65403
ldr	r4, =table_91
mov	r1, #0
	tbh	[r4, r1, LSL #1]              @ A7.7.185
label512_switch_1_case_1:
	smlal	r0, r4, r0, r4                @ A7.7.138
label512_switch_1_case_2:
ldr	r2, cell_3117
	stm	r2, {r0-r2,r4,r7-r10}         @ A7.7.159
ldr	r4, cell_3116
	ldr	r8, [r4, #2165]               @ A7.7.43
	mls	r0, r8, r4, r10               @ A7.7.75
label512_switch_1_case_3:
	ldrsb	r6, [r7, r5, LSL #1]          @ A7.7.61
ldr	r6, cell_3115
	stm	r10!, {r0-r1,r3-r4,r6-r7,r9}  @ A7.7.159
	adr	r0, cell_3114                 @ A7.7.7
	add	r5, r15                       @ A7.7.4
	ldrsb	r5, [r6, r3]                  @ A7.7.61
label512_switch_1_case_4:
	sdiv	r9, r1, r0                    @ A7.7.127
ldr	r1, cell_3113
ldr	r3, cell_3112
	ldrb	r6, [r3, #3982]               @ A7.7.46
	strb	r4, [r1, #2435]               @ A7.7.163
	add	r5, r15                       @ A7.7.4
	strh	r5, [r13, #-12]               @ A7.7.170
ldr	r5, cell_3111
	stmdb	r5, {r0-r1,r3,r6,r8}          @ A7.7.160
	nop	                              @ A7.7.88
label512_switch_1_case_5:
	cmn	r4, r5, LSL #3                @ A7.7.26
	ldrsb	r6, cell_3110                 @ A7.7.60
	adds	r7, r1, #204                  @ A7.7.3
	smlal	r4, r1, r1, r4                @ A7.7.138
	ldrsh	r5, [r13]                     @ A7.7.63
	add	r3, r6, #47                   @ A7.7.3
	ldrh	r7, cell_3109                 @ A7.7.56
end_label512:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.global	cell_3112
cell_3112:	.word	safeSpaceGpramSram-3524

.space	0, 45
.global	cell_3114
cell_3114:	.byte	0x75

.space	1, 45
.global	cell_3116
cell_3116:	.word	safeSpaceFlash-1993

.space	0, 45
.global	cell_3113
cell_3113:	.word	safeSpaceGpramSram-2066

.space	2, 45
.global	cell_3110
cell_3110:	.byte	0x80

.space	2, 45
.global	cell_3118
cell_3118:	.word	safeSpaceSram+612

.space	0, 45
.global	cell_3111
cell_3111:	.word	safeSpaceGpramSram+700

.space	0, 45
.global	cell_3109
cell_3109:	.short	0xa1d1

.space	3, 45
.global	cell_3119
cell_3119:	.word	safeSpaceSram-44229

.space	0, 45
.global	cell_3117
cell_3117:	.word	safeSpaceSram+400

.space	3, 45
.global	cell_3115
cell_3115:	.word	safeSpaceFlash-64771

.align	1
.space 1138 % {{space_mod|default("0x10000000")}}
label518:
ldr	r2, cell_3172
	nop	                              @ A7.7.88
	ldm	r2, {r0-r3,r5,r7,r9-r10}      @ A7.7.41
	ldrsh	r4, [r13]                     @ A7.7.63
	ldr	r7, cell_3171                 @ A7.7.44
	ldr	r4, cell_3170                 @ A7.7.44
	nop	                              @ A7.7.88
	ldrb	r1, cell_3169                 @ A7.7.47
	.align	2
	ldrd	r4, r2, cell_3168             @ A7.7.51
	movt	r2, #27441                    @ A7.7.79
mov	r3, #4
	ldrsh	r5, [r13, r3, LSL #2]         @ A7.7.65
	adds	r2, #88                       @ A7.7.1
	mrs	r4, apsr                      @ A7.7.82
	ands	r5, #164                      @ A7.7.8
	cmp	r5, #206                      @ A7.7.27
end_label518:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_3171
cell_3171:	.word	0xdec6df47

.space	0, 45
.global	cell_3172
cell_3172:	.word	safeSpaceSram+136

.align	2
.global	cell_3168
cell_3168:	.quad	0x4a95e081d7e7cdb4

.space	0, 45
.global	cell_3170
cell_3170:	.word	0x24c9f9da

.space	2, 45
.global	cell_3169
cell_3169:	.byte	0xc3

.align	1
.space 1018 % {{space_mod|default("0x10000000")}}
label522:
ldr	r10, cell_3246
ldr	r0, cell_3245
mov	r6, #28537
	sdiv	r3, r1, r0                    @ A7.7.127
	mrs	r7, apsr                      @ A7.7.82
	ldrsb	r8, [r10, r6, LSL #2]         @ A7.7.61
	adr	r4, cell_3244                 @ A7.7.7
	tst	r2, r6, LSL #1                @ A7.7.189
	ldrb	r10, cell_3243                @ A7.7.47
ldr	r8, cell_3242
	strd	r8, r4, [r13, #-44]           @ A7.7.166
mov	r4, #6
	nop	                              @ A7.7.88
	strb	r3, [r8, #92]!                @ A7.7.163
	ldrb	r2, [r13, r4, LSL #1]         @ A7.7.48
	ldrh	r7, cell_3241                 @ A7.7.56
	strb	r3, [r0, r6]                  @ A7.7.164
	bfc	r9, #2, #26                   @ A7.7.13
end_label522:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_3242
cell_3242:	.word	safeSpaceGpramSram+699

.space	0, 45
.global	cell_3246
cell_3246:	.word	safeSpaceGpramSram-113938

.space	0, 45
.global	cell_3245
cell_3245:	.word	safeSpaceSram-28453

.space	3, 45
.global	cell_3241
cell_3241:	.short	0x0755

.space	2, 45
.global	cell_3244
cell_3244:	.byte	0x3f

.space	0, 45
.global	cell_3243
cell_3243:	.byte	0x6b

.align	1
.space 562 % {{space_mod|default("0x10000000")}}
label526:
ldr	r5, cell_3277  @ 2b
ldr	r2, =func_20  @ 2b
mov	r0, #1  @ 4b
orr	r2, #1  @ 4b
ldr	r8, =table_94  @ 4b
	tbb	[r8, r0]                      @ A7.7.185  @ 4b
label526_switch_1_case_1:
ldr	r0, cell_3276  @ 4b
.space 4
label526_switch_1_case_2:
.space 4
.space 4
.space 4
label526_switch_1_case_3:
	blx	r2                            @ A7.7.19  @ 2b


	ands	r8, r7, #219                  @ A7.7.8  @ 4b
	ldrsb	r3, cell_3275                 @ A7.7.60  @ 4b
label526_switch_1_case_4:
.space 2
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
label526_switch_1_case_5:
ldr	r3, cell_3273  @ 4b
	ldrh	r7, [r3]                      @ A7.7.55  @ 2b
mov	r4, #7562  @ 4b
	ldrh	r10, [r5, r4]                 @ A7.7.57  @ 4b
end_label526:
	b.w	{{jump_label526}}

.ltorg
.align	2
.space	1, 46
.global	table_94
table_94:
.byte	0
.byte	((label526_switch_1_case_2-label526_switch_1_case_1)/2)
.byte	((label526_switch_1_case_3-label526_switch_1_case_1)/2)
.byte	((label526_switch_1_case_4-label526_switch_1_case_1)/2)
.byte	((label526_switch_1_case_5-label526_switch_1_case_1)/2)

.align	2
.global	cell_3274
cell_3274:	.quad	0x6c82bfc9c1dabd1a

.space	1, 45
.global	cell_3276
cell_3276:	.word	safeSpaceGpramSram-560

.space	3, 45
.global	cell_3277
cell_3277:	.word	safeSpaceFlash-7472

.space	2, 45
.global	cell_3273
cell_3273:	.word	safeSpaceGpramSram+136

.space	2, 45
.global	cell_3275
cell_3275:	.byte	0x82

.align	1
.space 592 % {{space_mod|default("0x10000000")}}
label532:
ldr	r1, cell_3303
ldr	r0, cell_3302
mov	r6, #40526
ldr	r10, cell_3301
	adc	r5, r7                        @ A7.7.2
	ldrsb	r9, [r1, #-110]               @ A7.7.59
	asr	r1, r9                        @ A7.7.11
	add	r8, r15                       @ A7.7.4
	ldrh	r3, [r13, #-14]               @ A7.7.55
	bics	r1, r8, #197                  @ A7.7.15
ldr	r5, cell_3300
	cbz	r6, forward_label_970         @ A7.7.21

	ldrh	r7, [r13]                     @ A7.7.55
	ldrsh	r9, [r5, #1162]               @ A7.7.63
	bfi	r9, r10, #1, #27              @ A7.7.14
mov	r5, #27019
	ldrsh	r2, [r10, r6, LSL #1]         @ A7.7.65
ldr	r1, cell_3299
	strh	r0, [r0, r5]                  @ A7.7.171
	isb	                              @ A7.7.37
	umlal	r9, r0, r9, r0                @ A7.7.203
mov	r6, #572
	ldr	r7, [r1, r6]                  @ A7.7.45
	bfi	r4, r0, #5, #23               @ A7.7.14
	asr	r2, r0, #19                   @ A7.7.10
mov	r2, #1
	ldrh	r5, [r13, r2, LSL #2]         @ A7.7.57
	adr	r2, cell_3298                 @ A7.7.7

forward_label_970:
	umull	r4, r5, r2, r5                @ A7.7.204
end_label532:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_3302
cell_3302:	.word	safeSpaceSram-26733

.space	1, 45
.global	cell_3300
cell_3300:	.word	safeSpaceGpramSram-406

.space	1, 45
.global	cell_3298
cell_3298:	.byte	0xae

.space	3, 45
.global	cell_3301
cell_3301:	.word	safeSpaceFlash-80801

.space	0, 45
.global	cell_3303
cell_3303:	.word	safeSpaceFlash+789

.space	0, 45
.global	cell_3299
cell_3299:	.word	safeSpaceGpramSram-315

.align	1
.space 116 % {{space_mod|default("0x10000000")}}
label534:
	sub	r13, #40
ldr	r5, cell_3310
ldr	r3, cell_3309
ldr	r14, =post_branch_662
orr	r14, #1
	stm	r13!, {r0,r2-r3,r5-r9}        @ A7.7.159
	strd	r4, r0, [r3, #-104]!          @ A7.7.166
	bhi	func_58                       @ A7.7.12
post_branch_662:


	push	{r0,r4,r8}                    @ A7.7.101
	.align	2
	ldrd	r6, r1, cell_3308             @ A7.7.51
	bfi	r6, r3, #1, #31               @ A7.7.14
	ldrb	r4, cell_3307                 @ A7.7.47
	strh	r8, [r13]                     @ A7.7.170
	mls	r2, r7, r7, r6                @ A7.7.75
	adc	r9, r3                        @ A7.7.2
	pop	{r1,r3,r7,r9-r10}             @ A7.7.99
mov	r9, #16
	ldrsb	r10, [r5], #-13               @ A7.7.59
	ldrh	r7, [r13, r9]                 @ A7.7.57
	mrs	r4, apsr                      @ A7.7.82
	tst	r6, r4                        @ A7.7.189
	cmp	r3, #149                      @ A7.7.27
end_label534:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.global	cell_3310
cell_3310:	.word	safeSpaceFlash+519

.space	0, 45
.global	cell_3307
cell_3307:	.byte	0x66

.space	3, 45
.global	cell_3309
cell_3309:	.word	safeSpaceSram+180

.align	2
.global	cell_3308
cell_3308:	.quad	0x2eb4f8476369e590

.align	1
.space 624 % {{space_mod|default("0x10000000")}}
label541:
mov	r5, #2981  @ 4b
ldr	r6, cell_3337  @ 2b
ldr	r8, cell_3336  @ 4b
ldr	r1, cell_3335  @ 2b
ldr	r3, cell_3334  @ 4b
mov	r0, #55  @ 4b
	strb	r5, [r3, r5, LSL #1]          @ A7.7.164  @ 4b
	strh	r4, [r6, #-96]                @ A7.7.170  @ 4b
mov	r3, #5466  @ 4b
.space 4
	movt	r4, #7154                     @ A7.7.79  @ 4b
	asr	r0, r0, #9                    @ A7.7.10  @ 4b
	smlal	r9, r6, r5, r4                @ A7.7.138  @ 4b
	ldrsh	r6, cell_3333                 @ A7.7.64  @ 4b
	itt	cc  @ 2b
	strcc	r8, [r8, r5]                  @ A7.7.162 @ 4b
	mlscc	r2, r3, r1, r1                @ A7.7.75 @ 4b


	ldrsh	r0, [r1, r3]                  @ A7.7.65  @ 2b
ldr	r0, cell_3332  @ 4b
	ldr	r3, [r0, #-24]!               @ A7.7.43  @ 4b
	smull	r0, r2, r0, r3                @ A7.7.149  @ 4b
	isb	                              @ A7.7.37  @ 4b
end_label541:
	b.w	{{jump_label541}}

.ltorg
.align	2
.space	2, 45
.global	cell_3334
cell_3334:	.word	safeSpaceGpramSram-5547

.space	0, 45
.global	cell_3333
cell_3333:	.short	0xf16c

.space	3, 45
.global	cell_3336
cell_3336:	.word	safeSpaceSram-2276

.space	1, 45
.global	cell_3337
cell_3337:	.word	safeSpaceSram+912

.space	0, 45
.global	cell_3335
cell_3335:	.word	safeSpaceFlash-4777

.space	3, 45
.global	cell_3332
cell_3332:	.word	safeSpaceFlash+520

.align	1
.space 1066 % {{space_mod|default("0x10000000")}}
func_70:
	sub	r13, #112
mov	r2, #40
ldr	r1, cell_3389
ldr	r3, cell_3388
ldr	r0, cell_3387
	ldr	r9, [r0, #1833]               @ A7.7.43
	bfi	r0, r10, #2, #17              @ A7.7.14
	ldr	r9, [r3]                      @ A7.7.43
ldr	r9, cell_3386
	strh	r8, [r13, r2]                 @ A7.7.171
	udiv	r0, r8, r3                    @ A7.7.195
	strb	r2, [r9, #158]!               @ A7.7.163
	ldrb	r0, [r13]                     @ A7.7.46
ldr	r0, cell_3385
	ldr	r3, [r13, r2]                 @ A7.7.45
mov	r10, #7689
ldr	r2, cell_3384
	bge	forward_label_996             @ A7.7.12

	cmp	r1, #43                       @ A7.7.27
ldr	r3, cell_3383
	and	r9, r6                        @ A7.7.9
	ldr	r9, [r2, r10, LSL #2]         @ A7.7.45
	ldrb	r2, [r1, #2042]               @ A7.7.46
	ldrh	r1, [r3, #20]!                @ A7.7.55

forward_label_996:
mov	r3, #5
	ldrsh	r9, [r0, r10, LSL #3]         @ A7.7.65
	ldmdb	r13, {r0,r10}                 @ A7.7.42
	msr	apsr_nzcvq, r3                @ A7.7.83
mov	r0, #13916
mov	r1, #68
	cbz	r3, forward_label_995         @ A7.7.21


forward_label_995:
	push	{r0-r1,r3,r6,r8,r10}          @ A7.7.101
	bfi	r10, r9, #2, #26              @ A7.7.14
	movt	r10, #28616                   @ A7.7.79
mov	r10, #55
	mla	r2, r0, r6, r9                @ A7.7.74
	umlal	r9, r2, r2, r0                @ A7.7.203
	ldrsb	r2, [r13], #68                @ A7.7.59
ldr	r2, cell_3382
	add	r13, r1                       @ A7.7.6
	strh	r6, [r13, r10]                @ A7.7.171
	ldrb	r9, [r2, r0, LSL #2]          @ A7.7.48
ldr	r10, cell_3381
	str	r10, [r13, r3, LSL #3]        @ A7.7.162
	ittte	ls
movls	r2, #0
	ldrls	r9, [r13, r2]                 @ A7.7.45
	mlsls	r1, r3, r1, r6                @ A7.7.75
	ldrhi	r1, cell_3380                 @ A7.7.44
	strh	r3, [r10]                     @ A7.7.170
	smull	r10, r2, r10, r8              @ A7.7.149
	isb	                              @ A7.7.37
	.align	2
	ldrd	r3, r0, cell_3379             @ A7.7.51
	movs	r0, r10                       @ A7.7.77
	bfc	r0, #4, #27                   @ A7.7.13
	ldrb	r0, cell_3378                 @ A7.7.47
end_func_70:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_3382
cell_3382:	.word	safeSpaceFlash-55197

.space	0, 45
.global	cell_3385
cell_3385:	.word	safeSpaceGpramSram-61209

.space	0, 45
.global	cell_3384
cell_3384:	.word	safeSpaceSram-30209

.space	2, 45
.global	cell_3381
cell_3381:	.word	safeSpaceSram+945

.space	1, 45
.global	cell_3383
cell_3383:	.word	safeSpaceGpramSram+616

.space	1, 45
.global	cell_3380
cell_3380:	.word	0xdf8696f9

.space	2, 45
.global	cell_3386
cell_3386:	.word	safeSpaceSram+255

.space	1, 45
.global	cell_3388
cell_3388:	.word	safeSpaceSram+365

.space	2, 45
.global	cell_3387
cell_3387:	.word	safeSpaceGpramSram-958

.space	2, 45
.global	cell_3389
cell_3389:	.word	safeSpaceGpramSram-1414

.align	2
.global	cell_3379
cell_3379:	.quad	0x153c721903e9fcec

.space	3, 45
.global	cell_3378
cell_3378:	.byte	0x3a

.align	1
.space 1358 % {{space_mod|default("0x10000000")}}
label561:
	bl	forward_label_1017            @ A7.7.18  @ 4b

	smull	r3, r5, r10, r0               @ A7.7.149  @ 4b
ldr	r6, cell_3453  @ 2b
.space 4
mov	r1, #0  @ 4b
.space 4
	bfc	r5, #1, #30                   @ A7.7.13  @ 4b
	bfi	r3, r10, #17, #14             @ A7.7.14  @ 4b
	stmdb	r6!, {r0-r3,r8-r10}           @ A7.7.160  @ 4b
.space 4
	ldrb	r6, cell_3452                 @ A7.7.47  @ 4b
	ldr	r10, cell_3451                @ A7.7.44  @ 4b
	bl	func_53                       @ A7.7.18  @ 4b


	mls	r4, r2, r0, r0                @ A7.7.75  @ 4b

forward_label_1017:
.space 4
.space 4
mov	r1, #9  @ 4b
.space 4
ldr	r10, cell_3449  @ 4b
.space 4
.space 4
.space 4
end_label561:
	b.w	{{jump_label561}}

.ltorg
.align	2
.space	3, 45
.global	cell_3449
cell_3449:	.word	safeSpaceSram+692

.space	3, 45
.global	cell_3450
cell_3450:	.byte	0xb5

.space	3, 45
.global	cell_3452
cell_3452:	.byte	0xaf

.space	1, 45
.global	cell_3453
cell_3453:	.word	safeSpaceGpramSram+216

.space	3, 45
.global	cell_3451
cell_3451:	.word	0x0b1a7549

.align	1
.space 1442 % {{space_mod|default("0x10000000")}}
func_72:
ldr	r2, =forward_label_1037
orr	r2, #1
ldr	r3, cell_3521
ldr	r0, =forward_label_1036
	bx	r2                            @ A7.7.20

	adr	r9, cell_3520                 @ A7.7.7
mov	r1, #39
	str	r6, [r13, r1]                 @ A7.7.162
	smull	r1, r10, r1, r8               @ A7.7.149
	adr	r9, cell_3519                 @ A7.7.7
	nop	                              @ A7.7.88
	and	r9, r7, #232                  @ A7.7.8
	ldrd	r10, r1, [r13]                @ A7.7.50
	asr	r9, r1, #11                   @ A7.7.10

forward_label_1037:
orr	r0, #1
	nop	                              @ A7.7.88
	ite	gt
	cmngt	r3, #65                       @ A7.7.25
	ldmle	r13, {r1-r2,r9-r10}           @ A7.7.41
	add	r9, r5, r5                    @ A7.7.4
	nop	                              @ A7.7.88
	mla	r10, r0, r0, r4               @ A7.7.74
mov	r10, #7
	.align	2
	ldrd	r2, r1, cell_3518             @ A7.7.51
	adc	r1, r3                        @ A7.7.2
ldr	r9, cell_3517
	ldrh	r1, [r3, #-15]!               @ A7.7.55
	mov	r2, r4, LSL #2                @ A7.7.78
	stm	r9!, {r0-r4,r6-r8,r10}        @ A7.7.159
ldr	r9, cell_3516
	smull	r1, r3, r6, r5                @ A7.7.149
ldr	r3, cell_3515
	ldr	r2, [r9, r10]                 @ A7.7.45
	bx	r0                            @ A7.7.20

mov	r9, #11
mov	r0, #4
	ldrsb	r2, [r13, r9]                 @ A7.7.61
	umull	r2, r9, r6, r1                @ A7.7.204
ldr	r2, cell_3514
	mov	r1, r13                       @ A7.7.77
	tst	r9, #86                       @ A7.7.188
	str	r8, [r2, r10, LSL #2]         @ A7.7.162
	ldrsb	r2, [r13, r0, LSL #3]         @ A7.7.61
	udiv	r1, r10, r6                   @ A7.7.195
	umlal	r1, r0, r10, r7               @ A7.7.203
	adds	r1, r13, #64                  @ A7.7.5
ldr	r0, cell_3513
	addw	r1, r13, #860                 @ A7.7.5
	stm	r0!, {r1-r4,r6,r8-r10}        @ A7.7.159
	umull	r1, r2, r0, r3                @ A7.7.204
	add	r9, r8, r4                    @ A7.7.4
	ldrsh	r2, [r13, #54]                @ A7.7.63

forward_label_1036:
	umlal	r1, r0, r10, r6               @ A7.7.203
	smlal	r0, r2, r2, r6                @ A7.7.138
ldr	r1, cell_3512
	ldr	r2, [r1, #399]                @ A7.7.43
	ldrd	r9, r1, [r3, #-96]!           @ A7.7.50
	adcs	r2, r0, #79                   @ A7.7.1
	adds	r2, #98                       @ A7.7.1
	ldrh	r2, cell_3511                 @ A7.7.56
	tst	r3, #81                       @ A7.7.188
	cmp	r10, #49                      @ A7.7.27
	mrs	r2, apsr                      @ A7.7.82
	itt	hi
	asrshi	r9, r10, #6                   @ A7.7.10
	ldrhhi	r0, [r13]                     @ A7.7.55
	ands	r0, r2, r6, LSL #2            @ A7.7.9
	ldrh	r0, [r13, r10, LSL #3]        @ A7.7.57
end_func_72:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_3515
cell_3515:	.word	safeSpaceFlash+972

.space	3, 45
.global	cell_3517
cell_3517:	.word	safeSpaceSram+880

.space	0, 45
.global	cell_3513
cell_3513:	.word	safeSpaceSram+244

.space	3, 45
.global	cell_3521
cell_3521:	.word	safeSpaceSram+625

.space	0, 45
.global	cell_3519
cell_3519:	.byte	0xa3

.space	1, 45
.global	cell_3516
cell_3516:	.word	safeSpaceFlash+289

.space	3, 45
.global	cell_3511
cell_3511:	.short	0xf6ec

.space	2, 45
.global	cell_3520
cell_3520:	.byte	0x9b

.space	3, 45
.global	cell_3512
cell_3512:	.word	safeSpaceSram+416

.space	1, 45
.global	cell_3514
cell_3514:	.word	safeSpaceSram+606

.align	2
.global	cell_3518
cell_3518:	.quad	0x2d3059454df70264

.align	1
label572:
ldr	r9, =forward_label_1040  @ 4b
ldr	r7, cell_3527  @ 4b
orr	r9, #1  @ 4b
.space 2

.space 4
.space 4
ldr	r1, =func_41  @ 2b
.space 4
orr	r1, #1  @ 4b
.space 4
.space 2


.space 4

forward_label_1040:
	b	forward_label_1039            @ A7.7.12  @ 2b

.space 4
	cmn	r5, r6, LSL #2                @ A7.7.26  @ 4b
ldr	r1, =func_4  @ 2b
	ldrd	r10, r5, [r7, #212]           @ A7.7.50  @ 4b
mov	r6, #8  @ 4b
	adds	r2, #79                       @ A7.7.1  @ 2b
orr	r1, #1  @ 4b
.space 4
	blx	r1                            @ A7.7.19  @ 2b


	movt	r5, #46756                    @ A7.7.79  @ 4b
	nop.n  @ was .align 2  @ 2b
	ldrd	r8, r9, cell_3524             @ A7.7.51  @ 4b
	smull	r5, r0, r5, r5                @ A7.7.149  @ 4b
ldr	r1, cell_3523  @ 4b
	and	r9, r7                        @ A7.7.9  @ 4b
mov	r7, #545  @ 4b
	cmn	r14, #248                     @ A7.7.25  @ 4b
	ldrh	r10, [r1, r7]                 @ A7.7.57  @ 4b
.space 4
	adc	r10, r0                       @ A7.7.2  @ 4b

forward_label_1039:
	ands	r8, #248                      @ A7.7.8  @ 4b
	ldrb	r3, cell_3522                 @ A7.7.47  @ 4b
end_label572:
	b.w	{{jump_label572}}

.ltorg
.align	2
.space	0, 45
.global	cell_3522
cell_3522:	.byte	0x02

.align	2
.global	cell_3524
cell_3524:	.quad	0x7039424cff608dd1

.space	1, 45
.global	cell_3527
cell_3527:	.word	safeSpaceGpramSram+664

.space	0, 45
.global	cell_3525
cell_3525:	.word	0x82bfef3c

.space	2, 45
.global	cell_3526
cell_3526:	.byte	0x6c

.space	3, 45
.global	cell_3523
cell_3523:	.word	safeSpaceFlash-102

.align	1
.space 1576 % {{space_mod|default("0x10000000")}}
func_73:
	sub	r13, #232
ldr	r2, cell_3587
ldr	r3, cell_3586
ldr	r1, cell_3585
mov	r10, #59501
ldr	r0, =forward_label_1063
	ldr	r9, [r1, r10, LSL #3]         @ A7.7.45
	ands	r1, #171                      @ A7.7.8
	add	r1, r13, #52                  @ A7.7.5
orr	r0, #1
	ldm	r13, {r1,r9-r10}              @ A7.7.41
	bx	r0                            @ A7.7.20

	nop	                              @ A7.7.88
	ldrd	r0, r10, [r2, #-100]!         @ A7.7.50
	add	r10, r15                      @ A7.7.4
	adds	r0, r13, r8, LSL #1           @ A7.7.6
	add	r10, r13, r0                  @ A7.7.6
ldr	r9, cell_3584
	ldm	r9, {r0-r2,r10}               @ A7.7.41

forward_label_1063:
ldr	r2, cell_3583
	it	ge
	sdivge	r9, r1                        @ A7.7.127
	ldrh	r0, [r2], #97                 @ A7.7.55
	.align	2
	ldrd	r0, r1, cell_3582             @ A7.7.51
	asrs	r9, r8, r2                    @ A7.7.11
	asrs	r10, r2, #17                  @ A7.7.10
	add	r2, #94                       @ A7.7.1
ldr	r10, cell_3581
mov	r2, #358
	ldrsh	r9, [r3, r2, LSL #2]          @ A7.7.65
ldr	r1, cell_3580
	ldrsb	r9, cell_3579                 @ A7.7.60
mov	r3, #2466
	ldr	r0, [r1, r3]                  @ A7.7.45
mov	r3, #0
	ldrsh	r1, [r13], #232               @ A7.7.63
	strb	r1, [r10, #3988]              @ A7.7.163
	umull	r9, r2, r9, r7                @ A7.7.204
	strb	r3, [r13, #-57]               @ A7.7.163
	str	r6, [r13, r3, LSL #1]         @ A7.7.162
	lsr	r10, r4                       @ A7.7.71
	nop	                              @ A7.7.88
mov	r2, #13853
	ldrd	r10, r3, [r13]                @ A7.7.50
ldr	r3, cell_3578
	ldr	r1, [r3, r2, LSL #3]          @ A7.7.45
end_func_73:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_3579
cell_3579:	.byte	0xa1

.space	1, 45
.global	cell_3581
cell_3581:	.word	safeSpaceSram-3395

.space	1, 45
.global	cell_3586
cell_3586:	.word	safeSpaceFlash-863

.space	3, 45
.global	cell_3587
cell_3587:	.word	safeSpaceFlash+784

.space	0, 45
.global	cell_3578
cell_3578:	.word	safeSpaceFlash-110548

.space	1, 45
.global	cell_3583
cell_3583:	.word	safeSpaceGpramSram+893

.space	1, 45
.global	cell_3584
cell_3584:	.word	safeSpaceFlash+84

.align	2
.global	cell_3582
cell_3582:	.quad	0x22a8913aed9be3f6

.space	2, 45
.global	cell_3580
cell_3580:	.word	safeSpaceGpramSram-2318

.space	3, 45
.global	cell_3585
cell_3585:	.word	safeSpaceSram-475692

.align	1
.space 1002 % {{space_mod|default("0x10000000")}}
func_75:
ldr	r3, cell_3635
ldr	r10, cell_3634
ldr	r9, =forward_label_1080
orr	r9, #1
ldr	r1, cell_3633
	adds	r2, r9, r3                    @ A7.7.4
ldr	r0, cell_3632
	udiv	r2, r0, r7                    @ A7.7.195
	ldrsh	r2, [r13]                     @ A7.7.63
	ldr	r2, [r1]                      @ A7.7.43
	ldrsb	r1, [r0], #203                @ A7.7.59
	stmdb	r3!, {r5,r10}                 @ A7.7.160
	mul	r1, r1, r4                    @ A7.7.84
	bfc	r2, #25, #4                   @ A7.7.13
	cmn	r6, r7                        @ A7.7.26
	sdiv	r0, r8, r10                   @ A7.7.127
	nop	                              @ A7.7.88
	adr	r2, cell_3631                 @ A7.7.7
	msr	apsr_nzcvq, r7                @ A7.7.83
	stmdb	r13, {r0-r1,r3,r5-r8,r10}     @ A7.7.160
	nop	                              @ A7.7.88
	bx	r9                            @ A7.7.20

	ldrb	r2, cell_3630                 @ A7.7.47
	mla	r0, r7, r4, r2                @ A7.7.74
	sdiv	r9, r8                        @ A7.7.127
	asr	r2, r8, #5                    @ A7.7.10
	ldr	r9, cell_3629                 @ A7.7.44

forward_label_1080:
	ldrsb	r9, [r10, #-182]!             @ A7.7.59
	bfi	r10, r2, #17, #15             @ A7.7.14
ldr	r2, cell_3628
	smull	r0, r9, r1, r1                @ A7.7.149
	cmp	r1, r8                        @ A7.7.28
	.align	2
	ldrd	r0, r10, cell_3627            @ A7.7.51
ldr	r9, cell_3626
	udiv	r10, r0                       @ A7.7.195
mov	r0, #1948
	strd	r0, r10, [r9, #-60]!          @ A7.7.166
	mov	r9, r4, LSL #2                @ A7.7.78
	umlal	r3, r9, r0, r8                @ A7.7.203
	ldm	r13, {r1,r3,r9-r10}           @ A7.7.41
	mov	r1, r13                       @ A7.7.77
	str	r2, [r2, r0, LSL #2]          @ A7.7.162
end_func_75:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_3631
cell_3631:	.byte	0xe3

.space	2, 45
.global	cell_3629
cell_3629:	.word	0x9d35ac39

.space	3, 45
.global	cell_3634
cell_3634:	.word	safeSpaceFlash+673

.space	0, 45
.global	cell_3632
cell_3632:	.word	safeSpaceGpramSram+844

.space	0, 45
.global	cell_3630
cell_3630:	.byte	0xd2

.align	2
.global	cell_3627
cell_3627:	.quad	0x11d849806967b811

.space	2, 45
.global	cell_3628
cell_3628:	.word	safeSpaceGpramSram-7340

.space	1, 45
.global	cell_3635
cell_3635:	.word	safeSpaceSram+380

.space	2, 45
.global	cell_3626
cell_3626:	.word	safeSpaceGpramSram+504

.space	1, 45
.global	cell_3633
cell_3633:	.word	safeSpaceFlash+864

.align	1
.space 504 % {{space_mod|default("0x10000000")}}
label595:
mov	r2, #7
	strh	r2, [r13, r2, LSL #1]         @ A7.7.171
	ldrsb	r2, cell_3657                 @ A7.7.60
	cmn	r3, r0                        @ A7.7.26
	add	r2, r13, r5                   @ A7.7.6
	tst	r3, #200                      @ A7.7.188
	cmp	r5, #89                       @ A7.7.27
	add	r2, r8, #237                  @ A7.7.3
	bcs	forward_label_1087            @ A7.7.12

ldr	r8, cell_3656
	bfc	r2, #10, #1                   @ A7.7.13
	ldrh	r0, [r8], #118                @ A7.7.55
	mls	r7, r4, r3, r10               @ A7.7.75
	smlal	r2, r10, r2, r8               @ A7.7.138
	bics	r7, r3, r8                    @ A7.7.16
	nop	                              @ A7.7.88
ldr	r8, cell_3655
	ldrsb	r4, [r8], #224                @ A7.7.59

forward_label_1087:
	ldrsh	r10, cell_3654                @ A7.7.64
	movs	r10, #65                      @ A7.7.76
end_label595:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_3655
cell_3655:	.word	safeSpaceGpramSram+460

.space	0, 45
.global	cell_3654
cell_3654:	.short	0xfd30

.space	3, 45
.global	cell_3657
cell_3657:	.byte	0x7e

.space	3, 45
.global	cell_3656
cell_3656:	.word	safeSpaceGpramSram+588

.align	1
.space 850 % {{space_mod|default("0x10000000")}}
label604:
	sub	r13, #40
ldr	r6, cell_3699
mov	r3, #14799
	ldrsh	r0, [r6, r3, LSL #2]          @ A7.7.65
ldr	r6, cell_3698
	strh	r4, [r13]                     @ A7.7.170
ldr	r5, cell_3697
	ldrsh	r2, cell_3696                 @ A7.7.64
mov	r1, #1887
	tst	r2, r5                        @ A7.7.189
	cmp	r5, #98                       @ A7.7.27
mov	r10, #41604
	lsr	r2, r6                        @ A7.7.71
	strb	r0, [r5, r1]                  @ A7.7.164
	ldrsh	r9, cell_3695                 @ A7.7.64
ldr	r0, =func_25
	strh	r5, [r6, r10]                 @ A7.7.171
	bfc	r3, #1, #31                   @ A7.7.13
	cmp	r5, r8, LSL #2                @ A7.7.28
orr	r0, #1
	blx	r0                            @ A7.7.19


	ldrsb	r0, cell_3694                 @ A7.7.60
ldr	r5, cell_3693
	ldmdb	r5!, {r1-r3,r6-r9}            @ A7.7.42
mov	r8, #8
	bfc	r1, #9, #11                   @ A7.7.13
	ldrsh	r4, [r13, r8, LSL #3]         @ A7.7.65
	pop	{r0-r7,r9-r10}                @ A7.7.99
	ldrh	r6, cell_3692                 @ A7.7.56
end_label604:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.global	cell_3698
cell_3698:	.word	safeSpaceSram-40666

.space	3, 45
.global	cell_3693
cell_3693:	.word	safeSpaceFlash+300

.space	0, 45
.global	cell_3699
cell_3699:	.word	safeSpaceFlash-58720

.space	2, 45
.global	cell_3696
cell_3696:	.short	0x7011

.space	3, 45
.global	cell_3692
cell_3692:	.short	0x20aa

.space	1, 45
.global	cell_3697
cell_3697:	.word	safeSpaceSram-1528

.space	2, 45
.global	cell_3694
cell_3694:	.byte	0xd5

.space	0, 45
.global	cell_3695
cell_3695:	.short	0xcdd6

.align	1
.space 2978 % {{space_mod|default("0x10000000")}}
label625:
	sub	r13, #212
ldr	r4, cell_3812
	bl	forward_label_1136            @ A7.7.18

	add	r5, r15                       @ A7.7.4
	smull	r2, r6, r1, r3                @ A7.7.149
	ldrh	r10, [r13, #8]                @ A7.7.55
	asr	r10, r3                       @ A7.7.11

forward_label_1136:
	mrs	r10, apsr                     @ A7.7.82
	ldr	r2, [r4, #-138]               @ A7.7.43
	ldm	r4, {r0-r10}                  @ A7.7.41
	cmp	r3, r1                        @ A7.7.28
	umull	r4, r3, r2, r5                @ A7.7.204
	and	r6, r6                        @ A7.7.9
	ldrsb	r5, cell_3811                 @ A7.7.60
	ldr	r0, [r13], #212               @ A7.7.43
	asr	r10, r1                       @ A7.7.11
end_label625:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.global	cell_3811
cell_3811:	.byte	0x9a

.space	2, 45
.global	cell_3812
cell_3812:	.word	safeSpaceFlash+648



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
.space	2, 46
.space 42 % {{space_mod|default("0x10000000")}}
.global	table_1
table_1:
.hword	0
.hword	((func_1_switch_1_case_2-func_1_switch_1_case_1)/2)
.hword	((func_1_switch_1_case_3-func_1_switch_1_case_1)/2)

.space	2, 46
.global	table_25
table_25:
.hword	0
.hword	((func_20_switch_2_case_2-func_20_switch_2_case_1)/2)
.hword	((func_20_switch_2_case_3-func_20_switch_2_case_1)/2)
.hword	((func_20_switch_2_case_4-func_20_switch_2_case_1)/2)
.hword	((func_20_switch_2_case_5-func_20_switch_2_case_1)/2)

.space	2, 46
.space 109 % {{space_mod|default("0x10000000")}}
.global	table_4
table_4:
.byte	0
.byte	((label45_switch_1_case_2-label45_switch_1_case_1)/2)
.byte	((label45_switch_1_case_3-label45_switch_1_case_1)/2)

.space	1, 46
.space 6 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 23 % {{space_mod|default("0x10000000")}}
.global	table_59
table_59:
.hword	0
.hword	((label373_switch_1_case_2-label373_switch_1_case_1)/2)
.hword	((label373_switch_1_case_3-label373_switch_1_case_1)/2)

.space	3, 46
.space 44 % {{space_mod|default("0x10000000")}}
.global	table_32
table_32:
.hword	0
.hword	((label221_switch_1_case_2-label221_switch_1_case_1)/2)
.hword	((label221_switch_1_case_3-label221_switch_1_case_1)/2)
.hword	((label221_switch_1_case_4-label221_switch_1_case_1)/2)
.hword	((label221_switch_1_case_5-label221_switch_1_case_1)/2)

.space	2, 46
.space 6 % {{space_mod|default("0x10000000")}}
.global	table_10
table_10:
.hword	0
.hword	((func_6_switch_1_case_2-func_6_switch_1_case_1)/2)
.hword	((func_6_switch_1_case_3-func_6_switch_1_case_1)/2)

.space	2, 46
.global	table_79
table_79:
.byte	0
.byte	((label441_switch_1_case_2-label441_switch_1_case_1)/2)
.byte	((label441_switch_1_case_3-label441_switch_1_case_1)/2)
.byte	((label441_switch_1_case_4-label441_switch_1_case_1)/2)
.byte	((label441_switch_1_case_5-label441_switch_1_case_1)/2)
.byte	((label441_switch_1_case_6-label441_switch_1_case_1)/2)

.space	1, 46
.space 85 % {{space_mod|default("0x10000000")}}
.global	table_47
table_47:
.byte	0
.byte	((func_31_switch_1_case_2-func_31_switch_1_case_1)/2)
.byte	((func_31_switch_1_case_3-func_31_switch_1_case_1)/2)
.byte	((func_31_switch_1_case_4-func_31_switch_1_case_1)/2)
.byte	((func_31_switch_1_case_5-func_31_switch_1_case_1)/2)
.byte	((func_31_switch_1_case_6-func_31_switch_1_case_1)/2)


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	0, 46
.space 80 % {{space_mod|default("0x10000000")}}
.global	table_91
table_91:
.hword	0
.hword	((label512_switch_1_case_2-label512_switch_1_case_1)/2)
.hword	((label512_switch_1_case_3-label512_switch_1_case_1)/2)
.hword	((label512_switch_1_case_4-label512_switch_1_case_1)/2)
.hword	((label512_switch_1_case_5-label512_switch_1_case_1)/2)

.space	2, 46
.space 81 % {{space_mod|default("0x10000000")}}
.global	table_66
table_66:
.byte	0
.byte	((label391_switch_1_case_2-label391_switch_1_case_1)/2)
.byte	((label391_switch_1_case_3-label391_switch_1_case_1)/2)
.byte	((label391_switch_1_case_4-label391_switch_1_case_1)/2)
.byte	((label391_switch_1_case_5-label391_switch_1_case_1)/2)

.space	3, 46
.global	table_15
table_15:
.byte	0
.byte	((label113_switch_1_case_2-label113_switch_1_case_1)/2)
.byte	((label113_switch_1_case_3-label113_switch_1_case_1)/2)
.byte	((label113_switch_1_case_4-label113_switch_1_case_1)/2)

.space	0, 46
.space 39 % {{space_mod|default("0x10000000")}}
.global	table_56
table_56:
.hword	0
.hword	((label358_switch_1_case_2-label358_switch_1_case_1)/2)
.hword	((label358_switch_1_case_3-label358_switch_1_case_1)/2)



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