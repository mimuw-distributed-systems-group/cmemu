---
name: Fred-generated test
description: 'Test flow: (conf. 0) label196 -> label563'
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
  jump_start: label196
  jump_label196: label563
  jump_label563: code_end
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
	mov.w	r0, #2226
	mov.w	r1, #57452
	mov.w	r2, #31933
	mov.w	r3, #43194
	mov.w	r4, #63300
	mov.w	r5, #30440
	mov.w	r6, #27780
	mov.w	r7, #19141
	mov.w	r8, #14896
	mov.w	r9, #36026
	mov.w	r10, #49724

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
.space 12300 % {{space_mod|default("0x10000000")}}
label196:
	sub	r13, #44  @ 2b
.space 14   @ 14b
ldr	r4, cell_1306  @ 4b
	ldrh	r10, [r4], #0                 @ A7.7.55 @ 4b @ looks important!  @ 4b @ looks important!
	cmp	r10, r8, LSL #1               @ A7.7.28 @ 4b @ looks important!  @ 4b @ looks important!
ldr	r8, cell_1305 @ looks important!  @ 4b
	and	r0, r10                       @ A7.7.9 @ 4b @ looks important!  @ 4b @ looks important!
	stm	r8!, {r0-r7,r9-r10}           @ A7.7.159 @ 4b @ looks important!  @ 4b @ looks important!
	pop	{r0-r10}                      @ A7.7.99 @ 4b @ looks important!  @ 4b
	it	ge  @ 2b
	stmge	r13, {r0-r10}                 @ A7.7.159 @ 4b
end_label196:
	b.w	{{jump_label196}}

.ltorg
.align	2
.space	1, 45
.global	cell_1306
cell_1306:	.word	safeSpaceFlash+779

.space	2, 45
.global	cell_1305
cell_1305:	.word	safeSpaceSram+784

.align	1
.space 21144 % {{space_mod|default("0x10000000")}}
label563:
.space 4
	str	r4, [r13]                     @ A7.7.161  @ 2b @ looks important!
	nop.n  @ was .align 2  @ 2b @ looks important!
.space 12 
ldr	r3, cell_3637  @ 4b
.space 8   @ 8b
	strh	r3, [r13]                     @ A7.7.170  @ 4b
end_label563:
	b.w	{{jump_label563}}

.ltorg
.align	2
.space	2, 45
.global	cell_3637
cell_3637:	.word	safeSpaceGpramSram-182739

.align	2
.space 8 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 4571 % {{space_mod|default("0x10000000")}}



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
.space 263 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	3, 46
.space 307 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	1, 46
.space 189 % {{space_mod|default("0x10000000")}}



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