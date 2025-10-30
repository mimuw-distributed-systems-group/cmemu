---
name: Fred-generated test
description: 'Test flow: (conf. 0) label606'
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
  jump_start: label606
  jump_label606: code_end
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
	mov.w	r0, #2242
	mov.w	r1, #24196
	mov.w	r2, #47148
	mov.w	r3, #5567
	mov.w	r4, #61790
	mov.w	r5, #60132
	mov.w	r6, #10002
	mov.w	r7, #1793
	mov.w	r8, #56798
	mov.w	r9, #42063
	mov.w	r10, #5861

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
.space	2, 45
.space 18296 % {{space_mod|default("0x10000000")}}
func_36:
.space 2  @ 2b  @ 2b
ldr	r2, =forward_label_543  @ 2b  @ 2b  @ 2b
orr	r2, #1  @ 4b  @ 4b  @ 4b
mov	r10, #7  @ 4b  @ 4b  @ 4b
mov	r3, #17645  @ 4b  @ 4b  @ 4b
.space 20   @ 20b
	bx	r2                            @ A7.7.20  @ 2b  @ 2b  @ 2b

.space 20   @ 20b  @ 20b  @ 20b
mov	r3, #34  @ 4b  @ 4b  @ 4b
.space 12   @ 12b  @ 12b
forward_label_543:
.space 6   @ 6b  @ 6b  @ 6b
ldr	r10, cell_1952  @ 4b  @ 4b  @ 4b
.space 44   @ 44b  @ 44b
ldr	r10, cell_1949  @ 4b  @ 4b  @ 4b
ldr	r1, cell_1948  @ 2b  @ 2b  @ 2b
.space 8   @ 8b  @ 8b  @ 8b
	ite	vc  @ 2b  @ 2b  @ 2b
	lsrvc	r3, r6, r10                   @ A7.7.71 @ 4b @ 4b @ 4b
	stmvs	r10, {r1,r3,r6}               @ A7.7.159 @ 4b @ 4b @ 4b
end_func_36:
	bx	r14

.ltorg
.align	2
.space	2, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_1952
cell_1952:	.word	safeSpaceSram+608

.space	1, 45
.space 3 % {{space_mod|default("0x10000000")}}
.global	cell_1948
cell_1948:	.word	safeSpaceSram-514

.space	2, 45
.space 5 % {{space_mod|default("0x10000000")}}
.global	cell_1949
cell_1949:	.word	safeSpaceSram+672

.align	1
.space 17696 % {{space_mod|default("0x10000000")}}
label606:
ldr	r4, =func_36  @ 2b  @ 2b  @ 2b
orr	r4, #1  @ 4b  @ 4b  @ 4b
ldr	r6, cell_3771  @ 2b  @ 2b  @ 2b
	blx	r4                            @ A7.7.19  @ 2b  @ 2b  @ 2b


	ldrh	r7, [r6, #3154]               @ A7.7.55 @ 4b @ looks important!  @ 4b  @ 4b  @ 4b
	nop.n  @ was .align 2  @ 2b
.space 66   @ 66b
end_label606:
	b.w	{{jump_label606}}

.ltorg
.align	2
.space	2, 45
.space 6 % {{space_mod|default("0x10000000")}}
.global	cell_3771
cell_3771:	.word	safeSpaceSram-2540

.align	1
.space 1348 % {{space_mod|default("0x10000000")}}



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
.space 277 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 174 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 46
.space 227 % {{space_mod|default("0x10000000")}}



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