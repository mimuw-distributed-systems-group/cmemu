---
name: Test proving that Speculative Branch Target Fetch can be promoted to Branch Target Forwarding
description: >
    Fetch can be informed whether the B<cc> branch will be taken or not
    even before execution of the B<cc> instruction.

    Take a look at configurations with div_time = 10.
    If the information whether the branch will be taken or not was not accessible
    earlier than the execute stage of branch, the fetch wouldn't have enough time
    to load all necessary instructions.

    Probably, if currently executed instruction does not set the flags,
    the B<cc> is treated by decode as B or NOP depending on the flags.

dumped_symbols:
  times: 8 words
  results: 8 words
configurations:
- { cc: "", div_time: 12 }
- { cc: "eq", div_time: 12 }
- { cc: "ne", div_time: 12 }
- { cc: "", div_time: 10 }
- { cc: "eq", div_time: 10 }
- { cc: "ne", div_time: 10 }
- { cc: "eq", div_time:  9 }
- { cc: "ne", div_time:  9 }
...

{% set div_inputs = {
   2: [0, 0],
   3: [2144514007, 3828339523],
   5: [2, 1],
   6: [1000, 7],
   7: [1000, 3],
   9: [712027995, 2514],
  10: [4260791909, 3050],
  11: [3795273750, 58],
  12: [2147483648, 1]
} %}

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
{% for adds in range(0, 8) %}
  @ Prepare register contents
  ldr.w  r5, =#{{div_inputs[div_time].0}}
  ldr.w  r6, =#{{div_inputs[div_time].1}}
  movs.w  r8, #0  @ clear R8, but also set Zero flag

  .align 4
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]

  udiv.w r7, r5, r6  @ takes {{div_time}} cycles
  b{{cc}}.w jump_target_{{adds}}

  @ did not jump...
  {% for _ in range(adds) %}  @ this is to check how many words were fetched during execution of UDIV
    adds.w r8, r8, 10
  {% endfor %}

  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]
  b.w next_{{adds}}

  @ These ADDS should not execute, but they are to prevent prefetching code from jump_target
  adds.w r8, r8, 100
  adds.w r8, r8, 100
  adds.w r8, r8, 100

  .ltorg

.align 4
jump_target_{{adds}}:
  {% for _ in range(adds) %}  @ this is to check how many words were fetched during execution of UDIV
    adds.w r8, r8, 1
  {% endfor %}

  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]

next_{{adds}}:
  bl.w save
{% endfor %}
    
  b.w end_label

save:
  subs.w r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("results", r8, r3, r4)}}
  bx.n lr
{% endblock %}
