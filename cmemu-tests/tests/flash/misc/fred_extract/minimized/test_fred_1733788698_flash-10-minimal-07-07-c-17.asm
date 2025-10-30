---
name: Fred-generated test
description: 'Test flow: (conf. 0) label366 -> label556'
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
  jump_start: label366
  jump_label366: label556
  jump_label556: code_end
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
	mov.w	r0, #28600
	mov.w	r1, #49684
	mov.w	r2, #34695
	mov.w	r3, #59544
	mov.w	r4, #30025
	mov.w	r5, #15152
	mov.w	r6, #3979
	mov.w	r7, #24885
	mov.w	r8, #8254
	mov.w	r9, #44602
	mov.w	r10, #13321

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
.space 228 % {{space_mod|default("0x10000000")}}
end_func_1:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 21536 % {{space_mod|default("0x10000000")}}
func_16:
.space 4  @ 4b
ldr	r0, =forward_label_275  @ 2b  @ 2b  @ 2b
.space 8 
ldr	r10, =forward_label_274  @ 4b  @ 4b  @ 4b
.space 98   @ 98b
forward_label_275:
.space 42   @ 42b  @ 42b
forward_label_274:
.space 12   @ 12b  @ 12b  @ 12b
end_func_16:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 22 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 21854 % {{space_mod|default("0x10000000")}}
func_35:
	sub	r13, #148  @ 2b  @ 2b  @ 2b
	strh	r7, [r13, #12]!               @ A7.7.170  @ 4b  @ 4b  @ 4b
	nop	                              @ A7.7.88 @ 2b @ looks important!  @ 2b  @ 2b  @ 2b
.space 2  @ 2b
ldr	r2, cell_1766  @ 2b  @ 2b  @ 2b
.space 18   @ 18b  @ 18b  @ 18b
ldr	r1, cell_1764  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b  @ 8b
mov	r10, #2  @ 4b  @ 4b  @ 4b
.space 30   @ 30b  @ 30b
	ldrsb	r3, [r13, r10, LSL #2]        @ A7.7.61  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b  @ 12b
mov	r1, #7  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
	ldrsh	r10, [r13, r1, LSL #3]        @ A7.7.65  @ 4b  @ 4b  @ 4b
mov	r10, #116  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	ldmdb	r13, {r0-r2}                  @ A7.7.42  @ 4b  @ 4b  @ 4b
.space 2  @ 2b  @ 2b  @ 2b
	add	r13, r10                      @ A7.7.6  @ 2b  @ 2b  @ 2b
.space 12   @ 12b  @ 12b  @ 12b
ldr	r1, cell_1758  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b  @ 8b
	adds	r1, r13, r10, LSL #2          @ A7.7.6  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
end_func_35:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 9 % {{space_mod|default("0x10000000")}}
.global	cell_1766
cell_1766:	.word	safeSpaceSram+488

.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1758
cell_1758:	.word	safeSpaceGpramSram+364

.space	2, 45
.space 15 % {{space_mod|default("0x10000000")}}
.global	cell_1764
cell_1764:	.word	safeSpaceSram+80

.align	1
.space 5996 % {{space_mod|default("0x10000000")}}
label366:
.space 2  @ 2b  @ 2b
ldr	r9, =func_16  @ 4b  @ 4b  @ 4b
.space 12   @ 12b
orr	r9, #1  @ 4b  @ 4b  @ 4b
.space 24   @ 24b
ldr	r5, =func_35  @ 2b  @ 2b  @ 2b
.space 38   @ 38b  @ 38b
orr	r5, #1  @ 4b  @ 4b  @ 4b
.space 6   @ 6b  @ 6b
	blx	r9                            @ A7.7.19  @ 2b  @ 2b  @ 2b


	blx	r5                            @ A7.7.19  @ 2b  @ 2b  @ 2b


.space 4  @ 4b  @ 4b  @ 4b
end_label366:
	b.w	{{jump_label366}}

.ltorg
.align	2
.space	1, 45
.space 15 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 28216 % {{space_mod|default("0x10000000")}}
label556:
.space 32   @ 32b
end_label556:
	b.w	{{jump_label556}}

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 10048 % {{space_mod|default("0x10000000")}}



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
.space 188 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 482 % {{space_mod|default("0x10000000")}}



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