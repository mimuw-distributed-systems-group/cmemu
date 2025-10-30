---
name: LDM RX {... PC}
description: Timings of various LSU branches
dumped_symbols:
  results: auto
  times: auto
  lsucnts: auto B
  cpicnts: auto B
configurations:
# Normal base reg
- { code: "gpram", lbEn: True,  memory: "gpram", base_reg: r5 }
- { code: "gpram", lbEn: True,  memory: "sram" , base_reg: r5 }
- { code: "gpram", lbEn: True,  memory: "flash", base_reg: r5 }
- { code: "gpram", lbEn: False, memory: "flash", base_reg: r5 }
- { code: "sram",  lbEn: True,  memory: "gpram", base_reg: r5 }
- { code: "sram",  lbEn: True,  memory: "sram" , base_reg: r5 }
- { code: "sram",  lbEn: True,  memory: "flash", base_reg: r5 }
- { code: "sram",  lbEn: False, memory: "flash", base_reg: r5 }
- { code: "flash", lbEn: True,  memory: "gpram", base_reg: r5 }
- { code: "flash", lbEn: False, memory: "gpram", base_reg: r5 }
- { code: "flash", lbEn: True,  memory: "sram" , base_reg: r5 }
- { code: "flash", lbEn: False, memory: "sram" , base_reg: r5 }
- { code: "flash", lbEn: True,  memory: "flash", base_reg: r5 }
- { code: "flash", lbEn: False, memory: "flash", base_reg: r5 }
# SP as base reg
- { code: "gpram", lbEn: True,  memory: "gpram", base_reg: sp }
- { code: "gpram", lbEn: True,  memory: "sram" , base_reg: sp }
- { code: "gpram", lbEn: True,  memory: "flash", base_reg: sp }
- { code: "gpram", lbEn: False, memory: "flash", base_reg: sp }
- { code: "sram",  lbEn: True,  memory: "gpram", base_reg: sp }
- { code: "sram",  lbEn: True,  memory: "sram" , base_reg: sp }
- { code: "sram",  lbEn: True,  memory: "flash", base_reg: sp }
- { code: "sram",  lbEn: False, memory: "flash", base_reg: sp }
- { code: "flash", lbEn: True,  memory: "gpram", base_reg: sp }
- { code: "flash", lbEn: False, memory: "gpram", base_reg: sp }
- { code: "flash", lbEn: True,  memory: "sram" , base_reg: sp }
- { code: "flash", lbEn: False, memory: "sram" , base_reg: sp }
- { code: "flash", lbEn: True,  memory: "flash", base_reg: sp }
- { code: "flash", lbEn: False, memory: "flash", base_reg: sp }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r8, {{counter}}
    ldr.w r9, ={{save_func}}+1

    bl.w    tested_code
{% endfor %}
{% endblock %}

{% set register_sets = [
    [pc,],
    [r1, pc],
    [r1, r4, pc],
] %}
{% set jump_ops = [
    "ldm.w {base_reg}, {{{regs}}}",
    "ldm.w {base_reg}!, {{{regs}}}",
     "ldmdb.w {base_reg}, {{{regs}}}",
     "ldmdb.w {base_reg}!, {{{regs}}}",
] %}
{% if base_reg == sp %}
{% do jump_ops.append("pop.n {{{regs}}}") %}
{% endif %}

{% block after %}
{{ section(code) }}
.align 3
.thumb_func
.type tested_code, %function
tested_code:
    mov.w r12, lr

{% set ns = namespace() %}
{% set ns.targets = [] %}

{% for jump_op in jump_ops %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for regs in register_sets %}
{% for non_executed_op in ("b.n 1f",) %} @ "bx.n lr", "udf.n 42", "ldr.n r6, [r7]"
{% for nops in range(2 if code != "flash" else 5) %}
{% for x_cycles in ((0, 3) if code != "flash" else (0,3,6,9))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True, flags_scrambling=True) %}
    {% set jump_label = uniq_label("jump") %}
    {% set target_label = uniq_label("target") %}

    {{ x_word_load }}
    @ Prepare branch arguments
    {% do ns.targets.append((target_label, regs|length)) %}
    ldr.w {{base_reg}}, ={{target_label}}_veneer + 4 - 4*{{0 if "ldmdb" in jump_op else regs|length}}
    @ Clear flags
    movs.n r7, #1
    movs.n r6, #0

    @ Align and clear PIQ
    .align {{ 3 if code == "flash" else 2 }}
    isb.w
    {{ inc_auto_syms() }}

    @ Get start time
    ldr.w  r2, [r0, r8]

    {{align}}
    {{x_word_exec}}
{{jump_label}}:
    {{jump_op.format(base_reg=base_reg, regs=regs|join(", "))}}

    @ This won't execute
    {{non_executed_op}}
    add.n  r6, r7
    add.n  r6, r7
    1:
    udf.n 42

    .align {{ 3 if code == "flash" else 2 }}
    {% for i in range(nops) %}
        nop.n
    {% endfor %}
{{target_label}}:
    @ Get finish time
    ldr.w  r3, [r0, r8]
    blx r9

{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    bx r12


.align 2
save_time_flags_and_result:
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4, "b")}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4, "b")}}
    bx.n lr

{{ section(memory) }}
.align {{3 if memory == "flash" else 2 }}
.space 4*6
@ List just veneers to save memory!
{% for target, n_regs in ns.targets %}
{{target}}_veneer: .word {{target}}+1
{% endfor %}
{% endblock %}
