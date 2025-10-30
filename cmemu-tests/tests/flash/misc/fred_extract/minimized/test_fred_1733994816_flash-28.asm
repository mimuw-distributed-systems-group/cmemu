---
name: Fred-generated test
description: 'Test flow: (conf. 0) label177'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: false
  lb_en: true
  wb_en: false
  jump_start: label177
  jump_label177: code_end
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
	mov.w	r0, #7341
	mov.w	r1, #8470
	mov.w	r2, #62331
	mov.w	r3, #61262
	mov.w	r4, #37107
	mov.w	r5, #31838
	mov.w	r6, #57152
	mov.w	r7, #7887
	mov.w	r8, #51605
	mov.w	r9, #53366
	mov.w	r10, #7018

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
.space 110 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 25426 % {{space_mod|default("0x10000000")}}
label177:
.space 18   @ 18b
ldr	r4, =func_54  @ 2b  @ 2b  @ 2b
.space 10   @ 10b
orr	r4, #1  @ 4b  @ 4b  @ 4b
.space 32 
	cmp	r4, r0                        @ A7.7.28  @ 2b @ looks important!  @ 2b  @ 2b @ looks important!
	blx	r4                            @ A7.7.19  @ 2b @ looks important!  @ 2b  @ 2b @ looks important!


.space 110   @ 110b
end_label177:
	b.w	{{jump_label177}}

.ltorg
.align	2
.space	1, 45
.space 25 % {{space_mod|default("0x10000000")}}
func_54:
mov	r9, #24107  @ 4b  @ 4b  @ 4b
mov	r3, #50  @ 4b  @ 4b  @ 4b
ldr	r2, cell_2546  @ 4b  @ 4b  @ 4b
ldr	r1, =table_78  @ 2b  @ 2b  @ 2b @ looks important!
ldr	r10, cell_2545  @ 4b  @ 4b  @ 4b @ looks important!
ldr	r0, cell_2544  @ 4b  @ 4b  @ 4b
	strh	r0, [r13, r3]                 @ A7.7.171  @ 4b  @ 4b  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	iteee	lt  @ 2b  @ 2b  @ 2b
	ldrsblt	r3, [r2, r9]                  @ A7.7.61 @ 4b @ 4b @ 4b
	addge	r3, #150                      @ A7.7.3 @ 2b @ 2b @ 2b
	asrge	r3, r1, #31                   @ A7.7.10 @ 2b @ 2b @ 2b
	addsge	r3, r3, r2                    @ A7.7.4 @ 4b @ looks important! @ 4b @ 4b @ looks important!
 @ looks important!



.space 10   @ 10b
func_54_switch_1_case_1:
.space 4  @ 4b  @ 4b
func_54_switch_1_case_2:
.space 16   @ 16b
func_54_switch_1_case_3:
.space 16   @ 16b
func_54_switch_1_case_4:
.space 26   @ 26b  @ 26b
func_54_switch_1_case_5:
end_func_54:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_2545
cell_2545:	.word	safeSpaceGpramSram+147

.space	1, 45
.space 18 % {{space_mod|default("0x10000000")}}
.global	cell_2544
cell_2544:	.word	safeSpaceSram+108

.space	0, 45
.space 3 % {{space_mod|default("0x10000000")}}
.global	cell_2546
cell_2546:	.word	safeSpaceSram-23293

.space	1, 45
.space 8168 % {{space_mod|default("0x10000000")}}

.align	1
.space 22383 % {{space_mod|default("0x10000000")}}



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
.space 102 % {{space_mod|default("0x10000000")}}
.global	table_78
table_78:
.byte	0
.byte	((func_54_switch_1_case_2-func_54_switch_1_case_1)/2)
.byte	((func_54_switch_1_case_3-func_54_switch_1_case_1)/2)
.byte	((func_54_switch_1_case_4-func_54_switch_1_case_1)/2)
.byte	((func_54_switch_1_case_5-func_54_switch_1_case_1)/2)

.space	1, 46
.space 181 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 197 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	1, 46
.space 238 % {{space_mod|default("0x10000000")}}

.space	1, 46
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