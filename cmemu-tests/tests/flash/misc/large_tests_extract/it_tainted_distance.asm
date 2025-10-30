---
name: Test the distance of the tainted last narrow instruction from it.
description: >
    The tainted instruction is one 5 halfwords past it. But not for single item IT.
    Here we test the various distance and overlapping from the it.

    The tests has an overall structure like below, with IT changing place.

      D  X
      ^  ^
      Fd D  X  -- it
      ^           D  X
      Fa Fd Fd Fd ^  ^
      ^  ^  ^  ^     D  Xa Xd
         Fa Fa Fa Fd ^  ^  ^  /- tainted last narrow instruction
         ^  ^  ^  ^     D  Xs
                  Fa Fd Fd Fd ^  ^
                  ^  ^  ^  ^     D  Xs
                     Fa Fa Fa Fd ^  ^
                              Fa?Fd? <- where should here be speculative fetch?
dumped_symbols:
  times: auto
  lsucnts: auto
configurations: []
product:
    - code:
      - flash
      memory:
      - sram
      lbEn: [True,]
      final_half: # Which half of a word the instruction would occupy
      - 0
      - 1
      base_reg2:
      - sp
      - r1
      - r5
      base_reg1:
      - r1
      - sp
      symbol_offset:
      - 2
      - 0
      offset:
      - [2, r9] # Both 2 ; with (B, B) there are no new cases
      - [4, r4] # r4 = 0, but aligned...
      it1_len:
      - 0
      - 1
      it2_len:
      - 0
      - 1
      - 2
      - 3
      - 4
      extra:
      - ''
      - b.n .+2  #  bus conflict at decode time
      - ldr.w r3, [pc, 0]  # conflict at execute time
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set cond = 'eq' %}
{% set set1, set2 = 'A', 'B' %}

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
@ r10 -
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
    "ldr.w r5, [{base_reg}, {off_imm}]",
    "ldr.w sp, [{base_reg}, {off_imm}]",
    "ldr.w r5, [{base_reg}, {off_reg}]",
    "ldr.w sp, [{base_reg}, {off_reg}]",

    "str.w r5, [{base_reg}, {off_imm}]",
    "str.w sp, [{base_reg}, {off_imm}]",
    "str.w r5, [{base_reg}, {off_reg}]",
    "str.w sp, [{base_reg}, {off_reg}]",

    "ldr.w {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "ldr.w {sp_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "str.w {r5_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "str.w {sp_or_non_colliding}, [{base_reg}, {off_imm}]!",
    "str.w {sp_or_non_colliding}, [{base_reg}], {off_imm}",
] %}

{% set instruction_set_B = [
    "nop.n",
    "add.n {base_reg}, r4",

    "ldr.n r5, [{base_reg}, 4]",
    "str.n r5, [{base_reg}, 4]",

    "cmp.n {base_reg}, r5 @ can change flags",
    "b.n .+2",
] %}

{% if base_reg2 != sp %}
{% do instruction_set_B.extend([
    "ldr.n r5, [{base_reg}, r4]",
    "str.n r5, [{base_reg}, r4]",
]) %}

@{% if offset[0] % 4 == 0 or base_reg1 != base_reg2 %}
@@ LDM cannot have unaligned base
@{% do instruction_set_B.extend([
@    "ldm.n {base_reg}!, {{r3, {r5_or_non_colliding} }}",
@    "stm.n {base_reg}!, {{r3, {r5_or_non_colliding} }}",
@]) %}
@{% endif %}
{% endif %}

{% set format_args1 = dict(base_reg=base_reg1, r5_or_non_colliding=(r5 if base_reg1 != r5 else r6), sp_or_non_colliding=(sp if base_reg1 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}
{% set format_args2 = dict(base_reg=base_reg2, r5_or_non_colliding=(r5 if base_reg2 != r5 else r6), sp_or_non_colliding=(sp if base_reg2 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}

{% set notcond = {'eq': 'ne', 'ne': 'eq', '': ''}[cond] %}
{% macro itify(repl) %}
{% if not repl or repl.strip() == "" %}
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

{% set it2_pads = (it2_len - 2) if it2_len >= 2 else 0 %}
{% set it_pads_total = it1_len + it2_pads %}

{% for first, second in itertools.product(instruction_set_A if set1 == 'A' else instruction_set_B, (instruction_set_A if set2 == 'A' else instruction_set_B)) %}
{% for cond_2 in ('t', 'e', '') if (not cond_2) == (it2_len < 2) or (second.startswith('b') and it2_len < 2)  %}
{% for widths in itertools.product('wn', repeat=it_pads_total) %}
{% for last_pad in ('', ) %}

  ldr.w r5, =memory_cell
  mov.n r1, r5
  mov.n sp, r5

  @ Set flags and use as offset
  movs.n r4, #0

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r2, [r0]

@ Place padding so that the "second" instruction is in the tainted situation
{% set pad_len =  (2 + final_half - it_pads_total - widths.count('w') ) % 4 %}
.rept {{ pad_len // 2 }}
  add.w r3, r3
.endr
.rept {{ pad_len % 2 }}
  add.n r3, r3
.endr

  {% if it1_len %}i{{ 't' * it1_len }}.n {{cond}}{% else %}add.n r3, r3{% endif %}
  {% for i in range(it1_len) %}add{{cond}}.{{widths[i]}} r3, r3; {% endfor %}
  {% if it2_len %}it{{ 't' * it2_pads }}{{cond_2}}.n {{cond}}{% else %}add.n r3, r3{% endif %}
  {% for i in range(it2_pads) %}add{{cond}}.{{widths[it1_len + i]}} r3, r3; {% endfor %}
  {% call itify(it2_len and cond) %}{{ first.format(**format_args1) }}{% endcall %}
  {% call itify(cond_2|replace('e', 'ne')|replace('t', 'eq')) %}{{ second.format(s='s' if not cond_2 else '', **format_args2) }}{% endcall %}

  {% call itify('') %}{{ last_pad }}{% endcall %}
  {% call itify('') %}{{ extra }}{% endcall %}

  @ Get finish time
  ldr.w  r3, [r0]

    @ Save the times and results
   blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}

  ldr.w r11, =test_lr
  ldr.w pc, [r11]

.align 4
.thumb_func
restore_order:
  ldr.w r5, =sram_2000_2000
  @ Write "self-refferential" to successive words
  ldr.w r1, =sram_2000_2000
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


{{ section("sram") }}
.equ memory_cell, sram_2000_2000 + {{symbol_offset}} @ (Un)alignment (2 + 2 = ok)

@ Magical way to put a symbol at a fixed address

.section .bigalignbss.aaaa, "wa" @progbits
.align 20
sram_2000_0000: .space 32
test_lr: .word 0
.space 0x40 @ As a safety zone + to fit possible stacking
.align 2
.space 12


@ Put after "lsucnts" before "times"
.section .bss.mmm, "wa" @progbits
@.section .bigalignbss.bbb, "wa" @progbits
.align 13
@ Self-referential
@sram_2000_2000: .rept 16; .short 0x2000; .endr
{{ assert_aligned(13, True) }}
sram_2000_2000: .space 32
.align 2
.space 12

@ This is required in order to fail if the above would be placed there
.section .heap.aaaa, "wa" @progbits
.align 14
sram_2000_4000: .space 16
.align 2
.space 12
.align 2

{% endblock %}
