---
name: IT fold
description: >-
  Main focus is to test whether `IT` instruction folds with the previous
  instruction. Additionaly (and less importantly), conditional execution  of the IT block is verified
  (note: zero flag is set before each measurement).
dumped_symbols:
  times: 170 words    # 2 (zero flag) * 5 (repetitions) * 17 (preInstrs)
  results: 170 words
  foldcnts: 170 words
configurations:
- { code: "flash", lbEn: True, cache_enabled: True }
- { code: "flash", lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{# Avoid memory bus conflicts #}
{% set data = "gpram" if code == "sram" else "sram" %}

{# LDM & STM need at least two registers.
   Otherwise, they are assembled as LDR/STR.
   r1 value is discarded.
   r5 & r6 contains addresses of memories (for loads and stores),
   and are aligned to 2^4 bytes. BFC is used to "undo" write-back.
   Note: LDM/STM is stalled 1 cycle because of AGU waiting for "dirty"
   address register from BFC (r5/r6). #}
{% set preInstrs = [
    "",
    "add.n r7, r7",
    "add.w r7, r7",
    "nop.n",
    "nop.w",

    "ldr.n r7, [r5]",
    "ldr.n r7, [r5]; nop.n",
    "bfc.w r5, #0, #4; ldm.n r5!, {r1, r7}",
    "bfc.w r5, #0, #4; ldm.n r5!, {r1, r7}; nop.n",

    "str.n r7, [r6]",
    "str.n r7, [r6]; nop.n",
    "bfc.w r6, #0, #4; stm.n r6!, {r1, r7}",
    "bfc.w r6, #0, #4; stm.n r6!, {r1, r7}; nop.n",

    "cmp.n r7, #1",
    "cmp.n r7, #0",
    "ldr.n r7, [r5]; cmp.n r7, #1",
    "ldr.n r7, [r5]; cmp.n r1, #1",
] %}

{# Split the cases into multiple flashes, so the code fits in the memory #}
{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set preInstrs = preInstrs[:7] %}
    {% elif gpram_part == 1 %}
        {% set preInstrs = preInstrs[7:14] %}
    {% elif gpram_part == 2 %}
        {% set preInstrs = preInstrs[14:] %}
    {% else %}
        unreachable("invalid gpram_part")
    {% endif %}
{% endif %}

{% block code %}
    ldr.w  r0, dwt
    ldr.w  r5, =read_memory
    ldr.w  r6, =write_memory
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
{% for preInstr in preInstrs %}
{% for zeroFlag in [True, False] %}
{% for reps in range(5) %}
    @ Initialize registers
    mov.w r1, 0
    mov.w r7, 0

    @ Flush flash line buffer
    ldr.w r8, [r7]

    @ Set/clear zero flag
    movs.w r8, {{ 0 if zeroFlag else 1 }}

    @ Get start folds
    ldr.w  r3, [r0, {{ FOLDCNT }}]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r10, [r0, {{ CYCCNT }}]

    @ Prevent pipelining (ldr, str, nop)
    add.w  r8, r8

    {% for i in range(reps) %}
        {{preInstr}}
        it.n eq
        orreq.w r7, 0b1{% for _ in range(i) %}0{% endfor %}
    {% endfor %}

    @ Get finish time
    @ Use narrow to shield againts stalls
    ldr.n  r4, [r0, {{ CYCCNT }}]

    @ Get finish folds
    ldr.w  r11, [r0, {{ FOLDCNT }}]

    @ Save the times and results
    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    sub.w r10, r4, r10
    sub.w r3, r11, r3
    and.w r3, 0xFF

    {{saveValue("times", r10, r8, r9)}}
    {{saveValue("foldcnts", r3, r8, r9)}}
    {{saveValue("results", r7, r8, r9)}}
    bx.n lr

{{ section(data) }}
.align 4
read_memory:
    .word 0xFA57F00D
    .word 0x0B4DF00D

.align 4
write_memory:
    .word 0xFA57C475
    .word 0x0B4DC475

{% endblock %}
