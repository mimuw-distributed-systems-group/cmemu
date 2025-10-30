name: RNG updates test
description: "Checking which accesses update RNG"
dumped_symbols:
  debug_sum: 24 words
  address: 16 words
  COMB_res_number_LEN_4_time_3_LEN_7_time2_LEN_7_time1_LEN_7_time0_LEN_7: 256 words
  variant_cycles: 8 words
configurations:
- { code: "sram", lbEn: true, pull_count: 1024, test_count: 32, test_offset: 64, test_jump: -8, extra_loads: 0, second_ldr_part: 0, prefetch: False}
- { code: "sram", lbEn: true, pull_count: 1024, test_count: 32, test_offset: 64, test_jump: -8, extra_loads: 0, second_ldr_part: 0, prefetch: True}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set mode_gpram = 0 %}
{% set mode_cache = 5 if prefetch else 1 %}
{% set mode_off = 3 %}
{% set mode_changing = 8 %}
{% set mode_invalidating = 4 %}
{% set cache_size = 1024 * 2 + 256 + 4 %}
{% set cache_bytes = cache_size * 8 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section("sram") }}

.align	4
@ [TI-TRM] 3.2.8 Cortex-M3 Memory Map
@ [TI-TRM] 7.9.2.1 STAT Register
@ [TI-TRM] 7.9.2.2 CTL Register
VIMS_STAT:	    	.word	0x40034000
VIMS_CTL:	    	.word	0x40034004
TEST_OFFSET:        .word   {{test_offset}}
between_ldr_variant: .word  0
second_ldr_variant: .word   1
tag_hit:            .word   0
cache_hit:          .word   0

{{ section(code) }}

.align 4
.thumb_func
.type tested_code, %function

tested_code:
    {{callHelper('enable_cache', r1)}}
    {{callHelper('synchronize_rng', r1, '#240')}}
    {{callHelper('disable_cache', r1)}}

    mov.w  r1, #1
    ldr.w  r11, =second_ldr_variant
    str.w  r1, [r11]
second_ldr_variant_loop:
    mov.w  r1, #0
    ldr.w  r11, =between_ldr_variant
    str.w  r1, [r11]
between_ldr_variant_loop:
    mov.w  r1, #0
    ldr.w  r11, =tag_hit
    str.w  r1, [r11]
tag_hit_loop:
    mov.w  r1, #0
    ldr.w  r11, =cache_hit
    str.w  r1, [r11]
cache_hit_loop:
    ldr.w  r12, =between_ldr_variant
    ldr.w  r1, [r12]
    lsl.w  r1, #3
    ldr.w  r12, =second_ldr_variant
    ldr.w  r11, [r12]
    orr.w  r1, r11
    lsl.w  r1, #1
    ldr.w  r12, =tag_hit
    ldr.w  r11, [r12]
    orr.w  r1, r11
    lsl.w  r1, #1
    ldr.w  r12, =cache_hit
    ldr.w  r11, [r12]
    orr.w  r1, r11

    @ Prepare register addresses
    ldr.w  r11, VIMS_STAT
    ldr.w  r12, VIMS_CTL
    
    @ Change mode
    ldr.w  r2, [r12]
    and.w  r2, r2, #0xfffffffc
    orr.w  r2, r2, #{{mode_cache}}
    str.w  r2, [r12]
    
    bl.w stabilize_mode
    
    
    mov.w  r9, #0
{% if pull_count > 0 %}
    ldr.w  r4, =CACHE_ENTRY
    mov.w  r2, #{{pull_count}}
load_cache:
    ldr.w  r3, [r4]
    add.w  r9, r3
    add.w  r4, #8
    subs.w r2, #1
    bne.w  load_cache
{% endif %}
	
	
	@ Do extra loads to offset RNG sequence.
	mov.w  r3, #0                   @ counter
	mov.w  r5, #{{extra_loads}}     @ limit
	mov.w  r6, #0                   @ address offset
	mov.w  r7, #{{cache_bytes}}     @ cache size in bytes
extra_load_loop:
	cmp.w  r3, r5
	bhs.w  extra_load_loop_end
	add.w  r3, #1
	
	ldr.w  r4, =CACHE_ENTRY
	add.w  r4, r6
	ldr.w  r2, [r4]
	add.w  r8, r2

	add.w  r6, #0x800
	cmp.w  r6, r7
	blo.w  extra_load_skip_reset_r6
	mov.w  r6, #0
