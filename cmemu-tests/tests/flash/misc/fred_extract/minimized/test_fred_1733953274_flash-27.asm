---
name: Fred-generated test
description: 'Test flow: (conf. 0) label573 -> label611'
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
  jump_start: label573
  jump_label573: label611
  jump_label611: code_end
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
	mov.w	r0, #35256
	mov.w	r1, #50691
	mov.w	r2, #19076
	mov.w	r3, #4854
	mov.w	r4, #47997
	mov.w	r5, #18726
	mov.w	r6, #5653
	mov.w	r7, #28984
	mov.w	r8, #39493
	mov.w	r9, #41773
	mov.w	r10, #21306

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
.space 88 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.space 19125 % {{space_mod|default("0x10000000")}}
func_22:
	sub	r13, #8  @ 2b  @ 2b  @ 2b
ldr	r9, =table_33  @ 4b  @ 4b  @ 4b
ldr	r2, cell_791  @ 4b  @ 4b  @ 4b
mov	r10, #2  @ 4b  @ 4b  @ 4b
ldr	r0, cell_790  @ 4b  @ 4b  @ 4b
mov	r1, #5  @ 4b  @ 4b  @ 4b
mov	r3, #26432  @ 4b  @ 4b  @ 4b
.space 6   @ 6b  @ 6b
func_22_switch_2_case_1:
.space 4  @ 4b  @ 4b
	ldrsb	r9, [r13, r1, LSL #1]         @ A7.7.61  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
func_22_switch_2_case_2:
ldr	r9, cell_789  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	stmdb	r13, {r0-r3,r6-r7,r9-r10}     @ A7.7.160  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
func_22_switch_2_case_3:
.space 8   @ 8b  @ 8b
func_22_switch_2_case_4:
.space 4  @ 4b  @ 4b
ldr	r10, cell_787  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	ittee	hi  @ 2b  @ 2b  @ 2b
	strhhi	r8, [r13, r1, LSL #3]         @ A7.7.171 @ 4b @ 4b @ 4b
	mrshi	r0, apsr                      @ A7.7.82 @ 4b @ 4b @ 4b
	ldrbls	r1, [r0, r3]                  @ A7.7.48 @ 2b @ 2b @ 2b
	movwls	r9, #12845                    @ A7.7.76 @ 4b @ 4b @ 4b
mov	r1, #4833  @ 4b  @ 4b  @ 4b
.space 6   @ 6b  @ 6b
ldr	r9, cell_785  @ 4b  @ 4b  @ 4b
	pop	{r0,r3}                       @ A7.7.99  @ 2b  @ 2b  @ 2b
ldr	r3, cell_784  @ 4b  @ 4b  @ 4b
.space 26   @ 26b  @ 26b
	add	r1, r13, r1, LSL #2           @ A7.7.6  @ 4b  @ 4b  @ 4b
.space 28   @ 28b  @ 28b
	ldm	r13, {r0-r3,r9-r10}           @ A7.7.41  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
end_func_22:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_787
cell_787:	.word	safeSpaceGpramSram+808

.space	0, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_789
cell_789:	.word	safeSpaceFlash+449

.space	1, 45
.global	cell_791
cell_791:	.word	safeSpaceFlash+484

.space	3, 45
.space 2 % {{space_mod|default("0x10000000")}}
.global	cell_790
cell_790:	.word	safeSpaceGpramSram-26341

.space	2, 45
.global	cell_785
cell_785:	.word	safeSpaceGpramSram+248

.space	1, 45
.space 1 % {{space_mod|default("0x10000000")}}
.global	cell_784
cell_784:	.word	safeSpaceSram-8947

.space	3, 45
.space 17384 % {{space_mod|default("0x10000000")}}
label573:
.space 14   @ 14b
ldr	r5, cell_3886  @ 4b  @ 4b  @ 4b
.space 56   @ 56b
	msr	apsr_nzcvq, r5                @ A7.7.83  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
.space 8   @ 8b @ looks important!  @ 8b @ looks important!
end_label573:
	b.w	{{jump_label573}}

.ltorg
.align	2
.space	0, 46
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_3886
cell_3886:	.word	safeSpaceGpramSram+648

.space	1, 45
.space 12 % {{space_mod|default("0x10000000")}}
label611:
ldr	r3, =func_22  @ 2b  @ 2b  @ 2b
orr	r3, #1  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_786  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b
.space 74   @ 74b
	ands	r2, #228                      @ A7.7.8  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	bx	r3                            @ A7.7.20  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_786:


.space 12   @ 12b  @ 12b
end_label611:
	b.w	{{jump_label611}}

.ltorg
.align	2
.space	3, 45
.space 26 % {{space_mod|default("0x10000000")}}

.align	2
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 1962 % {{space_mod|default("0x10000000")}}



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
.space 110 % {{space_mod|default("0x10000000")}}
.global	table_33
table_33:
.byte	0
.byte	((func_22_switch_2_case_2-func_22_switch_2_case_1)/2)
.byte	((func_22_switch_2_case_3-func_22_switch_2_case_1)/2)
.byte	((func_22_switch_2_case_4-func_22_switch_2_case_1)/2)

.space	2, 46
.space 92 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 381 % {{space_mod|default("0x10000000")}}



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