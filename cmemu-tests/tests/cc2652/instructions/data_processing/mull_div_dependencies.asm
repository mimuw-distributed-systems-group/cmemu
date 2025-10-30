---
name: Checks multiplication and division register dependency
description: "Test checks if there is timing change when the register dependency is introduced"
dumped_symbols:
  # 22 (bInputCombinations) * 3 (bInstructions) * 4 (aInstructions)
  times: 264 words
configurations:
- { code: "sram", aInstructionPart: 1, bInstructionPart: 1 }
- { code: "sram", aInstructionPart: 1, bInstructionPart: 2 }
- { code: "sram", aInstructionPart: 2, bInstructionPart: 1 }
- { code: "sram", aInstructionPart: 2, bInstructionPart: 2 }
- { code: "flash", aInstructionPart: 1, bInstructionPart: 1 }
- { code: "flash", aInstructionPart: 1, bInstructionPart: 2 }
- { code: "flash", aInstructionPart: 2, bInstructionPart: 1 }
- { code: "flash", aInstructionPart: 2, bInstructionPart: 2 }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% if aInstructionPart == 1 %}
    {% set aInstructions = [
        "smull.w r7, r6, r5, r4",
        "umull.w r7, r6, r5, r4",
        "smlal.w r7, r6, r5, r4",
        "umlal.w r7, r6, r5, r4",
    ] %}
{% elif aInstructionPart == 2 %}
    {% set aInstructions = [
        "mla.w r7, r6, r5, r4",
        "mls.w r7, r6, r5, r4",
        "sdiv.w r6, r5, r4",
        "udiv.w r6, r5, r4",
    ] %}
{% else %}
    panic("Unsupported aInstructionPart value!")
{% endif %}

@ Cover all cases where a given input argument depends on a earlier instruction's output
{% if bInstructionPart == 1 %}
    {% set bInstructions = [
        "smull.w r11, r10",
        "umull.w r11, r10",
        "smlal.w r11, r10",
        "umlal.w r11, r10",
        "sdiv.w r11",
        "udiv.w r11",
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
        "mla.w r11",
        "mla.w r6",
        "mls.w r11",
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

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 2
.thumb_func
.type tested_code, %function
tested_code:
{% for aInstr in aInstructions %}
{% for bInstr in bInstructions %}
{% for bInput in bInputCombinations %}
    bl.w zeroRegisters

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start time
    ldr.w  r1, [r0, {{CYCCNT}}]

    {{aInstr}}
    {{bInstr}}, {{bInput}}

    @ Get finish time
    ldr.w  r2, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r1, r2, r1
    {{saveValue('times', r1, r2, r3)}}
    bx.n lr

zeroRegisters:
    @ Cleanup all in/out registers
    movs.n r4, #0
    movs.n r5, #0
    movs.n r6, #0
    movs.n r7, #0
    mov.w  r8, #0
    mov.w  r9, #0
    mov.w  r10, #0
    mov.w  r11, #0
    bx.n lr

{% endblock %}
