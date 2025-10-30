---
name: Fred-generated test
description: 'Test flow: (conf. 0) label384 -> label10 -> label452 -> label525'
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
  jump_start: label384
  jump_label384: label10
  jump_label10: label452
  jump_label452: label525
  jump_label525: code_end
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
	mov.w	r0, #46749
	mov.w	r1, #6839
	mov.w	r2, #52209
	mov.w	r3, #63010
	mov.w	r4, #644
	mov.w	r5, #61614
	mov.w	r6, #63968
	mov.w	r7, #51338
	mov.w	r8, #17259
	mov.w	r9, #20754
	mov.w	r10, #25722

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
.space 134 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 965 % {{space_mod|default("0x10000000")}}
label10:
.space 6   @ 6b @ looks important!
ldr	r7, cell_49  @ 4b  @ 4b  @ 4b @ looks important!
.space 50 
ldr	r2, cell_47  @ 4b  @ 4b  @ 4b @ looks important!
.space 32 
	bl	func_34                       @ A7.7.18  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!


end_label10:
	b.w	{{jump_label10}}

.ltorg
.align	2
.space	1, 45
.global	cell_47
cell_47:	.word	safeSpaceSram-3134

.space	0, 45
.space 9 % {{space_mod|default("0x10000000")}}
.global	cell_49
cell_49:	.word	safeSpaceSram+764

.align	1
.space 10371 % {{space_mod|default("0x10000000")}}

.align	1
.space 8353 % {{space_mod|default("0x10000000")}}

.align	1
.space 6616 % {{space_mod|default("0x10000000")}}

.align	1
.space 1380 % {{space_mod|default("0x10000000")}}
func_34:
	sub	r13, #92  @ 2b  @ 2b  @ 2b
mov	r0, #9485  @ 4b  @ 4b  @ 4b
ldr	r3, cell_1523  @ 2b  @ 2b  @ 2b
ldr	r9, cell_1522  @ 4b  @ 4b  @ 4b
mov	r1, #2  @ 4b  @ 4b  @ 4b
ldr	r2, cell_1521  @ 4b  @ 4b  @ 4b
	ittet	pl  @ 2b  @ 2b  @ 2b
	tstpl	r2, #45                       @ A7.7.188 @ 4b @ 4b @ 4b
	addwpl	r2, r13, #2073                @ A7.7.5 @ 4b @ 4b @ 4b
	strmi	r6, [r2, r0]                  @ A7.7.162 @ 2b @ 2b @ 2b
	cmppl	r1, r3                        @ A7.7.28 @ 2b @ 2b @ 2b
ldr	r0, cell_1520  @ 2b  @ 2b  @ 2b
.space 4  @ 4b  @ 4b
ldr	r10, cell_1519  @ 4b  @ 4b  @ 4b
.space 26   @ 26b  @ 26b
ldr	r2, cell_1518  @ 4b  @ 4b  @ 4b
	strh	r8, [r13, r1, LSL #2]         @ A7.7.171  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
mov	r2, #19  @ 4b  @ 4b  @ 4b
	add	r1, r13, r5, LSL #3           @ A7.7.6  @ 4b  @ 4b  @ 4b
	ldrh	r1, [r13], #-164              @ A7.7.55  @ 4b  @ 4b  @ 4b
	strb	r9, [r13], #228               @ A7.7.163  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	strb	r5, [r13, r2]                 @ A7.7.164  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r2, cell_1507  @ 2b  @ 2b  @ 2b
.space 4  @ 4b  @ 4b
	stm	r13, {r0-r10}                 @ A7.7.159  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	pop	{r0-r1}                       @ A7.7.99  @ 2b  @ 2b  @ 2b
	pop	{r0-r1}                       @ A7.7.99  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b
mov	r1, #13  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	ldrb	r0, [r13]                     @ A7.7.46  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r9, cell_1506  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	ldrsb	r0, [r13, r1]                 @ A7.7.61  @ 4b  @ 4b  @ 4b
mov	r1, #8  @ 4b  @ 4b  @ 4b
	pop	{r0,r10}                      @ A7.7.99  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	ittt	cs  @ 2b  @ 2b  @ 2b
	ldrbcs	r0, [r9]                      @ A7.7.46 @ 4b @ 4b @ 4b
	addwcs	r0, r13, #1028                @ A7.7.5 @ 4b @ 4b @ 4b
	mulcs	r9, r4                        @ A7.7.84 @ 4b @ 4b @ 4b
	strh	r3, [r13, r1, LSL #3]         @ A7.7.171  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	cmp	r13, r1, LSL #2               @ A7.7.28  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	strh	r7, [r13], #4                 @ A7.7.170  @ 4b  @ 4b  @ 4b
end_func_34:
	bx	r14

.ltorg
.align	2
.space 24 % {{space_mod|default("0x10000000")}}
.global	cell_1523
cell_1523:	.word	safeSpaceGpramSram+450

.space	2, 45
.global	cell_1522
cell_1522:	.word	safeSpaceFlash-838

.align	2
.space 19 % {{space_mod|default("0x10000000")}}
.global	cell_1518
cell_1518:	.word	safeSpaceSram+586

.space	1, 45
.global	cell_1506
cell_1506:	.word	safeSpaceFlash+821

.align	2
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_1507
cell_1507:	.word	safeSpaceSram+108

.space	3, 45
.global	cell_1519
cell_1519:	.word	safeSpaceSram+80

.align	2
.space 34 % {{space_mod|default("0x10000000")}}
.global	cell_1521
cell_1521:	.word	safeSpaceGpramSram-8981

.space	2, 45
.global	cell_1520
cell_1520:	.word	safeSpaceGpramSram+281

.align	1
.space 1819 % {{space_mod|default("0x10000000")}}

.align	1
.space 3556 % {{space_mod|default("0x10000000")}}

.align	1
.space 3808 % {{space_mod|default("0x10000000")}}
label384:
ldr	r0, =func_74  @ 2b  @ 2b  @ 2b
.space 4  @ 4b
orr	r0, #1  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_504  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
orr	r14, #1  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	bx	r0                            @ A7.7.20  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_504:


.space 74   @ 74b
end_label384:
	b.w	{{jump_label384}}

.ltorg
.align	2
.space	3, 45
.space 867 % {{space_mod|default("0x10000000")}}
label452:
.space 114   @ 114b
end_label452:
	b.w	{{jump_label452}}

.ltorg
.align	2
.space 35 % {{space_mod|default("0x10000000")}}

.align	1
.space 1920 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 4060 % {{space_mod|default("0x10000000")}}
label525:
.space 60   @ 60b
end_label525:
	b.w	{{jump_label525}}

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 584 % {{space_mod|default("0x10000000")}}

.align	1
.space 2140 % {{space_mod|default("0x10000000")}}

.align	1
.space 3398 % {{space_mod|default("0x10000000")}}
func_74:
.space 110   @ 110b
end_func_74:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 14 % {{space_mod|default("0x10000000")}}

.align	2
.space 16 % {{space_mod|default("0x10000000")}}

.align	1
.space 3122 % {{space_mod|default("0x10000000")}}

.align	1
.space 1161 % {{space_mod|default("0x10000000")}}

.align	1
.space 1181 % {{space_mod|default("0x10000000")}}



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
.space 32 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 145 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 435 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 69 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 32 % {{space_mod|default("0x10000000")}}



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