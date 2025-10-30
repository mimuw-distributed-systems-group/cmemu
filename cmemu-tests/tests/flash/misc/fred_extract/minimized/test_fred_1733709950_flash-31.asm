---
name: Fred-generated test
description: 'Test flow: (conf. 0) label88 -> label29'
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
  jump_start: label88
  jump_label88: label29
  jump_label29: code_end
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
	mov.w	r0, #774
	mov.w	r1, #24376
	mov.w	r2, #57193
	mov.w	r3, #19050
	mov.w	r4, #16648
	mov.w	r5, #64165
	mov.w	r6, #40420
	mov.w	r7, #23984
	mov.w	r8, #10162
	mov.w	r9, #46207
	mov.w	r10, #44621

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
.space 96 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 3135 % {{space_mod|default("0x10000000")}}
label29:
.space 4   @ 4b  @ 4b @ looks important!
	bl	func_54                       @ A7.7.18  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
 @ looks important!

.space 48   @ 48b
end_label29:
	b.w	{{jump_label29}}

.ltorg
.align	2
.space	0, 45
.space 10 % {{space_mod|default("0x10000000")}}
label88:
.space 136   @ 136b
end_label88:
	b.w	{{jump_label88}}

.ltorg
.align	2
.space	1, 45
.space 23 % {{space_mod|default("0x10000000")}}
func_54:
	sub	r13, #112  @ 2b  @ 2b  @ 2b
	strh	r4, [r13], #-52               @ A7.7.170  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
	ldrd	r2, r10, cell_2533            @ A7.7.51  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
mov	r0, #2  @ 4b  @ 4b  @ 4b
	ldr	r1, [r13, r0, LSL #1]         @ A7.7.45  @ 4b  @ 4b  @ 4b
	ldrd	r2, r1, [r13, #-32]           @ A7.7.50  @ 4b  @ 4b  @ 4b
mov	r2, #24  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	adds	r13, r13, r2                  @ A7.7.6  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
	str	r10, [r13, #60]               @ A7.7.161  @ 4b  @ 4b  @ 4b
mov	r2, #9400  @ 4b  @ 4b  @ 4b
	stmdb	r13, {r0-r10}                 @ A7.7.160  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b
	stm	r13, {r2,r8}                  @ A7.7.159  @ 4b  @ 4b  @ 4b
ldr	r3, cell_2530  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	adds	r9, r13, r6                   @ A7.7.6  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r1, cell_2529  @ 4b  @ 4b  @ 4b
	add	r13, #140                     @ A7.7.5  @ 2b  @ 2b  @ 2b
.space 2  @ 2b  @ 2b
ldr	r9, cell_2528  @ 4b  @ 4b  @ 4b
.space 14   @ 14b  @ 14b
ldr	r2, cell_2525  @ 2b  @ 2b  @ 2b
.space 4  @ 4b  @ 4b
ldr	r1, cell_2524  @ 4b  @ 4b  @ 4b
	ldm	r13, {r0,r9-r10}              @ A7.7.41  @ 4b  @ 4b  @ 4b
.space 12   @ 12b
mov	r10, #50945  @ 4b  @ 4b  @ 4b
.space 14   @ 14b  @ 14b
mov	r1, #37422  @ 4b  @ 4b  @ 4b
ldr	r10, cell_2520  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
mov	r10, #5  @ 4b  @ 4b  @ 4b
	ldrh	r2, [r13, r10, LSL #2]        @ A7.7.57  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b

	ldmdb	r13, {r0-r3}                  @ A7.7.42  @ 4b  @ 4b  @ 4b
end_func_54:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_2525
cell_2525:	.word	safeSpaceSram-73589

.space	1, 45
.global	cell_2530
cell_2530:	.word	safeSpaceFlash+757

.align	2
.global	cell_2533
cell_2533:	.quad	0x47682f2122a07d23

.space	2, 45
.space 1 % {{space_mod|default("0x10000000")}}
.global	cell_2520
cell_2520:	.word	safeSpaceSram-36945

.space	3, 45
.global	cell_2528
cell_2528:	.word	safeSpaceGpramSram+19

.space	2, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_2524
cell_2524:	.word	safeSpaceSram-50867

.space	1, 45
.space 17 % {{space_mod|default("0x10000000")}}
.global	cell_2529
cell_2529:	.word	safeSpaceGpramSram-8712

.space	1, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 32658 % {{space_mod|default("0x10000000")}}



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
.space 259 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 291 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	0, 46
.space 236 % {{space_mod|default("0x10000000")}}



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