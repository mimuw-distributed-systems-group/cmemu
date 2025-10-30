---
name: Test when there is an AGU stall after dependency.
description: >
    The previous test it_skipped_lsu_fetch were supposed to test weird prefetcher behavior, but it turned out
    to be just an issue with AGU forwarding.
    This seems to be a complex issue when the LSU instr is skipped, because some instruction will generate
    a stall, while others won't.
    One idea is that it depends on the forwarding (critical path) or critical path for flags.
    This may also vary with encoding/order of arguments.

    Conclusion: the stall is introduced only when there is a register dependency and
    a) the prior instruction updates flags, b) the condition succeeds.
    That is ``*s.w`` for wide instructions, but there are narrow instrs that can update the flags conditionally on
    being in the IT block. Apparently, they introduce a stall just like the flags would be actually set.
    Moreover, SAT instructions, which update the APSR.Q, are not stalling.
    This is probably because Q cannot be used by a condition.

    Note: add.n r5, r8 is T2 and never updates the flags. add(s).n r5, r4 is T1 and the **s** is not encoded.
    Note: there is a forwarding path to AGU from LSU (with requirements on input timing in TRM), but not from ALU,
          which may lay on the critical path.
dumped_symbols:
  times: auto
  cyc_in_it: auto
  flags: auto
  foldcnts: 1000 B
  cpicnts: 1000 B
  lsucnts: 1000 B
configurations: []
product:
    - code: [flash ]
      instructions:
        - alu
        - other
      lbEn: [False, True]
      first_width:
        - w
        - n
      second_width:
        - w
        - n
      base_reg: [r6, r5, sp]
      it:
        - pl
        - cc
      prev_before_it:
        - True
        - False
...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - reset alias to r0
@ r3 - final counter

@ r4 - always 0 (used in saving)
@ r5 - memory cell
@ r6,
@ r7 - ~0 (all ones: -1)
@ r8 - always 0 in hi register (not modified)
@ r9 - staller's 2
@ r10 - first staller
@ r11 - second staller ( (used in saving))
@ r12 - after-test func (unchanged!)
@ r13 - used in test
@ lr - navigation

{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}
{% extends "asm.s.tpl" %}

{% macro itify(repl) %}
{% if repl.strip() == "" %}
{{ caller() }}
{% else %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}

@ pl is N == 0 (not-negative) -- easy to keep with ALU instrs
@ cc is C == 0
{% set cond, notcond = (it, 'mi' if it == 'pl' else 'cs') %}


{% set save_func_reg = "r12" %}

{% block code %}
    @ Prepare cycle counter timer address
