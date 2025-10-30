---
name: Fred-generated test
description: 'Test flow: (conf. 0) label364'
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
  jump_start: label364
  jump_label364: code_end
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
	mov.w	r2, #25882
	mov.w	r3, #45953
	mov.w	r4, #34085
	mov.w	r5, #63648
	mov.w	r6, #40704
	mov.w	r7, #51168
	mov.w	r8, #22044
	mov.w	r9, #49433
	mov.w	r10, #52483
	mov.w	r11, #34768
	mov.w	r12, #59457

    @ Start the test
    b.w    start_test


.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r0, =stack
    add.w  r0, r0, #328
    mov.w  sp, r0

    @ Get counter address
    ldr.w  r0, =counter_idx
    ldr.w  r0, [r0]
    ldr.w  r1, =counters_to_test
    ldr.w  r0, [r1, r0]
    @ Get counter start value
    ldr.w  r1, [r0]
        @ r0 – counter address
        @ r1 – counter start value

    @ Jump to the 1st block
    b.w    {{jump_start}}
.ltorg



.align	1
.space 14968 % {{space_mod|default("0x10000000")}}
func_41:
	sub	r13, #176  @ 2b  @ 2b  @ 2b
ldr	r11, cell_1275  @ 4b  @ 4b  @ 4b
mov	r2, #2  @ 4b  @ 4b  @ 4b
	ldrd	r3, r9, [r13, #172]!          @ A7.7.50  @ 4b  @ 4b  @ 4b
.space 20   @ 20b  @ 20b
	strh	r11, [r13, #4]!               @ A7.7.170  @ 4b @ looks important!  @ 4b  @ 4b
	rsbs	r3, r10, r10                  @ A7.7.120  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
ldr	r10, cell_1274  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	nop	                              @ A7.7.88  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
	ldr	r9, [r10, #4041]              @ A7.7.43  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
.space 4  @ 4b  @ 4b
	cmp	r13, #204                     @ A7.7.27  @ 4b  @ 4b  @ 4b
ldr	r10, =table_47  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
func_41_switch_1_case_1:
.space 12   @ 12b  @ 12b
func_41_switch_1_case_2:
mov	r10, #20  @ 4b  @ 4b  @ 4b
	strh	r2, [r13, r10]                @ A7.7.171  @ 4b  @ 4b  @ 4b
func_41_switch_1_case_3:
.space 24   @ 24b
func_41_switch_1_case_4:
.space 12   @ 12b  @ 12b
func_41_switch_1_case_5:
	addw	r9, r13, #1739                @ A7.7.5  @ 4b  @ 4b  @ 4b
func_41_switch_1_case_6:
end_func_41:
	bx	r14

.ltorg
.align	2
.space	1, 46
.global	table_47
table_47:
.byte	0
.byte	((func_41_switch_1_case_2-func_41_switch_1_case_1)/2)
.byte	((func_41_switch_1_case_3-func_41_switch_1_case_1)/2)
.byte	((func_41_switch_1_case_4-func_41_switch_1_case_1)/2)
.byte	((func_41_switch_1_case_5-func_41_switch_1_case_1)/2)
.byte	((func_41_switch_1_case_6-func_41_switch_1_case_1)/2)

.space	1, 45
.global	cell_1274
cell_1274:	.word	safeSpaceGpramSram-3835

.space	3, 45
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_1275
cell_1275:	.word	safeSpaceFlash-3480

.align	1
.space 1478 % {{space_mod|default("0x10000000")}}
label364:
ldr	r12, cell_1405  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_457  @ 4b  @ 4b  @ 4b
ldr	r8, cell_1404  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b
.space 30 
	lsl	r11, r11                      @ A7.7.69  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	bne	func_41                       @ A7.7.12  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_457:


.space 46 
end_label364:
	b.w	{{jump_label364}}

.ltorg
.align	2
.space	3, 45
.global	cell_1404
cell_1404:	.word	safeSpaceSram+428

.space	1, 45
.global	cell_1405
cell_1405:	.word	safeSpaceSram+660

.space	2, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 12625 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:

    @ Get counter finish value
    ldr.w  r14, [r0]
    @ Calculate counter difference
    sub.w  r14, r14, r1
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r1, cyccnt_addr
    cmp.w  r0, r1
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{saveValue("counters", r14, r0, r1)}}

    @ Save values of registers
	{{saveValue("registers", r2, r0, r1)}}
	{{saveValue("registers", r3, r0, r1)}}
	{{saveValue("registers", r4, r0, r1)}}
	{{saveValue("registers", r5, r0, r1)}}
	{{saveValue("registers", r6, r0, r1)}}
	{{saveValue("registers", r7, r0, r1)}}
	{{saveValue("registers", r8, r0, r1)}}
	{{saveValue("registers", r9, r0, r1)}}
	{{saveValue("registers", r10, r0, r1)}}
	{{saveValue("registers", r11, r0, r1)}}
	{{saveValue("registers", r12, r0, r1)}}

    @ Advance counter_idx and repeat or end the test
    ldr.w  r0, =counter_idx
    ldr.w  r1, [r0]
    add.w  r1, r1, #4
    str.w  r1, [r0]
    cmp.w  r1, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2
cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	3, 46
.space 197 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 373 % {{space_mod|default("0x10000000")}}



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