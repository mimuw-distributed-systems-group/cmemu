---
name: Fred-generated test
description: 'Test flow: (conf. 0) label58 -> label450'
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
  jump_start: label58
  jump_label58: label450
  jump_label450: code_end
  code_end: code_end
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
	mov.w	r0, #40087
	mov.w	r1, #29823
	mov.w	r2, #48800
	mov.w	r3, #65449
	mov.w	r4, #53848
	mov.w	r5, #59525
	mov.w	r6, #57862
	mov.w	r7, #10691
	mov.w	r8, #30877
	mov.w	r9, #63567
	mov.w	r10, #31449

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
.space 82 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	1, 45
.space 9337 % {{space_mod|default("0x10000000")}}
label58:
.space 2
ldr	r6, cell_391  @ 4b  @ 4b
ldr	r9, cell_390  @ 4b  @ 4b
.space 4 + 4 + 4  @ 12b
ldr	r0, cell_389  @ 4b  @ 4b
.space 4  @ 4b
mov	r8, #20731  @ 4b  @ 4b
.space 18 
end_label58:
	b.w	{{jump_label58}}

.ltorg
.align	2
.space	1, 45
.global	cell_391
cell_391:	.word	safeSpaceGpramSram+500

.space	1, 45
.global	cell_389
cell_389:	.word	safeSpaceSram-20107

.space	3, 45
.global	cell_390
cell_390:	.word	safeSpaceSram-2702

.align	1
.space 2855 % {{space_mod|default("0x10000000")}}

.align	1
.space 54418 % {{space_mod|default("0x10000000")}}
label450:
	sub	r13, #36  @ 2b @ looks important!  @ 2b
	bl	forward_label_865             @ A7.7.18  @ 4b @ looks important!  @ 4b @ looks important!
ldr	r14, =post_branch_578  @ 4b  @ 4b @ looks important!

.space 30 
post_branch_578:


mov	r7, #5  @ 4b  @ 4b
	ldrb	r3, [r13, r7, LSL #3]         @ A7.7.48  @ 4b  @ 4b
.space 2  @ 2b
	addw	r0, r13, #222                 @ A7.7.5  @ 4b  @ 4b
.space 4 + 4  @ 8b
forward_label_865:
	ldr	r2, [r13], #-140              @ A7.7.43  @ 4b @ looks important!  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
.space 4 + 4 + 2  @ 10b
	ldrd	r6, r4, [r13, #176]!          @ A7.7.50  @ 4b  @ 4b
end_label450:
	b.w	{{jump_label450}}

.ltorg
.align	2
.space 8 % {{space_mod|default("0x10000000")}}

.align	1
.space 24221 % {{space_mod|default("0x10000000")}}



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
.space 197 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 46
.space 224 % {{space_mod|default("0x10000000")}}


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  2
.space	2, 46
.space 242 % {{space_mod|default("0x10000000")}}



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