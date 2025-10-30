---
name: Checks multiplication and division register dependency
description: |
    Test checks if there is timing change when the register dependency is introduced

    This is the "large tests" version of the original test.
dumped_symbols:
  times: auto
product:
-
  aInstr: [
        "smull.w r7, r6, r5, r4",
        "umull.w r7, r6, r5, r4",
        "smlal.w r7, r6, r5, r4",
        "umlal.w r7, r6, r5, r4",
        "mla.w r7, r6, r5, r4",
        "mls.w r7, r6, r5, r4",
        "sdiv.w r6, r5, r4",
        "udiv.w r6, r5, r4",
  ]
  bInstructionPart: [1, 2]
  b_out_reg: [r11, r7, r6, r4]
  regs_val: [0, 0xffff, 0xf0f00000, 0xffffffff]
  cInstr: [
        "mla.w {out}",
        "mls.w {out}",
        "smull.w {out}",
        "umull.w {out}",
        "smlal.w {out}",
        "umlal.w {out}",
  ]
  again: [a, b, c, '']
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ Cover all cases where a given input argument depends on a earlier instruction's output
{% if bInstructionPart == 1 %}
    {% set bInstructions = [
        "smull.w {out}, r10",
        "umull.w {out}, r10",
        "smlal.w {out}, r10",
        "umlal.w {out}, r10",
        "smull.w r10, {out}",
        "umull.w r10, {out}",
        "smlal.w r10, {out}",
        "umlal.w r10, {out}",
        "sdiv.w {out}",
        "udiv.w {out}",
    ] %}
    {% set bInputCombinations = [
        "r9, r8", "r7, r8",
        "r9, r7", "r6, r8",
        "r9, r6", "r6, r7",
        "r7, r6", "r6, r6",
        "r7, r7",
    ] %}
{% elif bInstructionPart == 2 %}
    {% set bInstructions = [
        "mla.w {out}",
        "mls.w {out}",
    ] %}
    {% set bInputCombinations = [
        "r10, r9, r8", "r7, r9, r8",
        "r10, r7, r8", "r10, r9, r7",
        "r6, r9, r8", "r10, r6, r8",
        "r10, r9, r6", "r6, r7, r8",
        "r7, r6, r8", "r6, r9, r7",
        "r7, r9, r6", "r10, r6, r7",
        "r10, r7, r6", "r6, r6, r8",
        "r6, r9, r6", "r10, r6, r6",
        "r6, r6, r6", "r7, r7, r8",
        "r7, r9, r7", "r10, r7, r7",
        "r7, r7, r7",
    ] %}
{% else %}
    panic("Unsupported bInstructionPart value!")
{% endif %}

{% set cInputCombinations = [
    "r10, r9, r8", "r7, r9, r8",
    "r10, r7, r8", "r10, r9, r7",
    "r6, r9, r8", "r10, r6, r8",
    "r10, r9, r6", "r6, r7, r8",
    "r7, r6, r8", "r6, r9, r7",
    "r7, r9, r6", "r10, r6, r7",
    "r10, r7, r6", "r6, r6, r8",
    "r6, r9, r6", "r10, r6, r6",
    "r6, r6, r6", "r7, r7, r8",
    "r7, r9, r7", "r10, r7, r7",
    "r7, r7, r7",
] %}

{% block code %}
    @ Prepare cycle counter timer address
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

@ Stallers are useful for Flash
{% set x_loader, x_word_exec = n_x_cycles(9, "r12", "r2") %}

{% block after %}
{{ section("flash") }}
.align 2
.thumb_func
.type tested_code, %function
tested_code:
    bl.w zeroRegisters
{% for bInstr in bInstructions %}
{% for bInput in bInputCombinations %}
{% for cInput in cInputCombinations if ('mlal' not in cInstr and 'mull' not in cInstr) or not cInput.startswith(b_out_reg) %}
    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start time
    ldr.w  r1, [r0, {{CYCCNT}}]
    {{ x_word_exec }}

    {{aInstr}}
    {{bInstr.format(out=b_out_reg)}}, {{bInput}}
    {{cInstr.format(out=b_out_reg)}}, {{cInput}}

    {% if again == "a" %}
    {{aInstr}}
    {% elif again == "b" %}
    {{bInstr.format(out=b_out_reg)}}, {{bInput}}
    {% elif again == "c" %}
    {{cInstr.format(out=b_out_reg)}}, {{cInput}}
    {% endif %}

    @ Get finish time
    ldr.w  r2, [r0, {{CYCCNT}}]

    {{ inc_auto_syms() }}
    bl.w save_and_zero
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_and_zero:
    sub.w r1, r2, r1
    {{saveValue('times', r1, r2, r0)}}

zeroRegisters:
    ldr.w  r0, ={{DWT_BASE}}

    @ Cleanup all in/out registers
    {% for reg in range(4, 11+1) %}
    ldr.w r{{reg}}, ={{regs_val}}
    {% endfor %}
    {{ x_loader }}
    bx.n lr

.ltorg

{% endblock %}
