---
name: Timing of UMULL/SMULL and MLA pipelinig
description: >-
    Test checks register dependency between `rd_hi` from SMULL/UMULL and `ra` from MLA.
    The expected counter values should be smaller by one, compared to situation with no register dependency (see: `mull_div_dependencies.asm`).
    Different inputs values are tested for both SMULL/UMULL and MLA.
    We also prove the asymmetry with MLS.

    The test was updated to show a chain on MLA behaves in a similar fashion.
dumped_symbols:
  times: 2000 words
  cpicnts: 2000 B
  lsucnts: 2000 B
configurations:
- { xmullInstr: "smull.w", a_or_s: "a" }
- { xmullInstr: "umull.w", a_or_s: "a" }
- { xmullInstr: "smull.w", a_or_s: "a", a_or_s_chain: "s", }
- { xmullInstr: "umull.w", a_or_s: "a", a_or_s_chain: "s", }
- { xmullInstr: "smull.w", a_or_s: "s" }
- { xmullInstr: "umull.w", a_or_s: "s" }
- { xmullInstr: "smull.w", a_or_s: "s", a_or_s_chain: "a", }
- { xmullInstr: "umull.w", a_or_s: "s", a_or_s_chain: "a", }
# xMLAL versions
- { xmullInstr: "smlal.w", a_or_s: "a" }
- { xmullInstr: "smlal.w", a_or_s: "s" }
- { xmullInstr: "umlal.w", a_or_s: "a" }
# flash
- { code: flash, xmullInstr: "smull.w", a_or_s: "a" }
- { code: flash, xmullInstr: "umull.w", a_or_s: "a" }
- { code: flash, xmullInstr: "smull.w", a_or_s: "a", a_or_s_chain: "s", }
- { code: flash, xmullInstr: "umull.w", a_or_s: "a", a_or_s_chain: "s", }
- { code: flash, xmullInstr: "smull.w", a_or_s: "s" }
- { code: flash, xmullInstr: "umull.w", a_or_s: "s" }
- { code: flash, xmullInstr: "smull.w", a_or_s: "s", a_or_s_chain: "a", }
- { code: flash, xmullInstr: "umull.w", a_or_s: "s", a_or_s_chain: "a", }
# xMLAL versions
- { code: flash, xmullInstr: "smlal.w", a_or_s: "a" }
- { code: flash, xmullInstr: "smlal.w", a_or_s: "s" }
- { code: flash, xmullInstr: "umlal.w", a_or_s: "a" }
...
{% set code = code|default("gpram") %}
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

# Values enforcing different execution times of SMULL/UMULL instruction
{% set mullInputValues = [
    (0x00002DA0, 0x00000480),
    (0x98395B39, 0x824704EA),
    ] +
    ([] if code != "flash" else [
    ])
     %}
@ it won't fit
@    (0x402DF0A0, 0x00000480),
{% set mlaInputValues = [0x0, 0x00002DA0, 0xA942F28C] %}

@ Register assignment
@ r4 - staller
@ r5, r6, r9, r8 - timing params (preserve them!)
@ r7, r10, r11 - scratch (used for results)
@ r0-r3, r12 - reserved for counters

{% block code %}
    @ Prepare cycle counter timer address
    {% for counter, save_func in [(CYCCNT, "save_times"), (LSUCNT, "save_lsucnts"), (CPICNT, "save_cpicnts")] %}
    ldr.w  r0, dwt
    add.w  r0, {{counter}}
    ldr.w r3, ={{save_func}}

    bl.w  tested_code
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test.
    mov.n r12, lr
    bl.w reset_regs
{% for mull1Value, mull2Value in mullInputValues %}
    @ Set all in/out registers - these are preserved!
    {{put_efficiently(r5, mull1Value)}}
    {{put_efficiently(r6, mull2Value)}}
{% for mla1Value in mlaInputValues %}
    {{put_efficiently(r9, mla1Value)}}
{% for mla2Value in mlaInputValues %}
    {{put_efficiently(r8, mla2Value)}}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3')) %}
{% for x_cycles in ((6,) if code != "flash" else (0, 1, 6, 7, 10))  %}
{% for chain in range(5) %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r4", "r2", load_2=False, compact=True, flags_scrambling=True) %}
    {{ x_word_load }}
    @ Align and clear PIQ
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start counter value
    ldr.n  r1, [r0]
    {{align}}
    {{ x_word_exec }}

    {{xmullInstr}} r11, r7, r6, r5
    ml{{a_or_s}}.w r10, r8, r9, r7
    {% for i in range(chain) %}
    @ alternate result registers
    ml{{a_or_s_chain|default(a_or_s)}}.w r1{{(i+1)%2}}, r8, r9, r1{{i%2}}
    {% endfor %}

    @ Get finish counter value
    ldr.n  r2, [r0]
    {{ inc_auto_syms() }}

    @ Save counters.
    blx.n r3
{% endfor %}
    {{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    @ Return to counters loop.
    bx.n r12


.thumb_func
reset_regs:
    mov.w  r7, #0
    mov.w  r10, #0
    mov.w  r11, #0
    mov.w  r2, #2
    bx.n lr

.thumb_func
save_times:
    sub.w r2, r2, r1
    {{saveValue("times", r2, r10, r7)}}
    @ Tail call
    b.w reset_regs

.thumb_func
save_cpicnts:
    sub.w r2, r2, r1
    ands.w r2, r2, #0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r10, r7, "b")}}
    b.w reset_regs

.thumb_func
save_lsucnts:
    sub.w r2, r2, r1
    ands.w r2, r2, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r10, r7, "b")}}
    b.w reset_regs

{% endblock %}