@    ldr.w r5, =memory_cell
@    ldr.w sp, =memory_cell

    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (FOLDCNT, "save_foldcnt"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"),] %}
        ldr.w r0, dwt
        add.w r0, {{counter}}
        ldr.w r5, =memory_cell
        ldr.w sp, =memory_cell
        mov.w r8, 0

        ldr.w {{save_func_reg}}, ={{save_func}}

        bl.w tested_code
    {% endfor %}

.thumb_func
end_label:
{% endblock %}

{% if instructions == "alu" %}
@ A List of interesting kinds of instructions with various encodings
{% set allowed_in_it = [
    "add.w r5, r5, #0x2000000 @ T3 - expanded imm",
    "adds.w r5, r5, #0x2000000 @ T3 - expanded imm",
    "add.w r5, r5, #0x2af @ T4 - 12 bits",
    "add.n r5, r8 @ T2",
    "add.w r5, r8 @ T3",
    "adds.w r5, r8 @ T3",

    "add.n r5, sp, 4 @ T1 of ADD (SP+imm)",
    "add.n sp, sp, 4 @ T2",
    "adds.w r5, sp, 4 @ T3",
    "adds.w sp, sp, 4 @ T3",
    "cmn.w sp, 4 @ variant of T3",
    "add.w r5, sp, #0x2000000 @ T3 - expanded imm",
    "add.w sp, sp, #0x2000000 @ T3 - expanded imm",
    "add.w r5, sp, #0x2af @ T4 - 12 bits",
    "add.w sp, sp, #0x2af @ T4 - 12 bits",

    "add.n r7, sp, r7 @ T1 of ADD (SP+reg) HOPEFULLY",
    "add.n sp, r7 @ T2",
    "add.w r5, sp, r4, LSL #1 @ T3",
    "add.w sp, sp, r4, LSL #1 @ T3",
    "adds.w r5, sp, r4, LSL #1 @ T3",
    "adds.w sp, sp, r4, LSL #1 @ T3",
    "cmn.w sp, r4, LSL #1 @ variant of T3",

    "add.n r5, pc, 4 @ adr T1",
    "sub.w r5, pc, #0 @ adr T2",
    "add.w r5, pc, 4 @ adr T3",

    "bfc.w r5, #1, #1 @ T1",
    "bic.w r5, r5, r4 @ T2",
    "bics.w r5, r5, r4 @ T2",

    "cmp.w r4, #-1 @ T2",
    "cmp.w sp, #-1 @ T2",
    "cmp.n r5, r7 @ T1",
    "cmp.n sp, r7 @ T2",
    "cmp.w r5, r7 @ T3",
    "cmp.w sp, r7 @ T3",

    "mov.w r5, #0x20000000 @ T2 - expanded imm",
    "movs.w r5,#0x20000000 @ T2 - expanded imm",
    "mov.w r5, #0x2af @ T3 - 12 bits",
    "movt.w r5, #0x2000",
    "mov.n r5, r8 @ T1",
    "mov.n sp, r5 @ T1",
    "mov.n r5, sp @ T1",
    "mov.w r5, r8 @ T3",
    "movs.w r5, r8 @ T3",
    "mov.w sp, r5 @ T3",
    "mov.w r5, sp @ T3",

    "mul.w r5, r0, r4 @ T1",

    "rev.n r5, r5",
    "rev.w r5, r5",

    "ssat.w r5, #27, r5 @ T1",
    "usat.w r5, #27, r5 @ T1",
]
%}
@ Note: in MOV.W cannot set flags if using SP! (unpredictable!)

{% if it == 'pl' %}
@ Ok, I don't know how to use this without changing C
{% do allowed_in_it.extend([
    "cmp.n r4, #0 @ T1",
    "cmp.n r5, sp @ T2",
])
%}
{% endif %}

{% set only_in_it = [
    "adc.n r5, r4 @ T1",
    "add.n r5, r5, #16 @ T2",
    "add.n r5, r4, #4 @ T1",
    "add.n r5, r0, r4 @ T1",
    "bic.n r5, r4 @ T1",
    "mov.n r5, #16 @ T1",
    "mvn.n r5, r7 @ T1",
    "mul.n r5, r4 @ T1",
] %}
{% set only_outside_it = [
    "adcs.n r5, r4 @ T1",
    "adds.n r5, r5, #16 @ T2",
    "adds.n r5, r4, #4 @ T1",
    "adds.n r5, r6, r4 @ T1",
    "bics.n r5, r4 @ T1",
    "movs.n r5, #16 @ T1",
    "movs.n r5, r4 @ T2",
    "mvns.n r5, r7 @ T1",
    "muls.n r5, r4 @ T1",
] %}

{% elif instructions == "other" %}
{% set allowed_in_it = [
    "mla.w r5, r1, r4, r5 @ T1",
    "mls.w r5, r1, r4, r5 @ T1",
    "umull.w r5, r3, r4, r5 @ T1",
    "umull.w r3, r5, r4, r5 @ T1",
    "umlal.w r5, r3, r4, r5 @ T1",
    "umlal.w r3, r5, r4, r5 @ T1",
    "sdiv.w r5, r4, r5 @ T1",

    "nop.n",
    "nop.w",

    "mrs.w r5, APSR @ T1",
    "msr.w APSR_nzcvq, r4 @ T1, all not set",

    "mrs.w r5, PSP @ T1",
    "msr.w PSP, r4 @ T1",

    "ldr.n r5, [r1] @ T1",
    "ldr.n r5, [sp] @ T2",
    "ldr.w r5, [r1] @ T3",
    "ldr.w r5, [sp] @ T3",
    "ldr.w sp, [r1] @ T3",
    "ldr.w r5, [sp, -2] @ T4",
    "ldr.w r6, [r5, -2] @ T4",
    "ldr.w sp, [r5, 2] @ T4",
    "ldr.w r5, [sp, -2]! @ T4",
    "ldr.w r6, [r5, -2]! @ T4",
    "ldr.w sp, [r5, 2]! @ T4",
    "ldr.w sp, [r5], 2 @ T4",

    "ldr.n r5, [pc, #0] @ T1",
    "ldr.n r5, [r1, r4] @ T1",
    "ldr.w r5, [r1, r4] @ T2",
    "ldr.w r5, [sp, r4] @ T2",
    "ldr.w sp, [r1, r4] @ T2",

    "ldm.n r5!, {{r6, r7}} @ T1",
    "ldm.n r5, {{r5, r6}} @ T1",
    "pop.n {{r1, r5}} @ T1",
    "pop.w {{r1, r5}} @ T2",
    "pop.w {{r5}} @ T3",
] %}
@ TODO: STR
@    "mrs.w r5, MSP @ T1",
@    "msr.w MSP, r4 @ T1",
{% set only_in_it = [
] %}
{% set only_outside_it = [
    "cbnz.n r4, {skip_label}",
] %}
@ More: CPS

{% endif %}
@ TODO: test if instr is known to not change the given flag (e.g., SSAT only C)

{% set instr_list = allowed_in_it + (only_in_it if not prev_before_it else only_outside_it) %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
   ldr.w r11, =test_lr
   str.w lr, [r11]

{% for prev_instr in instr_list %}
{% set skipped_offset = '[{reg}, 0]' %}
{% for x_cycles1 in (0, 1) %}

  {% set x_loader1, x_word_exec1 = n_x_cycles(x_cycles1, "r10", "r9") %}
{% for b_instr in ('b.n .+2', 'b.w .+4') %}
{% set b_in_it, bcond = it, notcond %}
{% for x_cycles2 in ((5, 6, 7) if not lbEn else ((1, 2, 3) if x_cycles1 == 0 else (3, 4, 5))) %}

  {% set x_loader2, x_word_exec2 = n_x_cycles(x_cycles2, "r11", "r9", load_2=False) %}
  {% set jump_label = uniq_label("bx_n") %}
  {{ x_loader1 }}
  {{ x_loader2 }}
    movs.n r4, #0
    mvns.n r7, r4
    mov.n r6, r4
    ldr.w r1, =memory_cell
    mov.n r5, {{ r1 if '[r5' in prev_instr or 'ldm' in prev_instr else r4 }} @ r0 for loads
    mov.n sp, {{ r1 if '[sp' in prev_instr or 'pop' in prev_instr else r4 }} @ r0 for loads

  @ Set flags and use as offset
  msr.w APSR_nzcvq, r4 @ clear all flags

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r2, [r0]
  {{ x_word_exec1 }}
  {{ x_word_exec2 }}

  @ The conditions are that "prev" is always executed, next is always skipped, ldr [r0] is executed and b is configurable

  {% if not prev_before_it %}itet{{ {cond: 't', notcond: 'e'}[bcond] if b_in_it else ''}}.n {{cond}}{% endif %}
  {% call itify('' if prev_before_it else cond) %}{{prev_instr.format(skip_label=jump_label)}}{% endcall %}
  {% if prev_before_it %}ite{{ {notcond: 't', cond: 'e'}[bcond] if b_in_it else ''}}.n {{notcond}}{% endif %}
  ldr{{notcond}}.{{first_width}} r6, {{skipped_offset.format(reg=base_reg)}}
  ldr{{cond}}.{{second_width}} r6, [r0]
  {% call itify(bcond) %}{{b_instr}}{% endcall %}


  @ either inplace or jump to jump_label
{{ jump_label }}:
  @ Get finish time
  ldr.n  r3, [r0]

  blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{guarded_ltorg()}}
{% endfor %}
{% endfor %}

  ldr.w r11, =test_lr
  ldr.w pc, [r11]

.thumb_func
save_times_and_flags:
  mrs.w r1, apsr
  sub.w r11, r3, r2
  sub.w r2, r6, r2
  {{saveValue("times", r11, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
  {{saveValue("cyc_in_it", r2, r3, r4)}}
  bx.n lr

.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4, 'b')}}
    bx.n lr


.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4, 'b')}}
    bx.n lr

.thumb_func
save_foldcnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("foldcnts", r2, r3, r4, 'b')}}
    bx.n lr

{{ section('flash') }}
.align 3
flash_aligned: .word .
flash_aligned4: .word 345
flash_aligned8: .word 907

{{ section('sram') }}
test_lr: .word 0
.align 3
memory_cell:
sram_aligned: .word .
sram_aligned4: .word 345
{{ section('gpram') }}
.align 3
gpram_aligned: .word .
gpram_aligned4: .word 345
{% endblock %}
