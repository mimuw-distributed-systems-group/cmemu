---
name: Measurements of timings of branch instructions using memory depending on jump distance.
description:
    We want to prove that the timing does not depend whether the branch target
    is far from the branch instruction or near from it.
    It is used to refute optimizations of branches to a following instruction
    or to an instruction that is already present in the PIQ.

    Only LDR, LDM, TBB, TBH, ADD, POP are tested in this file.

    'memory' in test's name comes from the fact that
    four out of seven tested instructions write and read from memory
    (and another two use the stack).
dumped_symbols:
  # 7 instructions * 2 alignments * 8 distances
  results: 112 words
  times: 112 words
configurations:
- { code: "gpram", memory: "gpram", lbEn: true }
- { code: "gpram", memory: "sram", lbEn: true }
- { code: "sram", memory: "gpram", lbEn: true }
- { code: "sram", memory: "sram", lbEn: true }
- { code: "flash", memory: "gpram", lbEn: false }
- { code: "flash", memory: "sram", lbEn: false }
- { code: "flash", memory: "gpram", lbEn: true }
- { code: "flash", memory: "sram", lbEn: true }

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set memoryCell = 'memory_cell' %}

@ "format string" == string which is supplied with labels by "%" operator
@ [(
@   format string of jump instruction,
@   format string of initialization instructions,
@ )]
{% set testParameters = [
    ("add.n pc, r8", "mov r8, #(%(jmp)s - %(pre)s); sub.w r8, #4"),
    ("pop.n {pc}", "adr.w r8, %(jmp)s+1; push.w {r8}"),
    ("pop.w {pc}", "adr.w r8, %(jmp)s+1; push.w {r8}"),
    ("ldr.w pc, [r9];", "adr.w r8, %(jmp)s+1; ldr.w r9, =%(mem)s; str.w r8, [r9]"),
    ("ldm.w r9, {pc}", "adr.w r8, %(jmp)s+1; ldr.w r9, =%(mem)s; str.w r8, [r9]"),
    ("tbb.w [r8, r9]", "ldr.w r8, =%(mem)s; mov.w r9, #0; mov.w r6, #(%(jmp)s - %(pre)s); sub.w r6, #4; asr r6, #1; str.w r6, [r8]"),
    ("tbh.w [r8, r9]", "ldr.w r8, =%(mem)s; mov.w r9, #0; mov.w r6, #(%(jmp)s - %(pre)s); sub.w r6, #4; asr r6, #1; str.w r6, [r8]"),
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

{% for jumpInstrFstring, preInstrFstring in testParameters %}
{% for wordAligned in [True, False] %}
{% for distance in range(0, 8) %}
    {% set jumpLabel = 'jump_target_%s_%d_%d' % (jumpInstrFstring.split()[0], distance, wordAligned) %}
    {% set preJumpLabel = 'pre_%s' % (jumpLabel) %}
    {% set env = { 'jmp': jumpLabel, 'pre': preJumpLabel, 'mem': memoryCell } %}

    {{ preInstrFstring % env }}

    @ Set flags
    movs.n r7, #0

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
    {{preJumpLabel}}:
    {{ jumpInstrFstring % env }}

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

{{ section(memory) }}
.align 2
{{memoryCell}}:
    .word 0x0

{% endblock %}
