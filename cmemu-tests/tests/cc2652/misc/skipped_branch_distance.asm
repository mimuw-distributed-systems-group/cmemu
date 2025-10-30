---
name: Measurements of timings of skipped branching instructions.
description:
    Timing tests for skipped branching instructions.

    Only B<cc>, BX<cc>, BL<cc>, BLX<cc>, CBZ, CBNZ, MOV<cc>.n
    are tested in this file.
    <cc> instructions are placed inside IT blocks.
dumped_symbols:
  # 13 instructions * 2 alignments * 8 distances
  results: 208 words
  times: 208 words
configurations:
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: false }
- { code: "flash", lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ testParameters: [(
@   tested instruction,
@   optional register argument or None (if no such argument),
@   register containing jump target or None (if the target is a label),
@   immediate used to initialize r7 register and set xPSR flags (by using MOVS instruction),
@   boolean flag "insert `it.n eq` before the tested instruction",
@ )]
{% set testParameters = [
    ("beq.n", None, None, "1", False),
    ("beq.n", None, None, "1", True),
    ("beq.w", None, None, "1", False),
    ("beq.w", None, None, "1", True),
    ("bleq.w", None, None, "1", True),
    ("bxeq.n", None, "r8", "1", True),
    ("bxeq.n", None, "lr", "1", True),
    ("blxeq.n", None, "r8", "1", True),
    ("blxeq.n", None, "lr", "1", True),
    ("cbz.n", "r7", None, "1", False),
    ("cbnz.n", "r7", None, "0", False),
    ("moveq.n", "pc", "lr", "1", True),
    ("moveq.n", "pc", "r8", "1", True),
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set testParameters = testParameters[:6] %}
    {% elif gpram_part == 1 %}
        {% set testParameters = testParameters[6:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

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

{% for instr, registerArgument, jumpTargetRegister, flagImm, inItBlock in testParameters %}
{% for wordAligned in [True, False] %}
{% for distance in range(0, 8) %}
    {% set jumpLabel = "jump_target_%d_%s_%d_%d_%s" % (distance, instr, inItBlock, wordAligned, jumpTargetRegister) %}

    {% if jumpTargetRegister %}
        adr.w {{jumpTargetRegister}}, {{jumpLabel}} + 1
    {% endif %}

    @ Set flags and sometimes used r7 register
    movs.n r7, #{{flagImm}}

    @ Zero r5 so that number of adds not branched over is recorded
    mov.w r5, #0

    @ Align and clear PIQ
    .align 3
    {% if (wordAligned and inItBlock) or (not wordAligned and not inItBlock) %}
        nop.n
    {% endif %}
    isb.w

    @ Get start time and prevent the following it.n from folding
    ldr.w r2, [r0, {{CYCCNT}}]

    {% if inItBlock %}
        it.n eq
    {% endif %}

    @ Unsuccessfully try to branch
    {% set target = jumpTargetRegister if jumpTargetRegister else jumpLabel %}
    {{instr}} {% if registerArgument %} {{registerArgument}}, {% endif %} {{target}}

    @ Get finish time
    ldr.w r3, [r0, {{CYCCNT}}]
    {% for _ in range(distance) %}
      adds.n r5, 1
    {% endfor %}

.thumb_func
{{jumpLabel}}:
    bl.w save

{% endfor %}
{% endfor %}
{% endfor %}
b.w end_label

.align 2
save:
    sub.w r2, r3, r2
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    bx.n lr
{% endblock %}
