---
name: Fred-generated test
description: 'Test flow: (conf. 0) label274 -> label75 -> label537'
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
  jump_start: label274
  jump_label274: label75
  jump_label75: label537
  jump_label537: code_end
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
	mov.w	r0, #19606
	mov.w	r1, #6145
	mov.w	r2, #47242
	mov.w	r3, #9807
	mov.w	r4, #12697
	mov.w	r5, #21600
	mov.w	r6, #45828
	mov.w	r7, #37107
	mov.w	r8, #24869
	mov.w	r9, #59997
	mov.w	r10, #8501

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
.space 84 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 1887 % {{space_mod|default("0x10000000")}}

.align	1
.space 1462 % {{space_mod|default("0x10000000")}}
func_2:
	sub	r13, #28  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
mov	r9, #20272  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r0, cell_138  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 28   @ 28b  @ 28b  @ 28b  @ 28b
mov	r0, #3  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
ldr	r1, cell_136  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b  @ 8b
	ldrb	r3, [r13, r0, LSL #3]         @ A7.7.48  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	stm	r13!, {r0-r3,r5,r7-r8}        @ A7.7.159  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
ldr	r2, cell_134  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 24   @ 24b  @ 24b
ldr	r10, cell_132  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 6   @ 6b  @ 6b  @ 6b  @ 6b
ldr	r9, cell_131  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	mul	r0, r7                        @ A7.7.84  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r0, =forward_label_54  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	str	r9, [r2, #229]!               @ A7.7.161  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	str	r5, [r10], #252               @ A7.7.161  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
orr	r0, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	ldrb	r10, cell_130                 @ A7.7.47  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	it	le  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	addsle	r10, #80                      @ A7.7.1 @ 4b @ 4b @ 4b @ 4b @ 4b



	mov	r15, r0                       @ A7.7.77  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b

.space 4  @ 4b  @ 4b  @ 4b

forward_label_54:
	tst	r4, r10, LSL #2               @ A7.7.189  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	ldr	r1, [r13]                     @ A7.7.43  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	nop.n  @ was .align 2  @ 2b
.space 30 
end_func_2:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_138
cell_138:	.word	safeSpaceSram-80218

.space	2, 45
.space 16 % {{space_mod|default("0x10000000")}}
.global	cell_136
cell_136:	.word	safeSpaceGpramSram+300

.align	2
.space 8 % {{space_mod|default("0x10000000")}}
.global	cell_130
cell_130:	.byte	0xb0

.space	3, 45
.space 3 % {{space_mod|default("0x10000000")}}
.global	cell_132
cell_132:	.word	safeSpaceGpramSram+533

.space	2, 45
.global	cell_131
cell_131:	.word	safeSpaceGpramSram+108

.space	0, 45
.global	cell_134
cell_134:	.word	safeSpaceSram+522

.align	1
.space 1920 % {{space_mod|default("0x10000000")}}
label75:
.space 32   @ 32b

end_label75:
	b.w	{{jump_label75}}

.ltorg
.align	2
.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 2018 % {{space_mod|default("0x10000000")}}

.align	1
.space 9701 % {{space_mod|default("0x10000000")}}

.align	1
.space 2419 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 1118 % {{space_mod|default("0x10000000")}}

.align	1
.space 7668 % {{space_mod|default("0x10000000")}}
label274:
ldr	r14, =post_branch_350  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 2  @ 2b
ldr	r0, =func_2  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b  @ 8b  @ 8b
orr	r0, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	ands	r10, r8, #49                  @ A7.7.8  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	mov	r15, r0                       @ A7.7.77  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
post_branch_350:


.space 4  @ 4b  @ 4b  @ 4b
end_label274:
	b.w	{{jump_label274}}

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 17802 % {{space_mod|default("0x10000000")}}
func_47:
.space 190   @ 190b
end_func_47:
	bx	r14

.ltorg
.align	2
.space	2, 46
.space 35 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 28 % {{space_mod|default("0x10000000")}}

.align	1
.space 5838 % {{space_mod|default("0x10000000")}}

.align	1
.space 4086 % {{space_mod|default("0x10000000")}}

.align	1
.space 5035 % {{space_mod|default("0x10000000")}}

.align	1
.space 3098 % {{space_mod|default("0x10000000")}}
label537:
.space 22   @ 22b
	bl	func_47                       @ A7.7.18  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b


.space 30   @ 30b  @ 30b  @ 30b
end_label537:
	b.w	{{jump_label537}}

.ltorg
.align	1
.space 2596 % {{space_mod|default("0x10000000")}}

.align	1
.space 5535 % {{space_mod|default("0x10000000")}}

.align	1
.space 5245 % {{space_mod|default("0x10000000")}}



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
.space 154 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 72 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 399 % {{space_mod|default("0x10000000")}}



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