---
name: Fred-generated test
description: 'Test flow: (conf. 0) label148'
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
  jump_start: label148
  jump_label148: code_end
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
	mov.w	r0, #52519
	mov.w	r1, #20319
	mov.w	r3, #25467
	mov.w	r5, #58092
	mov.w	r6, #5735
	mov.w	r7, #10864
	mov.w	r8, #42629
	mov.w	r9, #21657
	mov.w	r10, #3566
	mov.w	r11, #3166
	mov.w	r12, #64662
	mov.w	r14, #31201

    @ Start the test
    b.w    start_test

.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r2, =stack
    add.w  r2, r2, #328
    mov.w  sp, r2

    @ Get counter address
    ldr.w  r2, =counter_idx
    ldr.w  r2, [r2]
    ldr.w  r4, =counters_to_test
    ldr.w  r2, [r4, r2]
    @ Get counter start value
    ldr.w  r4, [r2]
        @ r2 – counter address
        @ r4 – counter start value

    @ Jump to the 1st block

    b.w    {{jump_start}}

.ltorg


.align	1
.space 8700 % {{space_mod|default("0x10000000")}}
label148:
mov	r7, #59247  @ 4b  @ 4b
ldr	r3, cell_869  @ 4b  @ 4b
ldr	r14, cell_867  @ 4b @ looks important!  @ 4b @ looks important!
	ldr	r15, [r3, r7]                 @ A7.7.45x  @ 4b @ looks important!  @ 4b @ looks important!


forward_label_279: @ looks important!
	bx	r14                           @ A7.7.20  @ 2b @ looks important!  @ 2b @ looks important!

.space 24   @ 24b
forward_label_278:
.space 22   @ 22b
end_label148:
	b.w	{{jump_label148}}

.ltorg
.align	2
.space	1, 0xbf
.global	cell_869
cell_869:	.word	cell_868-59247

.space	2, 0xbf
.global	cell_867
cell_867:	.word	forward_label_278+1

.align	1
.space 25264 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:
    @ Get counter finish value
    ldr.w  r14, [r2]
    @ Calculate counter difference
    sub.w  r14, r14, r4
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r4, cyccnt_addr
    cmp.w  r2, r4
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{ saveValue("counters", r14, r2, r4) }}
    @ Save values of registers
	{{saveValue("registers", r0, r2, r4)}}
	{{saveValue("registers", r1, r2, r4)}}
	{{saveValue("registers", r3, r2, r4)}}
	{{saveValue("registers", r5, r2, r4)}}
	{{saveValue("registers", r6, r2, r4)}}
	{{saveValue("registers", r7, r2, r4)}}
	{{saveValue("registers", r8, r2, r4)}}
	{{saveValue("registers", r9, r2, r4)}}
	{{saveValue("registers", r10, r2, r4)}}
	{{saveValue("registers", r11, r2, r4)}}
	{{saveValue("registers", r12, r2, r4)}}
    @ Advance counter_idx and repeat or end the test
    ldr.w  r2, =counter_idx
    ldr.w  r4, [r2]
    add.w  r4, r4, #4
    str.w  r4, [r2]
    cmp.w  r4, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2

cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	0, 0xbf
.space 384 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 0xbf
.space 194 % {{space_mod|default("0x10000000")}}
.global	cell_868
cell_868:	.word	forward_label_279+1

.align	2
.space 560 % {{space_mod|default("0x10000000")}}



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