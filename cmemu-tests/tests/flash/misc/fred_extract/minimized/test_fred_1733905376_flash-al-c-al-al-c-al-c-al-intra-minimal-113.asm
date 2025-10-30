---
name: Fred-generated test
description: 'Test flow: (conf. 0) label558 -> label622 -> label459 -> label471 ->
  label108'
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
  jump_start: label558
  jump_label558: label622
  jump_label622: label459
  jump_label459: label471
  jump_label471: label108
  jump_label108: code_end
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
	mov.w	r0, #27400
	mov.w	r1, #25387
	mov.w	r2, #57272
	mov.w	r3, #29153
	mov.w	r4, #53471
	mov.w	r5, #24094
	mov.w	r6, #5490
	mov.w	r7, #16193
	mov.w	r8, #16615
	mov.w	r9, #19046
	mov.w	r10, #12490

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
.space 126 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.space 6229 % {{space_mod|default("0x10000000")}}
label108:
ldr	r14, =post_branch_132  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 12   @ 12b
ldr	r0, cell_620  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r2, cell_619  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 2  @ 2b  @ 2b  @ 2b  @ 2b

ldr	r9, cell_618  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 46   @ 46b
mov	r8, #1663  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r9, =func_25  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
orr	r9, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	strb	r0, [r2, r8, LSL #1]          @ A7.7.164  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	mov	r15, r9                       @ A7.7.77  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
post_branch_132:


end_label108:
	b.w	{{jump_label108}}

.ltorg
.align	2
.space	2, 45
.global	cell_620
cell_620:	.word	safeSpaceGpramSram-65960

.space	3, 45
.space 11 % {{space_mod|default("0x10000000")}}
.global	cell_619
cell_619:	.word	safeSpaceSram-2464

.space	2, 45
.global	cell_618
cell_618:	.word	safeSpaceSram-907

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 2016 % {{space_mod|default("0x10000000")}}

.align	1
.space 2370 % {{space_mod|default("0x10000000")}}
func_11:
.space 92   @ 92b  @ 92b
ldr	r2, =forward_label_290  @ 2b  @ 2b  @ 2b  @ 2b
.space 40   @ 40b
forward_label_290:
.space 16   @ 16b  @ 16b  @ 16b
end_func_11:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 126 % {{space_mod|default("0x10000000")}}

.align	1
.space 31 % {{space_mod|default("0x10000000")}}

.align	1
.space 2042 % {{space_mod|default("0x10000000")}}
func_25:
.space 18   @ 18b  @ 18b
ldr	r9, =forward_label_481  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 122   @ 122b  @ 122b  @ 122b
forward_label_481:
.space 52   @ 52b  @ 52b
end_func_25:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 27 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 430 % {{space_mod|default("0x10000000")}}

.align	2
.space 6431 % {{space_mod|default("0x10000000")}}

.align	1
.space 7697 % {{space_mod|default("0x10000000")}}

.align	1
.space 3374 % {{space_mod|default("0x10000000")}}
label391:
.space 2  @ 2b  @ 2b  @ 2b
ldr	r6, =func_11  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r3, =forward_label_710  @ 2b  @ 2b  @ 2b  @ 2b
orr	r3, #1  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_485  @ 4b  @ 4b  @ 4b  @ 4b
.space 44   @ 44b  @ 44b
forward_label_710:
.space 30   @ 30b  @ 30b
post_branch_485:


end_label391:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 7 % {{space_mod|default("0x10000000")}}

.align	1
.space 4302 % {{space_mod|default("0x10000000")}}
label459:
.space 76   @ 76b  @ 76b
end_label459:
	b.w	{{jump_label459}}

.ltorg
.align	2
.space	0, 45
.space 9 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 1872 % {{space_mod|default("0x10000000")}}
label471:
.space 78   @ 78b  @ 78b
end_label471:
	b.w	{{jump_label471}}

.ltorg
.align	2
.space	2, 45
.space 16 % {{space_mod|default("0x10000000")}}

.align	1
.space 1946 % {{space_mod|default("0x10000000")}}

.align	1
.space 4102 % {{space_mod|default("0x10000000")}}
label558:
mov	r8, #10350  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r7, =forward_label_1019  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r1, cell_3591  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
orr	r7, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r2, cell_3590  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	it	ls  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	bxls	r7                            @ A7.7.20 @ 2b @ 2b @ 2b @ 2b @ 2b



.space 38   @ 38b  @ 38b  @ 38b
forward_label_1019:
	ldrsb	r3, [r1, r8]                  @ A7.7.61  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	nop	                              @ A7.7.88  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b  @ 8b
end_label558:
	b.w	{{jump_label558}}

.ltorg
.align	2
.space	3, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_3591
cell_3591:	.word	safeSpaceGpramSram-10011

.align	2
.space 15 % {{space_mod|default("0x10000000")}}
.global	cell_3590
cell_3590:	.word	safeSpaceGpramSram+96

.align	1
.space 4010 % {{space_mod|default("0x10000000")}}
label622:
.space 44   @ 44b  @ 44b  @ 44b
end_label622:
	b.w	{{jump_label622}}

.ltorg
.align	2
.space	0, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 457 % {{space_mod|default("0x10000000")}}



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
.space 30 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 73 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 204 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 468 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 24 % {{space_mod|default("0x10000000")}}



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