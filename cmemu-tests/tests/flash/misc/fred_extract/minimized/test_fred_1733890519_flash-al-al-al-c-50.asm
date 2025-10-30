---
name: Fred-generated test
description: 'Test flow: (conf. 0) label301 -> label445 -> label76'
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
  jump_start: label301
  jump_label301: label445
  jump_label445: label76
  jump_label76: code_end
  code_end: code_end
  space_mod: 2048
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
	mov.w	r0, #40051
	mov.w	r1, #45355
	mov.w	r2, #7159
	mov.w	r3, #41208
	mov.w	r4, #47446
	mov.w	r5, #50362
	mov.w	r6, #35764
	mov.w	r7, #58801
	mov.w	r8, #48669
	mov.w	r9, #20815
	mov.w	r10, #12860

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
.space 98 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 2691 % {{space_mod|default("0x10000000")}}

.align	1
.space 7544 % {{space_mod|default("0x10000000")}}
label76:
ldr	r14, =post_branch_97  @ 4b  @ 4b  @ 4b @ looks important!
.space 28 
post_branch_97:


.space 40   @ 40b
end_label76:
	b.w	{{jump_label76}}

.ltorg
.align	2
.space	1, 45
.space 21 % {{space_mod|default("0x10000000")}}

.align	1
.space 1281 % {{space_mod|default("0x10000000")}}

.align	1
.space 1642 % {{space_mod|default("0x10000000")}}

.align	1
.space 4849 % {{space_mod|default("0x10000000")}}

.align	1
.space 2207 % {{space_mod|default("0x10000000")}}

.align	1
.space 11655 % {{space_mod|default("0x10000000")}}

.align	1
.space 1566 % {{space_mod|default("0x10000000")}}

.align	1
.space 1226 % {{space_mod|default("0x10000000")}}
label301:
.space 4  @ 4b
ldr	r10, cell_1850  @ 4b  @ 4b  @ 4b
.space 4  @ 4b
ldr	r9, cell_1848  @ 4b  @ 4b  @ 4b
.space 4  @ 4b
	msr	apsr_nzcvq, r1                @ A7.7.83  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	sdiv	r7, r5, r2                    @ A7.7.127  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	isb	                              @ A7.7.37  @ 4b @ looks important!  @ 4b  @ 4b @ looks important!
	iteet	ge  @ 2b  @ 2b  @ 2b
	strbge	r4, [r10], #30                @ A7.7.163 @ 4b @ 4b @ 4b
	cmplt	r0, #92                       @ A7.7.27 @ 2b @ 2b @ 2b
	ldrhlt	r0, [r9, #-57]!               @ A7.7.55 @ 4b @ 4b @ 4b
	movtge	r1, #6033                     @ A7.7.79 @ 4b @ looks important! @ 4b @ 4b @ looks important!

.space 68   @ 68b
end_label301:
	b.w	{{jump_label301}}

.ltorg
.align	2
.space	3, 45
.space 15 % {{space_mod|default("0x10000000")}}
.global	cell_1848
cell_1848:	.word	safeSpaceFlash+343

.space	0, 45
.space 12 % {{space_mod|default("0x10000000")}}
.global	cell_1850
cell_1850:	.word	safeSpaceGpramSram+677

.align	1
.space 3764 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 3 % {{space_mod|default("0x10000000")}}

.align	1
.space 10464 % {{space_mod|default("0x10000000")}}

.align	1
.space 4978 % {{space_mod|default("0x10000000")}}

.align	1
.space 1092 % {{space_mod|default("0x10000000")}}
label445:
.space 22 
ldr	r1, cell_2888  @ 4b  @ 4b  @ 4b @ looks important!
.space 108   @ 108b @ looks important!
end_label445:
	b.w	{{jump_label445}}

.ltorg
.align	2
.space 9 % {{space_mod|default("0x10000000")}}
.global	cell_2888
cell_2888:	.word	safeSpaceGpramSram+520

.space	3, 45
.space 18 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 1083 % {{space_mod|default("0x10000000")}}

.align	1
.space 6699 % {{space_mod|default("0x10000000")}}

.align	1
.space 6215 % {{space_mod|default("0x10000000")}}

.align	1
.space 5443 % {{space_mod|default("0x10000000")}}

.align	1
.space 2181 % {{space_mod|default("0x10000000")}}



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
.space 120 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 122 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	1, 46
.space 29 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 245 % {{space_mod|default("0x10000000")}}

.space	3, 46
.space 232 % {{space_mod|default("0x10000000")}}



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