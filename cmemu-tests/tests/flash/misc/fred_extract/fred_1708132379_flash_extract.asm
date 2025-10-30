---
name: Fred-generated test (SUSCEPTIBLE TO OFFSETS)
description: |
  This test has found that cache initialization in the base template was unsound:
  as the code called `cache_reset` and `sync_cache` from Flash, this location could have aliased with
  the dedicated cache set.
  The solution was twofold:
  a) the synchronization sequence was made independent of the cache contents
  b) the cache is cleared and synchronized atomically from SRAM

  This test is very susceptible to changes in offsets.
  Note: it only uses literal load in start_test and then just jumps.
  It was breaking the emulator when:

    nm build/build/0/build/cherry/app.nobl.elf | grep -e func_1$ -e func_2$ -e func_66$ -e label367 -e label252 -e label496  -e label363 -e code_end -e safeSpaceFlash -e counters_to_test -e _emulator_memset -e rng_canonical_seq  -e safeSpaceSram -e pAuxRamImage -e cyccnt_addr -e start_test  -e end_label$ | sort
    00000251 t end_label
    000003b1 t start_test
    0000066e t func_1
    000006ec t end_func_1
    00000e9e t func_2
    00000f18 t end_func_2
    00008278 t label252
    00008296 t end_label252
    0000aec6 t label363
    0000aeee t end_label363
    0000b4c2 t label367
    0000b51e t end_label367
    0000fe06 t label496
    0000fe2e t end_label496
    00010ec8 t func_66
    00010f86 t end_func_66
    00017571 t code_end
    00017a0c t cyccnt_addr
    00017b00 T safeSpaceFlash
    00017f00 T counters_to_test
    00017f1c t end_counters_to_test
    00017f21 t _emulator_memset
    00017f24 t _emulator_memset_loop_begin
    00017f30 t _emulator_memset_loop_end
    0001a8ca t pAuxRamImage
    200007d0 T safeSpaceSram
    200018f2 t rng_canonical_seq

dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: true # works without cache
  lb_en: true
  wb_en: false
  jump_start: label252
  jump_label252: label363
  jump_label363: label367
  jump_label367: label496
  label_252_func: func_1
  label_363_func: func_2
  label_496_func: func_66
...

{% set end_label = 'end_label' %}
{% set code_end = 'code_end' %}
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
	mov.w	r0, #63261
	mov.w	r1, #46779
	mov.w	r2, #63670
	mov.w	r3, #62511
	mov.w	r4, #27438
	mov.w	r5, #61890
	mov.w	r6, #61492
	mov.w	r7, #3459
	mov.w	r8, #22526
	mov.w	r9, #2542
	mov.w	r10, #39103

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

.skip 144
@ place is important for use above!
.ltorg
.skip 510

.align	1
func_1:
.skip 126
end_func_1:
	bx	r14

.skip 1968

.align	1
func_2:
.skip 122
end_func_2:
	bx	r14

.skip 29534

.align	1
label252:
.skip 26
	bl	{{label_252_func}}                       @ A7.7.18
end_label252:
	b.w	{{jump_label252}}

.skip 11308

.align	1
label363:
.skip 36
    bl.w {{label_363_func}}
end_label363:
	b.w	{{jump_label363}}

.skip 1488

.align	1
label367:

.skip 92
end_label367:
	b.w	{{jump_label367}}

.skip 18660
.align	1
label496:
.skip 36
    bl.w {{label_496_func}}
end_label496:
	b.w	code_end

.skip 4246

func_66:
.skip 190
end_func_66:
	bx	r14

.skip 26074

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
.space 238


{{section('sram')}}
.align  2
.space 527

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
