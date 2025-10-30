---
name: Fred-generated test
description: 'Test flow: (conf. 0) label568'
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
  jump_start: label568
  jump_label568: code_end
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
	mov.w	r0, #1963
	mov.w	r1, #8676
	mov.w	r2, #19755
	mov.w	r3, #51854
	mov.w	r4, #327
	mov.w	r5, #8176
	mov.w	r6, #20832
	mov.w	r7, #24238
	mov.w	r8, #12282
	mov.w	r9, #41035
	mov.w	r10, #60240

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
.space	1, 45
.space 35227 % {{space_mod|default("0x10000000")}}
label568:
	sub	r13, #188  @ 2b  @ 2b
mov	r10, #40  @ 4b  @ 4b
.space 2
mov	r0, #63  @ 4b  @ 4b
ldr	r1, cell_3726  @ 4b  @ 4b
ldr	r8, =forward_label_1054  @ 4b  @ 4b
orr	r8, #1  @ 4b @ looks important!  @ 4b
	blx	r8                            @ A7.7.19  @ 2b @ looks important!  @ 2b @ looks important!
 @ looks important!
	ldr	r7, [r13, r10]                @ A7.7.45  @ 4b  @ 4b
	adds	r5, r13, #185                 @ A7.7.5  @ 4b  @ 4b
.space 4  @ 4b
ldr	r6, cell_3724  @ 2b  @ 2b
.space 12   @ 12b
ldr	r4, cell_3723  @ 2b  @ 2b
mov	r5, #2025  @ 4b  @ 4b
.space 4  @ 4b
ldr	r2, cell_3722  @ 2b  @ 2b
.space 14   @ 14b
forward_label_1054:
.space 8   @ 8b
ldr	r2, cell_3720  @ 4b  @ 4b
	ldrsh	r5, [r13], #-20               @ A7.7.63  @ 4b  @ 4b
	ldrb	r8, [r13, #44]                @ A7.7.46  @ 4b  @ 4b
.space 6   @ 6b
	addw	r13, r13, #208                @ A7.7.5  @ 4b  @ 4b
	ldrb	r7, [r13, #28]                @ A7.7.46  @ 4b  @ 4b
.space 12 
	str	r2, [r2, r0, LSL #3]          @ A7.7.162  @ 4b @ looks important!  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b @ looks important!  @ 4b @ looks important!
	it	ne  @ 2b  @ 2b
	strbne	r6, [r13, r0]                 @ A7.7.164 @ 4b @ looks important! @ 4b
	nop	                              @ A7.7.88  @ 2b @ looks important!  @ 2b @ looks important!
	adc	r4, r5                        @ A7.7.2  @ 4b @ looks important!  @ 4b @ looks important!
end_label568:
	b.w	{{jump_label568}}

.ltorg
.align	2
.space	2, 45
.global	cell_3726
cell_3726:	.word	safeSpaceFlash-1381

.space	2, 45
.global	cell_3723
cell_3723:	.word	safeSpaceSram+812

.space	2, 45
.global	cell_3720
cell_3720:	.word	safeSpaceSram+69

.space	3, 45
.space 3 % {{space_mod|default("0x10000000")}}
.global	cell_3724
cell_3724:	.word	safeSpaceGpramSram-684

.space	0, 45
.space 16 % {{space_mod|default("0x10000000")}}
.global	cell_3722
cell_3722:	.word	safeSpaceFlash-7204

.align	1
.space 2136 % {{space_mod|default("0x10000000")}}



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
.space 347 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 122 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	1, 46
.space 173 % {{space_mod|default("0x10000000")}}



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