---
name: Fred-generated test
description: 'Test flow: (conf. 0) label365'
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
  jump_start: label365
  jump_label365: code_end
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
	mov.w	r0, #8117
	mov.w	r1, #18265
	mov.w	r2, #22160
	mov.w	r3, #21112
	mov.w	r4, #19981
	mov.w	r5, #26582
	mov.w	r8, #19202
	mov.w	r9, #28224
	mov.w	r10, #44815
	mov.w	r11, #45584
	mov.w	r12, #31836
	mov.w	r14, #40175

    @ Start the test
    b.w    start_test

.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r7, =stack
    add.w  r7, r7, #328
    mov.w  sp, r7

    @ Get counter address
    ldr.w  r7, =counter_idx
    ldr.w  r7, [r7]
    ldr.w  r6, =counters_to_test
    ldr.w  r7, [r6, r7]
    @ Get counter start value
    ldr.w  r6, [r7]
        @ r7 – counter address
        @ r6 – counter start value

    @ Jump to the 1st block

    b.w    {{jump_start}}

.ltorg


.align	1
.space 18546 % {{space_mod|default("0x10000000")}}
label365:
mov	r10, r13  @ 2b  @ 2b
ldr	r13, cell_1618  @ 4b  @ 4b
mov	r0, #2  @ 4b  @ 4b
jump_from_13:
	add	r15, r13, r15                 @ A7.7.46  @ 2b  @ 2b

.space 10   @ 10b
	cpy	r12, r13                      @ A7.7.30  @ 2b  @ 2b
.space 40 
forward_label_602:
mov	r13, r10  @ 2b  @ 2b
.space 12   @ 12b
	ldrsh	r1, [r13, r0, LSL #2]         @ A7.7.65  @ 4b  @ 4b
.space 20   @ 20b
	subs	r10, r13, #255                @ A7.7.176  @ 4b  @ 4b
.space 8   @ 8b
end_label365:
	b.w	{{jump_label365}}

.ltorg
.align	2
.space	2, 0xbf
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_1618
cell_1618:	.word	(forward_label_602 - jump_from_13 - 4)

.align	1
.space 12400 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:
    @ Get counter finish value
    ldr.w  r14, [r7]
    @ Calculate counter difference
    sub.w  r14, r14, r6
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r6, cyccnt_addr
    cmp.w  r7, r6
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{ saveValue("counters", r14, r7, r6) }}
    @ Save values of registers
	{{saveValue("registers", r0, r7, r6)}}
	{{saveValue("registers", r1, r7, r6)}}
	{{saveValue("registers", r2, r7, r6)}}
	{{saveValue("registers", r3, r7, r6)}}
	{{saveValue("registers", r4, r7, r6)}}
	{{saveValue("registers", r5, r7, r6)}}
	{{saveValue("registers", r8, r7, r6)}}
	{{saveValue("registers", r9, r7, r6)}}
	{{saveValue("registers", r10, r7, r6)}}
	{{saveValue("registers", r11, r7, r6)}}
	{{saveValue("registers", r12, r7, r6)}}
    @ Advance counter_idx and repeat or end the test
    ldr.w  r7, =counter_idx
    ldr.w  r6, [r7]
    add.w  r6, r6, #4
    str.w  r6, [r7]
    cmp.w  r6, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2

cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	1, 0xbf
.space 163 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 0xbf
.space 419 % {{space_mod|default("0x10000000")}}



@ safeSpaces:
{{section('flash')}}
.align  4
.global safeSpaceFlash
safeSpaceFlash:      .space  1024, 0xbf       @ See DataAddrConstraint in instructions/constraints.py
.size               safeSpaceFlash, .-safeSpaceFlash

{{section('sram')}}
.align  4
.global safeSpaceSram
safeSpaceSram:      .space  1024, 0xbf       @ See DataAddrConstraint in instructions/constraints.py
.size               safeSpaceSram, .-safeSpaceSram

{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  4
.global safeSpaceGpramSram
safeSpaceGpramSram: .space  1024, 0xbf       @ See DataAddrConstraint in instructions/constraints.py
.size               safeSpaceGpramSram, .-safeSpaceGpramSram


@ Stack:
{{section('sram')}}
.align  4
.global stack
stack:  .space  400, 0xbf    @ 256B of stack + upper and lower safety offsets for ldm/stm
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