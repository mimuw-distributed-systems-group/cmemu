---
name: Test simple self-modifying code
description: >
    We measure how many bytes are in the Prefetch Input Queue when the queue is full.
    We do it by replacing `add.w r0, r1` with `mov.w r1, 0`.
    
    1) When the instruction was loaded to PIQ before `str`, the `str` instruction gives no effect.
    2) When the instruction was loaded after `str`, there is observable effect in final
       `r0` value change.
    
    The results suggests that when the `str` instruction is running, next 3 words are loaded into
    the core. This also means that "`str` decode" was the last moment when the transfer loading
    third of the three next words was requested.

dumped_symbols:
  results: 1 words
configurations:
- { offset: 12 }
- { offset: 16 }
- { offset: 20 }
- { offset: 24 }
- { offset: 28 }
- { offset: 32 }
...
{% device:line_buffer_enabled = False %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    ldr.w r4, =tested_code
    movs.n r1, 1
    subs.n r4, r1
    movs.n r0, 0
    ldr.w r5, exploit
    movs.n r3, {{offset}}

    b.w tested_code

.align 2
exploit:
    movs.w r1, 0

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section("gpram") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    isb.w

    @ fill Prefetch Input Queue
    {% for i in range(8) %}
      adds.n r0, r1
    {% endfor %}

    @ overwrite instruction in `tested_code + offset` with `movs r1, 0`
    str.w r5, [r4, r3]

    @ each `add` adds 1 to `r0`
    @ but if core notices overwriting, all adds after the overwritten instruction
    @ becomes nops (`add r0, 0`)
    {% for i in range(4) %}
      adds.w r0, r1
    {% endfor %}

    @ we will check if core notices overwriting or not
    @ (because the instruction was prefetched earlier)
    {{ saveResult(r0, r8, r9) }}

    b end_label
{% endblock %}
