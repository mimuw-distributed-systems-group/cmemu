---
name: ADD LDR with varying length tests
description: "Timing of instructions ADD.n/ADD.w/<nothing> intertwined with LDR.n/LDR.w/<nothing>."
dumped_symbols:
  results: 96 words
  times: 96 words
  flags: 96 words
  cpicnts: 96 words
  lsucnts: 96 words
configurations:
- { code: "sram" , lbEn: True, sram_part: 0 }
- { code: "sram" , lbEn: True, sram_part: 1 }
- { code: "flash", lbEn: True }
- { code: "flash", lbEn: False }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 12 %}
{% set load_val = "0xb" %}
{% set add_val = "0x3" %}

{#
    Total sizes of instructions in this order are
    [6, 4, 2, 6, 4, 2, 4, 8]
    With this all gpram subconfigurations have total of 12 bytes of instructions.
    Also both sram subconfigurations have total of 18 bytes of instructions.
    This way the test can be split into smaller number of configurations.
#}
{% set instructions = [
        ("add.w", "ldr.n"),
        ("add.n", "ldr.n"),
        ("add.n", None),
        ("add.n", "ldr.w"),
        ("add.w", None),
        (None, "ldr.n"),
        (None, "ldr.w"),
        ("add.w", "ldr.w"),
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set instructions = instructions[0:3] %}
    {% elif gpram_part == 1 %}
        {% set instructions = instructions[3:6] %}
    {% elif gpram_part == 2 %}
        {% set instructions = instructions[6:8] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% elif code == "sram" %}
    {% if sram_part == 0 %}
        {% set instructions = instructions[0:4] %}
    {% elif sram_part == 1 %}
        {% set instructions = instructions[4:8] %}
    {% else %}
        unreachable("invalid sram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    ldr.w r4, =load_arg

    b.w    tested_code

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

{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for add_instr, load_instr in instructions %}
{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare input values
    mov.w  r6, #{{load_val}}
    mov.w  r7, #{{add_val}}

    @ Align and clear PIQ
    .align 4
    isb.w
    
    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]
    {% for _ in range(reps) %}
        {% if add_instr is not none %}
            {{add_instr}} r7, r6, r7
        {% endif %}
        {% if load_instr is not none %}
            {{load_instr}} r6, [r4]
        {% endif %}
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_and_flags:
    mrs.w r5, apsr
    sub.w r2, r3, r2

    {{saveValue('times', r2, r11, r12)}}
    {{saveValue('results', r7, r11, r12)}}
    {{saveValue('flags', r5, r11, r12)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r11, r12)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r11, r12)}}

    bx.n lr

{{ section("sram") }}
.align 4
load_arg: .word {{load_val}}

{% endblock %}
