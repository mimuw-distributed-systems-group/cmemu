---
name: it + smull/umull [+ mla] + branch combination tests
description: > 
    Timing test of multiple it + smull/umull + branch combinations one after another.
    Such configuration tests when decode phase is run, if multicycle instruction is executing in it block.
    Moreover, test pipelining of (skipped) MLA.
dumped_symbols:
  times: 100 words
configurations:
# Scenario #0: both instructions execute
# smull
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
# umull 
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "bpl.n" }
# Scenario #1: multiplication doesn't execute
# smull
- { code: "gpram", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "gpram", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "sram",  lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "sram",  lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: false, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: false, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n smullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
# umull 
- { code: "gpram", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "gpram", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "sram",  lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "sram",  lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: false, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.w" }
- { code: "flash", lbEn: true,  itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
- { code: "flash", lbEn: false, itInstr: "itet.n pl", testedInstrs: "addpl.w r5, r5 \n umullmi.w r8, r9, r6, r7", bInstr: "bpl.n" }
# Scenario #2: branch is outside of it block
# smull
- { code: "gpram", lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "it.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7", bInstr: "b.n" }
# umull
- { code: "gpram", lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "it.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7", bInstr: "b.n" }
# Scenario #3: MLA pipelines and branch is outside of it block
# smull
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
# umull
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "itt.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlapl r11, r6, r7, r9", bInstr: "b.n" }
# Scenario #4: MLA is skipped and branch is outside of it block
# smull
- { code: "gpram", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "ite.n pl", testedInstrs: "smullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
# umull
- { code: "gpram", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "gpram", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "sram",  lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "sram",  lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: false, itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.w" }
- { code: "flash", lbEn: true,  itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
- { code: "flash", lbEn: false, itInstr: "ite.n pl", testedInstrs: "umullpl.w r8, r9, r6, r7 \n mlami r11, r6, r7, r9", bInstr: "b.n" }
...
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
        {% set jump_target = uniq_label('jump_target') %}
        {% if setFlags %}
            @ This instruction is inserted to check unaligned pipeline and register dependency.
            @ Sets flags, but without V, so bInstr shoudn't rely on it.
            movs.n r4, #0
        {% endif %}
        {{itInstr}}
        {{testedInstrs}}
        {{bInstr}} {{jump_target}}
        @ This padding with nops ensures that we jump to address, that wasn't prefetched.
        nop.w; nop.w; nop.w; nop.w
    {{jump_target}}:
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
