---
name: LDR SP pipelining
description: "A test in which LDR to SP causes pipelining issues, but LDR to a normal register doesn't"
dumped_symbols:
  cycles: 1 words
configurations: []
product:
- first_ldr_reg: ["sp", "r8"]
  lbEn: [True, False]
  wbEn: [True, False]
  cacheEn: [True, False]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = wbEn %}
{% device:cache_enabled = cacheEn %}
{% extends "asm.s.tpl" %}
{% block code %}
    ldr.w r0, dwt

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
    ldr.w {{first_ldr_reg}}, flash_addr_1
    ldr.w r1, flash_addr_2

    adds.n r1, #1
    adds.n r1, #1
    adds.n r1, #1
    ldr.n r2, [r0, {{CYCCNT}}]
    add.w r1, #1
    ldr.w r3, [r0, {{CYCCNT}}]

    sub.w r3, r2

    .align 2
    {{ saveValue('cycles', r3, r4, r5) }}

    .align 2
    b.w end_label

.align 3
flash_addr_1: .word 0

.skip 16

.align 3
flash_addr_2: .word 0

{% endblock %}
