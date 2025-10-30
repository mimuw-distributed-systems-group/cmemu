---
name: Conditional register dependency
description: >-
    Tests whether effect of register dependency (delayed AGU phase) happens
    when an instruction is conditionally skipped.
dumped_symbols:
  times: auto
  foldcnts: auto
  lsucnts: auto
  cpicnts: auto
configurations:
- { code: "gpram", memory: "sram", lbEn: true }
- { code: "sram",  memory: "sram", lbEn: true }
- { code: "flash", memory: "sram", lbEn: false }
- { code: "flash", memory: "sram", lbEn: true }
- { code: "gpram", memory: "flash", lbEn: true }
- { code: "sram",  memory: "flash", lbEn: true }
- { code: "flash", memory: "flash", lbEn: false }
- { code: "flash", memory: "flash", lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ Register assignment
@ r4 - is kept at 0!
@ r5 - settings flags (const value 0 or 1)
@ r3 - scratch until reading counter after cycle
@ r0, r2, r10-r12 reserved for saving

{% set cond_neg = {"eq": "ne", "ne": "eq"} %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    ldr.w  r6, =memory_cell
    mov.w  r7, r6
    
    {% for counter, save_func in [(CYCCNT, "save_times"), (FOLDCNT, "save_foldcnts"), (LSUCNT, "save_lsucnts"), (CPICNT, "save_cpicnts")] %}
        mov.w r10, {{counter}}
        ldr.w r11, ={{save_func}}

        bl.w  tested_code_regular_regs
        bl.w  tested_code_pc_reg
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 3
.thumb_func
.type tested_code_regular_regs, %function
tested_code_regular_regs:
    @ Save where to return after test.
    mov.n r12, lr
    movs.n r4, #0

{% for cond in ["eq", "ne"] %}
{% for case_no in range(7) %}  {# Cases explained later. #}
{% for blocks in ["t", "tt", "te"] %}
{% for reg_dep in [False, True] %}
{% for initial_ldr in [False, True] %}
    @ Initial LDR checks that a pipelined skipped instructions doesn't mess with dirty flags of non-skipped.
    {% set ldr_cond = "" if blocks|length == 1 else (cond if blocks[1] == "t" else cond_neg[cond]) %}
    

    @ Set not-zero flag, exact value used lated in the test
    movs.n r5, #1

    .align 3
    isb.w

    @ Get start time
    ldr.w r2, [r0, r10]

    {% if initial_ldr %}
    @ This flips t <-> e
    it{{blocks.translate({101: 116, 116: 101})}}.n {{cond_neg[cond]}}
    ldr{{cond_neg[cond]}}.n r6, [r6] @ This doesn't change r6
    {% else %}
    i{{blocks}}.n {{cond}}
    {% endif %}

    {% if case_no == 0 %}
        @ add with register dependency
        add{{cond}}.n r6, #0
    {% elif case_no == 1 %}
        @ load with register dependency (checking for pipeline'ing)
        ldr{{cond}}.n r6, [r6]
    {% elif case_no == 2 %}
        @ flipping flags (for neq case)
        tst{{cond}}.n r6, r5
    {% elif case_no == 3 %}
        @ flipping flags (for eq case)
        tst{{cond}}.n r6, r7
    {% elif case_no == 4 %}
        @ Divide by one
@        udiv{{cond}}.w r6, r6,  r5
        @ 2 cycles, reading from 3 regs, writing to one
        @ multiply by 1 and add 0
        mla{{cond}}.w r6, r6, r5,  r4
    {% elif case_no == 5 %}
        @ Multiply by one (dep on low register)
        @ Note: the timing changes between memories (r6)
        umull{{cond}}.w r6, r8, r6,  r5
    {% elif case_no == 6 %}
        @ Zero by one, (but accumulate r6 in place
        umlal{{cond}}.w r5, r6, r4,  r5
    {% else %}
        panic!("invalid case")
    {% endif %}
    ldr{{ldr_cond}}.w r8, [{{"r6" if reg_dep else "r7"}}]

    @ Get finish time
    ldr.w r3, [r0, r10]

    blx.n r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    @ Return to counters loop.
    bx.n r12

.align 3
.thumb_func
.type tested_code_pc_reg, %function
tested_code_pc_reg:
    @ Save where to return after test.
    mov.n r12, lr
    movs.n r4, #0

@ Checking for dependency with PC
{% for does_branch in [False, True] %}
{% for case_no in range(4) %}  {# Cases explained later. #}
{% for reg_dep_pc in [False, True] %}
{% for ldr_after_jump in [False, True] %}
{% for initial_ldr in [False, True] %}
    {% set jump_label = uniq_label("pc_jump") %}
    {# Unaligned address for ldr is still unsupported, so jump_label can't be used. Moreover, aligned label also makes test simpler. #}
    {% set aligned_label = uniq_label("aligned_ldr") %}
    {# LDR <label> (a.k.a. `LDR (literal)`) depends on PC register which is written by branches (see code below). #}
    {# Otherwise, LDR may have a reg-dependency with the initial_ldr (r6 should remain dirty). #}
    {% set ldr_instr = "ldr.w r8, %s"|format(aligned_label if reg_dep_pc else "[r6]") %}
    {# Condition for B<CC> and IT instructions. #}
    {% set bcond = "eq" if does_branch else "ne" %}

    .align 3
{{aligned_label}}:  @ Needed aligned address close to LDR (literal).
    isb.w

    @ Prepare jump target in a register.
    ldr.w r9, ={{jump_label}}

    @ Set zero flag
    movs.w r5, #0

    @ Get start time
    ldr.w r2, [r0, r10]

    {% if initial_ldr %}
        ldr.w r6, [r6] @ Memory_cell points to itself.
    {% endif %}

    {% if case_no == 0 %}
        @ Conditional decode-time branch.
        b{{bcond}}.n {{jump_label}}
    {% elif case_no == 1 %}
        @ Unonditional decode-time branch, but in IT block, thus effectively conditional.
        it.n {{bcond}}
        b{{bcond}}.n {{jump_label}}
    {% elif case_no == 2 %}
        @ Conditional execute-time branch.
        {{"cbz.n" if does_branch else "cbnz.n"}} r5, {{jump_label}}
    {% elif case_no == 3 %}
        @ Unonditional execute-time branch, but in IT block, thus effectively conditional.
        it.n {{bcond}}
        mov{{bcond}}.n pc, r9  @ Note: MOV PC, <non-LR> is execute-time branch.
    {% else %}
        panic!("invalid case")
    {% endif %}

    {{ldr_instr if not ldr_after_jump}}
{{jump_label}}:
    {{ldr_instr if ldr_after_jump}}

    @ Get finish time
    ldr.w r3, [r0, r10]

    blx.n r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    @ Return to counters loop.
    bx.n r12

.thumb_func
save_times:
    sub.w r2, r3, r2
    {{saveValue('times', r2, r3, r5)}}
    bx.n lr

{% for counter in ["foldcnts", "lsucnts", "cpicnts"] %}
.thumb_func
save_{{counter}}:
    sub.w r2, r3, r2
    and.w r2, 0xFF  @ This counter is 8-bit.
    {{saveValue(counter, r2, r3, r5)}}
    bx.n lr
{% endfor %}

{{ section(memory) }}
.align 3
memory_cell: .word . @ self referential

{% endblock %}
