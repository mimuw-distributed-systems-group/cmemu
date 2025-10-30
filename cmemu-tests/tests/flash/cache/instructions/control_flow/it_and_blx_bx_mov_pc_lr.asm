---
name: Speculative BLX/BX/MOV_PC_LR
description: Testing if decode time branches, that use lr as source are speculated if they are last in the it-block.
dumped_symbols:
  times: 20 words
  foldcnts: 20 words
  results: 20 words
configurations:
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "blxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "blxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "blxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n eq", bInstr: "blxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "blxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n ne", bInstr: "blxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "bxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "bxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "bxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n eq", bInstr: "bxeq.n lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "bxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n ne", bInstr: "bxne.n lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "moveq.n pc, lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "sram", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "movne.n pc, lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n eq", bInstr: "moveq.n pc, lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n eq", bInstr: "moveq.n pc, lr", branchDirections: ['forward', 'backward'], cache_enabled: True }
- { code: "flash", lbEn: True, preInstr: "movs.n r7, #0; it.n ne", bInstr: "movne.n pc, lr", branchDirections: ['forward'], cache_enabled: True }
- { code: "flash", lbEn: False, preInstr: "movs.n r7, #0; it.n ne", bInstr: "movne.n pc, lr", branchDirections: ['forward'], cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counters base address
    ldr.w  r0, dwt

    @ Zero the fold counter
    mov.w  r3, 0
    str.w  r3, [r0, {{FOLDCNT}}]

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
{% for branchDirection in branchDirections %}
{% set initIndex = loop.index %}
{% for reps in range(1, 11) %}
    @ Flush flash line buffer
    mov.w r8, #0
    ldr.w r8, [r8, r8]

    @ Get start folds
    ldr.w  r3, [r0, {{FOLDCNT}}]
    and.w  r3, #0xFF

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r0, {{CYCCNT}}]
    
    movs.n r7, #0
    {% if branchDirection == 'backward' %}
        b.w jump_target_{{branchDirection}}_{{initIndex}}_{{reps}}_{{reps-1}}
    {% endif %}

    {% for i in range(reps) %}
    .thumb_func
    jump_target_{{branchDirection}}_{{initIndex}}_{{reps}}_{{i}}:
        {% if branchDirection == 'forward' %}
            {% set index = i+1 %}
        {% elif branchDirection == 'backward' %}
            {% set index = i-1 if i > 0 else reps %}
        {% else %}
            panic!("Unrecognized branch direction")
        {% endif %}
        @ Get address that we jump to
        adr.w lr, jump_target_{{branchDirection}}_{{initIndex}}_{{reps}}_{{index}}
        orr.w lr, #1 @ Set lowest bit, since we jump to Thumb instruction

        {{preInstr}}
        {{bInstr}}

        @ Executes if not jumped, placed here to fill PIQ
        nop.w
        nop.w
        nop.w
        nop.w
    {% endfor %}
    .thumb_func
    jump_target_{{branchDirection}}_{{initIndex}}_{{reps}}_{{reps}}:

    @ Get finish time
    ldr.w  r5, [r0, {{CYCCNT}}]
    @ Get finish folds
    ldr.w  r6, [r0, {{FOLDCNT}}]
    @ Save the times and results
    bl.w save

{% endfor %}
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    @ Compute execution time
    subs.n r4, r5, r4
    @ Compute foldcnt difference
    and.w  r6, #0xFF
    subs.n r3, r6, r3

    {{saveTime(r4, r9, r10)}}
    {{saveValue("foldcnts", r3, r9, r10)}}
    {{saveResult(r8, r9, r10)}}
    bx.n lr

{% endblock %}
