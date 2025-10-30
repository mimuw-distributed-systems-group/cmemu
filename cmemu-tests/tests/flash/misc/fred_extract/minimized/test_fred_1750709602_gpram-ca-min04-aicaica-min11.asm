---
name: Fred-generated test
description: 'Test flow: (conf. 0) label16 -> label31'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: gpram
  cache_en: false
  lb_en: true
  wb_en: false
  jump_start: label16
  jump_label16: label31
  jump_label31: code_end
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
	mov.w	r0, #41758
	mov.w	r1, #38728
	mov.w	r2, #19754
	mov.w	r4, #19167
	mov.w	r6, #26846
	mov.w	r7, #7730
	mov.w	r8, #17378
	mov.w	r9, #9042
	mov.w	r10, #13978
	mov.w	r11, #38016
	mov.w	r12, #11535
	mov.w	r14, #4147

    @ Start the test
    b.w    start_test

.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r5, =stack
    add.w  r5, r5, #328
    mov.w  sp, r5

    @ Get counter address
    ldr.w  r5, =counter_idx
    ldr.w  r5, [r5]
    ldr.w  r3, =counters_to_test
    ldr.w  r5, [r3, r5]
    @ Get counter start value
    ldr.w  r3, [r5]
        @ r5 – counter address
        @ r3 – counter start value

    @ Jump to the 1st block

    b.w    {{jump_start}}

.ltorg


.align	1
.space 364 % {{space_mod|default("0x10000000")}}
label16:
.space 76 
	smull	r14, r11, r2, r11             @ A7.7.149  @ 4b @ looks important!  @ 4b @ looks important!
end_label16:
	b.w	{{jump_label16}}

.ltorg
.align	2
.space	1, 0xbf
.space 259 % {{space_mod|default("0x10000000")}}
label31:
	ittt	mi  @ 2b  @ 2b
	subwmi	r12, r13, #2229               @ A7.7.176  @ 4b  @ 4b
ldrmi	r14, cell_156  @ 4b  @ 4b
	bxmi	r14                           @ A7.7.20  @ 2b  @ 2b

.space 22   @ 22b
	ldrsb	r11, [r13, #4]                @ A7.7.59  @ 4b  @ 4b

forward_label_52:
.space 4  @ 4b
end_label31:
	b.w	{{jump_label31}}

.ltorg
.align	2
.space	0, 0xbf
.global	cell_156
cell_156:	.word	forward_label_52+1

.align	1
.space 412 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:
    @ Get counter finish value
    ldr.w  r14, [r5]
    @ Calculate counter difference
    sub.w  r14, r14, r3
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r3, cyccnt_addr
    cmp.w  r5, r3
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{ saveValue("counters", r14, r5, r3) }}
    @ Save values of registers
	{{saveValue("registers", r0, r5, r3)}}
	{{saveValue("registers", r1, r5, r3)}}
	{{saveValue("registers", r2, r5, r3)}}
	{{saveValue("registers", r4, r5, r3)}}
	{{saveValue("registers", r6, r5, r3)}}
	{{saveValue("registers", r7, r5, r3)}}
	{{saveValue("registers", r8, r5, r3)}}
	{{saveValue("registers", r9, r5, r3)}}
	{{saveValue("registers", r10, r5, r3)}}
	{{saveValue("registers", r11, r5, r3)}}
	{{saveValue("registers", r12, r5, r3)}}
    @ Advance counter_idx and repeat or end the test
    ldr.w  r5, =counter_idx
    ldr.w  r3, [r5]
    add.w  r3, r3, #4
    str.w  r3, [r5]
    cmp.w  r3, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2

cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2


{{section('sram')}}
.align  2
.space	2, 0xbf
.space 26 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 0xbf
.space 26 % {{space_mod|default("0x10000000")}}



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