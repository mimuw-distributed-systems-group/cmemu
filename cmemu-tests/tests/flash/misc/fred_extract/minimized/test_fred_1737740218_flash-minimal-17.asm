---
name: Fred-generated test
description: 'Test flow: (conf. 0) label95'
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
  jump_start: label95
  jump_label95: code_end
  code_end: code_end
  space_mod: 4
...

{% device:cache_enabled = cache_en %}
{% device:line_buffer_enabled = lb_en %}
{% device:write_buffer_enabled = wb_en %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Save original sp
    ldr.w  r0, =original_sp
    str.w  sp, [r0]

    b.w    tested_code
.thumb_func
end_label:
    @ Restore original sp
    ldr.w  r0, =original_sp
    ldr.w  sp, [r0]
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
    ldr.w  r1, [r7]

    @ Randomize values of registers
	mov.w	r2, #11904
	mov.w	r3, #12694
	mov.w	r4, #25633
	mov.w	r5, #12975
	mov.w	r6, #55692
	mov.w	r7, #378
	mov.w	r8, #45976
	mov.w	r9, #39821
	mov.w	r10, #35734
	mov.w	r11, #23904
	mov.w	r12, #33101

    @ Start the test
    b.w    start_test


.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r0, =stack
    add.w  r0, r0, #328
    mov.w  sp, r0

    @ Get counter address
    ldr.w  r0, =counter_idx
    ldr.w  r0, [r0]
    ldr.w  r1, =counters_to_test
    ldr.w  r0, [r1, r0]
    @ Get counter start value
    ldr.w  r1, [r0]
        @ r0 – counter address
        @ r1 – counter start value

    @ Jump to the 1st block
    b.w    {{jump_start}}
.ltorg



.align	1
.space 220 % {{space_mod|default("0x10000000")}}
end_func_1:
	bx	r14

.ltorg
.align	2
.space	1, 45
.space 3735 % {{space_mod|default("0x10000000")}}
label95:
.space 2  @ 2b  @ 2b
ldr	r14, =post_branch_118  @ 4b  @ 4b  @ 4b
ldr	r10, =func_41  @ 4b  @ 4b  @ 4b
.space 24   @ 24b
orr	r14, #1  @ 4b  @ 4b  @ 4b
orr	r10, #1  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	bx	r10                           @ A7.7.20  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_118:


end_label95:
	b.w	{{jump_label95}}

.ltorg
.align	2
.space	1, 45
.space 7 % {{space_mod|default("0x10000000")}}
func_41:
.space 4  @ 4b
ldr	r11, =forward_label_568  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
mov	r10, #36226  @ 4b  @ 4b  @ 4b
.space 10   @ 10b
orr	r11, #1  @ 4b  @ 4b  @ 4b
.space 14   @ 14b  @ 14b
forward_label_568:
.space 4  @ 4b  @ 4b
ldr	r10, cell_1352  @ 4b  @ 4b  @ 4b
.space 18   @ 18b
	ldr	r11, [r10, #39]!              @ A7.7.43  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	clz	r9, r8                        @ A7.7.24  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	ldrsb	r10, cell_1351                @ A7.7.60  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	nop	                              @ A7.7.88  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
.space 16   @ 16b @ looks important!  @ 16b @ looks important!
end_func_41:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_1351
cell_1351:	.byte	0x06

.space	2, 45
.space 7 % {{space_mod|default("0x10000000")}}
.global	cell_1352
cell_1352:	.word	safeSpaceFlash+738

.align	1
.space 12940 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:

    @ Get counter finish value
    ldr.w  r14, [r0]
    @ Calculate counter difference
    sub.w  r14, r14, r1
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r1, cyccnt_addr
    cmp.w  r0, r1
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{saveValue("counters", r14, r0, r1)}}

    @ Save values of registers
	{{saveValue("registers", r2, r0, r1)}}
	{{saveValue("registers", r3, r0, r1)}}
	{{saveValue("registers", r4, r0, r1)}}
	{{saveValue("registers", r5, r0, r1)}}
	{{saveValue("registers", r6, r0, r1)}}
	{{saveValue("registers", r7, r0, r1)}}
	{{saveValue("registers", r8, r0, r1)}}
	{{saveValue("registers", r9, r0, r1)}}
	{{saveValue("registers", r10, r0, r1)}}
	{{saveValue("registers", r11, r0, r1)}}
	{{saveValue("registers", r12, r0, r1)}}

    @ Advance counter_idx and repeat or end the test
    ldr.w  r0, =counter_idx
    ldr.w  r1, [r0]
    add.w  r1, r1, #4
    str.w  r1, [r0]
    cmp.w  r1, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2
cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	3, 46
.space 84 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 491 % {{space_mod|default("0x10000000")}}



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