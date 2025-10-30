---
name: Fred-generated test
description: 'Test flow: (conf. 0) label165'
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
  jump_start: label165
  jump_label165: code_end
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
	mov.w	r0, #49165
	mov.w	r1, #35207
	mov.w	r2, #28540
	mov.w	r3, #37434
	mov.w	r4, #51421
	mov.w	r5, #10398
	mov.w	r6, #45762
	mov.w	r7, #881
	mov.w	r8, #12704
	mov.w	r9, #61223
	mov.w	r10, #18129

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
.space 76 % {{space_mod|default("0x10000000")}}
end_label1:
	b.w	{{code_end}}

.ltorg
.align	2
.space	0, 45
.space 1581 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 3888 % {{space_mod|default("0x10000000")}}

.align	1
.space 3442 % {{space_mod|default("0x10000000")}}

.align	1
.space 1305 % {{space_mod|default("0x10000000")}}

.align	1
.space 3271 % {{space_mod|default("0x10000000")}}

.align	2
.space 1485 % {{space_mod|default("0x10000000")}}

.align	1
.space 1448 % {{space_mod|default("0x10000000")}}

.align	1
.space 1588 % {{space_mod|default("0x10000000")}}

.align	1
.space 5790 % {{space_mod|default("0x10000000")}}
label165:
.space 2  @ 2b  @ 2b
ldr	r6, cell_1044  @ 4b  @ 4b  @ 4b  @ 4b
.space 4  @ 4b  @ 4b
mov	r7, #47969  @ 4b  @ 4b  @ 4b  @ 4b
ldr	r2, cell_1043  @ 4b  @ 4b  @ 4b  @ 4b
.space 32   @ 32b  @ 32b
ldr	r2, cell_1039  @ 2b  @ 2b  @ 2b  @ 2b
.space 32   @ 32b  @ 32b
	bl	func_40                       @ A7.7.18  @ 4b  @ 4b @ looks important!  @ 4b  @ 4b


.space 4  @ 4b @ looks important!  @ 4b  @ 4b
	ldrsb	r5, [r6, r7]                  @ A7.7.61  @ 2b  @ 2b @ looks important!  @ 2b  @ 2b
	cbz	r2, forward_label_290         @ A7.7.21  @ 2b  @ 2b @ looks important!  @ 2b  @ 2b


forward_label_290:
.space 20   @ 20b  @ 20b  @ 20b
end_label165:
	b.w	{{jump_label165}}

.ltorg
.align	2
.space	2, 45
.global	cell_1043
cell_1043:	.word	safeSpaceSram+788

.space	0, 45
.space 11 % {{space_mod|default("0x10000000")}}
.global	cell_1044
cell_1044:	.word	safeSpaceSram-47592

.space	3, 45
.global	cell_1039
cell_1039:	.word	safeSpaceSram-17344

.space	3, 45
.space 4 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 1 % {{space_mod|default("0x10000000")}}

.align	1
.space 2035 % {{space_mod|default("0x10000000")}}

.space	2, 45
.space 2 % {{space_mod|default("0x10000000")}}

.align	1
.space 2073 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 6 % {{space_mod|default("0x10000000")}}

.align	1
.space 2568 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 16 % {{space_mod|default("0x10000000")}}

.align	1
.space 2390 % {{space_mod|default("0x10000000")}}

.align	1
.space 5851 % {{space_mod|default("0x10000000")}}

.align	1
.space 5072 % {{space_mod|default("0x10000000")}}

.space	3, 45
.space 3915 % {{space_mod|default("0x10000000")}}
func_40:
.space 2  @ 2b  @ 2b
ldr	r9, =forward_label_593  @ 4b  @ 4b  @ 4b  @ 4b
.space 26   @ 26b
ldr	r3, cell_2129  @ 4b  @ 4b  @ 4b  @ 4b
.space 46   @ 46b  @ 46b
forward_label_593:
.space 66   @ 66b
	cbnz	r3, forward_label_592         @ A7.7.21  @ 2b  @ 2b @ looks important!  @ 2b  @ 2b
 @ looks important!
.space 34   @ 34b
forward_label_592:
.space 16   @ 16b  @ 16b
end_func_40:
	bx	r14

.ltorg
.align	2
.space	3, 45
.space 88 % {{space_mod|default("0x10000000")}}
.global	cell_2129
cell_2129:	.word	safeSpaceGpramSram-29580

.space	3, 45
.space 63 % {{space_mod|default("0x10000000")}}

.align	1
.space 1490 % {{space_mod|default("0x10000000")}}

.align	1
.space 4345 % {{space_mod|default("0x10000000")}}

.align	1
.space 2872 % {{space_mod|default("0x10000000")}}

.align	1
.space 2226 % {{space_mod|default("0x10000000")}}

.align	1
.space 4170 % {{space_mod|default("0x10000000")}}

.align	1
.space 7905 % {{space_mod|default("0x10000000")}}

.align	1
.space 3184 % {{space_mod|default("0x10000000")}}

.align	1
.space 1186 % {{space_mod|default("0x10000000")}}

.align	1
.space 3356 % {{space_mod|default("0x10000000")}}

.align	1
.space 3634 % {{space_mod|default("0x10000000")}}

.align	1
.space 6272 % {{space_mod|default("0x10000000")}}
end_label604:
	b.w	{{code_end}}

.ltorg
.align	1
.space 3551 % {{space_mod|default("0x10000000")}}



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
.space	1, 46
.space 127 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 61 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 20 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	2, 46
.space 66 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 246 % {{space_mod|default("0x10000000")}}

.space	2, 46
.space 94 % {{space_mod|default("0x10000000")}}



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