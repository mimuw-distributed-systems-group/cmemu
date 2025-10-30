---
name: More exhaustive LDM.N tests
description:
    Tests LDM.N focusing on pipelining and registers dependency
dumped_symbols:
  # 8 repetitions * 32 instruction-offset combinations
  times: 256 words
  flags: 256 words
  cpicnts: 256 words
  lsucnts: 256 words
configurations:
- { code: "sram", memory: "sram", lbEn: true }
- { code: "sram", memory: "flash", lbEn: true }
- { code: "sram", memory: "flash", lbEn: false }
- { code: "flash", memory: "sram", lbEn: true }
- { code: "flash", memory: "sram", lbEn: false }
- { code: "flash", memory: "flash", lbEn: true }
- { code: "flash", memory: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set memoryCell = 'memory_cell' %}
{% set repetitions = 8 %}

@ i-th executed instruction is `instrs[(offset + i) % len(instrs)]`
@ [(
@   list of instructions to repeat,
@   list of starting offsets (to rotate previous list without copy-pasting it),
@ )]
@ Note that `ldm.n rX, {rY}` (no writeback, X!=Y) will be assembled to `ldr.n`. It is intentional.
@ Test cases: [
@     register-dependent `ldr.n`s
@     register-dependent `ldm.n`s
@     register-independent `ldr.n`s
@     register-independent `ldm.n`s
@     register-dependent `ldr.n`s and `ldm.n`s
@     register-independent `ldr.n`s and `ldm.n`s
@     register-dependent `ldr.n`s and `ldm.n`s
@     register-independent `ldr.n`s and `ldm.n`s
@     register-dependent `ldr.n`s and `ldm.n`s
@     register-independent `ldr.n`s and `ldm.n`s
@     register-dependent `ldr.n`s and `ldm.n`s
@     register-independent `ldr.n`s and `ldm.n`s

@     `ldm.n`s with writeback depending on the previous instruction
@     `ldm.n`s without writeback depending on the previous instruction
@     `ldm.n`s with writeback depending on instruction before the previous one
@     `ldm.n`s without writeback depending on instruction before the previous one
@ ]
{% set testParameters = [
    (["ldr.n r5, [r5];"], [0]),
    (["ldm.n r5, {r5};"], [0]),
    (["ldr.n r5, [r5];", "ldr.n r6, [r6];"], [0]),
    (["ldm.n r5, {r5};", "ldm.n r6, {r6};"], [0]),
    (["ldr.n r5, [r5];", "ldm.n r5, {r5};"], [0, 1]),
    (["ldr.n r5, [r5];", "ldm.n r6, {r6};"], [0, 1]),
    (["ldr.n r5, [r5];", "ldr.n r5, [r5];", "ldm.n r5, {r5};"], [0, 1, 2]),
    (["ldr.n r5, [r5];", "ldr.n r6, [r6];", "ldm.n r7, {r7};"], [0, 1, 2]),
    (["ldr.n r5, [r5];", "ldm.n r5, {r5};", "ldm.n r5, {r5};"], [0, 1, 2]),
    (["ldr.n r5, [r5];", "ldm.n r6, {r6};", "ldm.n r7, {r7};"], [0, 1, 2]),
    (["ldr.n r5, [r5];", "ldr.n r5, [r5];", "ldm.n r5, {r5};", "ldm.n r5, {r5};"], [0, 1, 2, 3]),
    (["ldr.n r5, [r5];", "ldr.n r6, [r6];", "ldm.n r5, {r5};", "ldm.n r6, {r6};"], [0, 1, 2, 3]),

    (["ldm.n r5!, {r6};", "ldm.n r6!, {r5};"], [0]),
    (["ldm.n r5, {r6};", "ldm.n r6, {r5};"], [0]),
    (["ldm.n r6!, {r5};", "ldm.n r7!, {r6};", "ldm.n r5!, {r7};"], [0]),
    (["ldm.n r6, {r5};", "ldm.n r7, {r6};", "ldm.n r5, {r7};"], [0]),
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set testParameters = testParameters[:10] %}
    {% elif gpram_part == 1 %}
        {% set testParameters = testParameters[10:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    {% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
        mov.w r1, {{counter}}
        ldr.w r9, ={{save_func}}

        bl.w tested_code
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov r11, lr
{% for repeatedInstrs, offsets in testParameters %}
{% set reapeatedInstrsLen = repeatedInstrs | length %}
{% for offset in offsets %}
{% for instrsUsed in range(1, 1 + repetitions) %}
    @ Prepare registers
    bl.w initialize

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, #0

    @ Execute instructions
    {% for i in range(instrsUsed) %}
        {{repeatedInstrs[(offset + i) % reapeatedInstrsLen]}}
    {% endfor %}

    @ ADDs to prohibit pipelining of LDRs with executed instructions
    add.w r10, #0

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    blx.n r9

{% endfor %}
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r11

.align 2
initialize:
    @ Clear flags
    mov.w r10, #0
    msr.w apsr_nzcvq, r10

    ldr.w r5, ={{memoryCell}}
    ldr.w r6, ={{memoryCell}}
    ldr.w r7, ={{memoryCell}}
    bx.n lr

.align 2
.thumb_func
save_time_and_flags:
    mrs.w r8, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r8, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr


{{ section(memory) }}
.align 2
{{memoryCell}}:
    .word {{memoryCell}}

{% endblock %}
