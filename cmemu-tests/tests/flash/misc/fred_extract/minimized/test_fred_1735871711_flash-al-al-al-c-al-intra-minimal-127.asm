---
name: Fred-generated test
description: 'Test flow: (conf. 0) label1 -> label598 -> label2 -> label38 -> label393
  -> label162 -> label345 -> label526 -> label608 -> label143 -> label336'
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
  jump_start: label1
  jump_label1: label598
  jump_label598: label2
  jump_label2: label38
  jump_label38: label393
  jump_label393: label162
  jump_label162: label345
  jump_label345: label526
  jump_label526: label608
  jump_label608: label143
  jump_label143: label336
  jump_label336: code_end
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
	mov.w	r0, #12363
	mov.w	r1, #13832
	mov.w	r2, #50489
	mov.w	r3, #30716
	mov.w	r4, #29753
	mov.w	r5, #35965
	mov.w	r6, #40907
	mov.w	r7, #24953
	mov.w	r8, #5999
	mov.w	r9, #7491
	mov.w	r10, #28438

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
label1:
.space 106   @ 106b
end_label1:
	b.w	{{jump_label1}}

.ltorg
.align	2
.space	2, 45
.space 186 % {{space_mod|default("0x10000000")}}
label2:
.space 30   @ 30b  @ 30b  @ 30b
end_label2:
	b.w	{{jump_label2}}

.ltorg
.align	2
.space	3, 45
.space 289 % {{space_mod|default("0x10000000")}}
end_func_2:
	bx	r14

.ltorg
.align	2
.space	2, 46
.space 31 % {{space_mod|default("0x10000000")}}

.align	1
.space 2054 % {{space_mod|default("0x10000000")}}
end_label26:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 1362 % {{space_mod|default("0x10000000")}}
label38:
	sub	r13, #48  @ 2b  @ 2b  @ 2b  @ 2b
	cbnz	r1, forward_label_56          @ A7.7.21  @ 2b  @ 2b  @ 2b  @ 2b

