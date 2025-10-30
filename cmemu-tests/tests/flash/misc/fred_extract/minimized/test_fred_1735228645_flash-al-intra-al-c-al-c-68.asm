---
name: Fred-generated test
description: 'Test flow: (conf. 0) label490 -> label210 -> label161 -> label191 ->
  label178 -> label17'
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
  jump_start: label490
  jump_label490: label210
  jump_label210: label161
  jump_label161: label191
  jump_label191: label178
  jump_label178: label17
  jump_label17: code_end
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
	mov.w	r0, #21560
	mov.w	r1, #3629
	mov.w	r2, #30242
	mov.w	r3, #57531
	mov.w	r4, #48279
	mov.w	r5, #2091
	mov.w	r6, #31432
	mov.w	r7, #45033
	mov.w	r8, #26043
	mov.w	r9, #17504
	mov.w	r10, #5514

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
.space 80 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 289 % {{space_mod|default("0x10000000")}}

.align	1
.space 1746 % {{space_mod|default("0x10000000")}}
label17:
.space 92   @ 92b
end_label17:
	b.w	{{jump_label17}}

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 4277 % {{space_mod|default("0x10000000")}}

.align	1
.space 3746 % {{space_mod|default("0x10000000")}}
func_12:
.space 176   @ 176b
end_func_12:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 28 % {{space_mod|default("0x10000000")}}

.align	1
.space 3491 % {{space_mod|default("0x10000000")}}

.align	1
.space 1720 % {{space_mod|default("0x10000000")}}
label161:
.space 12 
ldr	r6, =forward_label_278  @ 2b  @ 2b  @ 2b @ looks important!
.space 56   @ 56b @ looks important!
forward_label_278:
.space 82   @ 82b
end_label161:
	b.w	{{jump_label161}}

.ltorg
.align	2
.space	0, 45
.space 608 % {{space_mod|default("0x10000000")}}
label178:
.space 16 
ldr	r4, =table_36  @ 2b  @ 2b  @ 2b @ looks important!
.space 16 
label178_switch_1_case_1:
.space 12   @ 12b  @ 12b
label178_switch_1_case_2:
.space 26   @ 26b  @ 26b
label178_switch_1_case_3:
.space 20   @ 20b  @ 20b
label178_switch_1_case_4:
.space 18   @ 18b  @ 18b
end_label178:
	b.w	{{jump_label178}}

.ltorg
.align	2
.space	3, 46
.global	table_36
table_36:
.byte	0
.byte	((label178_switch_1_case_2-label178_switch_1_case_1)/2)
.byte	((label178_switch_1_case_3-label178_switch_1_case_1)/2)
.byte	((label178_switch_1_case_4-label178_switch_1_case_1)/2)

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 15 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 1967 % {{space_mod|default("0x10000000")}}
label191:
.space 44   @ 44b
end_label191:
	b.w	{{jump_label191}}

.ltorg
.align	2
.space	2, 45
.space 6 % {{space_mod|default("0x10000000")}}

.align	1
.space 1860 % {{space_mod|default("0x10000000")}}
func_23:
	sub	r13, #20  @ 2b  @ 2b  @ 2b
mov	r2, #24  @ 4b  @ 4b  @ 4b
mov	r9, #17532  @ 4b  @ 4b  @ 4b
ldr	r0, cell_1112  @ 2b  @ 2b  @ 2b
ldr	r10, cell_1111  @ 4b  @ 4b  @ 4b
	itett	mi  @ 2b  @ 2b  @ 2b
	cmnmi	r9, #218                      @ A7.7.25 @ 4b @ 4b @ 4b
	addwpl	r0, r13, #4058                @ A7.7.5 @ 4b @ 4b @ 4b
	strhmi	r5, [r0]                      @ A7.7.170 @ 2b @ 2b @ 2b
	adcmi	r3, r1                        @ A7.7.2 @ 2b @ 2b @ 2b
