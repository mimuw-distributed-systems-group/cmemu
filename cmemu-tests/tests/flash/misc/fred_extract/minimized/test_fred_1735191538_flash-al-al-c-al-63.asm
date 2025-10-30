---
name: Fred-generated test
description: 'Test flow: (conf. 0) label14'
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
  jump_start: label14
  jump_label14: code_end
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
	mov.w	r0, #40631
	mov.w	r1, #41921
	mov.w	r2, #64394
	mov.w	r3, #55700
	mov.w	r4, #35624
	mov.w	r5, #16070
	mov.w	r6, #65168
	mov.w	r7, #5279
	mov.w	r8, #54470
	mov.w	r9, #38297
	mov.w	r10, #3135

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
.space 56 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 2216 % {{space_mod|default("0x10000000")}}
label14:
ldr	r2, =func_44  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r14, =post_branch_19  @ 4b  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b  @ 12b
orr	r2, #1  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	bx	r2                            @ A7.7.20  @ 2b  @ 2b  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_19:


.space 12   @ 12b  @ 12b  @ 12b
end_label14:
	b.w	{{jump_label14}}

.ltorg
.align	1
.space 5876 % {{space_mod|default("0x10000000")}}
func_44:
	sub	r13, #132  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r10, cell_2162  @ 4b  @ 4b  @ 4b  @ 4b
	ldrh	r9, [r13]                     @ A7.7.55  @ 4b  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b  @ 16b
ldr	r10, cell_2161  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
mov	r2, #9259  @ 4b  @ 4b  @ 4b  @ 4b
mov	r10, #49  @ 4b  @ 4b  @ 4b  @ 4b
	addw	r0, r13, #602                 @ A7.7.5  @ 4b  @ 4b  @ 4b  @ 4b
mov	r0, #28  @ 4b  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b  @ 8b
	ldr	r9, [r13, r0]                 @ A7.7.45  @ 4b  @ 4b  @ 4b  @ 4b
	ldr	r1, [r13], #132               @ A7.7.43  @ 4b  @ 4b  @ 4b  @ 4b
mov	r3, #3  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b @ looks important!  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
.space 4  @ 4b
ldr	r0, cell_2158  @ 4b  @ 4b  @ 4b  @ 4b
	strb	r2, [r13, #-42]               @ A7.7.163  @ 4b  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b  @ 8b
	strb	r3, [r13, r3, LSL #1]         @ A7.7.164  @ 4b  @ 4b  @ 4b  @ 4b
.space 26   @ 26b
	isb	                              @ A7.7.37  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	it	ge  @ 2b  @ 2b  @ 2b  @ 2b
	ldrge	r0, [r13, r10]                @ A7.7.45 @ 4b @ 4b @ 4b @ looks important! @ 4b @ looks important!

	nop	                              @ A7.7.88  @ 2b  @ 2b  @ 2b @ looks important!  @ 2b @ looks important!
.space 24   @ 24b  @ 24b @ looks important!  @ 24b @ looks important!
end_func_44:
	bx	r14

.ltorg
.align	2
.space	1, 45
.global	cell_2158
cell_2158:	.word	safeSpaceSram-73136

.space	1, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_2162
cell_2162:	.word	safeSpaceSram+640

.space	0, 45
.space 19 % {{space_mod|default("0x10000000")}}
.global	cell_2161
cell_2161:	.word	safeSpaceGpramSram+784

.align	1
.space 1155 % {{space_mod|default("0x10000000")}}

.align	1
.space 7296 % {{space_mod|default("0x10000000")}}

.align	1
.space 1988 % {{space_mod|default("0x10000000")}}
end_func_66:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 59 % {{space_mod|default("0x10000000")}}

.align	1
.space 2799 % {{space_mod|default("0x10000000")}}

.align	1
.space 2530 % {{space_mod|default("0x10000000")}}



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
.space 217 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 18 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 9 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 331 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 14 % {{space_mod|default("0x10000000")}}



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