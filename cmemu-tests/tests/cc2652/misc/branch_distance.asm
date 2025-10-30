---
name: Measurements of branch instructions timings depending on jump distance.
description:
    We want to prove that the timing does not depend whether the branch target
    is far from the branch instruction or near from it.
    It is used to refute optimizations of branches to a following instruction
    or to an instruction that is already present in the PIQ.

    Only B, B<cc>, BX, BL, BLX, CBZ, CBNZ, MOV.n are tested in this file.
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
@ )]
{% set testParameters = [
    ("b.n", None, None, 0),
    ("b.w", None, None, 0),
    ("beq.n", None, None, 0),
    ("beq.w", None, None, 0),
    ("bl.w", None, None, 0),
    ("bx.n", None, "r8", 0),
    ("bx.n", None, "lr", 0),
    ("blx.n", None, "r8", 0),
    ("blx.n", None, "lr", 0),
    ("cbz.n", "r7", None, 0),
    ("cbnz.n", "r7", None, 1),
    ("mov.n", "pc", "lr", 0),
    ("mov.n", "pc", "r8", 0),
] %}

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

{% for instr, registerArgument, jumpTargetRegister, flagImm in testParameters %}
{% for wordAligned in [True, False] %}
{% for distance in range(0, 8) %}
    {% set jumpLabel = 'jump_target_%s_%d_%s_%s' % (instr, distance, wordAligned, jumpTargetRegister) %}

    {% if jumpTargetRegister %}
        adr.w {{jumpTargetRegister}}, {{jumpLabel}} + 1
    {% endif %}

    @ Set flags and sometimes used r7 register
    movs.n r7, #{{flagImm}}

    @ Clear jump guard value
    mov.w r5, #0

    @ Align and clear PIQ
    .align 3
    {% if not wordAligned %}
        nop.n
    {% endif %}
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    @ Execute branch
    {% set target = jumpTargetRegister if jumpTargetRegister else jumpLabel %}
    {{instr}} {% if registerArgument %} {{registerArgument}}, {% endif %} {{target}}

    {% for _ in range(distance) %}
      adds.n r5, 1
    {% endfor %}

.thumb_func
{{jumpLabel}}:
    @ Get finish time
    ldr.w  r3, [r0, {{CYCCNT}}]

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
