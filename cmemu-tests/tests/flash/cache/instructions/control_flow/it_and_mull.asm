---
name: it + smull/umull + branch combination tests
description: >
  Timing test of multiple it + smull/umull + branch combinations one after another.
  Such configuration tests when decode phase is run, if multicycle instruction is executing in it block.
dumped_symbols:
  times: 100 words
configurations:
- { code: "sram", lbEn: True, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "sram", lbEn: True, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w", cache_enabled: True }
- { code: "flash", lbEn: True, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
- { code: "flash", lbEn: False, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n", cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% set repetitions = 7 %}
@ Register values that take 3, 4, 5 cycles to multiply.
{% set registerValues = [["#0x00002DA0", "#0x00000480"], ["#0x402DF0A0", "#0x00000482"], ["#0x98395B39", "#0x824704EA"]] %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

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
{% for (r6Value, r7Value) in registerValues %}
{% set registersIndex = loop.index %}
    @ Prepare input values
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Clean result registers
    mov.w  r8, #0
    mov.w  r9, #0

{% for setFlags in [true, false] %}
{% for reps in range(repetitions) %}
    @ Sets flags, but without V, so bInstr shoudn't rely on it.
    movs.n r4, #0
    
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {% if setFlags %}
            @ This instruction is inserted to check unaligned pipeline and register dependency.
            @ Sets flags, but without V, so bInstr shoudn't rely on it.
            movs.n r4, #0
        {% endif %}
        {{itInstr}}
        {{testedInstrs}}
        {{bInstr}} jump_target_{{registersIndex}}_{{setFlags}}_{{reps}}_{{i}}
        @ This padding with nops ensures that we jump to address, that wasn't prefetched.
        nop.w; nop.w; nop.w; nop.w
    jump_target_{{registersIndex}}_{{setFlags}}_{{reps}}_{{i}}:
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    subs.n r2, r3, r2
    {{saveTime(r2, r3, r4)}}

    bx.n lr
{% endblock %}
