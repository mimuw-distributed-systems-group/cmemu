---
name: Measure when is triggered decode while executing LDRD instruction
description: >
  We can measure this by checking when the branch target address is forwarded
  to the fetch.

dumped_symbols:
  times: 8 words
  results: 8 words
configurations: [cache_enabled: True]
...
{% device:cache_enabled = cache_enabled %}

{% set div_inputs = {
   2: [0, 0],
   3: [2144514007, 3828339523],
   5: [2, 1],
   6: [1000, 7],
   7: [1000, 3],
   8: [28033508, 3183],
   9: [712027995, 2514],
  10: [4260791909, 3050],
  11: [3795273750, 58],
  12: [2147483648, 1]
} %}

@ The time should be long enough so that PIQ is filled after execution of UDIV.
{% set div_time = 12 %}

{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section("flash") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
  @ Prepare register contents
  ldr.w  r5, =#{{div_inputs[div_time].0}}
  ldr.w  r6, =#{{div_inputs[div_time].1}}
  movs.w  r8, #0
  ldr.w r10, =cell

  .align 4
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]

  udiv.w r7, r5, r6  @ takes {{div_time}} cycles, makes PIQ fully filled
  ldrd.w r8, r9, [r10]
  b.w jump_target
  
  @ Prevents from prefetching instructions from jump_target
  nop.w; nop.w; nop.w

.align 3
jump_target:
  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]
  subs.n r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}

  b.w end_label

{{ section("flash") }}
.align 4
cell:
  .word 0x01234567
  .word 0x89ABCDEF
{% endblock %}
