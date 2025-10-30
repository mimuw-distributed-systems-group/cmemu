---
name: Fred-generated test
description: 'Test flow: (conf. 0) label614 -> label35'
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
  jump_start: label614
  jump_label614: label35
  jump_label35: code_end
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
	mov.w	r0, #3025
	mov.w	r1, #19897
	mov.w	r2, #35394
	mov.w	r3, #58360
	mov.w	r4, #22109
	mov.w	r5, #41421
	mov.w	r6, #11074
	mov.w	r7, #25312
	mov.w	r8, #2646
	mov.w	r9, #38164
	mov.w	r10, #58768

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
.space 54 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	3, 45
.space 4209 % {{space_mod|default("0x10000000")}}
label35:
	sub	r13, #244  @ 2b  @ 2b  @ 2b
	ldr	r9, [r13]                     @ A7.7.43  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
	ldrd	r4, r2, cell_164              @ A7.7.51  @ 4b @ looks important!  @ 4b @ looks important!  @ 4b @ looks important!
.space 8   @ 8b  @ 8b
	add	r13, r13, #28                 @ A7.7.5  @ 2b  @ 2b  @ 2b
.space 12   @ 12b
	ldrd	r10, r5, [r13]                @ A7.7.50  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
	pop	{r1,r6,r10}                   @ A7.7.99  @ 4b  @ 4b  @ 4b
.space 8   @ 8b  @ 8b
mov	r10, #3  @ 4b  @ 4b  @ 4b
mov	r8, #0  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
	adds	r13, r13, r8                  @ A7.7.6  @ 4b  @ 4b  @ 4b
	str	r1, [r13, r10]                @ A7.7.162  @ 4b  @ 4b  @ 4b
.space 8   @ 8b
	ldrsh	r7, [r13, #204]!              @ A7.7.63  @ 4b  @ 4b  @ 4b
.space 14   @ 14b
end_label35:
	b.w	{{jump_label35}}

.ltorg
.align	2
.global	cell_164
cell_164:	.quad	0x523e04f28c1562a8

.space	2, 45
.space 14 % {{space_mod|default("0x10000000")}}
label614:
.space 54   @ 54b
end_label614:
	b.w	{{jump_label614}}

.ltorg
.align	2
.space	1, 45
.space 9 % {{space_mod|default("0x10000000")}}

.align	1
.space 2720 % {{space_mod|default("0x10000000")}}



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
.space 174 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 253 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	1, 46
.space 302 % {{space_mod|default("0x10000000")}}



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