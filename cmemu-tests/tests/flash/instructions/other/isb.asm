---
name: ISB
description: "Timing and correctness test of ISB instruction"
dumped_symbols:
  results: 100 words
  times: 100 words
configurations:
# ISB instruction timing
- { code: gpram, lbEn: true, data: gpram, init: '', pre: '', post: '' }
- { code: sram, lbEn: true, data: gpram, init: '', pre: '', post: '' }
- { code: flash, lbEn: true, data: gpram, init: '', pre: '', post: '' }
- { code: flash, lbEn: false, data: gpram, init: '', pre: '', post: '' }

# Is instruction preceding and following ISB executed?
- { code: gpram, lbEn: true, data: gpram, init: 'movs.n r5, 0; movs.n r6, 1', pre: 'adds.n r5, r6', post: '' }
- { code: gpram, lbEn: true, data: gpram, init: 'movs.n r5, 0; movs.n r6, 1', pre: '', post: 'adds.n r5, r6' }

# Pipelined instruction followed by ISB
- { code: gpram, lbEn: true, data: gpram, init: 'movs.n r5, 0; ldr.w r6, =cell; movs.n r7, 0', pre: 'ldr.w r5, [r6, r7]', post: '' }
- { code: gpram, lbEn: true, data: sram, init: 'movs.n r5, 0; ldr.w r6, =cell; movs.n r7, 0', pre: 'ldr.w r5, [r6, r7]', post: '' }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    
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
{% for i in range(8) %}
    @ Reset register saved to results so it does not infer next cases
    movs.n r5, 0

    @ Initialization code
    {{init}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for j in range(i) %}
        {{ pre }}
        isb.w
        {{ post }}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]    
    bl.w save
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}
    {{saveResult(r5, r3, r4)}}
    bx.n lr

{{ section(data) }}
.align 4
cell: .word 0xCAFE

{% endblock %}
