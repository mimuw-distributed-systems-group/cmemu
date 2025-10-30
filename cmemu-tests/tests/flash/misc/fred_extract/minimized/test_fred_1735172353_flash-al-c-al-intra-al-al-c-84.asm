---
name: Fred-generated test
description: 'Test flow: (conf. 0) label445 -> label47 -> label554 -> label433'
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
  jump_start: label445
  jump_label445: label47
  jump_label47: label554
  jump_label554: label433
  jump_label433: code_end
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
	mov.w	r0, #31394
	mov.w	r1, #61079
	mov.w	r2, #59990
	mov.w	r3, #9047
	mov.w	r4, #21001
	mov.w	r5, #5474
	mov.w	r6, #49070
	mov.w	r7, #16780
	mov.w	r8, #17164
	mov.w	r9, #56179
	mov.w	r10, #21921

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
.space	2, 45
.space 1549 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 3060 % {{space_mod|default("0x10000000")}}
label47:
	sub	r13, #8  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r14, =post_branch_54  @ 4b  @ 4b  @ 4b  @ 4b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r8, cell_280  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b
	b	func_31                       @ A7.7.12  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b
post_branch_54:


.space 4  @ 4b  @ 4b
	strb	r7, [r13, #0]                 @ A7.7.163  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b
	nop	                              @ A7.7.88  @ 2b  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b
.space 4  @ 4b
	ldrh	r5, [r13, #8]!                @ A7.7.55  @ 4b  @ 4b  @ 4b  @ 4b
.space 16   @ 16b  @ 16b  @ 16b
end_label47:
	b.w	{{jump_label47}}

.ltorg
.align	2
.space	3, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_280
cell_280:	.word	safeSpaceGpramSram+719

.align	1
.space 278 % {{space_mod|default("0x10000000")}}

.align	1
.space 6139 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 1930 % {{space_mod|default("0x10000000")}}

.align	2
.space 17 % {{space_mod|default("0x10000000")}}

.align	1
.space 1750 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 12 % {{space_mod|default("0x10000000")}}

.align	1
.space 3956 % {{space_mod|default("0x10000000")}}

.align	1
.space 8513 % {{space_mod|default("0x10000000")}}

.align	1
.space 5682 % {{space_mod|default("0x10000000")}}
func_31:
.space 36   @ 36b
ldr	r3, cell_1754  @ 2b  @ 2b  @ 2b  @ 2b
.space 62   @ 62b  @ 62b
ldr	r3, cell_1750  @ 4b  @ 4b  @ 4b  @ 4b
.space 52   @ 52b  @ 52b
end_func_31:
	bx	r14

.ltorg
.align	2
.space	1, 45
.space 20 % {{space_mod|default("0x10000000")}}
.global	cell_1750
cell_1750:	.word	safeSpaceSram+871

.space	1, 45
.space 2 % {{space_mod|default("0x10000000")}}
.global	cell_1754
cell_1754:	.word	safeSpaceFlash+718

.align	1
.space 7010 % {{space_mod|default("0x10000000")}}

.align	1
.space 5618 % {{space_mod|default("0x10000000")}}

.align	1
.space 5844 % {{space_mod|default("0x10000000")}}
label433:
.space 8   @ 8b
ldr	r0, cell_2617  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b
.space 74   @ 74b
end_label433:
	b.w	{{jump_label433}}

.ltorg
.align	2
.space	3, 45
.space 10 % {{space_mod|default("0x10000000")}}
.global	cell_2617
cell_2617:	.word	safeSpaceSram-60595

.align	1
.space 810 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 1124 % {{space_mod|default("0x10000000")}}
label445:
.space 156   @ 156b  @ 156b
end_label445:
	b.w	{{jump_label445}}

.ltorg
.align	2
.space 46 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 1685 % {{space_mod|default("0x10000000")}}

.align	1
.space 4300 % {{space_mod|default("0x10000000")}}

.align	1
.space 3019 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 12 % {{space_mod|default("0x10000000")}}

.align	1
.space 1062 % {{space_mod|default("0x10000000")}}
label554:
.space 94   @ 94b  @ 94b
end_label554:
	b.w	{{jump_label554}}

.ltorg
.align	2
.space	2, 45
.space 1870 % {{space_mod|default("0x10000000")}}

.align	1
.space 1192 % {{space_mod|default("0x10000000")}}

.align	1
.space 113 % {{space_mod|default("0x10000000")}}



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
.space 83 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 68 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 95 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 279 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 131 % {{space_mod|default("0x10000000")}}



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