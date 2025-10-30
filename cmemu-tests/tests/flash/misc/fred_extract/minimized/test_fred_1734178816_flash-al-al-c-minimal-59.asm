---
name: Fred-generated test
description: 'Test flow: (conf. 0) label472 -> label522 -> label361 -> label249 ->
  label260 -> label389 -> label250 -> label284 -> label118 -> label46 -> label392
  -> label253 -> label383 -> label309 -> label100 -> label579 -> label613 -> label419
  -> label595 -> label200'
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
  jump_start: label472
  jump_label472: label522
  jump_label522: label361
  jump_label361: label249
  jump_label249: label260
  jump_label260: label389
  jump_label389: label250
  jump_label250: label284
  jump_label284: label118
  jump_label118: label46
  jump_label46: label392
  jump_label392: label253
  jump_label253: label383
  jump_label383: label309
  jump_label309: label100
  jump_label100: label579
  jump_label579: label613
  jump_label613: label419
  jump_label419: label595
  jump_label595: label200
  jump_label200: code_end
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
	mov.w	r0, #32800
	mov.w	r1, #56443
	mov.w	r2, #11149
	mov.w	r3, #30323
	mov.w	r4, #32104
	mov.w	r5, #47392
	mov.w	r6, #16115
	mov.w	r7, #63627
	mov.w	r8, #59590
	mov.w	r9, #36818
	mov.w	r10, #6113

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
.space 58 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 46
.space 2397 % {{space_mod|default("0x10000000")}}

.align	1
.space 882 % {{space_mod|default("0x10000000")}}
end_label20:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 1596 % {{space_mod|default("0x10000000")}}
label46:
mov	r8, #23198  @ 4b  @ 4b
ldr	r9, cell_297  @ 4b  @ 4b
.space 4
.space 4
.space 4
ldr	r7, cell_296  @ 2b  @ 2b
.space 2
.space 4
.space 4
.space 4
.space 4
end_label46:
	b.w	{{jump_label46}}

.ltorg
.align	2
.space	1, 45
.global	cell_297
cell_297:	.word	safeSpaceSram-23077

.space	3, 45
.global	cell_296
cell_296:	.word	safeSpaceGpramSram+120

.align	1
.space 834 % {{space_mod|default("0x10000000")}}
func_13:
.space 2
ldr	r0, cell_489  @ 4b  @ 4b
ldr	r9, cell_488  @ 4b  @ 4b
mov	r3, #47  @ 4b  @ 4b
ldr	r10, cell_487  @ 4b  @ 4b
ldr	r2, cell_486  @ 4b  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
mov	r1, #11585  @ 4b  @ 4b
.space 4
.space 4
mov	r9, #4  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_155:
.space 4
.space 4
.space 4
	ldmdb	r10!, {r2,r9}                 @ A7.7.42  @ 4b  @ 4b
ldr	r3, =forward_label_154  @ 2b  @ 2b
.space 4
.space 4
.space 4
.space 4
	ldrd	r10, r2, cell_481             @ A7.7.51  @ 4b  @ 4b
.space 2
orr	r3, #1  @ 4b  @ 4b
.space 2

.space 4
.space 4

forward_label_154:
	cbnz	r4, forward_label_153         @ A7.7.21  @ 2b  @ 2b

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4

forward_label_153:
.space 4
.space 2
.space 2
.space 4
.space 4
end_func_13:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_486
cell_486:	.word	safeSpaceSram+188

.space	2, 45
.global	cell_482
cell_482:	.short	0xffbb

.space	3, 45
.global	cell_488
cell_488:	.word	safeSpaceSram-46141

.align	2
.global	cell_481
cell_481:	.quad	0x358d1e25bc35c804

.space	2, 45
.global	cell_489
cell_489:	.word	safeSpaceFlash+376

.space	3, 45
.global	cell_483
cell_483:	.short	0x3014

.space	2, 45
.global	cell_485
cell_485:	.byte	0x8c

.space	3, 45
.global	cell_487
cell_487:	.word	safeSpaceGpramSram+676

.space	2, 45
.global	cell_484
cell_484:	.byte	0x7f

.align	1
.space 1092 % {{space_mod|default("0x10000000")}}
label100:
ldr	r6, cell_624  @ 4b  @ 4b
ldr	r10, cell_623  @ 4b  @ 4b
ldr	r0, =table_26  @ 2b  @ 2b
ldr	r7, cell_622  @ 4b  @ 4b
mov	r4, #1  @ 4b  @ 4b
mov	r8, #38  @ 4b  @ 4b
mov	r5, #4  @ 4b  @ 4b
ldr	r2, cell_621  @ 4b  @ 4b
.space 4
label100_switch_1_case_1:
.space 4
label100_switch_1_case_2:
.space 4
label100_switch_1_case_3:
.space 4
label100_switch_1_case_4:
.space 4
label100_switch_1_case_5:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
label100_switch_1_case_6:
.space 2
.space 4
.space 4
.space 4
.space 4
end_label100:
	b.w	{{jump_label100}}

.ltorg
.align	2
.space	1, 45
.global	cell_624
cell_624:	.word	safeSpaceGpramSram+132

.space	1, 45
.global	cell_620
cell_620:	.word	0xb9d1360f

.space	3, 45
.global	cell_621
cell_621:	.word	safeSpaceGpramSram+852

.space	1, 45
.global	cell_622
cell_622:	.word	safeSpaceSram-1264

.space	2, 45
.global	cell_619
cell_619:	.byte	0xde

.space	1, 45
.global	cell_623
cell_623:	.word	safeSpaceFlash+543

.align	1
.space 2022 % {{space_mod|default("0x10000000")}}
label118:
.space 2
mov	r0, #43  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label118:
	b.w	{{jump_label118}}

.ltorg
.align	2
.space	1, 45
.global	cell_702
cell_702:	.byte	0x33

.space	3, 45
.global	cell_703
cell_703:	.word	0x0c13b136

