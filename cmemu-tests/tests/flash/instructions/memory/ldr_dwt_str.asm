---
name: LDR [DWT] + STR instruction tests
description: >
    Timing and correctness test of LDR from DWT and STR instruction.
    The idea is to check if STR to gpram/sram pipelines with LDR from dwt.
dumped_symbols:
  times: 64 words # 16 repetitions * 4 instructions combinations
  flags: 64 words
  cpicnts: 64 words
  lsucnts: 64 words
configurations:
- { code: gpram, strMemory: gpram, lbEn: True, gpram_part: 0 }
- { code: gpram, strMemory: gpram, lbEn: True, gpram_part: 1 }
- { code: gpram, strMemory: gpram, lbEn: True, gpram_part: 2 }
- { code: gpram, strMemory: gpram, lbEn: True, gpram_part: 3 }
- { code: gpram, strMemory: sram, lbEn: True, gpram_part: 0 }
- { code: gpram, strMemory: sram, lbEn: True, gpram_part: 1 }
- { code: gpram, strMemory: sram, lbEn: True, gpram_part: 2 }
- { code: gpram, strMemory: sram, lbEn: True, gpram_part: 3 }

- { code: sram, strMemory: gpram, lbEn: True, sram_part: 0 }
- { code: sram, strMemory: gpram, lbEn: True, sram_part: 1 }
- { code: sram, strMemory: sram, lbEn: True, sram_part: 0 }
- { code: sram, strMemory: sram, lbEn: True, sram_part: 1 }

- { code: flash, strMemory: gpram, lbEn: True }
- { code: flash, strMemory: sram, lbEn: True }
- { code: flash, strMemory: gpram, lbEn: False }
- { code: flash, strMemory: sram, lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 16 %}
{% set instructions = [
    ("ldr.w", "str.w"),
    ("ldr.w", "str.n"),
    ("ldr.n", "str.w"),
    ("ldr.n", "str.n"), 
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set instructions = instructions[:1] %}
    {% elif gpram_part == 1 %}
        {% set instructions = instructions[1:2] %}
    {% elif gpram_part == 2 %}
        {% set instructions = instructions[2:3] %}
    {% elif gpram_part == 3 %}
        {% set instructions = instructions[3:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% elif code == "sram" %}
    {% if sram_part == 0 %}
        {% set instructions = instructions[:2] %}
    {% elif sram_part == 1 %}
        {% set instructions = instructions[2:] %}
    {% else %}
        unreachable("invalid sram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
    
    @ Prepare ldr and str input values
    add.w  r5, r8, {{CYCCNT}} @ dwt + CYCCNT
    ldr.w  r6, =mem_{{strMemory}}
    mov.w  r7, #0

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
{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for ldrInstr, strInstr in instructions %}
{% for reps in range(repetitions) %}
    @ Reset flash line buffer
    ldr.w r2, [r7, r7]

    @ Clear flags
    mov.w r9, #0
    msr.w apsr_nzcvq, r9

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r8, {{counter}}]

    {% for i in range(reps) %}
        {{ldrInstr}} r0, [r5, r7]
        {{strInstr}} r1, [r6, r7] 
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r8, {{counter}}]
    
    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_and_flags:
    mrs.w r9, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r11)}}
    {{saveValue("flags", r9, r10, r11)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r2, r10, r11)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r2, r10, r11)}}
    
    bx.n lr

{{ section("sram")}}
.align 4
mem_sram: .word 0xCAFE

{{ section("gpram")}}
.align 4
mem_gpram: .word 0xBEE5

{% endblock %}