.space 4
	strd	r4, r7, [r13, #-16]           @ A7.7.166  @ 4b  @ 4b  @ 4b  @ 4b
.space 54   @ 54b  @ 54b
forward_label_56:
	str	r3, [r13], #48                @ A7.7.161  @ 4b  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b
.space 22   @ 22b  @ 22b
end_label38:
	b.w	{{jump_label38}}

.ltorg
.align	2
.space	2, 45
.space 25 % {{space_mod|default("0x10000000")}}

.align	1
.space 8374 % {{space_mod|default("0x10000000")}}
end_label129:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 15 % {{space_mod|default("0x10000000")}}

.align	1
.space 1670 % {{space_mod|default("0x10000000")}}
label143:
.space 4
ldr	r1, cell_538  @ 4b  @ 4b  @ 4b  @ 4b
.space 20   @ 20b  @ 20b
ldr	r6, cell_537  @ 2b  @ 2b  @ 2b  @ 2b
.space 42   @ 42b  @ 42b
ldr	r6, =forward_label_238  @ 2b  @ 2b  @ 2b  @ 2b
.space 18   @ 18b  @ 18b
forward_label_238:
.space 8   @ 8b  @ 8b  @ 8b
end_label143:
	b.w	{{jump_label143}}

.ltorg
.align	2
.space	1, 45
.global	cell_539
cell_539:	.word	safeSpaceGpramSram+498

.space	0, 45
.global	cell_538
cell_538:	.word	safeSpaceFlash-37758

.space	1, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_537
cell_537:	.word	safeSpaceFlash+396

.align	1
.space 1508 % {{space_mod|default("0x10000000")}}

.align	1
.space 1328 % {{space_mod|default("0x10000000")}}
label162:
.space 2  @ 2b  @ 2b
ldr	r7, =forward_label_272  @ 2b  @ 2b  @ 2b  @ 2b
.space 26   @ 26b
forward_label_272:
.space 4  @ 4b  @ 4b  @ 4b
.space 4
.space 4  @ 4b  @ 4b  @ 4b
.space 4
.space 32   @ 32b  @ 32b
ldr	r2, =func_43  @ 2b  @ 2b  @ 2b  @ 2b
.space 22   @ 22b
end_label162:
	b.w	{{jump_label162}}

.ltorg
.align	2
.space	2, 45
.space 12 % {{space_mod|default("0x10000000")}}
.global	cell_629
cell_629:	.word	safeSpaceGpramSram+424

.align	1
func_26:
.space 200   @ 200b
end_func_26:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 29 % {{space_mod|default("0x10000000")}}

.align	1
.space 8489 % {{space_mod|default("0x10000000")}}

.align	1
.space 2405 % {{space_mod|default("0x10000000")}}

.align	1
.space 1954 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 2404 % {{space_mod|default("0x10000000")}}
label290:
.space 62   @ 62b
end_label290:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 15 % {{space_mod|default("0x10000000")}}

.align	1
.space 267 % {{space_mod|default("0x10000000")}}

.align	1
.space 1138 % {{space_mod|default("0x10000000")}}
func_43:
.space 120   @ 120b
end_func_43:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 1032 % {{space_mod|default("0x10000000")}}
label336:
.space 16   @ 16b  @ 16b
.space 2
.space 30   @ 30b  @ 30b
end_label336:
	b.w	{{jump_label336}}

.ltorg
.align	2
.space	0, 45
.global	cell_1253
cell_1253:	.word	safeSpaceSram-12489

.align	1
.space 1334 % {{space_mod|default("0x10000000")}}
label345:
.space 2  @ 2b  @ 2b  @ 2b
ldr	r3, =func_76  @ 2b  @ 2b  @ 2b  @ 2b
.space 4
.space 4
.space 4
.space 2  @ 2b
post_branch_447:


.space 84   @ 84b  @ 84b
end_label345:
	b.w	{{jump_label345}}

.ltorg
.align	2
.space	1, 45
.space 11 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 20 % {{space_mod|default("0x10000000")}}

.align	1
.space 2270 % {{space_mod|default("0x10000000")}}
label368:
.space 162 
end_label368:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 26 % {{space_mod|default("0x10000000")}}

.align	1
.space 3416 % {{space_mod|default("0x10000000")}}
label393:
.space 82   @ 82b
end_label393:
	b.w	{{jump_label393}}

.ltorg
.align	2
.space	2, 45
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 2034 % {{space_mod|default("0x10000000")}}

.align	1
.space 1570 % {{space_mod|default("0x10000000")}}

.align	1
.space 6905 % {{space_mod|default("0x10000000")}}

.align	1
.space 3458 % {{space_mod|default("0x10000000")}}
label503:
ldr	r0, =func_71  @ 2b  @ 2b  @ 2b
orr	r0, #1  @ 4b  @ 4b  @ 4b
.space 18   @ 18b
end_label503:
	b.w	{{code_end}}

.ltorg
.align	1
.space 1124 % {{space_mod|default("0x10000000")}}
label526:
.space 70   @ 70b
end_label526:
	b.w	{{jump_label526}}

.ltorg
.align	2
.space	1, 45
.space 343 % {{space_mod|default("0x10000000")}}
func_71:
.space 168 
end_func_71:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 59 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 1359 % {{space_mod|default("0x10000000")}}
label544:
.space 38   @ 38b
end_label544:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 199 % {{space_mod|default("0x10000000")}}
label547:
.space 66   @ 66b
end_label547:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 3 % {{space_mod|default("0x10000000")}}

.align	1
.space 2038 % {{space_mod|default("0x10000000")}}
end_label565:
	b.w	{{code_end}}

.ltorg
.align	1
.space 599 % {{space_mod|default("0x10000000")}}

.align	1
.space 3128 % {{space_mod|default("0x10000000")}}
label598:
.space 4  @ 4b  @ 4b
ldr	r10, =func_26  @ 4b  @ 4b  @ 4b  @ 4b
.space 26   @ 26b  @ 26b
orr	r10, #1  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_756  @ 4b  @ 4b  @ 4b  @ 4b
.space 4
.space 2  @ 2b  @ 2b
post_branch_756:


.space 22   @ 22b  @ 22b  @ 22b
end_label598:
	b.w	{{jump_label598}}

.ltorg
.align	1
.space 724 % {{space_mod|default("0x10000000")}}
func_76:
.space 190   @ 190b
end_func_76:
	bx	r14

.ltorg
.align	2
.space 310 % {{space_mod|default("0x10000000")}}
label608:
.space 76   @ 76b  @ 76b
end_label608:
	b.w	{{jump_label608}}

.ltorg
.align	1
.space 1490 % {{space_mod|default("0x10000000")}}

.align	1
.space 713 % {{space_mod|default("0x10000000")}}



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
.space 151 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 11 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 300 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 6 % {{space_mod|default("0x10000000")}}



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