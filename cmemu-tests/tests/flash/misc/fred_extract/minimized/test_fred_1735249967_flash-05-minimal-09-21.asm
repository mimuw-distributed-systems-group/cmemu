---
name: Fred-generated test
description: 'Test flow: (conf. 0) label285'
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
  jump_start: label285
  jump_label285: code_end
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
	mov.w	r0, #47145
	mov.w	r1, #41330
	mov.w	r2, #59800
	mov.w	r3, #60108
	mov.w	r4, #8678
	mov.w	r5, #36130
	mov.w	r6, #37944
	mov.w	r7, #17274
	mov.w	r8, #15102
	mov.w	r9, #64115
	mov.w	r10, #49654

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
.space	3, 45
.space 9951 % {{space_mod|default("0x10000000")}}
func_18:
	sub	r13, #88  @ 2b  @ 2b  @ 2b
ldr	r2, =forward_label_325  @ 2b  @ 2b  @ 2b
orr	r2, #1  @ 4b  @ 4b  @ 4b
mov	r0, #31524  @ 4b  @ 4b  @ 4b
ldr	r9, cell_1161  @ 4b  @ 4b  @ 4b
.space 12   @ 12b
	strh	r8, [r13, #-17]               @ A7.7.170  @ 4b  @ 4b  @ 4b
ldr	r1, cell_1159  @ 4b  @ 4b  @ 4b
	addw	r13, r13, #88                 @ A7.7.5  @ 4b  @ 4b  @ 4b
.space 12   @ 12b
ldr	r2, cell_1157  @ 4b  @ 4b  @ 4b
.space 26   @ 26b

forward_label_325:
.space 14   @ 14b  @ 14b
	ldrd	r2, r0, [r13, #-28]           @ A7.7.50  @ 4b  @ 4b  @ 4b
.space 6   @ 6b  @ 6b
	itt	vs  @ 2b  @ 2b  @ 2b
movvs	r9, #23  @ 4b  @ 4b  @ 4b
	ldrshvs	r1, [r13, r9]                 @ A7.7.65  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b  @ 12b
end_func_18:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_1157
cell_1157:	.word	safeSpaceGpramSram-31342

.space	1, 45
.space 10 % {{space_mod|default("0x10000000")}}
.global	cell_1161
cell_1161:	.word	safeSpaceSram+688

.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1159
cell_1159:	.word	safeSpaceGpramSram-6671

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 4068 % {{space_mod|default("0x10000000")}}
label285:
	cbnz	r7, forward_label_482         @ A7.7.21  @ 2b  @ 2b  @ 2b

ldr	r8, cell_1632  @ 4b  @ 4b  @ 4b
.space 34   @ 34b  @ 34b
ldr	r8, cell_1629  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b


ldr	r14, =post_branch_342  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b
	ldrb	r9, [r8, #203]                @ A7.7.46 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
	b	func_18                       @ A7.7.12 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
post_branch_342:



forward_label_482:
	mls	r3, r2, r4, r9                @ A7.7.75 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
	ldrb	r9, cell_1628                 @ A7.7.47 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
	nop	                              @ A7.7.88 @ 2b @ looks important!  @ 2b  @ 2b  @ 2b
	ldrb	r3, cell_1627                 @ A7.7.47 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b  @ 8b
end_label285:
	b.w	{{jump_label285}}

.ltorg
.align	2
.space	3, 45
.global	cell_1628
cell_1628:	.byte	0x98

.space	2, 45
.global	cell_1627
cell_1627:	.byte	0xba

.space	3, 45
.global	cell_1632
cell_1632:	.word	safeSpaceGpramSram+307

.space	3, 45
.global	cell_1629
cell_1629:	.word	safeSpaceSram+634

.space	0, 45
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 1819 % {{space_mod|default("0x10000000")}}

.align	1
.space 5825 % {{space_mod|default("0x10000000")}}

.align	1
.space 15494 % {{space_mod|default("0x10000000")}}



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
.space	0, 46
.space 208 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 420 % {{space_mod|default("0x10000000")}}



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