.space 4  @ 4b  @ 4b
ldr	r3, cell_1110  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b
ldr	r0, cell_1108  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
mov	r0, #22914  @ 4b  @ 4b  @ 4b
	cmp	r13, r9, LSL #3               @ A7.7.28  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	ldmdb	r13, {r1,r3}                  @ A7.7.42  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
ldr	r10, cell_1106  @ 4b  @ 4b  @ 4b
mov	r3, #6  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
	strb	r7, [r13, r3, LSL #1]         @ A7.7.164  @ 4b  @ 4b  @ 4b
mov	r0, #4  @ 4b  @ 4b  @ 4b
	ldrsb	r10, [r13, r0, LSL #1]        @ A7.7.61  @ 4b  @ 4b  @ 4b
	pop	{r0-r1,r3,r9-r10}             @ A7.7.99  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	ldr	r1, [r13, r2]                 @ A7.7.45  @ 4b  @ 4b  @ 4b
.space 14   @ 14b
end_func_23:
	bx	r14

.ltorg
.align	2
.space	1, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_1111
cell_1111:	.word	safeSpaceFlash-91264

.space	1, 45
.space 3 % {{space_mod|default("0x10000000")}}
.global	cell_1112
cell_1112:	.word	safeSpaceSram+749

.space	0, 45
.global	cell_1106
cell_1106:	.word	safeSpaceSram+819

.space	1, 45
.global	cell_1108
cell_1108:	.word	safeSpaceSram+342

.space	3, 45
.global	cell_1110
cell_1110:	.word	safeSpaceSram-34130

.space	3, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 1520 % {{space_mod|default("0x10000000")}}
label210:
.space 28   @ 28b
ldr	r0, =func_23  @ 2b  @ 2b  @ 2b
.space 24   @ 24b
orr	r0, #1  @ 4b  @ 4b  @ 4b
.space 6   @ 6b @ looks important!
	blx	r0                            @ A7.7.19  @ 2b @ looks important!  @ 2b  @ 2b @ looks important!


.space 30   @ 30b  @ 30b
end_label210:
	b.w	{{jump_label210}}

.ltorg
.align	2
.space	2, 45
.space 9 % {{space_mod|default("0x10000000")}}

.align	1
.space 5799 % {{space_mod|default("0x10000000")}}

.align	1
.space 2567 % {{space_mod|default("0x10000000")}}

.align	1
.space 1466 % {{space_mod|default("0x10000000")}}

.align	1
.space 2014 % {{space_mod|default("0x10000000")}}

.align	1
.space 1887 % {{space_mod|default("0x10000000")}}

.align	1
.space 2536 % {{space_mod|default("0x10000000")}}
end_label332:
	b.w	{{code_end}}

.ltorg
.align	1
.space 5292 % {{space_mod|default("0x10000000")}}

.align	1
.space 4886 % {{space_mod|default("0x10000000")}}

.align	1
.space 3950 % {{space_mod|default("0x10000000")}}
label490:
.space 24   @ 24b
ldr	r14, =post_branch_631  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b
.space 4  @ 4b
ldr	r3, =func_12  @ 2b  @ 2b  @ 2b
.space 12   @ 12b  @ 12b
orr	r3, #1  @ 4b  @ 4b  @ 4b
	smull	r1, r7, r10, r7               @ A7.7.149  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	bx	r3                            @ A7.7.20  @ 2b @ looks important!  @ 2b  @ 2b @ looks important!
post_branch_631:


.space 28   @ 28b  @ 28b
end_label490:
	b.w	{{jump_label490}}

.ltorg
.align	2
.space	1, 45
.space 11 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 2170 % {{space_mod|default("0x10000000")}}

.align	1
.space 5549 % {{space_mod|default("0x10000000")}}

.align	1
.space 1100 % {{space_mod|default("0x10000000")}}

.align	1
.space 9353 % {{space_mod|default("0x10000000")}}

.align	1
.space 1115 % {{space_mod|default("0x10000000")}}



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
.space 154 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 57 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 46
.space 352 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 54 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 61 % {{space_mod|default("0x10000000")}}



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