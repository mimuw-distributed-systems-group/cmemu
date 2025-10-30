---
name: Fred-generated test
description: 'Test flow: (conf. 0) label492 -> label202 -> label524 -> label324 ->
  label505'
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
  jump_start: label492
  jump_label492: label202
  jump_label202: label524
  jump_label524: label324
  jump_label324: label505
  jump_label505: code_end
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
	mov.w	r0, #48382
	mov.w	r1, #10311
	mov.w	r2, #49768
	mov.w	r3, #16866
	mov.w	r4, #37829
	mov.w	r5, #38908
	mov.w	r6, #55562
	mov.w	r7, #42811
	mov.w	r8, #51204
	mov.w	r9, #61144
	mov.w	r10, #26761

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
.space 144 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	2, 45
.space 4622 % {{space_mod|default("0x10000000")}}
func_5:
.space 194   @ 194b  @ 194b  @ 194b
end_func_5:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 44 % {{space_mod|default("0x10000000")}}

.align	1
.space 2259 % {{space_mod|default("0x10000000")}}

.align	1
.space 7820 % {{space_mod|default("0x10000000")}}
func_16:
.space 188   @ 188b
end_func_16:
	bx	r14

.ltorg
.align	2
.space	1, 45
.space 27 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 1812 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 2561 % {{space_mod|default("0x10000000")}}

.align	1
.space 1297 % {{space_mod|default("0x10000000")}}

.align	1
.space 1216 % {{space_mod|default("0x10000000")}}
label202:
ldr	r5, =func_16  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 16   @ 16b  @ 16b  @ 16b
ldr	r14, =post_branch_254  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 10   @ 10b  @ 10b  @ 10b

ldr	r0, cell_1231  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 46   @ 46b  @ 46b  @ 46b  @ 46b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 22   @ 22b  @ 22b  @ 22b  @ 22b
orr	r5, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b  @ 4b
	movw	r0, #2185                     @ A7.7.76  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	mov	r15, r5                       @ A7.7.77  @ 2b  @ 2b  @ 2b  @ 2b @ looks important!  @ 2b @ looks important!
post_branch_254:


.space 4  @ 4b  @ 4b  @ 4b  @ 4b
end_label202:
	b.w	{{jump_label202}}

.ltorg
.align	2
.space	3, 45
.space 13 % {{space_mod|default("0x10000000")}}
.global	cell_1231
cell_1231:	.word	safeSpaceGpramSram-91372

.space	3, 45
.space 273 % {{space_mod|default("0x10000000")}}

.align	1
.space 2077 % {{space_mod|default("0x10000000")}}

.align	1
.space 1434 % {{space_mod|default("0x10000000")}}

.align	1
.space 1341 % {{space_mod|default("0x10000000")}}

.align	1
.space 1413 % {{space_mod|default("0x10000000")}}

.align	1
.space 4604 % {{space_mod|default("0x10000000")}}
label324:
.space 100   @ 100b  @ 100b
end_label324:
	b.w	{{jump_label324}}

.ltorg
.align	2
.space	1, 45
.space 24 % {{space_mod|default("0x10000000")}}

.align	1
.space 1770 % {{space_mod|default("0x10000000")}}

.align	1
.space 3971 % {{space_mod|default("0x10000000")}}

.align	1
.space 1754 % {{space_mod|default("0x10000000")}}
end_label376:
	b.w	{{code_end}}

.ltorg
.align	1
.space 1449 % {{space_mod|default("0x10000000")}}

.align	1
.space 1428 % {{space_mod|default("0x10000000")}}

.align	1
.space 8394 % {{space_mod|default("0x10000000")}}
label492:
.space 8   @ 8b  @ 8b
ldr	r4, cell_3015  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r14, =post_branch_626  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
mov	r2, #7  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 36   @ 36b  @ 36b
orr	r14, #1  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 26   @ 26b  @ 26b
	mul	r2, r4, r0                    @ A7.7.84  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	bvc	func_5                        @ A7.7.12  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
post_branch_626:


.space 16   @ 16b  @ 16b
end_label492:
	b.w	{{jump_label492}}

.ltorg
.align	2
.space	0, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_3015
cell_3015:	.word	safeSpaceGpramSram-24393

.space	0, 45
.space 31 % {{space_mod|default("0x10000000")}}

.align	1
.space 1374 % {{space_mod|default("0x10000000")}}
label505:
.space 2  @ 2b  @ 2b  @ 2b  @ 2b
ldr	r0, cell_3084  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 8   @ 8b
ldr	r5, =forward_label_903  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 4  @ 4b
ldr	r7, cell_3082  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
.space 2  @ 2b  @ 2b
mov	r8, #52120  @ 4b  @ 4b  @ 4b  @ 4b  @ 4b
.space 6   @ 6b
	ldrh	r2, cell_3079                 @ A7.7.56  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b  @ 4b  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	itett	ge  @ 2b  @ 2b  @ 2b  @ 2b  @ 2b
	ldrbge	r2, [r7, r8]                  @ A7.7.48 @ 4b @ 4b @ 4b @ 4b @ 4b
	tstlt	r5, r5                        @ A7.7.189 @ 2b @ 2b @ 2b @ 2b @ 2b
	ldrdge	r8, r7, [r0, #-192]           @ A7.7.50 @ 4b @ 4b @ 4b @ 4b @ 4b
	ldrge	r7, cell_3078                 @ A7.7.44 @ 4b @ 4b @ 4b @ 4b @ looks important! @ 4b @ looks important!



.space 66   @ 66b  @ 66b
forward_label_903:
.space 8   @ 8b  @ 8b
end_label505:
	b.w	{{jump_label505}}

.ltorg
.align	2
.space	1, 45
.global	cell_3084
cell_3084:	.word	safeSpaceGpramSram+324

.space	1, 45
.global	cell_3078
cell_3078:	.word	0xbef73396

.space	2, 45
.global	cell_3082
cell_3082:	.word	safeSpaceGpramSram-51183

.space	2, 45
.space 24 % {{space_mod|default("0x10000000")}}
.global	cell_3079
cell_3079:	.short	0xfead

.space	3, 45
.space 301 % {{space_mod|default("0x10000000")}}
label524:
.space 108   @ 108b
end_label524:
	b.w	{{jump_label524}}

.ltorg
.align	2
.space	1, 45
.space 15 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 11 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 2191 % {{space_mod|default("0x10000000")}}

.align	1
.space 3841 % {{space_mod|default("0x10000000")}}

.align	1
.space 3870 % {{space_mod|default("0x10000000")}}

.align	1
.space 1208 % {{space_mod|default("0x10000000")}}



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
.space 58 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 72 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 202 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 289 % {{space_mod|default("0x10000000")}}



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