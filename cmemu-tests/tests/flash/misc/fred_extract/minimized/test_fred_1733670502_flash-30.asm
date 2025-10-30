---
name: Fred-generated test
description: 'Test flow: (conf. 0) label214 -> label573'
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
  jump_start: label214
  jump_label214: label573
  jump_label573: code_end
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
	mov.w	r0, #23337
	mov.w	r1, #24066
	mov.w	r2, #44517
	mov.w	r3, #48988
	mov.w	r4, #44984
	mov.w	r5, #43899
	mov.w	r6, #50354
	mov.w	r7, #29682
	mov.w	r8, #24235
	mov.w	r9, #65204
	mov.w	r10, #55375

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
.space 190 % {{space_mod|default("0x10000000")}}
end_func_1:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 31982 % {{space_mod|default("0x10000000")}}
label214:
.space 12   @ 12b
ldr	r0, =table_47  @ 2b  @ 2b  @ 2b
mov	r10, #3  @ 4b  @ 4b  @ 4b
.space 4  @ 4b
ldr	r9, cell_1325  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
	tbb	[r0, r10]                     @ A7.7.185  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
label214_switch_1_case_1:
.space 20   @ 20b  @ 20b
label214_switch_1_case_2:
.space 14   @ 14b
label214_switch_1_case_3:
.space 12   @ 12b  @ 12b
label214_switch_1_case_4:
.space 54   @ 54b
end_label214:
	b.w	{{jump_label214}}

.ltorg
.align	2
.space	3, 45
.global	cell_1325
cell_1325:	.word	safeSpaceGpramSram+502

.space	3, 45
.space 9 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 11 % {{space_mod|default("0x10000000")}}

.align	1
.space 15456 % {{space_mod|default("0x10000000")}}

.align	1
.space 37404 % {{space_mod|default("0x10000000")}}
label573:
	sub	r13, #12  @ 2b  @ 2b  @ 2b
	ldrb	r1, [r13, #31]                @ A7.7.46  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
.space 32   @ 32b
	pop	{r3,r9-r10}                   @ A7.7.99  @ 4b  @ 4b  @ 4b
end_label573:
	b.w	{{jump_label573}}

.ltorg
.align	2
.space 19 % {{space_mod|default("0x10000000")}}

.align	1
.space 8270 % {{space_mod|default("0x10000000")}}



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
.space 259 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 237 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 46
.space 153 % {{space_mod|default("0x10000000")}}
.global	table_47
table_47:
.byte	0
.byte	((label214_switch_1_case_2-label214_switch_1_case_1)/2)
.byte	((label214_switch_1_case_3-label214_switch_1_case_1)/2)
.byte	((label214_switch_1_case_4-label214_switch_1_case_1)/2)

.space	0, 46
.space 4 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 146 % {{space_mod|default("0x10000000")}}



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