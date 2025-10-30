---
name: Test usage of PRI registers
description: "Test for timings and correct setting values for NVIC PRI_N registers."
dumped_symbols:
    results: 1024 words # 256 priority values * 4 interrupts
    read_times: 1024 words
    write_times: 1024 words
configurations:
- { priority_reg: 0xE000E400, lbEn: True }
- { priority_reg: 0xE000E404, lbEn: True }
- { priority_reg: 0xE000E408, lbEn: True }
- { priority_reg: 0xE000E40C, lbEn: True }
- { priority_reg: 0xE000E410, lbEn: True }
- { priority_reg: 0xE000E414, lbEn: True }
- { priority_reg: 0xE000E418, lbEn: True }
- { priority_reg: 0xE000E41C, lbEn: True }
- { priority_reg: 0xE000E420, lbEn: True }

- { priority_reg: 0xE000E400, lbEn: False }
- { priority_reg: 0xE000E404, lbEn: False }
- { priority_reg: 0xE000E408, lbEn: False }
- { priority_reg: 0xE000E40C, lbEn: False }
- { priority_reg: 0xE000E410, lbEn: False }
- { priority_reg: 0xE000E414, lbEn: False }
- { priority_reg: 0xE000E418, lbEn: False }
- { priority_reg: 0xE000E41C, lbEn: False }
- { priority_reg: 0xE000E420, lbEn: False }
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}

{# Last priority register is used for only 2 interrupts. Others are used for 4. #}
{% if priority_reg == "0xE000E420"|int(base=16) %}
    {% set irqs_per_reg = 2 %}
{% else %}
    {% set irqs_per_reg = 4 %}
{% endif %}

{% extends "asm.s.tpl" %}
{% block code %}
    ldr.w r0, dwt

    ldr.w r1, ={{priority_reg}}

    b.w tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section('flash') }}
.align 4
.thumb_func
tested_code:
{% for offset in range(irqs_per_reg) %}
{% for pri in range(2 ** 8) %}
    mov.w r3, #{{pri}}

    .align 4
    isb.w
    
    @ Get write priority start time
    ldr.w r4, [r0, {{CYCCNT}}]
    
    @ Set priority
    strb.w r3, [r1, #{{offset}}]
    
    @ Get write priority end time
    ldr.w r5, [r0, {{CYCCNT}}]

    @ Get priority
    ldrb.w r6, [r1, #{{offset}}]

    @ Get read priority end time
    ldr.w r7, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label
save:
    sub.w r4, r5, r4
    sub.w r5, r7, r5
    {{saveValue("read_times", r4, r8, r9)}}
    {{saveValue("write_times", r5, r8, r9)}}
    {{saveValue("results", r6, r8, r9)}}

    bx.n lr
{% endblock %}
