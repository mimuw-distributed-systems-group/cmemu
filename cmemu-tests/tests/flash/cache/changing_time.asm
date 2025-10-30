name: CACHE mode test
description: "Setting mode to cache and time"
dumped_symbols:
  times: 10 words
  stats: 10 words
  count: 10 words
configurations:
- { code: "sram", lbEn: True , repetitions: 10}

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set mode_gpram = 0 %}
{% set mode_cache = 1 %}
{% set mode_off = 3 %}
{% set mode_changing = 8 %}
{% set mode_invalidating = 4 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}

.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for reps in range(repetitions) %}
    @ Prepare register addresses
    ldr.w  r11, VIMS_STAT
    ldr.w  r12, VIMS_CTL
    eor.w r10, r10, r10
    
    @ Stabilize to be safe
    bl.w stabilize_mode
    
    @ Change mode
    ldr.w r2, [r12]
    and.w r2, r2, #0xfffffffc
    orr.w r2, r2, #{{mode_cache}}
    str.w r2, [r12]
    
    bl.w stabilize_mode
    
    @ Prepare to set to gpram
    ldr.w r2, [r12]
    and.w r2, r2, #0xfffffffc
    orr.w r2, r2, #{{mode_gpram}}
    
    @ Get start time
    ldr.w  r7, [r0, {{CYCCNT}}]
    
    str.w r2, [r12]
    
    ldr.w r9, [r11]
    {% for i in range(reps) %}
        nop.w
    {% endfor %}
    
    @ Get finish stats
    bl.w count_time
    
    @ Get finish time
    ldr.w  r2, [r0, {{CYCCNT}}]
    sub.w r7, r2, r7
    
    
    bl.w save
    
    bl.w reset_mode

{% endfor %}
    
    b.w end_label

@ Waits for mode to stabilize counting loops in r10
@ Assumes:
@ r10 = 0
@ r11 = VIMS_STAT
@ Destroys:
@ r2
.align 4
count_time:
    @ check if changing
    ldr.w r2, [r11]
    @ increment counter
    add.w r10, r10, #1
    ands.w r2, r2, #{{mode_changing + mode_invalidating}}
    bne.w count_time
    
    bx.n lr


@ Waits for mode to stabilize and turns off the cache.
@ Assumes:
@ r11 = VIMS_STAT
@ r12 = VIMS_CTL
@ Destroys:
@ r2
.align 4
stabilize_and_reset_mode:
    @ check if changing
    ldr.w r2, [r11]
    ands.w r2, r2, #{{mode_changing + mode_invalidating}}
    bne.w stabilize_and_reset_mode

reset_mode:
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
save:

    {{saveValue('times', r7, r2, r3)}}
    {{saveValue('stats', r9, r2, r3)}}
    {{saveValue('count', r10, r2, r3)}}

    bx.n lr


{{ section("flash") }}

.align 8
CACHE_ENTRY:    .skip 1024*8

{{ section("sram") }}

.align	4
@ [TI-TRM] 3.2.8 Cortex-M3 Memory Map
@ [TI-TRM] 7.9.2.1 STAT Register
@ [TI-TRM] 7.9.2.2 CTL Register
VIMS_STAT:	    	.word	0x40034000
VIMS_CTL:	    	.word	0x40034004


{% endblock %}
