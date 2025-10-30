name: CACHE mode test
description: "Setting mode to cache and time"
dumped_symbols:
  debug_sum: 3 words
  address: 2 words
  COMB_res_number_LEN_4_time_3_LEN_7_time2_LEN_7_time1_LEN_7_time0_LEN_7: 32 words
configurations:
- { code: "sram", lbEn: true, pull_count: 1024 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set mode_gpram = 0 %}
{% set mode_cache = 1 %}
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

{{ section(code) }}

.align 4
.thumb_func
.type tested_code, %function

tested_code:
    {{callHelper('enable_cache', r2)}}
    {{callHelper('synchronize_rng', r2, '#240')}}
    {{callHelper('disable_cache', r2)}}

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
	

    mov.w  r8, 0
	
    ldr.w  r4, =CACHE_ENTRY
    add.w  r4, #0x2000
    
{% set offsets = [2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24, 25, 26] %}
{% for offset in offsets %}
    ldr.w  r3, [r4, #{{offset * 8}}]
    add.w  r8, r3
    add.w  r3, #{{offset}} @ Wait a bit

{% endfor %}
    
    ldr.w  r4, =CACHE_ENTRY
    mov.w  r9, 0

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
    
    add.w  r4, #8
    add.w  r9, #1
    mov.w  r2, #32
    cmp.w  r9, r2
    
    blo.w test_loop
    
    ldr.n  r5, [r0, {{CYCCNT}}]
    
    bl.w save_final
    
    bl.w reset_mode

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
    and.w r2, r2, ~0x7
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




{{ section("flash") }}

.align 15
CACHE_ENTRY:    .4byte {% for i in range(cache_size) %} {{ i }}, {{ i + 1000000007 }}, {% endfor %}

{% endblock %}
