---
name: LDR/STR bitband access test
description: "Simple correctness test of LDR/STR from sram using bitband addresses"
dumped_symbols:
  times: 32 words
  results: 32 words
  cpicnts: 32 words
  lsucnts: 32 words
configurations:
 - { code: "gpram", lbEn: True, accessInstr: "ldr.w" }
 - { code: "gpram", lbEn: True, accessInstr: "str.w" }
 - { code: "sram", lbEn: True, accessInstr: "ldr.w" }
 - { code: "sram", lbEn: True, accessInstr: "str.w" }
 - { code: "flash", lbEn: True, accessInstr: "ldr.w" }
 - { code: "flash", lbEn: True, accessInstr: "str.w" }
 - { code: "flash", lbEn: False, accessInstr: "ldr.w" }
 - { code: "flash", lbEn: False, accessInstr: "str.w" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr arguments (calculate `cell` bitband address)
    @ r4 - cell address
    @ r5 - cell's 0 bit bitband address
    ldr.w r4, =cell
    lsl.w r5, r4, #5
    add.w r5, 0x22000000

    b.w tested_code
.ltorg
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for i in range(32) %}
    {% if accessInstr == "str.w" %}
        @ Prepare arguments
        mov.w r6, #{{i}}
    {% endif %}

    .align 4
    isb.w

    @ Get start time
    ldr.n r2, [r0, {{counter}}]

    {{accessInstr}} r6, [r5, #{{i*4}}]

    @ Get finish time
    ldr.n r3, [r0, {{counter}}]

    @ Save test results
    bl.w {{save_func}}

    @ Reset cell value
    mov.w r6, #0x33333333
    str.n r6, [r4]
{% endfor %}
{% endfor %}

    b.w end_label

save_time_and_result:
    sub.w r2, r3, r2

    {{saveValue("times", r2, r1, r3)}}

    {% if accessInstr == "str.w" %}
        {# As a result for STR we use an accessed cell value #}
        ldr.w r6, [r4]
    {% elif accessInstr != "ldr.w" %}
        {{ unreachable("Invalid instruction") }}
    {% endif %}

    {{saveValue("results", r6, r1, r3)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r1, r3)}}
    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r1, r3)}}
    bx.n lr

{{ section("sram") }}
.align 4
cell: .word 0x33333333
{% endblock %}
