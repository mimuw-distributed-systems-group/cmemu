---
name: Test PIQ status after executing ISB instruction
description: >
    To check PIQ status we overwrite code and see if overwrite results are visible to the core.
dumped_symbols:
  results: 100 words
configurations:
- { code: gpram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: '' }
- { code: sram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: '' }

- { code: gpram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3' }
- { code: gpram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3' }
- { code: gpram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3' }
- { code: sram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3' }
- { code: sram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3' }
- { code: sram, init: 'movs.w r2, 0; movs.w r3, 1', instrs: 'adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3; adds.n r2, r3' }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    ldr.w r8, exploit
    b.w tested_code

.align 2
exploit:
    movs.w r1, 0

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for offset in [0, 4, 8, 12, 16] %}
    @ Initialize
    movs.n r0, 0
    movs.n r1, 1
    ldr.w r9, =modification_start_{{loop.index}}
    movs.w r11, {{offset}}

    {{init}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Tested instructions
    {{instrs}}

    @ overwrite instruction in `modification_start_i + offset` with `movs r1, 0`
    str.w r8, [r9, r11]
    @ TODO: when write buffer gets enabled, there should be also DSB instruction
    isb.w
modification_start_{{loop.index}}:
    @ each `add` adds 1 to `r0`
    @ but if core notices overwriting, all adds after the overwritten instruction
    @ becomes nops (`add r0, 0`)
    {% for i in range(5) %}
      adds.w r0, r1
    {% endfor %}

    @ we will check if core notices overwriting or not
    @ core should notice, because of ISB instruction
    {{ saveResult(r0, r1, r11) }}
    
{% endfor %}

    b end_label

{% endblock %}