extra_load_skip_reset_r6:
	
	b.w    extra_load_loop
	
extra_load_loop_end:

    mov.w  r8, #0

    @ Prepare starting address.
    ldr.w  r4, =CACHE_ENTRY
    
    ldr.w  r2, TEST_OFFSET
    lsl.w  r2, #3
    add.w  r4, r2
    add.w  r4, #0x2000

    @ An address to use for the inserted ldrs
    add.w  r5, r4, #40

    @ Prepare a tag_miss,cache_hit address
    add.w  r3, r5, #40
    ldr.w  r10, [r3]
    
    mov.w  r3, #0
    
    @ Pull all test addresses.
test_pull_loop:
    ldr.w  r10, [r4]
    add.w  r8, r10
    add.w  r4, #{{test_jump}}
    add.w  r3, #1
    cmp.w  r3, #16
    bne.w  end_ldr_insertion

    @ Set up whether the first tested ldr will be a TAG hit/miss and cache hit/miss
    and.w  r12, r1, #3
    cmp.w  r12, #0
    bne.w  not_tag_miss_cache_miss
    b.w    end_tag_cache
not_tag_miss_cache_miss:
    cmp.w  r12, #1
    bne.w  not_tag_miss_cache_hit
    ldr.w  r10, [r5]
    @ Remove address r5 from the line buffer
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    b.w    end_tag_cache
not_tag_miss_cache_hit:
    cmp.w  r12, #2
    bne.w  not_tag_hit_cache_miss
    sub.w  r12, r5, #8
    ldr.w  r10, [r12]
    b.w    end_tag_cache
not_tag_hit_cache_miss:
    ldr.w  r10, [r5]
    sub.w  r12, r5, #8
    ldr.w  r10, [r12]
end_tag_cache:

    @ Choose the address of the second ldr in relation to the first
    and.w  r12, r1, #0b11100
    lsr.w  r12, #2
    cmp.w  r12, #0
    bne.w  not_line_buffer
    add.w  r6, r5, #4
    b.w  end_second_ldr_variant
not_line_buffer:
    cmp.w  r12, #1
    bne.w  not_next_line
    add.w  r6, r5, #8
    b.w  end_second_ldr_variant
not_next_line:
    cmp.w  r12, #2
    bne.w  not_next_next_line
    add.w  r6, r5, #16
    b.w  end_second_ldr_variant
not_next_next_line:
    cmp.w  r12, #3
    bne.w  not_same_set
    add.w  r6, r5, #2048
    b.w  end_second_ldr_variant
not_same_set:
    cmp.w  r12, #4
    bne.w  not_next_set
    add.w  r6, r5, #2056
    b.w  end_second_ldr_variant
not_next_set:
    add.w  r6, r5, #24
end_second_ldr_variant:

    @ Insert the ldr/ldrs possibly separated by some instructions
    lsr.w  r1, #5
    cmp.w  r1, #0
    bne.w  not_two_plus_4
    add.w  r1, r5, #4
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r1]
    ldr.n  r7, [r1]
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_two_plus_4:
    cmp.w  r1, #1
    bne.w  not_one_tag_miss_cache_hit
    add.w  r1, r5, #40
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r1]
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    nop.n
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_one_tag_miss_cache_hit:
    cmp.w  r1, #2
    bne.w  not_one_plus_4
    add.w  r1, r5, #4
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r1]
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    nop.n
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_one_plus_4:
    cmp.w  r1, #3
    bne.w  not_one_tag_miss_cache_miss
    add.w  r1, r5, #80
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r1]
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    nop.n
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_one_tag_miss_cache_miss:
    cmp.w  r1, #4
    bne.w  not_2_adds
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    adds.n  r7, #1
    adds.n  r7, #1
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_2_adds:
    cmp.w  r1, #5
    bne.w  not_3_adds
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    adds.n  r7, #1
    adds.n  r7, #1
    adds.n  r7, #1
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    nop.n
    bl.w save_variant_cycles
    b.w  end_between_ldr_variant
not_3_adds:
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    adds.n  r7, #1
    adds.n  r7, #1
    adds.n  r7, #1
    adds.n  r7, #1
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    bl.w save_variant_cycles
end_between_ldr_variant:

end_ldr_insertion:
    cmp.w r3, #{{test_count}}
    bne.w  test_pull_loop
    
    bl.w save_test_and_pull_sum

    @ Prepare cache test access.
    ldr.w  r4, =CACHE_ENTRY
    
    ldr.w  r2, TEST_OFFSET
    lsl.w  r2, #3
    add.w  r4, r2
    
    mov.w  r8, #0
    mov.w  r9, #0
    
    
test_loop:
    .align 4
    mov.w  r10, #0x0
    
    mov.w  r1, #0x9
    
{% for i in range(4) %}
{% set offset = i * 2048 %}
    
    add.w  r3, r4, #{{offset}}
    
    @ Flush queue.
    isb.w

    ldr.n  r7, [r0, {{CYCCNT}}]
    
    ldr.n  r6, [r3]
    
    ldr.n  r2, [r0, {{CYCCNT}}]
    add.n  r8, r6
    sub.w  r7, r2, r7
    
    mov.w   r2, #100
    cmp.w   r7, #7
    it      hs
    movhs.w r2, #{{i}}
    cmp.w   r1, r2
    it      hs
    movhs.w r1, r2
    
    lsl.w  r10, #7
    orr.w  r10, r7
    
{% endfor %}

	lsl.w  r10, #4
	orr.w  r10, r1
    
    bl.w save_single_round
    
    add.w  r4, #{{test_jump}}
    add.w  r9, #1
    mov.w  r2, #{{test_count}}
    cmp.w  r9, r2
    
    bne.w test_loop
    
    ldr.n  r5, [r0, {{CYCCNT}}]
    nop.n
    
    bl.w save_final
    
    bl.w reset_mode

    ldr.w  r11, =cache_hit
    ldr.w  r1, [r11]
    add.w  r1, #1
    str.w  r1, [r11]
    cmp.w  r1, #1
    blt.w  cache_hit_loop

    ldr.w  r11, =tag_hit
    ldr.w  r1, [r11]
    add.w  r1, #1
    str.w  r1, [r11]
    cmp.w  r1, #2
    blt.w  tag_hit_loop

    ldr.w  r11, =between_ldr_variant
    ldr.w  r1, [r11]
    add.w  r1, #1
    str.w  r1, [r11]
    cmp.w  r1, #4
    blt.w  between_ldr_variant_loop

    ldr.w  r11, =second_ldr_variant
    ldr.w  r1, [r11]
    add.w  r1, #1
    str.w  r1, [r11]
    cmp.w  r1, #2
    blt.w  second_ldr_variant_loop

    b.w end_label


@ Turns off the cache and waits to stabilize.
@ Assumes:
@ Destroys:
@ r2
.align 4
reset_mode:
    @ Prepare register addresses
    ldr.w  r11, VIMS_STAT
    ldr.w  r12, VIMS_CTL
    ldr.w r2, [r12]
    and.w r2, r2, #0xfffffffc
    orr.w r2, r2, #{{mode_off}}
    str.w r2, [r12]

@ Waits for mode to stabilize.
@ Assumes:
@ r11 = VIMS_STAT
@ Destroys:
@ r2
.align 4
stabilize_mode:
    @ check if changing
    ldr.w r2, [r11]
    ands.w r2, r2, #{{mode_changing + mode_invalidating}}
    bne.w stabilize_mode
    
    bx.n lr

.align 4
save_single_round:
    {{saveValue('COMB_res_number_LEN_4_time_3_LEN_7_time2_LEN_7_time1_LEN_7_time0_LEN_7', r10, r2, r3)}}

    bx.n lr

.align 4
save_test_and_pull_sum:
    {{saveValue('debug_sum', r8, r2, r3)}}
    {{saveValue('debug_sum', r9, r2, r3)}}
    
    bx.n lr

.align 4
save_final:
    ldr.w r4, =CACHE_ENTRY
    {{saveValue('address', r4, r2, r3)}}
    ldr.w r4, =test_loop
    {{saveValue('address', r4, r2, r3)}}
    {{saveValue('debug_sum', r8, r2, r3)}}

    bx.n lr

.align 4
save_variant_cycles:
    sub.w  r7, r2
    {{saveValue('variant_cycles', r7, r2, r6)}}

    bx.n lr



{{ section("flash") }}

.align 15
CACHE_ENTRY:    .4byte {% for i in range(cache_size) %} {{ i }}, {{ i + 1000000007 }}, {% endfor %}

{% endblock %}