.align	1
.space 954 % {{space_mod|default("0x10000000")}}
func_17:
.space 2
ldr	r3, cell_748  @ 4b  @ 4b
mov	r10, #5  @ 4b  @ 4b
.space 4
mov	r0, #39  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
ldr	r9, =table_30  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 2
	tbh	[r9, r10, LSL #1]             @ A7.7.185  @ 4b  @ 4b
func_17_switch_1_case_1:
.space 4
func_17_switch_1_case_2:
ldr	r1, cell_746  @ 4b  @ 4b
.space 4
.space 4
.space 4
func_17_switch_1_case_3:
.space 4
.space 4
ldr	r10, cell_745  @ 4b  @ 4b
.space 4
.space 4
.space 4
func_17_switch_1_case_4:
.space 4
func_17_switch_1_case_5:
.space 4
func_17_switch_1_case_6:
.space 4
mov	r1, #47  @ 4b  @ 4b
.space 4
ldr	r9, cell_744  @ 4b  @ 4b
.space 4
.space 4
ldr	r10, cell_743  @ 4b  @ 4b
.space 4
.space 4
mov	r0, #60240  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
	ldrd	r0, r2, [r9, #-132]!          @ A7.7.50  @ 4b  @ 4b
	mul	r0, r7, r9                    @ A7.7.84  @ 4b  @ 4b
end_func_17:
	bx	r14

.ltorg
.align	2
.space	0, 46
.global	table_30
table_30:
.hword	0
.hword	((func_17_switch_1_case_2-func_17_switch_1_case_1)/2)
.hword	((func_17_switch_1_case_3-func_17_switch_1_case_1)/2)
.hword	((func_17_switch_1_case_4-func_17_switch_1_case_1)/2)
.hword	((func_17_switch_1_case_5-func_17_switch_1_case_1)/2)
.hword	((func_17_switch_1_case_6-func_17_switch_1_case_1)/2)

.space	0, 45
.global	cell_745
cell_745:	.word	safeSpaceGpramSram+428

.space	2, 45
.global	cell_744
cell_744:	.word	safeSpaceGpramSram+704

.space	0, 45
.global	cell_747
cell_747:	.byte	0xa2

.space	2, 45
.global	cell_741
cell_741:	.byte	0xc2

.space	0, 45
.global	cell_748
cell_748:	.word	safeSpaceFlash+75

.space	2, 45
.global	cell_743
cell_743:	.word	safeSpaceGpramSram-119545

.space	2, 45
.global	cell_742
cell_742:	.byte	0x13

.space	0, 45
.global	cell_746
cell_746:	.word	safeSpaceGpramSram+568

.align	1
.space 1818 % {{space_mod|default("0x10000000")}}
func_22:
.space 2
mov	r3, #2  @ 4b  @ 4b
ldr	r9, cell_921  @ 4b  @ 4b
mov	r10, #46780  @ 4b  @ 4b
ldr	r2, =table_36  @ 2b  @ 2b
ldr	r0, cell_920  @ 4b  @ 4b
.space 2
.space 4
.space 4
.space 2
.space 2
.space 4
.space 4
func_22_switch_3_case_1:
mov	r1, #59  @ 4b  @ 4b
.space 4
ldr	r3, cell_908  @ 4b  @ 4b
.space 4
mov	r1, #32228  @ 4b  @ 4b
.space 4
func_22_switch_3_case_2:
.space 4
func_22_switch_3_case_3:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
func_22_switch_3_case_4:
.space 4
func_22_switch_3_case_5:
.space 2
func_22_switch_3_case_6:
.space 4
ldr	r0, cell_904  @ 4b  @ 4b
.space 4
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r0, cell_851  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_func_22:
	bx	r14

.ltorg
.align	2
.space	2, 46
.global	table_36
table_36:
.byte	0
.byte	((func_22_switch_3_case_2-func_22_switch_3_case_1)/2)
.byte	((func_22_switch_3_case_3-func_22_switch_3_case_1)/2)
.byte	((func_22_switch_3_case_4-func_22_switch_3_case_1)/2)
.byte	((func_22_switch_3_case_5-func_22_switch_3_case_1)/2)
.byte	((func_22_switch_3_case_6-func_22_switch_3_case_1)/2)

.space	0, 45
.global	cell_921
cell_921:	.word	safeSpaceFlash-46538

.align	2
.space 30 % {{space_mod|default("0x10000000")}}
.global	cell_907
cell_907:	.word	0x82f112b1

.align	2
.space 11 % {{space_mod|default("0x10000000")}}
.global	cell_851
cell_851:	.word	safeSpaceSram+257

.space	3, 45
.global	cell_906
cell_906:	.short	0x5f10

.align	2
.space 34 % {{space_mod|default("0x10000000")}}
.global	cell_920
cell_920:	.word	safeSpaceGpramSram+172

.space	0, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_904
cell_904:	.word	safeSpaceGpramSram-92988

.space	1, 45
.space 12 % {{space_mod|default("0x10000000")}}
.global	cell_850
cell_850:	.short	0x0a9b

.align	2
.space 103 % {{space_mod|default("0x10000000")}}
.global	cell_908
cell_908:	.word	safeSpaceSram-128000

.align	2
.space 72 % {{space_mod|default("0x10000000")}}
.global	cell_905
cell_905:	.quad	0x58411fefdcf0b386

.space	1, 45
.global	cell_909
cell_909:	.byte	0x31

.align	2
.space 1770 % {{space_mod|default("0x10000000")}}
func_26:
ldr	r0, cell_1157  @ 2b  @ 2b
ldr	r9, =forward_label_356  @ 4b  @ 4b
orr	r9, #1  @ 4b  @ 4b
.space 4
ldr	r1, cell_1156  @ 4b  @ 4b
.space 2
mov	r3, #46  @ 4b  @ 4b
.space 4
.space 10


.space 4
mov	r10, #20  @ 4b  @ 4b
.space 4
.space 4
.space 4
ldr	r2, cell_1155  @ 2b  @ 2b
.space 2

.space 4

forward_label_356:
.space 4
.space 4
.space 2
.space 4
.space 4
ldr	r1, cell_1153  @ 4b  @ 4b
.space 4
.space 2
.space 4
ldr	r9, cell_1152  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 4
mov	r9, #19  @ 4b  @ 4b
.space 4
.space 4
end_func_26:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_1154
cell_1154:	.byte	0x15

.space	0, 45
.global	cell_1156
cell_1156:	.word	safeSpaceSram+1018

.space	3, 45
.global	cell_1157
cell_1157:	.word	safeSpaceSram+392

.space	2, 45
.global	cell_1153
cell_1153:	.word	safeSpaceGpramSram+533

.space	2, 45
.global	cell_1152
cell_1152:	.word	safeSpaceFlash+43

.space	0, 45
.global	cell_1155
cell_1155:	.word	safeSpaceGpramSram-2072

.align	1
.space 1316 % {{space_mod|default("0x10000000")}}
label200:
	sub	r13, #188  @ 2b  @ 2b
mov	r9, #0  @ 4b  @ 4b
	bgt	forward_label_376             @ A7.7.12  @ 2b  @ 2b

.space 4
ldr	r4, cell_1216  @ 2b  @ 2b
.space 4
ldr	r7, cell_1215  @ 2b  @ 2b
.space 4
.space 4
ldr	r8, cell_1214  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
	str	r3, [r13, #10]                @ A7.7.161  @ 4b  @ 4b
	ldrsh	r10, [r13, #32]               @ A7.7.63  @ 4b  @ 4b
.space 4
.space 4

forward_label_376:
	ldrh	r8, [r13, r9, LSL #1]         @ A7.7.57  @ 4b  @ 4b
	nop	                              @ A7.7.88  @ 2b  @ 2b
	umlal	r7, r8, r3, r1                @ A7.7.203  @ 4b  @ 4b
	bfc	r9, #13, #6                   @ A7.7.13  @ 4b  @ 4b
	adds	r13, r13, #188                @ A7.7.5  @ 4b  @ 4b
.space 4
.space 4
.space 2
.space 4
end_label200:
	b.w	{{jump_label200}}

.ltorg
.align	2
.space	0, 45
.global	cell_1215
cell_1215:	.word	safeSpaceSram+868

.space	2, 45
.global	cell_1212
cell_1212:	.byte	0x96

.space	2, 45
.global	cell_1214
cell_1214:	.word	safeSpaceFlash+136

.space	2, 45
.global	cell_1213
cell_1213:	.byte	0x56

.space	0, 45
.global	cell_1216
cell_1216:	.word	safeSpaceSram+438

.align	1
.space 1912 % {{space_mod|default("0x10000000")}}
func_31:
mov	r0, #42655  @ 4b  @ 4b
ldr	r10, =forward_label_439  @ 4b  @ 4b
ldr	r9, cell_1504  @ 4b  @ 4b
mov	r1, #20  @ 4b  @ 4b
orr	r10, #1  @ 4b  @ 4b
ldr	r2, cell_1503  @ 4b  @ 4b
mov	r3, #0  @ 4b  @ 4b
.space 2

.space 4
.space 4
.space 2
.space 4
.space 4

forward_label_440:
.space 2
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_439:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
mov	r3, #24264  @ 4b  @ 4b
ldr	r0, cell_1454  @ 4b  @ 4b
.space 4
.space 4
.space 4
end_func_31:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 21 % {{space_mod|default("0x10000000")}}
.global	cell_1504
cell_1504:	.word	safeSpaceGpramSram-170403

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}
.global	cell_1459
cell_1459:	.quad	0x7b6dc90f46917157

.space	2, 45
.space 13 % {{space_mod|default("0x10000000")}}
.global	cell_1462
cell_1462:	.short	0x7b02

.space	0, 45
.space 53 % {{space_mod|default("0x10000000")}}
.global	cell_1461
cell_1461:	.short	0x85ea

.align	2
.space 17 % {{space_mod|default("0x10000000")}}
.global	cell_1503
cell_1503:	.word	safeSpaceGpramSram+307

.space	0, 45
.space 37 % {{space_mod|default("0x10000000")}}
.global	cell_1457
cell_1457:	.byte	0x8a

.space	0, 45
.global	cell_1455
cell_1455:	.short	0x7ea3

.space	2, 45
.space 11 % {{space_mod|default("0x10000000")}}
.global	cell_1460
cell_1460:	.short	0x93b1

.space	1, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_1458
cell_1458:	.byte	0x96

.space	2, 45
.space 12 % {{space_mod|default("0x10000000")}}
.global	cell_1456
cell_1456:	.short	0x3df5

.space	0, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1454
cell_1454:	.word	safeSpaceGpramSram-23680

.space	2, 45
.space 1255 % {{space_mod|default("0x10000000")}}
label249:
mov	r5, #57  @ 4b  @ 4b
ldr	r14, =post_branch_318  @ 4b  @ 4b
ldr	r6, cell_1591  @ 2b  @ 2b
ldr	r2, cell_1590  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r3, cell_1587  @ 4b  @ 4b
.space 4
mov	r8, #47913  @ 4b  @ 4b
ldr	r7, cell_1586  @ 4b  @ 4b
.space 4
.space 4
.space 4

forward_label_472:
ldr	r8, =func_63  @ 4b  @ 4b
.space 4
.space 4
orr	r8, #1  @ 4b  @ 4b
	bx	r8                            @ A7.7.20  @ 2b  @ 2b
post_branch_318:


.space 2
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
end_label249:
	b.w	{{jump_label249}}

.ltorg
.align	2
.space	2, 45
.global	cell_1588
cell_1588:	.byte	0xcf

.space	3, 45
.global	cell_1586
cell_1586:	.word	safeSpaceSram+366

.space	0, 45
.global	cell_1583
cell_1583:	.byte	0x8c

.space	0, 45
.global	cell_1587
cell_1587:	.word	safeSpaceSram-95685

.space	1, 45
.global	cell_1585
cell_1585:	.byte	0xa8

.space	3, 45
.global	cell_1591
cell_1591:	.word	safeSpaceSram+444

.space	2, 45
.global	cell_1590
cell_1590:	.word	safeSpaceGpramSram-1209

.space	0, 45
.global	cell_1589
cell_1589:	.byte	0xc8

.space	3, 45
.global	cell_1584
cell_1584:	.byte	0x7f

.align	1
label250:
ldr	r8, cell_1596  @ 4b  @ 4b
.space 4

.space 4
ldr	r3, cell_1594  @ 4b  @ 4b
ldr	r1, cell_1593  @ 2b  @ 2b
.space 2
.space 4
.space 4
.space 4
ldr	r1, cell_1592  @ 2b  @ 2b
.space 4
.space 2
.space 4
.space 4
.space 2
.space 4
.space 2

forward_label_474:
.space 4
end_label250:
	b.w	{{jump_label250}}

.ltorg
.align	2
.global	cell_1595
cell_1595:	.quad	0x1625768770a6c33d

.space	2, 45
.global	cell_1594
cell_1594:	.word	safeSpaceFlash+736

.space	1, 45
.global	cell_1596
cell_1596:	.word	safeSpaceGpramSram-2108

.space	1, 45
.global	cell_1593
cell_1593:	.word	safeSpaceGpramSram+439

.space	0, 45
.global	cell_1592
cell_1592:	.word	safeSpaceGpramSram+236

.align	1
.space 176 % {{space_mod|default("0x10000000")}}
label253:
ldr	r0, cell_1606  @ 4b  @ 4b
ldr	r14, =post_branch_323  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b
ldr	r8, =func_49  @ 4b  @ 4b
orr	r8, #1  @ 4b  @ 4b
	cbz	r3, forward_label_480         @ A7.7.21  @ 2b  @ 2b

	strb	r5, [r13]                     @ A7.7.163  @ 4b  @ 4b
	strd	r0, r2, [r0, #-20]            @ A7.7.166  @ 4b  @ 4b
	bx	r8                            @ A7.7.20  @ 2b  @ 2b
post_branch_323:


ldr	r2, cell_1605  @ 2b  @ 2b
	mla	r0, r8, r3, r8                @ A7.7.74  @ 4b  @ 4b
mov	r7, #11755  @ 4b  @ 4b
.space 4
.space 4
.space 2

forward_label_480:
.space 4
.space 4
.space 4
.space 4
end_label253:
	b.w	{{jump_label253}}

.ltorg
.align	2
.global	cell_1604
cell_1604:	.quad	0x6f58b9b7386c82b2

.space	0, 45
.global	cell_1605
cell_1605:	.word	safeSpaceSram-11249

.space	1, 45
.global	cell_1606
cell_1606:	.word	safeSpaceSram+384

.align	1
.space 234 % {{space_mod|default("0x10000000")}}
label260:
.space 2
ldr	r7, cell_1632  @ 2b  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r9, #53  @ 4b  @ 4b
.space 4
	ldrsh	r1, cell_1630                 @ A7.7.64  @ 4b  @ 4b
	bl	func_13                       @ A7.7.18  @ 4b  @ 4b


ldr	r7, cell_1629  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
mov	r10, #36066  @ 4b  @ 4b
ldr	r8, cell_1627  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label260:
	b.w	{{jump_label260}}

.ltorg
.align	2
.space	2, 45
.global	cell_1629
cell_1629:	.word	safeSpaceSram+88

.space	3, 45
.global	cell_1631
cell_1631:	.byte	0x55

.space	3, 45
.global	cell_1630
cell_1630:	.short	0xaa08

.space	1, 45
.global	cell_1628
cell_1628:	.byte	0x5c

.space	2, 45
.global	cell_1627
cell_1627:	.word	safeSpaceSram-35674

.space	1, 45
.global	cell_1632
cell_1632:	.word	safeSpaceGpramSram+228

.align	1
.space 1009 % {{space_mod|default("0x10000000")}}

.align	1
.space 1266 % {{space_mod|default("0x10000000")}}
label284:
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r1, #6  @ 4b  @ 4b
.space 4
ldr	r0, cell_1794  @ 4b  @ 4b
.space 4
	sdiv	r8, r9, r1                    @ A7.7.127  @ 4b  @ 4b
	ldm	r0, {r0-r3,r5-r10}            @ A7.7.41  @ 4b  @ 4b
ldr	r1, cell_1793  @ 4b  @ 4b
mov	r7, #52139  @ 4b  @ 4b
.space 4
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
end_label284:
	b.w	{{jump_label284}}

.ltorg
.align	2
.space	0, 45
.global	cell_1791
cell_1791:	.byte	0xa7

.space	0, 45
.global	cell_1790
cell_1790:	.byte	0x98

.space	3, 45
.global	cell_1793
cell_1793:	.word	safeSpaceFlash-103699

.space	3, 45
.global	cell_1792
cell_1792:	.byte	0x0f

.space	1, 45
.global	cell_1794
cell_1794:	.word	safeSpaceFlash+96

.align	1
.space 2398 % {{space_mod|default("0x10000000")}}
func_38:
	sub	r13, #248  @ 2b  @ 2b
mov	r3, #30013  @ 4b  @ 4b
ldr	r10, cell_1907  @ 4b  @ 4b
ldr	r9, cell_1906  @ 4b  @ 4b
ldr	r2, cell_1905  @ 4b  @ 4b
.space 4
.space 2
mov	r1, #5  @ 4b  @ 4b
.space 2
.space 4
	ldrh	r0, [r13]                     @ A7.7.55  @ 4b  @ 4b
	ldrb	r0, [r13, #236]!              @ A7.7.46  @ 4b  @ 4b
ldr	r0, cell_1902  @ 2b  @ 2b
.space 4
	ldrsh	r2, [r13, #6]                 @ A7.7.63  @ 4b  @ 4b
.space 4

	ldrb	r2, [r13, r1, LSL #2]         @ A7.7.48  @ 4b  @ 4b
.space 4
.space 4
.space 4
	ldrh	r2, [r13, #-40]               @ A7.7.55  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
	ldrb	r9, [r13, r1, LSL #2]         @ A7.7.48  @ 4b  @ 4b
mov	r3, #6999  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
	stm	r13!, {r1,r4-r5}              @ A7.7.159  @ 2b  @ 2b
.space 4
	cmn	r13, #176                     @ A7.7.25  @ 4b  @ 4b
ldr	r1, cell_1899  @ 4b  @ 4b
.space 4
end_func_38:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_1901
cell_1901:	.short	0xf7b3

.space	0, 45
.global	cell_1905
cell_1905:	.word	safeSpaceSram+196

.space	2, 45
.global	cell_1904
cell_1904:	.word	0xfe0ed47e

.space	2, 45
.global	cell_1903
cell_1903:	.byte	0xb1

.space	3, 45
.global	cell_1907
cell_1907:	.word	safeSpaceSram-29744

.space	0, 45
.global	cell_1906
cell_1906:	.word	safeSpaceSram-1975

.space	3, 45
.global	cell_1899
cell_1899:	.word	safeSpaceSram+576

.space	3, 45
.global	cell_1902
cell_1902:	.word	safeSpaceGpramSram-55191

.space	2, 45
.global	cell_1900
cell_1900:	.word	0xe2ba4909

.align	1
.space 938 % {{space_mod|default("0x10000000")}}
label309:
	ldrb	r5, cell_1949                 @ A7.7.47  @ 4b  @ 4b
.space 14



	movt	r6, #6552                     @ A7.7.79  @ 4b  @ 4b
	bvc	forward_label_578             @ A7.7.12  @ 2b  @ 2b

mov	r2, #1  @ 4b  @ 4b
.space 4
.space 4
ldr	r14, =post_branch_396  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b
.space 4
post_branch_396:


.space 4
.space 4

forward_label_578:
mov	r9, #6  @ 4b  @ 4b
.space 4
.space 2
end_label309:
	b.w	{{jump_label309}}

.ltorg
.align	2
.space	0, 45
.global	cell_1949
cell_1949:	.byte	0xb6

.space	0, 45
.global	cell_1948
cell_1948:	.byte	0x4f

.align	1
.space 217 % {{space_mod|default("0x10000000")}}

.align	1
.space 2358 % {{space_mod|default("0x10000000")}}

.align	1
.space 1824 % {{space_mod|default("0x10000000")}}

.align	1
.space 3132 % {{space_mod|default("0x10000000")}}
label361:
.space 4
ldr	r10, cell_2263  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
end_label361:
	b.w	{{jump_label361}}

.ltorg
.align	2
.space	2, 45
.global	cell_2264
cell_2264:	.short	0x78fe

.space	0, 45
.global	cell_2263
cell_2263:	.word	safeSpaceFlash+204

.space	3, 45
.global	cell_2261
cell_2261:	.byte	0xef

.space	1, 45
.global	cell_2262
cell_2262:	.byte	0x8a

.align	1
.space 1690 % {{space_mod|default("0x10000000")}}
label383:
mov	r0, #7  @ 4b  @ 4b
ldr	r3, =func_67  @ 2b  @ 2b
orr	r3, #1  @ 4b  @ 4b
ldr	r7, =func_26  @ 2b  @ 2b
ldr	r8, cell_2374  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
orr	r7, #1  @ 4b  @ 4b
.space 4
.space 4
.space 2


.space 4
.space 4
.space 2


.space 4
ldr	r1, cell_2370  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
end_label383:
	b.w	{{jump_label383}}

.ltorg
.align	2
.space	3, 45
.global	cell_2369
cell_2369:	.byte	0x1c

.align	2
.global	cell_2372
cell_2372:	.quad	0x01b66b9b907ad255

.space	2, 45
.global	cell_2373
cell_2373:	.short	0x46a5

.space	0, 45
.global	cell_2374
cell_2374:	.word	safeSpaceGpramSram+380

.space	1, 45
.global	cell_2371
cell_2371:	.word	0x92fb20d3

.space	2, 45
.global	cell_2370
cell_2370:	.word	safeSpaceSram+628

.align	1
.space 156 % {{space_mod|default("0x10000000")}}
label389:
ldr	r10, cell_2400  @ 4b  @ 4b
ldr	r0, cell_2399  @ 4b  @ 4b
ldr	r6, cell_2398  @ 4b  @ 4b
.space 4

.space 4
.space 4
ldr	r8, cell_2396  @ 4b  @ 4b
.space 4
mov	r3, #58044  @ 4b  @ 4b
.space 4
.space 4
.space 4

forward_label_719:
.space 2
.space 4
.space 4
ldr	r4, cell_2395  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
ldr	r8, cell_2393  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4


.space 4
end_label389:
	b.w	{{jump_label389}}

.ltorg
.align	2
.space	0, 45
.global	cell_2397
cell_2397:	.byte	0x6d

.space	1, 45
.global	cell_2398
cell_2398:	.word	safeSpaceGpramSram+260

.space	3, 45
.global	cell_2394
cell_2394:	.word	0xebf1aa6f

.space	1, 45
.global	cell_2395
cell_2395:	.word	safeSpaceGpramSram+117

.space	1, 45
.global	cell_2396
cell_2396:	.word	safeSpaceSram-57577

.space	3, 45
.global	cell_2392
cell_2392:	.byte	0x51

.space	3, 45
.global	cell_2393
cell_2393:	.word	safeSpaceSram+838

.space	3, 45
.global	cell_2399
cell_2399:	.word	safeSpaceSram+556

.space	0, 45
.global	cell_2400
cell_2400:	.word	safeSpaceSram+688

.align	1
.space 132 % {{space_mod|default("0x10000000")}}
label392:
.space 2
ldr	r14, =post_branch_501  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b
ldr	r2, cell_2406  @ 4b  @ 4b
ldr	r1, cell_2405  @ 2b  @ 2b
ldr	r0, =func_17  @ 2b  @ 2b
.space 2

.space 4
.space 2
.space 4
.space 2
.space 4
.space 4
.space 4

forward_label_723:
.space 4
.space 4
.space 4
.space 4
orr	r0, #1  @ 4b  @ 4b
.space 4
.space 2
post_branch_501:


.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label392:
	b.w	{{jump_label392}}

.ltorg
.align	2
.space	1, 45
.global	cell_2403
cell_2403:	.short	0xb47c

.space	0, 45
.global	cell_2406
cell_2406:	.word	safeSpaceSram+592

.space	1, 45
.global	cell_2405
cell_2405:	.word	safeSpaceGpramSram+792

.align	2
.global	cell_2404
cell_2404:	.quad	0x6b57b726c8fc7f3a

.align	1
.space 556 % {{space_mod|default("0x10000000")}}
func_49:
	sub	r13, #76  @ 2b  @ 2b
mov	r9, #2  @ 4b  @ 4b
mov	r0, #30083  @ 4b  @ 4b
.space 4
.space 4
ldr	r2, cell_2480  @ 2b  @ 2b
.space 4
ldr	r3, cell_2479  @ 2b  @ 2b
	ldrb	r10, [r13, r9, LSL #1]        @ A7.7.48  @ 4b  @ 4b
	ldrb	r10, [r13, r9, LSL #2]        @ A7.7.48  @ 4b  @ 4b
.space 10



.space 4
.space 4
	ldrh	r9, [r13]                     @ A7.7.55  @ 4b  @ 4b
mov	r1, #10  @ 4b  @ 4b
	strb	r4, [r13, r1]                 @ A7.7.164  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 2

	ldrh	r9, [r13, r1]                 @ A7.7.57  @ 4b  @ 4b
.space 4
.space 4
mov	r2, #7  @ 4b  @ 4b
	ldrh	r0, [r13, r2, LSL #2]         @ A7.7.57  @ 4b  @ 4b
.space 4
.space 4
.space 4

forward_label_745:
.space 4
.space 4
mov	r1, #76  @ 4b  @ 4b
	add	r13, r13, r1                  @ A7.7.6  @ 2b  @ 2b
.space 4
end_func_49:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_2476
cell_2476:	.word	0x24e9fb6b

.space	0, 45
.global	cell_2478
cell_2478:	.byte	0xf3

.space	1, 45
.global	cell_2480
cell_2480:	.word	safeSpaceGpramSram-119470

.space	3, 45
.global	cell_2475
cell_2475:	.short	0xe95d

.space	0, 45
.global	cell_2477
cell_2477:	.byte	0xd6

.space	2, 45
.global	cell_2479
cell_2479:	.word	safeSpaceFlash+129

.space	3, 45
.global	cell_2474
cell_2474:	.short	0x879e

.align	1
.space 1640 % {{space_mod|default("0x10000000")}}
label419:
.space 4
.space 4
.space 4
.space 4
.space 2
	ittt	mi  @ 2b  @ 2b
ldrmi	r8, cell_2634  @ 4b  @ 4b
	strmi	r0, [r8], #-155               @ A7.7.161  @ 4b  @ 4b
	movmi	r5, #95                       @ A7.7.76  @ 2b  @ 2b
.space 2
end_label419:
	b.w	{{jump_label419}}

.ltorg
.align	2
.space	3, 45
.global	cell_2634
cell_2634:	.word	safeSpaceGpramSram+466

.space	3, 45
.global	cell_2636
cell_2636:	.short	0xb2d5

.space	3, 45
.global	cell_2635
cell_2635:	.short	0x8956

.align	1
.space 786 % {{space_mod|default("0x10000000")}}
label472:
.space 2
ldr	r10, =forward_label_878  @ 4b  @ 4b
ldr	r6, cell_3003  @ 4b  @ 4b
orr	r10, #1  @ 4b  @ 4b
ldr	r5, cell_3002  @ 4b  @ 4b
mov	r4, #13018  @ 4b  @ 4b
mov	r9, #286  @ 4b  @ 4b
mov	r1, #8  @ 4b  @ 4b
.space 16



.space 4
.space 4
.space 4
.space 4

forward_label_878:
.space 6

ldr	r5, cell_3001  @ 2b  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label472:
	b.w	{{jump_label472}}

.ltorg
.align	2
.space	1, 45
.global	cell_3002
cell_3002:	.word	safeSpaceFlash-25528

.space	3, 45
.global	cell_3001
cell_3001:	.word	safeSpaceGpramSram+201

.space	2, 45
.global	cell_3003
cell_3003:	.word	safeSpaceGpramSram+588

.align	1
.space 1066 % {{space_mod|default("0x10000000")}}

.align	1
.space 1104 % {{space_mod|default("0x10000000")}}
func_63:
.space 2
mov	r10, #21934  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 4
.space 4
ldr	r9, cell_3220  @ 4b  @ 4b
.space 4
.space 4
.space 4
ldr	r1, cell_3219  @ 4b  @ 4b
.space 4
.space 4
ldr	r0, cell_3218  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r3, #7828  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
ldr	r1, cell_3216  @ 4b  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_940:
.space 2
.space 4
ldr	r2, cell_3213  @ 2b  @ 2b
.space 4
.space 2
.space 4
end_func_63:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_3213
cell_3213:	.word	safeSpaceGpramSram+657

.space	2, 45
.global	cell_3220
cell_3220:	.word	safeSpaceSram-21001

.space	3, 45
.global	cell_3219
cell_3219:	.word	safeSpaceGpramSram+796

.space	3, 45
.global	cell_3215
cell_3215:	.byte	0xe6

.space	0, 45
.global	cell_3216
cell_3216:	.word	safeSpaceSram-61670

.space	0, 45
.global	cell_3214
cell_3214:	.byte	0x57

.space	1, 45
.global	cell_3217
cell_3217:	.byte	0xea

.space	3, 45
.global	cell_3218
cell_3218:	.word	safeSpaceSram+180

.align	1
.space 488 % {{space_mod|default("0x10000000")}}
label522:
.space 2
ldr	r2, =func_38  @ 2b  @ 2b
ldr	r5, cell_3333  @ 4b  @ 4b
orr	r2, #1  @ 4b  @ 4b
.space 2

mov	r8, #4371  @ 4b  @ 4b
ldr	r4, cell_3332  @ 4b  @ 4b
.space 4
mov	r1, #36062  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 2

forward_label_976:
	movt	r4, #9480                     @ A7.7.79  @ 4b  @ 4b
	blx	r2                            @ A7.7.19  @ 2b  @ 2b


.space 4
.space 4
end_label522:
	b.w	{{jump_label522}}

.ltorg
.align	2
.space	1, 45
.global	cell_3333
cell_3333:	.word	safeSpaceSram-17144

.space	2, 45
.global	cell_3331
cell_3331:	.short	0x1241

.space	0, 45
.global	cell_3332
cell_3332:	.word	safeSpaceFlash-143497

.align	1
.space 362 % {{space_mod|default("0x10000000")}}
func_67:
mov	r0, #2416  @ 4b  @ 4b
mov	r1, #5  @ 4b  @ 4b
	strb	r3, [r13, r1, LSL #3]         @ A7.7.164  @ 4b  @ 4b
ldr	r2, cell_3369  @ 4b  @ 4b
ldr	r1, cell_3368  @ 4b  @ 4b
	strb	r9, [r13, #32]                @ A7.7.163  @ 4b  @ 4b
ldr	r3, cell_3367  @ 4b  @ 4b
.space 4
.space 4
mov	r9, #6  @ 4b  @ 4b
.space 2
ldr	r0, cell_3366  @ 4b  @ 4b
.space 4
.space 2

ldr	r2, cell_3365  @ 2b  @ 2b
.space 4
.space 4
.space 4
.space 4
	ldr	r2, [r13, r9, LSL #2]         @ A7.7.45  @ 4b  @ 4b
.space 4
	str	r9, [r13, r9, LSL #3]         @ A7.7.162  @ 4b  @ 4b
.space 4
.space 4
.space 4

forward_label_985:
.space 4
	ldrsh	r3, [r13, #-59]               @ A7.7.63  @ 4b  @ 4b
.space 4
.space 4
.space 4
	stm	r13, {r1,r3-r7,r9}            @ A7.7.159  @ 4b  @ 4b
.space 4
.space 4
.space 4
end_func_67:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_3368
cell_3368:	.word	safeSpaceGpramSram+945

.space	1, 45
.global	cell_3365
cell_3365:	.word	safeSpaceFlash+335

.space	3, 45
.global	cell_3367
cell_3367:	.word	safeSpaceFlash-4116

.space	2, 45
.global	cell_3364
cell_3364:	.word	0xdafc8030

.space	0, 45
.global	cell_3369
cell_3369:	.word	safeSpaceFlash+340

.space	2, 45
.global	cell_3366
cell_3366:	.word	safeSpaceGpramSram+276

.align	1
.space 2458 % {{space_mod|default("0x10000000")}}
label579:
.space 2
mov	r6, #37529  @ 4b  @ 4b
ldr	r4, cell_3717  @ 2b  @ 2b
ldr	r3, cell_3716  @ 4b  @ 4b
mov	r10, #6  @ 4b  @ 4b
ldr	r9, cell_3715  @ 4b  @ 4b
.space 4
ldr	r7, cell_3714  @ 4b  @ 4b
.space 4
ldr	r14, =post_branch_738  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
orr	r14, #1  @ 4b  @ 4b
.space 4
.space 10


.space 4
.space 4
.space 4
.space 4
post_branch_738:


end_label579:
	b.w	{{jump_label579}}

.ltorg
.align	2
.space	0, 45
.global	cell_3717
cell_3717:	.word	safeSpaceFlash+496

.space	0, 45
.global	cell_3715
cell_3715:	.word	safeSpaceGpramSram-74384

.space	2, 45
.global	cell_3713
cell_3713:	.byte	0x33

.space	2, 45
.global	cell_3716
cell_3716:	.word	safeSpaceSram-480

.space	1, 45
.global	cell_3714
cell_3714:	.word	safeSpaceFlash+616

.align	1
.space 468 % {{space_mod|default("0x10000000")}}
func_71:
.space 2
mov	r10, #50083  @ 4b  @ 4b
mov	r2, #4  @ 4b  @ 4b
ldr	r0, cell_3744  @ 4b  @ 4b
ldr	r9, =table_107  @ 4b  @ 4b
ldr	r3, cell_3743  @ 2b  @ 2b
.space 4
func_71_switch_1_case_1:
.space 4
func_71_switch_1_case_2:
.space 2
func_71_switch_1_case_3:
.space 4
func_71_switch_1_case_4:
ldr	r1, cell_3741  @ 4b  @ 4b
.space 4
mov	r0, #58316  @ 4b  @ 4b
.space 4
mov	r1, #18  @ 4b  @ 4b
.space 4
.space 4
mov	r0, #50  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
func_71_switch_1_case_5:
.space 4
ldr	r9, cell_3740  @ 4b  @ 4b
.space 4
func_71_switch_1_case_6:
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r1, cell_3739  @ 4b  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_func_71:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_3739
cell_3739:	.word	safeSpaceFlash+396

.space	0, 45
.global	cell_3741
cell_3741:	.word	safeSpaceFlash-466142

.space	2, 45
.global	cell_3738
cell_3738:	.byte	0x30

.space	0, 45
.global	cell_3744
cell_3744:	.word	safeSpaceGpramSram-3374

.space	1, 45
.global	cell_3740
cell_3740:	.word	safeSpaceGpramSram+912

.space	1, 45
.global	cell_3743
cell_3743:	.word	safeSpaceSram-99962

.space	0, 45
.global	cell_3742
cell_3742:	.short	0x3fd9

.align	1
.space 798 % {{space_mod|default("0x10000000")}}

.align	2
.space 1256 % {{space_mod|default("0x10000000")}}
label595:
	sub	r13, #192  @ 2b  @ 2b
	bl	forward_label_1108            @ A7.7.18  @ 4b  @ 4b

.space 4
.space 4
.space 4
.space 2

forward_label_1108:
mov	r8, #5  @ 4b  @ 4b
	sdiv	r6, r6                        @ A7.7.127  @ 4b  @ 4b
ldr	r2, cell_3847  @ 4b  @ 4b
.space 4
	.align	2  @ 0b  @ 0b
	ldrd	r1, r9, cell_3846             @ A7.7.51  @ 4b  @ 4b
.space 4
	ldr	r6, [r13, r8, LSL #1]         @ A7.7.45  @ 4b  @ 4b
ldr	r3, cell_3845  @ 2b  @ 2b
	push	{r0-r10}                      @ A7.7.101  @ 4b  @ 4b
.space 4
	strb	r2, [r13]                     @ A7.7.163  @ 4b  @ 4b
	ldrb	r9, [r3], #189                @ A7.7.46  @ 4b  @ 4b
mov	r10, #58566  @ 4b  @ 4b
	push	{r1-r2,r10}                   @ A7.7.101  @ 4b  @ 4b
	smull	r9, r6, r2, r10               @ A7.7.149  @ 4b  @ 4b
	str	r6, [r2, r10, LSL #3]         @ A7.7.162  @ 4b  @ 4b
	ldrh	r0, [r13], #96                @ A7.7.55  @ 4b  @ 4b
	strh	r2, [r13, #152]!              @ A7.7.170  @ 4b  @ 4b
.space 4
end_label595:
	b.w	{{jump_label595}}

.ltorg
.align	2
.space	1, 45
.global	cell_3848
cell_3848:	.byte	0xbf

.space	2, 45
.global	cell_3845
cell_3845:	.word	safeSpaceGpramSram+689

.space	1, 45
.global	cell_3847
cell_3847:	.word	safeSpaceGpramSram-468223

.align	2
.global	cell_3846
cell_3846:	.quad	0x2620443f4b5d95bd

.align	1
.space 342 % {{space_mod|default("0x10000000")}}

.align	1
.space 1530 % {{space_mod|default("0x10000000")}}
label613:
ldr	r0, =forward_label_1144  @ 2b  @ 2b
mov	r6, #1  @ 4b  @ 4b
orr	r0, #1  @ 4b  @ 4b
ldr	r9, =func_31  @ 4b  @ 4b
orr	r9, #1  @ 4b  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2



forward_label_1144:
.space 4
	strh	r1, [r13, r6, LSL #2]         @ A7.7.171  @ 4b  @ 4b
.space 4
.space 4
	strh	r5, [r13, #-28]               @ A7.7.170  @ 4b  @ 4b
.space 4
	ldrd	r5, r7, [r13, #-4]            @ A7.7.50  @ 4b  @ 4b
end_label613:
	b.w	{{jump_label613}}

.ltorg
.align	2
.space	3, 45
.global	cell_3999
cell_3999:	.byte	0x09

.align	1
.space 190 % {{space_mod|default("0x10000000")}}
func_78:
.space 4
.space 4
mov	r10, #0  @ 4b  @ 4b
ldr	r2, cell_4017  @ 4b  @ 4b
.space 2
mov	r3, #5  @ 4b  @ 4b
.space 4
.space 4
.space 2

.space 4
.space 4
.space 4
ldr	r0, cell_4015  @ 4b  @ 4b
.space 4
ldr	r0, cell_4014  @ 4b  @ 4b
.space 4
.space 4
.space 4
ldr	r0, cell_4012  @ 4b  @ 4b
mov	r9, #19796  @ 4b  @ 4b
.space 4
.space 2
.space 4
.space 4

forward_label_1147:
ldr	r1, =table_116  @ 2b  @ 2b
.space 4
func_78_switch_1_case_1:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
func_78_switch_1_case_2:
.space 4
.space 4
.space 4
.space 4
mov	r0, #27009  @ 4b  @ 4b
.space 2
.space 4
ldr	r1, cell_4009  @ 4b  @ 4b
.space 4
.space 4
func_78_switch_1_case_3:
.space 4
.space 4
end_func_78:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_4017
cell_4017:	.word	safeSpaceGpramSram+576

.space	0, 45
.global	cell_4009
cell_4009:	.word	safeSpaceGpramSram-107918

.space	0, 45
.global	cell_4013
cell_4013:	.byte	0x88

.space	1, 45
.global	cell_4015
cell_4015:	.word	safeSpaceGpramSram+373

.space	3, 45
.global	cell_4012
cell_4012:	.word	safeSpaceSram-78494

.space	3, 45
.global	cell_4014
cell_4014:	.word	safeSpaceSram+524

.space	1, 45
.global	cell_4010
cell_4010:	.word	0x4c35bdd9

.space	3, 45
.global	cell_4011
cell_4011:	.short	0x6a85

.space	0, 45
.global	cell_4016
cell_4016:	.word	0xb94b8f21

.align	1
.space 1158 % {{space_mod|default("0x10000000")}}



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
.space 132 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 44 % {{space_mod|default("0x10000000")}}
.global	table_107
table_107:
.hword	0
.hword	((func_71_switch_1_case_2-func_71_switch_1_case_1)/2)
.hword	((func_71_switch_1_case_3-func_71_switch_1_case_1)/2)
.hword	((func_71_switch_1_case_4-func_71_switch_1_case_1)/2)
.hword	((func_71_switch_1_case_5-func_71_switch_1_case_1)/2)
.hword	((func_71_switch_1_case_6-func_71_switch_1_case_1)/2)

.space	2, 46
.global	table_26
table_26:
.byte	0
.byte	((label100_switch_1_case_2-label100_switch_1_case_1)/2)
.byte	((label100_switch_1_case_3-label100_switch_1_case_1)/2)
.byte	((label100_switch_1_case_4-label100_switch_1_case_1)/2)
.byte	((label100_switch_1_case_5-label100_switch_1_case_1)/2)
.byte	((label100_switch_1_case_6-label100_switch_1_case_1)/2)

.space	3, 46
.space 11 % {{space_mod|default("0x10000000")}}
.global	table_116
table_116:
.byte	0
.byte	((func_78_switch_1_case_2-func_78_switch_1_case_1)/2)
.byte	((func_78_switch_1_case_3-func_78_switch_1_case_1)/2)

.space	3, 46
.space 19 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 110 % {{space_mod|default("0x10000000")}}



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