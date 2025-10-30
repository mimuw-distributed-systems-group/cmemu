---
name: Fred-generated test
description: 'Test flow: (conf. 0) label457 (conf. 1) label457 (conf. 2) label457
  (conf. 3) label457'
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
  jump_start: label457
  jump_label457: code_end
  code_end: code_end
  space_mod: 4
- code_memory: flash
  cache_en: true
  lb_en: true
  wb_en: false
  jump_start: label457
  jump_label457: code_end
  code_end: code_end
  space_mod: 8
- code_memory: flash
  cache_en: true
  lb_en: true
  wb_en: false
  jump_start: label457
  jump_label457: code_end
  code_end: code_end
  space_mod: 2048
- code_memory: flash
  cache_en: true
  lb_en: true
  wb_en: false
  jump_start: label457
  jump_label457: code_end
  code_end: code_end
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
	mov.w	r2, #31640
	mov.w	r3, #51002
	mov.w	r4, #43613
	mov.w	r5, #35433
	mov.w	r6, #39797
	mov.w	r7, #55505
	mov.w	r8, #14694
	mov.w	r9, #50347
	mov.w	r10, #57608
	mov.w	r11, #55721
	mov.w	r12, #56175

    @ Start the test
    b.w    start_test


.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r0, =stack
    add.w  r0, r0, #328
    mov.w  sp, r0

    @ Get counter address
    ldr.w  r0, =counter_idx
    ldr.w  r0, [r0]
    ldr.w  r1, =counters_to_test
    ldr.w  r0, [r1, r0]
    @ Get counter start value
    ldr.w  r1, [r0]
        @ r0 – counter address
        @ r1 – counter start value

    @ Jump to the 1st block
    b.w    {{jump_start}}
.ltorg



.align	1
.space 58 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 794 % {{space_mod|default("0x10000000")}}

.align	1
.space 8856 % {{space_mod|default("0x10000000")}}
end_label88:
	b.w	{{code_end}}

.ltorg
.align	2
.space 552 % {{space_mod|default("0x10000000")}}

.align	1
.space 1648 % {{space_mod|default("0x10000000")}}

.align	1
.space 3536 % {{space_mod|default("0x10000000")}}

.align	1
.space 4123 % {{space_mod|default("0x10000000")}}

.align	1
.space 6817 % {{space_mod|default("0x10000000")}}

.align	1
.space 2858 % {{space_mod|default("0x10000000")}}
label457:
.space 8 
mov	r10, #43  @ 4b  @ 4b  @ 4b
ldr	r2, cell_1722  @ 4b  @ 4b  @ 4b
.space 8 
ldr	r7, cell_1721  @ 2b  @ 2b  @ 2b
.space 36 
	ldrsb	r11, [r7, #3903]              @ A7.7.59  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	sub	r11, r13, #48                 @ A7.7.176  @ 4b  @ 4b @ looks important!  @ 4b @ looks important!
	strb	r2, [r13, r10]                @ A7.7.164  @ 4b  @ 4b @ looks important!  @ 4b
	nop	                              @ A7.7.88  @ 2b  @ 2b @ looks important!  @ 2b @ looks important!
.space 24 
end_label457:
	b.w	{{jump_label457}}

.ltorg
.align	2
.space	0, 45
.global	cell_1721
cell_1721:	.word	safeSpaceFlash-3096

.space	3, 45
.global	cell_1722
cell_1722:	.word	safeSpaceSram+526

.space	2, 45
.space 9 % {{space_mod|default("0x10000000")}}

.align	1
.space 2256 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 6890 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:

    @ Get counter finish value
    ldr.w  r14, [r0]
    @ Calculate counter difference
    sub.w  r14, r14, r1
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r1, cyccnt_addr
    cmp.w  r0, r1
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{saveValue("counters", r14, r0, r1)}}

    @ Save values of registers
	{{saveValue("registers", r2, r0, r1)}}
	{{saveValue("registers", r3, r0, r1)}}
	{{saveValue("registers", r4, r0, r1)}}
	{{saveValue("registers", r5, r0, r1)}}
	{{saveValue("registers", r6, r0, r1)}}
	{{saveValue("registers", r7, r0, r1)}}
	{{saveValue("registers", r8, r0, r1)}}
	{{saveValue("registers", r9, r0, r1)}}
	{{saveValue("registers", r10, r0, r1)}}
	{{saveValue("registers", r11, r0, r1)}}
	{{saveValue("registers", r12, r0, r1)}}

    @ Advance counter_idx and repeat or end the test
    ldr.w  r0, =counter_idx
    ldr.w  r1, [r0]
    add.w  r1, r1, #4
    str.w  r1, [r0]
    cmp.w  r1, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2
cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	2, 46
.space 110 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 70 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 24 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 244 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 83 % {{space_mod|default("0x10000000")}}



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