---
name: IT fold
description: >-
    Main focus is to test whether `IT` instruction folds with the previous
    instruction. Additionaly (and less importantly), conditional execution 
    of the IT block is verified
    (note: zero flag is set before each measurement).
dumped_symbols:
  times: auto
  results: auto
  foldcnts: auto
configurations:
- { code: "gpram", lbEn: true, gpram_part: 0 }
- { code: "gpram", lbEn: true, gpram_part: 1 }
- { code: "gpram", lbEn: true, gpram_part: 2 }
- { code: "sram", lbEn: true, sram_part: 0 }
- { code: "sram", lbEn: true, sram_part: 1 }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }
...
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
    "add.n r7, sp, 4",
    "add.n sp, sp, 4",
    "add.n sp, r7",
    "add.n r7, sp, r7",
    "mov.n sp, r7",
    "mov.n r7, sp",
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

    "cmp.n sp, r7",
    "cmp.n r7, sp",
    "cmp.n r7, #1",
    "cmp.n r7, #0",
    "ldr.n r7, [r5]; cmp.n r7, #1",
    "ldr.n r7, [r5]; cmp.n r1, #1",

    "muls.n r7, r7",
    "movs.n r7, r1",
    "it ne; mulne.n r7, r7",
    "it eq; muleq.n r7, r7",
    "it eq; cmpeq.n sp, r7",
    "it eq; cmpeq.n r7, sp",
    "it ne; nopne.n",
    "it eq; addeq.n r6, r1, r7",
    "adds.n r7, r7, r7",

    "ldr.n r7, [sp, 4]",
    "cbnz.n r1, {skip_label}",
    "adcs.n r7, r1",
    "adr.n r7, .+4",
] %}


{# Split the cases into multiple flashes, so the code fits in the memory #}
{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set preInstrs = preInstrs[:12] %}
    {% elif gpram_part == 1 %}
        {% set preInstrs = preInstrs[12:25] %}
    {% elif gpram_part == 2 %}
        {% set preInstrs = preInstrs[25:] %}
    {% else %}
        {{unreachable("invalid gpram_part")}}
    {% endif %}
{% elif code == "sram" %}
    {% if sram_part == 0 %}
        {% set preInstrs = preInstrs[:24] %}
    {% elif sram_part == 1 %}
        {% set preInstrs = preInstrs[24:] %}
    {% else %}
        {{unreachable("invalid sram_part")}}
    {% endif %}
{% endif %}

{% block code %}
    ldr.w  r0, dwt
    ldr.w  r5, =read_memory
    ldr.w  r6, =write_memory
    mov.w  sp, r5
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
    {% set skip_label = uniq_label("skip") %}
    @ Initialize registers
    movs.n r1, 0
    movs.n r7, 0

    @ Flush flash line buffer
    ldr.n r2, [r7]

    @ Set/clear zero flag
    movs.n r2, {{ 0 if zeroFlag else 1 }}

    @ Get start folds
    ldr.w  r3, [r0, {{ FOLDCNT }}]

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start time
    ldr.w  r10, [r0, {{ CYCCNT }}]

    @ Prevent pipelining (ldr, str, nop)
    add.w  r2, r2

    {% for i in range(reps) %}
        {{preInstr.replace('{skip_label}', skip_label)}}
        it.n eq
        orreq.w r7, 0b1{% for _ in range(i) %}0{% endfor %}
    {% endfor %}

    @ Get finish time
    @ Use narrow to shield againts stalls
    {{skip_label}}:
    ldr.n  r4, [r0, {{ CYCCNT }}]

    @ Get finish folds
    ldr.w  r11, [r0, {{ FOLDCNT }}]

    @ Save the times and results
    bl.w save
    {{ inc_auto_syms() }}
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
