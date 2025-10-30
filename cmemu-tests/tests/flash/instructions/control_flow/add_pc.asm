---
name: Add PC
description: Timings of add PC, rX
dumped_symbols:
  results: auto
  times: auto
  flags: auto
  lsucnts: auto
  cpicnts: auto
configurations:
- { code: "gpram", lbEn: true,  nopCountRange: 4, add_reg: r5, target_pad: '' }
- { code: "sram",  lbEn: true,  nopCountRange: 4, add_reg: r5, target_pad: '' }
- { code: "flash", lbEn: false, nopCountRange: 6, add_reg: r5, target_pad: '' }
- { code: "flash", lbEn: true,  nopCountRange: 6, add_reg: r5, target_pad: '' }
- { code: "gpram", lbEn: true,  nopCountRange: 4, add_reg: r5, target_pad: 'mov.n r3, r3' }
- { code: "sram",  lbEn: true,  nopCountRange: 4, add_reg: r5, target_pad: 'mov.n r3, r3' }
- { code: "flash", lbEn: false, nopCountRange: 6, add_reg: r5, target_pad: 'mov.n r3, r3' }
- { code: "flash", lbEn: true,  nopCountRange: 6, add_reg: r5, target_pad: 'mov.n r3, r3' }
# ARM-ARM A7.7.6 (add.n pc, sp is deprecated, but legal...)
- { code: "gpram", lbEn: true,  nopCountRange: 4, add_reg: sp, target_pad: '' }
- { code: "sram",  lbEn: true,  nopCountRange: 4, add_reg: sp, target_pad: '' }
- { code: "flash", lbEn: false, nopCountRange: 6, add_reg: sp, target_pad: '' }
- { code: "flash", lbEn: true,  nopCountRange: 6, add_reg: sp, target_pad: '' }
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

{% block after %}
{{ section(code) }}
.align 3
.thumb_func
.type tested_code, %function
tested_code:
    mov.w r12, lr

{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for pad2 in ('', 'mov.n r3, r3', 'movs.n r3, r3') +  (() if code != "flash" else ('mov.n r3, r3; mov.n r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for x_cycles in ((0, 3) if code != "flash" else range(10))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    @ NOTE: sp can only encode multiplies of 4
{% for nops in range(nopCountRange) if not (add_reg == sp and ((align ~ pad2)|select("eq", "n")|list|count + nops) % 2 != 0) %}
    {% set jump_label = uniq_label("jump") %}
    {% set target_label = uniq_label("target") %}

    {{ x_word_load }}
    @ Prepare branch arguments (-4 because it's PC relative, not .)
    .equ {{target_label}}_diff, ({{target_label}} - {{jump_label}} - 4)
    ldr.w {{add_reg}}, ={{target_label}}_diff
    @ Clear flags
    movs.n r7, #1
    movs.n r6, #0

    @ Align and clear PIQ
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r8]

    {{align}}
    {{x_word_exec}}
    {{pad2}}
{{jump_label}}:
    add.n pc, {{add_reg}}

    @ These `add`s shouldn't execute
    add.n  r6, r7
    add.n  r6, r7
    add.n  r6, r7

.align 3
    {% for i in range(nops) %}
        nop.n
    {% endfor %}
{{target_label}}:
    {{target_pad}}
    @ Get finish time
    ldr.w  r3, [r0, r8]
    {{ inc_auto_syms() }}
    blx r9

{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}

    bx r12


.align 2
save_time_flags_and_result:
    mrs.w r5, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4, "b")}}
    {{saveValue("flags", r5, r3, r4)}}

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
{% endblock %}
