---
name: SDIV/UDIV/SMULL/UMULL/SMLAL/UMLAL/MLA/MLS and B interaction tests
description: "Timing test for x+b combination, x should give prefetch queue more time to fill."
dumped_symbols:
  results: 10 words
  times: 10 words
configurations:
# SDIV
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "sdiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }

# UDIV
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "udiv.w", inputValues: [["0x1", "0x0"], ["0x1", "0x1"], ["0x10", "0x1"], ["0x100", "0x1"], ["0x1000", "0x1"], ["0x10000", "0x1"], ["0x100000", "0x1"], ["0x1000000", "0x1"], ["0x10000000", "0x1"], ["0x80000000", "0x1"]] }

# SMULL
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "smull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }

# UMULL
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "umull.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }

# SMLAL
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "smlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0x98395B39", "0x824704EA"]] }

# UMLAL
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "umlal.w", inputValues: [["0x00002DA0", "0x00000480"], ["0x402DF0A0", "0x00000480"], ["0xA942F28C", "0x824704EA"]] }

# MLA
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "mla.w", inputValues: [["0xA942F28C", "0x824704EA"]] }

# MLS
- { code: "gpram", lbEn: true, bInstr: "b.n", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "gpram", lbEn: true, bInstr: "b.w", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.n", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "sram", lbEn: true, bInstr: "b.w", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.n", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: false, bInstr: "b.w", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.n", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
- { code: "flash", lbEn: true, bInstr: "b.w", testedInstr: "mls.w", inputValues: [["0xA942F28C", "0x824704EA"]] }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}

{% set hasTwoRegisterResult = testedInstr != "sdiv.w" and testedInstr != "udiv.w" %}
{% extends "asm.s.tpl" %}
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
{% for (valA, valB) in inputValues %}
    @ Initialize input values
    ldr.w r6, ={{valA}}
    ldr.w r7, ={{valB}}

    @ Prepare add arguments
    mov.w  r5, #42

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    @ ldr gives prefetch queue time to fill
    {{testedInstr}} r3, {% if hasTwoRegisterResult %}r4,{% endif %} r6, r7
    @ Execute branch
    {{bInstr}} jump_target_{{loop.index}}
    @ These `add`s shouldn't execute
    add.w  r5, 1
    add.w  r5, 1
    add.w  r5, 1
    add.w  r5, 1

.align 4
jump_target_{{loop.index}}:
    @ Get finish time
    ldr.w  r3, [r0, {{CYCCNT}}]
    bl.w save

{% endfor %}
    b.w end_label

.align 4
save:
    subs.n r2, r3, r2
    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r5, r3, r4)}}
    bx.n lr

{% endblock %}
