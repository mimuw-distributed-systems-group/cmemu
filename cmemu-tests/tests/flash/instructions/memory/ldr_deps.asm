---
name: Tests of registers dependencies of LDR
description: "Timing test of registers dependencies of LDR"
dumped_symbols: 
  times: 17 words
  flags: 17 words
  cpicnts: 17 words
  lsucnts: 17 words
configurations:
- { code: "gpram", addr: "gpram", lbEn: True }
- { code: "gpram", addr: "sram", lbEn: True }
- { code: "gpram", addr: "flash", lbEn: True }
- { code: "gpram", addr: "flash", lbEn: False }
- { code: "sram", addr: "gpram", lbEn: True }
- { code: "sram", addr: "sram", lbEn: True }
- { code: "sram", addr: "flash", lbEn: True }
- { code: "sram", addr: "flash", lbEn: False }
- { code: "flash", addr: "gpram", lbEn: True }
- { code: "flash", addr: "gpram", lbEn: False }
- { code: "flash", addr: "sram", lbEn: True }
- { code: "flash", addr: "sram", lbEn: False }
- { code: "flash", addr: "flash", lbEn: True }
- { code: "flash", addr: "flash", lbEn: False }
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r3, dwt

    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% set test_cases = [
    [
        "ldr.w r4, [r5, r11];",
        "ldr.w r4, [r5, r11];",
        "ldr.w r6, [r4, r5];",
        "ldr.w r7, [r6, r5];",
        "ldr.w r8, [r7, r6];",
        "ldr.w r9, [r7, r8];",
    ],
    [
        "ldr.w r7, [r7];",
        "ldr.w r7, [r7];",
        "ldr.w r6, [r6];",
        "ldr.w r6, [r6];",
        "ldr.w r7, [r7];",
        "ldr.w r6, [r6];",
        "ldr.w r7, [r7];",
        "ldr.w r6, [r6];",
    ],
    [
        "ldr.w r7, [r7]",
        "nop",
        "ldr.w r7, [r7]",
    ],
] %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for case in test_cases %}
{% for prefix_len in range(1, (case | length) + 1) %}
    @ Initialize test
    bl.w initialize

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w r2, [r3, {{counter}}]
    @ Those adds are to prohibit pipelining of ldrs
    add.w r10, r10, r11

    @ Execute instructions
    {% for i in range(prefix_len) %}
        {{case[i]}}
    {% endfor %}

    add.w r10, r10, r11
    @ Get end counter value
    ldr.w r0, [r3, {{counter}}]

    @ Save measurements
    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

initialize:
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Prepare ldr arguments
    ldr.w r5, =cell_0
    ldr.w r6, =cell_2
    mov.w r7, r6
    mov.w r11, #0

    bx.n lr

save_times_and_flags:
    mrs.w r1, apsr
    sub.w r0, r0, r2

    {{saveValue("times", r0, r2, r4)}}
    {{saveValue("flags", r1, r2, r4)}}

    bx.n lr

save_cpicnt:
    sub.w r0, r0, r2
    ands.w r0, r0, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r0, r2, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r0, r0, r2
    ands.w r0, r0, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r0, r2, r4)}}
    
    bx.n lr

{{ section(addr) }}
.align 4
cell_0: .word 0x4
cell_1: .word 0x8
cell_2: .word cell_2
cell_3: .word 0x4
cell_4: .word 0x4
cell_5: .word 0x4
cell_6: .word 0x4
cell_7: .word 0x4
cell_8: .word 0x4
{% endblock %}
