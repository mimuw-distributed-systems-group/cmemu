---
name: Fred-generated test
description: 'Test flow: (conf. 0) label253'
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
  jump_start: label253
  jump_label253: code_end
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
	mov.w	r0, #6793
	mov.w	r1, #53668
	mov.w	r2, #14859
	mov.w	r3, #10956
	mov.w	r4, #37914
	mov.w	r5, #55839
	mov.w	r6, #55803
	mov.w	r7, #39518
	mov.w	r8, #58840
	mov.w	r9, #3930
	mov.w	r10, #42185

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
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 14731 % {{space_mod|default("0x10000000")}}
label253:
ldr	r0, =func_74  @ 2b  @ 2b  @ 2b
.space 44   @ 44b
orr	r0, #1  @ 4b  @ 4b @ looks important!  @ 4b
	blx	r0                            @ A7.7.19  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
 @ looks important! @ looks important!

.space 4  @ 4b  @ 4b
end_label253:
	b.w	{{jump_label253}}

.ltorg
.align	2
.space	1, 45
.space 9 % {{space_mod|default("0x10000000")}}
func_74:
	sub	r13, #28  @ 2b  @ 2b  @ 2b
ldr	r1, =forward_label_1094  @ 2b  @ 2b  @ 2b
orr	r1, #1  @ 4b  @ 4b  @ 4b
mov	r3, #2  @ 4b  @ 4b  @ 4b
.space 2  @ 2b
mov	r9, #51  @ 4b  @ 4b  @ 4b
	itt	ls  @ 2b  @ 2b  @ 2b
	andls	r2, r9, #23                   @ A7.7.8 @ 4b @ 4b @ 4b
	bxls	r1                            @ A7.7.20 @ 2b @ looks important! @ 2b @ looks important! @ 2b @ looks important!
 @ looks important!
 @ looks important!
 @ looks important!

	ldrd	r1, r2, [r13]                 @ A7.7.50  @ 4b  @ 4b  @ 4b
	strh	r3, [r13, r9]                 @ A7.7.171  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
ldr	r10, cell_3625  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
ldr	r1, cell_3624  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
mov	r2, #8  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
	strb	r8, [r13, r2, LSL #3]         @ A7.7.164  @ 4b  @ 4b  @ 4b
.space 2  @ 2b  @ 2b

forward_label_1094:
	ldrsh	r10, [r13, r9]                @ A7.7.65  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
.space 8   @ 8b  @ 8b @ looks important!
	strh	r4, [r13, r3, LSL #1]         @ A7.7.171  @ 4b  @ 4b  @ 4b
.space 20   @ 20b
ldr	r10, cell_3619  @ 4b  @ 4b  @ 4b
.space 28   @ 28b
	ldrd	r1, r2, [r13, #-228]!         @ A7.7.50  @ 4b  @ 4b  @ 4b
mov	r2, #5  @ 4b  @ 4b  @ 4b
ldr	r10, cell_3617  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	adds	r13, #32                      @ A7.7.5  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
	ldrsb	r10, [r13, r2, LSL #1]        @ A7.7.61  @ 4b  @ 4b  @ 4b
.space 34   @ 34b
	ldr	r1, [r13, #224]!              @ A7.7.43  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
end_func_74:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_3617
cell_3617:	.word	safeSpaceGpramSram+946

.space	1, 45
.space 21 % {{space_mod|default("0x10000000")}}
.global	cell_3624
cell_3624:	.word	safeSpaceGpramSram+652

.space	1, 45
.global	cell_3625
cell_3625:	.word	safeSpaceGpramSram+498

.space	3, 45
.global	cell_3619
cell_3619:	.word	safeSpaceFlash+455

.align	1
.space 1635 % {{space_mod|default("0x10000000")}}



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
.space 211 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 207 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	0, 46
.space 288 % {{space_mod|default("0x10000000")}}



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