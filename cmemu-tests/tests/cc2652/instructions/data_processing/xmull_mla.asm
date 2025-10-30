---
name: Timing of UMULL/SMULL and MLA pipelinig
description: >-
    Test checks register dependency between `rd_hi` from SMULL/UMULL and `ra` from MLA.
    The expected counter values should be smaller by one, compared to situation with no register dependency (see: `mull_div_dependencies.asm`).
    Different inputs values are tested for both SMULL/UMULL and MLA.
    We also prove the asymmetry with MLS.
dumped_symbols:
  times: 27 words
  cpicnts: 27 words
  lsucnts: 27 words
configurations:
- { code: "sram", xmullInstr: "smull.w", a_or_s: "a" }
- { code: "sram", xmullInstr: "umull.w", a_or_s: "a" }
- { code: "sram", xmullInstr: "smull.w", a_or_s: "s" }
- { code: "sram", xmullInstr: "umull.w", a_or_s: "s" }
- { code: "flash", xmullInstr: "smull.w", a_or_s: "a" }
- { code: "flash", xmullInstr: "umull.w", a_or_s: "a" }
- { code: "flash", xmullInstr: "smull.w", a_or_s: "s" }
- { code: "flash", xmullInstr: "umull.w", a_or_s: "s" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

# Values enforcing different execution times of SMULL/UMULL instruction
{% set mullInputValues = [("#0x00002DA0", "#0x00000480"), ("#0x402DF0A0", "#0x00000480"), ("#0x98395B39", "#0x824704EA")] %}
{% set mlaInputValues = ["#0x0", "#0x00002DA0", "#0xA942F28C"] %}

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
{% for counter, save_func in [(CYCCNT, "save_time"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for mull1Value, mull2Value in mullInputValues %}
{% for mla1Value in mlaInputValues %}
{% for mla2Value in mlaInputValues %}
    @ Set all in/out registers
    ldr.w  r5, ={{mull1Value}}
    ldr.w  r6, ={{mull2Value}}
    ldr.w  r9, ={{mla1Value}}
    ldr.w  r10, ={{mla2Value}}
    mov.w  r7, #0
    mov.w  r8, #0
    mov.w  r11, #0
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r1, [r0, {{counter}}]

    {{xmullInstr}} r8, r7, r6, r5
    ml{{a_or_s}}.w r11, r10, r9, r7

    @ Get finish counter value
    ldr.w  r2, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
    {{guarded_ltorg()}}
{% endfor %}
{% endfor %}
    b.w end_label

save_time:
    sub.w r2, r2, r1
    {{saveValue("times", r2, r3, r4)}}
    bx.n lr

save_cpicnt:
    sub.w r2, r2, r1
    ands.w r2, r2, #0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    sub.w r2, r2, r1
    ands.w r2, r2, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr

{% endblock %}
