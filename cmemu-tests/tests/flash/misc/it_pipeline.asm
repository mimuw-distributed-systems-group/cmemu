---
name: LSU pipelining inside IT block
description: >-
    Checks LSU pipelining inside IT block. It appears to be different a bit.
    This test is explicitly not testing the effects on Fetch.
dumped_symbols:
  times: auto
  lsucnts: auto
configurations: []
product:
    - code:
      - flash
#      - gpram
#      - sram
      memory: [gpram, sram]
      lbEn: [True, False]
#      base_regs:
#      - [r1, r1]
#      - [r5, r5]
#      - [r5, sp]
#      - [sp, sp]
#      - [sp, r1]
      base_reg2:
      - r1
      - r5
      - sp
      base_reg1:
      - r1
      - r5
      - sp
      set2:
      - A
      - B
      set1:
      - A
      # - B # This needs the magic 0x2000_2000 address
      offset:
      - [2, r9] # Both 2 ; with (B, B) there are no new cases
      - [4, r4] # r4 = 0, but aligned...
      cond:
      - eq
      - ''

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@{% set base_reg1, base_reg2 = base_regs %}

@ Register assignment:
@ r0, r2, - Counter
@ r1 - possible base offset
@ r3 - x_staller, then trash, then final counter
@ r4 - always 0 (used in saving)
@ r5 - possible base offset
@ r6 - alternative trash
@ r7,
@ r8 -
@ r9 - staller's 2
@ r10 - holds address of address with "1f+1" (case end; ldr rx, ==label)
@ r11 - auxiliary (used during navigation between counters loop and tested code)
@ r12 - after-test func (unchanged!)
@ r13 (sp) - possible base offset
@ lr - navigation, may be used in test

{% set save_func_reg = "r12" %}

{% block code %}
    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (LSUCNT, "save_lsucnt"),] %}
        ldr.w r0, dwt
        add.w r0, {{counter}}

        @ Prepare memory cell address
        ldr.w  r5, =memory_cell
        ldr.w  sp, =memory_cell

        ldr.w {{save_func_reg}}, ={{save_func}}

        bl.w tested_code
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

