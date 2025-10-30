name: RNG updates test
description: "Checking which accesses update RNG"
dumped_symbols:
  debug_sum: 36 words
  address: 24 words
  COMB_res_number_LEN_4_time_3_LEN_7_time2_LEN_7_time1_LEN_7_time0_LEN_7: 168 words
  variant_cycles: 12 words
configurations:
- { code: "sram", lbEn: true, pull_count: 1024, test_count: 14, test_offset: 64, test_jump: -8}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

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
TEST_OFFSET:        .word   {{test_offset}}

{{ section(code) }}

.align 4
.thumb_func
.type tested_code, %function

tested_code:
    {{callHelper('enable_cache', r2)}}
    {{callHelper('synchronize_rng', r2, '#240')}}
    {{callHelper('disable_cache', r2)}}

    mov.w  r11, #0
rng_case_loop:
    {{callHelper('enable_cache', r2)}}

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

    mov.w  r8, #0

    @ Prepare starting address.
    ldr.w  r4, =CACHE_ENTRY
    
    ldr.w  r2, =TEST_OFFSET
    ldr.w  r2, [r2]
    lsl.w  r2, #3
    add.w  r4, r2
    add.w  r4, #0x2000

    @ An address to use for the inserted ldrs
    add.w  r5, r4, #40
    
    mov.w  r3, #0
    
    @ Pull all test addresses.
test_pull_loop:
    ldr.w  r10, [r4]
    add.w  r8, r10
    add.w  r4, #{{test_jump}}
    add.w  r3, #1
    cmp.w  r3, #7
    bne.w  end_case_insertion

    @ Insert the special case
    cmp.w  r11, #0
    beq.w  add_case
    cmp.w  r11, #1
    beq.w  tag_lag
    cmp.w  r11, #2
    beq.w  advanced_next_line_miss_0
    cmp.w  r11, #3
    beq.w  advanced_next_line_miss_1
    cmp.w  r11, #4
    beq.w  advanced_next_line_miss_2
    cmp.w  r11, #5
    beq.w  advanced_next_line_miss_3
    cmp.w  r11, #6
    beq.w  disappearing_cache_write_0
    cmp.w  r11, #7
    beq.w  disappearing_cache_write_0
    cmp.w  r11, #8
    beq.w  disappearing_cache_write_another_0
    cmp.w  r11, #9
    beq.w  disappearing_cache_write_another_1
    cmp.w  r11, #10
    beq.w  disappearing_cache_write_another_2
    cmp.w  r11, #11
    beq.w  disappearing_cache_write_another_3
    b.w  end_case_insertion
add_case:
    add.w  r6, r5, #8
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    adds.n  r7, #1
    ldr.n  r7, [r6]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
tag_lag:
    add.w  r6, r5, #8
    ldr.w  r10, [r6]
    @ Remove address r6 from the line buffer
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r6]
    ldr.n  r7, [r5]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
advanced_next_line_miss_0:
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    ldr.w  r10, [r6]
    @ Remove address r6 from the line buffer and r1 from TAG prefetch
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r6]
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
advanced_next_line_miss_1:
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    ldr.w  r10, [r6]
    @ Remove address r6 from the line buffer and r1 from TAG prefetch
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    nop.n
    ldr.n  r7, [r6]
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
advanced_next_line_miss_2:
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    ldr.w  r10, [r6]
    @ Remove address r6 from the line buffer and r1 from TAG prefetch
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r6]
    nop.n
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
advanced_next_line_miss_3:
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    ldr.w  r10, [r6]
    @ Remove address r6 from the line buffer and r1 from TAG prefetch
    sub.w  r12, r5, #24
    ldr.w  r10, [r12]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    nop.n
    ldr.n  r7, [r6]
    nop.n
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_0:
    add.w  r6, r5, #8
    add.w  r1, r6, #2048
    ldr.w  r10, [r5]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r6]
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_1:
    add.w  r6, r5, #8
    add.w  r1, r6, #2048
    ldr.w  r10, [r5]
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r6]
    nop.n
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_another_0:
    ldr.w  r10, [r5]
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    add.w  r5, #8
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r6]
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_another_1:
    ldr.w  r10, [r5]
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    add.w  r5, #8
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    nop.n
    ldr.n  r7, [r6]
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_another_2:
    ldr.w  r10, [r5]
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    add.w  r5, #8
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    ldr.n  r7, [r6]
    nop.n
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion
disappearing_cache_write_another_3:
    ldr.w  r10, [r5]
    add.w  r6, r5, #24
    add.w  r1, r6, #8
    add.w  r5, #8
    isb.w
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r7, [r5]
    nop.n
    ldr.n  r7, [r6]
    nop.n
    ldr.n  r7, [r1]
    ldr.n  r7, [r0, {{CYCCNT}}]
    .align 2
    b.w  finish_case_insertion

finish_case_insertion:
    bl.w save_variant_cycles

end_case_insertion:

    cmp.w r3, #{{test_count}}
    bne.w  test_pull_loop

    bl.w save_test_and_pull_sum

    @ Prepare cache test access.
    ldr.w  r4, =CACHE_ENTRY

    ldr.w  r2, =TEST_OFFSET
    ldr.w  r2, [r2]
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

    {{callHelper('disable_cache', r2)}}

    add.w  r11, #1
    cmp.w  r11, #12
    blt.w  rng_case_loop

    b.w end_label

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
