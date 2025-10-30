---
name: smull/umull + branch combination tests
description: > 
    Timing test of multiple smull/umull + branch combinations one after another.
    Such configuration tests when decode phase is run, if multicycle instruction is executing.
    Results indicate that it is run as soon as possible.

    Moreover, check if xmull+mla behaves like a pipelined instruction or just a single-cycle mla.
dumped_symbols:
  times: 400 words
  cpicnts: 400 B
configurations:
# smull takes 3 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
# smull takes 4 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "smull.w" }
# smull takes 5 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "smull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "smull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "smull.w" }
# umull takes 3 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x00002DA0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
# umull takes 4 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x402DF0A0", "r7Value": "#0x00000480", "testedInstr": "umull.w" }
# umull takes 5 cycles
- { "code": "gpram", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "umull.w" }
- { "code": "sram", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: true, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "umull.w" }
- { "code": "flash", lbEn: false, "r6Value": "#0x98395B39", "r7Value": "#0x824704EA", "testedInstr": "umull.w" }
...
{% set branchInstrs = ["b.w 1f", "b.n 1f", "bx.n lr", "blx.n lr", "bl.w 1f", "mov pc, lr"] %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare input values
    ldr.w  r6, ={{r6Value}}
    ldr.w  r7, ={{r7Value}}

    @ Clean result registers
    mov.w  r8, #0
    mov.w  r9, #0

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
{% for add_mla in (True, False) %}
{% for bInstr in branchInstrs %}
{% for align in ('', 'add.w r3, r3', 'mov.n r3, r3', 'add.w r3, r3; mov.n r3, r3') %}
{% for x_cycles in ((0, 10) if code != "flash" else (0, 1, 4, 6, 7, 10))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    @ Align and clear PIQ
    {% if 'lr' in bInstr %}adr.w lr, 1f+1{% endif %}
    {{ x_word_load }}
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r1, [r0, {{CPICNT}}]
    {{align}}
    {{ x_word_exec }}

    {{testedInstr}} r8, r9, r6, r7
    {% if add_mla %}
    mla.w r8, r6, r7, r9
    {% endif %}
    {{bInstr}}
    @ This padding with nops ensures that we jump to address, that wasn't prefetched.
    nop.w; nop.w; nop.w; nop.w
    1:

    @ Get finish time
    ldr.n  r3, [r0, {{CYCCNT}}]
    ldr.n  r4, [r0, {{CPICNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r2, r3, r2
    sub.w r1, r4, r1
    ands.w r1, r1, #0xFF  @ CPICNT is 8-bit wide
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("cpicnts", r1, r3, r4, "b")}}

    bx.n lr
{% endblock %}
