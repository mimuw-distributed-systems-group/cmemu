---
name: Fred-generated test
description: 'Test flow: (conf. 0) label604'
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
  jump_start: label604
  jump_label604: code_end
  code_end: code_end
  space_mod: 8
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
	mov.w	r0, #23987
	mov.w	r1, #19216
	mov.w	r2, #2583
	mov.w	r3, #37029
	mov.w	r4, #34114
	mov.w	r5, #37652
	mov.w	r6, #21358
	mov.w	r7, #51329
	mov.w	r8, #7538
	mov.w	r9, #55778
	mov.w	r10, #31982

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
.space	0, 45
.space 34852 % {{space_mod|default("0x10000000")}}
label604:
	sub	r13, #52  @ 2b
mov	r5, #2  @ 4b
mov	r4, #3  @ 4b
ldr	r7, cell_3794  @ 2b
.space 4
	adc	r9, r9, r2, LSL #3            @ A7.7.2  @ 4b @ looks important!
	ittee	cc  @ 2b
	mlacc	r7, r5, r2, r5                @ A7.7.74 @ 4b
	asrcc	r7, r7, r8                    @ A7.7.11 @ 4b
	strbcs	r6, [r7]                      @ A7.7.163 @ 2b
	movcs	r2, r15                       @ A7.7.77 @ 2b @ looks important!
.space 4
ldr	r1, cell_3793  @ 4b
	ldrsh	r3, [r13, r4, LSL #3]         @ A7.7.65  @ 4b
	ldrsh	r7, [r13, r4, LSL #1]         @ A7.7.65  @ 4b
	pop	{r4,r10}                      @ A7.7.99  @ 4b
	ldrb	r4, [r13, #60]                @ A7.7.46  @ 4b
.space 4
	ldr	r7, [r13, r5, LSL #2]         @ A7.7.45  @ 4b
.space 4
mov	r5, #2  @ 4b
ldr	r9, cell_3791  @ 4b
.space 4
	ldrb	r10, [r13], #-160             @ A7.7.46  @ 4b
.space 2
	ldrh	r4, [r13, r5, LSL #1]         @ A7.7.57  @ 4b
	strd	r2, r7, [r13], #204           @ A7.7.166  @ 4b
.space 4
mov	r2, #6  @ 4b
	itee	cs  @ 2b
	ldrhcs	r1, [r13, r2, LSL #1]         @ A7.7.57  @ 4b
ldrcc	r5, cell_3790  @ 4b
	ldrbcc	r7, [r5], #-34                @ A7.7.46  @ 4b
.space 4
end_label604:
	b.w	{{jump_label604}}

.ltorg
.align	2
.space	2, 45
.global	cell_3793
cell_3793:	.word	safeSpaceSram+666

.space	2, 45
.space 4 % {{space_mod|default("0x10000000")}}
.global	cell_3794
cell_3794:	.word	safeSpaceGpramSram+379

.space	3, 45
.global	cell_3791
cell_3791:	.word	safeSpaceSram+727

.space	3, 45
.global	cell_3790
cell_3790:	.word	safeSpaceGpramSram+616

.align	1
.space 1225 % {{space_mod|default("0x10000000")}}



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
.space 239 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 46
.space 228 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	0, 46
.space 210 % {{space_mod|default("0x10000000")}}



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