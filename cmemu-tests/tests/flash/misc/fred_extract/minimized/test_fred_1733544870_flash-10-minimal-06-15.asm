---
name: Fred-generated test
description: 'Test flow: (conf. 0) label541 -> label59'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: false
  lb_en: false
  wb_en: false
  jump_start: label541
  jump_label541: label59
  jump_label59: code_end
  code_end: code_end
  space_mod: 4
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
	mov.w	r0, #61496
	mov.w	r1, #29722
	mov.w	r2, #17269
	mov.w	r3, #32592
	mov.w	r4, #3485
	mov.w	r5, #64401
	mov.w	r6, #20897
	mov.w	r7, #50708
	mov.w	r8, #14474
	mov.w	r9, #59639
	mov.w	r10, #63761

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
.space 108 % {{space_mod|default("0x10000000")}}
end_func_1:
	bx	r14

.ltorg
.align	2
.space	1, 45
.space 9165 % {{space_mod|default("0x10000000")}}
label59:
.space 44   @ 44b
end_label59:
	b.w	{{jump_label59}}

.ltorg
.align	2
.space	3, 45
.space 9 % {{space_mod|default("0x10000000")}}
func_57:
	sub	r13, #240  @ 2b  @ 2b
	strb	r0, [r13], #132               @ A7.7.163  @ 4b  @ 4b
	nop	                              @ A7.7.88 @ 2b  @ 2b  @ 2b
.space 20   @ 20b
	str	r6, [r13], #-16               @ A7.7.161  @ 4b  @ 4b
ldr	r9, cell_2609  @ 4b  @ 4b
.space 16   @ 16b
ldr	r2, cell_2607  @ 4b  @ 4b
.space 4  @ 4b
mov	r10, #41727  @ 4b  @ 4b
.space 36   @ 36b
	str	r9, [r13]                     @ A7.7.161  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	ldr	r9, [r13]                     @ A7.7.43  @ 4b  @ 4b
ldr	r10, cell_2602  @ 4b  @ 4b
.space 24   @ 24b
	ldr	r2, [r13], #-8                @ A7.7.43  @ 4b  @ 4b
.space 24   @ 24b  @ 24b
ldr	r10, cell_2599  @ 4b  @ 4b
.space 20   @ 20b  @ 20b
	ldr	r10, [r13], #132              @ A7.7.43  @ 4b  @ 4b
.space 2  @ 2b  @ 2b
	add	r2, r13, #109                 @ A7.7.5  @ 4b  @ 4b
.space 14   @ 14b  @ 14b
end_func_57:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_2609
cell_2609:	.word	safeSpaceSram-41597

.space	2, 45
.global	cell_2602
cell_2602:	.word	safeSpaceGpramSram-14453

.space	1, 45
.space 12 % {{space_mod|default("0x10000000")}}
.global	cell_2607
cell_2607:	.word	safeSpaceFlash+362

.space	0, 45
.space 17 % {{space_mod|default("0x10000000")}}
.global	cell_2599
cell_2599:	.word	safeSpaceSram+657

.space	3, 45
.space 14 % {{space_mod|default("0x10000000")}}
label541:
.space 8 
mov	r10, #9416  @ 4b  @ 4b
ldr	r14, =post_branch_677  @ 4b  @ 4b
.space 4
ldr	r3, cell_3279  @ 4b  @ 4b
.space 16   @ 16b  @ 16b
ldr	r10, cell_3278  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
ldr	r4, =func_57  @ 2b  @ 2b
	mov	r0, r13                       @ A7.7.77  @ 2b  @ 2b
.space 16   @ 16b
	isb	                              @ A7.7.37 @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
orr	r4, #1  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	umull	r0, r7, r0, r7                @ A7.7.204 @ 4b  @ 4b  @ 4b
	smlal	r3, r1, r1, r10               @ A7.7.138 @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b
	bx	r4                            @ A7.7.20 @ 2b  @ 2b  @ 2b
post_branch_677:


end_label541:
	b.w	{{jump_label541}}

.ltorg
.align	2
.space	3, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_3279
cell_3279:	.word	safeSpaceSram+713

.space	1, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_3278
cell_3278:	.word	safeSpaceGpramSram+952

.space	2, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 11515 % {{space_mod|default("0x10000000")}}



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
.space	1, 46
.space 241 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 245 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 46
.space 281 % {{space_mod|default("0x10000000")}}



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