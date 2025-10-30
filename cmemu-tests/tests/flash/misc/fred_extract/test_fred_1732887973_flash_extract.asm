---
name: Fred-generated test
description: 'Test flow: (conf. 0) label58 -> label450'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations: []
product:
- code_memory: [flash]
  cache_en: [false]
  lb_en: [false]
  wb_en: [false]
  jump_start:
    - label58
    - label_reverse
  jump_label58: [
  #  label450,
    forward_label_865
    ]
  jump_label450: [code_end]
  code_end: [code_end]
  # del_a: [-32, -8, -4, -2, 0, 2, 4, 6, 8, 12, 32]
  #del_b: [-2, 0, 2, 4, 8]
  del_b: [0]
  #land_shift: [0, 2]
  shift: [0,] # 0x100, 0xe00, 0x1000,]
  #reg: [r10] # r13 too!
  #load_mem: [stack, safeSpaceFlash, safeSpaceGpramSram] # not-conclicted
  # load_form: # any form works
  #   - '[r10], 4'
  #   - '[r7, r1]'
  #   - '[r10, 4]'
  #   - '[r7, 4]!'
  #pre_instr:
  #  - ''
  #  - nop.w
  #  - nop.n; nop.n
  #  - b.w .+4
  #  - b.n .+4; nop.n
  #  - umull r1, r3, r4, r5
  #  - add.w r10, 4

  #  - .space 2
  #  - b.n .+2;
  #  - nop.n
  #  - ldr.n r3, [r7]
  #  - ldr.n r3, [r13]
  # We can't really touch J1/2 bits, as these are 23&24th bit of the address
  branch_distance: [
    '0xe04', '0xe00', '0xc04',
    '0xffc', '0x1000', '0x1004', '0x1e04', '0x3ffc',
    '0x060',
    '0xe08', '0xe10', '0xe20', '0xe40', '0xe80', '0xf00', '0xf04', '0xff0',
    '0xdfc',
    ]
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
	mov.w	r1, #8
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
    @ new here:
    @ conflicts would  push the pipelining moment way into the future
@    ldr.w  r11, ={{load_mem}}
@    add.w  r11, r11, #328
    mov.w  r10, r11
    mov.w  r7, r11

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


.ltorg

.align 12
.space 2 + {{shift}}
label58:
@ weirdly many offsets just work
.space 4
@nop.w @ works too
end_label58:
    @ it seems that the jump must be strictly greater than 0xe00 for it to break (or >= 0xe04)
    @ 0xe00 makes the 2nd 11:8 bits all 1
    @ also, far b(l)-s are historically two "inseparable" instructions
    @ TODO: check if other far branches may trigger this (ldr pc, blx reg)
    @ the prefix for second halfword of 0xe00 jump is same as SVC
    @ b.w and b.l

    @ TODO: check distances if it's bit pattern than matters
    @ WHOA, the bit pattern is important! 0x1000 won't trigger that, but 0x1e04 does!
    @ it seems like we need to have an enc with 11:8 all one and 3:0 not zero! '0xf04',
    @ !!!!!!!!!
    @ for B the second halfword is EXACTLY the encoding of IT!
    @ TODO: check other instructions that may encode IT as the second one
	b.w	{{jump_label58}}

@ the distance of this jump seems important to trigger non-pipelining!

@ most of offsets are fixing it
@.space 0xe04 - {{del_b}} {{ "- 4" if jump_label58 == "forward_label_865" else ""}}
.space {{branch_distance}} {{ "- 4" if jump_label58 == "forward_label_865" else ""}}
@	.space {{land_shift}}
label450:
@    {{pre_instr}}
    @ also narrow instructions are okay, but not-narrow doesn't seem to change the flow much,
    @   but the pipelining
    @ this branch is somewhat redundant
    @ we could just remove it if we landed on the next label
    @ (seems like the jump length is important)
    @ but let's keep to to show that it's not the "latest jump" that counts
	b.w	forward_label_865             @ A7.7.18  @ 4b @ looks important!
	@ we can add some free space here without "breaking the chain"
forward_label_865:
    @ seems like we can just add another branch here to preserve the behavior
    @ also narrow instructions are okay, but not-narrow doesn't seem to change the flow much,
    @   but the pipelining

    @ no dependency on the address observed (like changing the distance, may be unaligned)
	ldr.w	r2, [r7, 4]   @ any form works (writeback and not)
	nop.n  @ this seems to pipeline, but sometimes not for a weird reason!
.space 2
end_label450:
	b.w	{{jump_label450}}


@ distance
@.space {{branch_distance}} -6*4 {{ "+ 4" if jump_label58 == "forward_label_865" else ""}}
@ bit pattern in 2nd
.space 0x5000 - {{branch_distance}} -6*4 {{ "+ 4" if jump_label58 == "forward_label_865" else ""}}

@ reverse_part
label_reverse:
.space 4
@nop.w @ works too
    @ Same happens when the 2nd halfword is the same as with positive jump!
	b.w	{{jump_label58}}



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
counters_to_test:    .word {{CYCCNT_ADDR}}, {{CPICNT_ADDR}}, {{LSUCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CPICNT_ADDR}}, {{LSUCNT_ADDR}}, {{FOLDCNT_ADDR}}
end_counters_to_test:



{{section('sram')}}

.align  2
.global original_sp
original_sp:        .word   0x00000000

.align  2
.global counter_idx
counter_idx:     .word   0


{% endblock %}
