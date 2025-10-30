---
name: Fred-generated test
description: 'Test flow: (conf. 0) label451'
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
  jump_start: label451
  jump_label451: code_end
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
	mov.w	r0, #48252
	mov.w	r1, #59971
	mov.w	r2, #22113
	mov.w	r4, #55282
	mov.w	r6, #31639
	mov.w	r7, #29851
	mov.w	r8, #25180
	mov.w	r9, #38748
	mov.w	r10, #14886
	mov.w	r11, #61445
	mov.w	r12, #42152
	mov.w	r14, #11906

    @ Start the test
    b.w    start_test

.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r3, =stack
    add.w  r3, r3, #328
    mov.w  sp, r3

    @ Get counter address
    ldr.w  r3, =counter_idx
    ldr.w  r3, [r3]
    ldr.w  r5, =counters_to_test
    ldr.w  r3, [r5, r3]
    @ Get counter start value
    ldr.w  r5, [r3]
        @ r3 – counter address
        @ r5 – counter start value

    @ Jump to the 1st block

    b.w    {{jump_start}}

.ltorg


.align	1
.space 2500 % {{space_mod|default("0x10000000")}}
label451:
.space 2  @ 2b
mov	r0, (forward_label_728 - jump_from_20 - 4)  @ 4b  @ 4b  @ 4b
jump_from_20:
	add	r15, r0                       @ A7.7.4  @ 2b @ looks important!  @ 2b @ looks important!  @ 2b @ looks important!
 @ looks important!
.space 32   @ 32b  @ 32b
forward_label_728:
.space 66   @ 66b
end_label451:
	b.w	{{jump_label451}}

.ltorg
.align	2
.space	0, 45
.space 16 % {{space_mod|default("0x10000000")}}

.space	1, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 6849 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:
    @ Get counter finish value
    ldr.w  r14, [r3]
    @ Calculate counter difference
    sub.w  r14, r14, r5
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r5, cyccnt_addr
    cmp.w  r3, r5
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{ saveValue("counters", r14, r3, r5) }}
    @ Save values of registers
	{{saveValue("registers", r0, r3, r5)}}
	{{saveValue("registers", r1, r3, r5)}}
	{{saveValue("registers", r2, r3, r5)}}
	{{saveValue("registers", r4, r3, r5)}}
	{{saveValue("registers", r6, r3, r5)}}
	{{saveValue("registers", r7, r3, r5)}}
	{{saveValue("registers", r8, r3, r5)}}
	{{saveValue("registers", r9, r3, r5)}}
	{{saveValue("registers", r10, r3, r5)}}
	{{saveValue("registers", r11, r3, r5)}}
	{{saveValue("registers", r12, r3, r5)}}
    @ Advance counter_idx and repeat or end the test
    ldr.w  r3, =counter_idx
    ldr.w  r5, [r3]
    add.w  r5, r5, #4
    str.w  r5, [r3]
    cmp.w  r5, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2

cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	3, 46
.space 246 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 169 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	3, 46
.space 151 % {{space_mod|default("0x10000000")}}



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