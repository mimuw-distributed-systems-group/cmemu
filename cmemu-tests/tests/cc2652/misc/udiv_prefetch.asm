---
name: Measure how many words are loaded during single UDIV
description: >
    We can measure this by executing wide ADDs and checking when pipeline stalls
    become visible.

dumped_symbols:
  times: 8 words
  results: 8 words
configurations:
- { div_time: 12 }
- { div_time: 11 }
- { div_time: 10 }
- { div_time:  9 }
- { div_time:  8 }
- { div_time:  7 }
- { div_time:  6 }
- { div_time:  5 }
- { div_time:  3 }
- { div_time:  2 }
...

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
  movs.w  r8, #0

  .align 4
  isb.w   @ Clear PIQ

  @ Get start time
  ldr.w  r2, [r0, {{CYCCNT}}]

  udiv.w r7, r5, r6  @ takes {{div_time}} cycles
  
  {% for _ in range(adds) %}  @ this is to check how many words were fetched during execution of UDIV
    adds.w r8, r8, 1
  {% endfor %}

  @ Get finish time
  ldr.w  r3, [r0, {{CYCCNT}}]
  bl.w save
{% endfor %}
    
  b.w end_label

save:
  subs.n r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("results", r8, r3, r4)}}
  bx.n lr
{% endblock %}
