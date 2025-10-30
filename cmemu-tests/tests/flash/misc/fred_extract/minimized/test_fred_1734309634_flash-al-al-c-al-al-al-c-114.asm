---
name: Fred-generated test
description: 'Test flow: (conf. 0) label384 -> label370 -> label259 -> label453'
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
  jump_start: label384
  jump_label384: label370
  jump_label370: label259
  jump_label259: label453
  jump_label453: code_end
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
	mov.w	r0, #3070
	mov.w	r1, #39242
	mov.w	r2, #18010
	mov.w	r3, #36551
	mov.w	r4, #4086
	mov.w	r5, #4543
	mov.w	r6, #28196
	mov.w	r7, #53340
	mov.w	r8, #14888
	mov.w	r9, #6323
	mov.w	r10, #50530

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
.space 92 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 46
.space 309 % {{space_mod|default("0x10000000")}}

.align	1
.space 1941 % {{space_mod|default("0x10000000")}}

.align	1
.space 2045 % {{space_mod|default("0x10000000")}}

.align	1
.space 3420 % {{space_mod|default("0x10000000")}}

.align	1
.space 1780 % {{space_mod|default("0x10000000")}}

.align	1
.space 2046 % {{space_mod|default("0x10000000")}}

.align	1
.space 1919 % {{space_mod|default("0x10000000")}}

.align	1
.space 2684 % {{space_mod|default("0x10000000")}}
func_19:
	sub	r13, #168  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r2, cell_833  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r1, cell_832  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r0, cell_830  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 10   @ 10b  @ 10b  @ 10b  @ 10b
ldr	r9, cell_829  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
mov	r3, #2  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 34 
	itt	ls  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	addwls	r0, r13, #3334                @ A7.7.5 @ 4b @ 4b @ 4b @ 4b @ 4b
	tstls	r2, r9, LSL #2                @ A7.7.189 @ 4b @ 4b @ 4b @ 4b @ 4b
mov	r0, #5  @ 4b @ looks important!  @ 4b  @ 4b  @ 4b  @ 4b
	strb	r9, [r2, r3]                  @ A7.7.164  @ 4b @ looks important!  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b @ looks important!  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!
	itett	ne  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	ldrbne	r1, [r13, r3, LSL #1]         @ A7.7.48 @ 4b @ 4b @ 4b @ 4b @ 4b
	lsreq	r3, r5                        @ A7.7.71 @ 2b @ 2b @ 2b @ 2b @ 2b
	asrne	r3, r7                        @ A7.7.11 @ 2b @ 2b @ 2b @ 2b @ 2b
	ldrsbne	r1, [r13]                     @ A7.7.59 @ 4b @ 4b @ 4b @ 4b @ 4b
mov	r2, #108  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	itt	pl  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	ldrsbpl	r3, cell_826                  @ A7.7.60 @ 4b @ 4b @ 4b @ 4b @ 4b
	addwpl	r3, r13, #3384                @ A7.7.5 @ 4b @ 4b @ 4b @ 4b @ 4b

mov	r3, #60  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	add	r13, r3                       @ A7.7.6  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 14   @ 14b
	str	r3, [r13, r0, LSL #3]         @ A7.7.162  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
	adds	r13, r2                       @ A7.7.6  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
	strh	r7, [r13, #52]                @ A7.7.170  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
end_func_19:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_832
cell_832:	.word	safeSpaceGpramSram+592

.space	2, 45
.global	cell_829
cell_829:	.word	safeSpaceSram+288

.space	2, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_830
cell_830:	.word	safeSpaceGpramSram+64

.space	0, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_826
cell_826:	.byte	0x09

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_833
cell_833:	.word	safeSpaceSram+475

.align	2
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 1027 % {{space_mod|default("0x10000000")}}

.align	1
.space 2092 % {{space_mod|default("0x10000000")}}
end_label166:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 14 % {{space_mod|default("0x10000000")}}

.align	1
.space 2458 % {{space_mod|default("0x10000000")}}

.align	1
.space 3374 % {{space_mod|default("0x10000000")}}

.align	1
.space 2524 % {{space_mod|default("0x10000000")}}

.align	1
.space 2644 % {{space_mod|default("0x10000000")}}
label259:
.space 134   @ 134b
end_label259:
	b.w	{{jump_label259}}

.ltorg
.align	2
.space	2, 45
.space 18 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 16 % {{space_mod|default("0x10000000")}}

.align	1
.space 9178 % {{space_mod|default("0x10000000")}}

.align	1
.space 1446 % {{space_mod|default("0x10000000")}}

.align	1
.space 1524 % {{space_mod|default("0x10000000")}}
label370:
.space 84   @ 84b  @ 84b  @ 84b
end_label370:
	b.w	{{jump_label370}}

.ltorg
.align	2
.space	2, 45
.space 21 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 1852 % {{space_mod|default("0x10000000")}}
label384:
ldr	r2, cell_2194  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!
.space 90   @ 90b  @ 90b
end_label384:
	b.w	{{jump_label384}}

.ltorg
.align	2
.space	2, 45
.space 24 % {{space_mod|default("0x10000000")}}
.global	cell_2194
cell_2194:	.word	safeSpaceSram+392

.align	2
.space 1678 % {{space_mod|default("0x10000000")}}

.align	1
.space 600 % {{space_mod|default("0x10000000")}}

.align	1
.space 1689 % {{space_mod|default("0x10000000")}}

.align	1
.space 1895 % {{space_mod|default("0x10000000")}}

.align	1
.space 1808 % {{space_mod|default("0x10000000")}}
end_label444:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 14 % {{space_mod|default("0x10000000")}}

.align	1
.space 311 % {{space_mod|default("0x10000000")}}

.align	1
.space 1190 % {{space_mod|default("0x10000000")}}
label453:
.space 8   @ 8b  @ 8b  @ 8b
ldr	r6, =func_19  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 6   @ 6b  @ 6b  @ 6b
ldr	r14, =post_branch_572  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 46   @ 46b  @ 46b  @ 46b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b  @ 4b
orr	r6, #1  @ 4b @ looks important!  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!
	bx	r6                            @ A7.7.20  @ 2b @ looks important!  @ 2b  @ 2b  @ 2b  @ 2b @ looks important!
post_branch_572:


.space 4  @ 4b  @ 4b  @ 4b  @ 4b
end_label453:
	b.w	{{jump_label453}}

.ltorg
.align	2
.space	0, 45
.space 13 % {{space_mod|default("0x10000000")}}

.align	1
.space 2497 % {{space_mod|default("0x10000000")}}

.align	1
.space 10232 % {{space_mod|default("0x10000000")}}
end_label542:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.space 20 % {{space_mod|default("0x10000000")}}

.align	1
.space 5152 % {{space_mod|default("0x10000000")}}

.align	1
.space 1554 % {{space_mod|default("0x10000000")}}

.align	1
.space 2714 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 4 % {{space_mod|default("0x10000000")}}

.align	1
.space 2010 % {{space_mod|default("0x10000000")}}



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
.space 195 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 17 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 318 % {{space_mod|default("0x10000000")}}



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