@ The Flag setting instr can only change flags as the second instr!
{% set instruction_set_A = [
    "add.w {base_reg}, r4",
    "ldr.w r5, [{base_reg}, {off_imm}]",
    "ldr.w sp, [{base_reg}, {off_imm}]",
    "ldr.w r5, [{base_reg}, {off_reg}]",
    "ldr.w sp, [{base_reg}, {off_reg}]",
    "ldr.w sp, 2f @ in next ltorg",

    "str.w r5, [{base_reg}, {off_imm}]",
    "str.w sp, [{base_reg}, {off_imm}]",
    "str.w r5, [{base_reg}, {off_reg}]",
    "str.w sp, [{base_reg}, {off_reg}]",
] %}
{% if offset[0] % 4 == 0 or set2 != "B" or base_reg1 != base_reg2 %}
@ LDM cannot have unaligned base
@ XXX: now there is no str wb, unaligned; add.w! with reg dep
{% do instruction_set_A.extend([
    "ldr.w {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "ldr.w {sp_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "str.w {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "str.w {sp_or_non_colliding}, [{base_reg}, {off_imm}]!",
]) %}
{% endif %}

{% set instruction_set_B = [
    "nop.w",
    "add.w {base_reg}, r4",
    "cmp.w {base_reg}, r5 @ T2",
    "umlal.w r1, r3, r4, r5 @ T1",
    "umlal.w r3, r1, r4, r5 @ T1",

    "ldrd.w r5, r6, 2f @ literal next ltorg",
    "ldm.w {base_reg}, {{r3, r5}}",
    "ldm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",

    "stm.w {base_reg}, {{r3, r5}}",
    "stm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",
] %}
{% if offset[0] % 4 == 0 %}
{% do instruction_set_B.extend([
    "ldrd.w r3, r5, [{base_reg}, {off_imm}]",
    "ldrd.w r3, {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "strd.w r3, r5, [{base_reg}, {off_imm}]",
    "strd.w r3, {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
]) %}
{% endif %}
{% set format_args1 = dict(base_reg=base_reg1, r5_or_non_colliding=(r5 if base_reg1 != r5 else r6), sp_or_non_colliding=(sp if base_reg1 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}
{% set format_args2 = dict(base_reg=base_reg2, r5_or_non_colliding=(r5 if base_reg2 != r5 else r6), sp_or_non_colliding=(sp if base_reg2 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}

{% set only_last = [
    "ldr.w pc, =1f+1",
    "ldr.w pc, [r10, r4]",
    "ldr.w pc, [r10, 4]! @ T4, only aligned",
    "ldm.w r10, {{r5, pc}}",
    "b.w .+4",
    "tbb.w [pc, r4]; .byte 1; .byte 0x24; @ encodes LSL(S) r4, r4, 4 if skipped",
] %}

{% set notcond = {'eq': 'ne', 'ne': 'eq', '': ''}[cond] %}
{% macro itify(repl) %}
{% if repl.strip() == "" %}
{{ caller() }}
{% else %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}

{% endmacro %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
   ldr.w r11, =test_lr
   str.w lr, [r11]
   mov.w r9, #2 @ For n-X-cycles

{% for first, second in itertools.product(instruction_set_A if set1 == 'A' else instruction_set_B, (instruction_set_A if set2 == 'A' else instruction_set_B) + only_last) %}
{% for cond_1, cond_2 in itertools.product('t', 'te') %}
{% for x_cycles2 in ( (range(11, 12) if not lbEn else range(8, 9)) if code == 'flash' else range(5, 6)) %}
  {% set x2_loader, x2_word_exec = n_x_cycles(x_cycles2, "r3", "r9", load_2=False, compact=True, flags_scrambling=True) %}
{% for last_pad in ('', 'nop.n') %}

  {{ x2_loader }}
  ldr.w r5, =memory_cell
  ldr.w r10, =3f
  mov.n r1, r5
  mov.n sp, r5

  @ Set flags and use as offset
  movs.n r4, #0

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.n  r2, [r0]
  {% if cond %}
  it{{cond_1}}{{cond_2}}.n {{cond}}
  {% else %}
  nop.n
  {% endif %}
  {% call itify(cond) %}{{ x2_word_exec }}{% endcall %}

  {% call itify(cond if cond_1 == 't' else notcond) %}{{ first.format(**format_args1) }}{% endcall %}
  {{ assert_aligned() }} @ Needed for PC-relative loads
  {% call itify(cond if cond_2 == 't' else notcond) %}{{ second.format(**format_args2) }}{% endcall %}
  {{ last_pad }}

  @ Get finish time
1: @ see https://ftp.gnu.org/old-gnu/Manuals/gas-2.9.1/html_chapter/as_5.html#SEC48
  ldr.n  r3, [r0]

    @ Save the times and results
   blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
b.w 9f
.align 2
2: @ Literal addresses to cell
.word memory_cell
.word memory_cell
.word memory_cell
.word memory_cell
3:
.word 1b+1 @ address of previous
.word 1b+1 @ address of previous
.ltorg
9:
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  ldr.w r11, =test_lr
  ldr.w pc, [r11]

.align 4
.thumb_func
restore_order:
  ldr.w r5, =memory_self_ref
  @ Write "." to successive words
@  mov.w r1, r5
@  .rept 5
@  str.w r1, [r5], 4
@  add.w r1, 4
@  .endr
  @ Write "memory_cell" to successive words
  ldr.w r1, =memory_cell
  .rept 5
  str.w r1, [r5], 4
  .endr
  bx.n lr

.thumb_func
save_times_and_flags:
  mrs.w r1, apsr
  sub.w r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  b.w restore_order @ tail-call


.thumb_func
save_lsucnt:
  sub.w r2, r3, r2
  and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
  {{saveValue("lsucnts", r2, r3, r4)}}
  b.w restore_order @ tail-call


{{ section(memory) }}

.align 2
test_lr: .word 0
.space 0x40 @ As a safety zone + to fit possible stacking
.align 2
.equ memory_cell, memory_self_ref + {{offset[0]}} @ (Un)alignment (2 + 2 = ok)
.space {{offset[0]}}
.word .
memory_self_ref:
.rept 6
  .word memory_cell
.endr
.space 16 @ As a safety zone

{% endblock %}
