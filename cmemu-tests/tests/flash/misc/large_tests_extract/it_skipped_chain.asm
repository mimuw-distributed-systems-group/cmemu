---
name: Test the behavior with chained it with a skipped, and pipelined last instruction before next it block.
description: >
    This is an elaboration around code such as:

        itete.n	ge
        ldrge.w	r2, [r0, #-29]
        rorlt.w	r1, r3, r1      @ <- skipped, pipelines?
        strge.w	r3, [r13, #-41]
        noplt.n	                @ <- skipped, pipelines? what if would set flags? what if wide?
        itt.n	mi              @ <- another it, theoretically could fold (does't!)
        sbcmi.w	r3, r7, r1, LSL #1       @ <- random skipped instrs
        rorsmi.w	r0, r0, #3                    @ A7.7.116

dumped_symbols:
  times: auto
  lsucnts: auto
configurations: []
product:
    - code:
      - flash
      memory:
      - gpram
      - sram
      lbEn: [True,]
      base_reg2:
      - sp
      - r1
      - r5
      base_reg1:
      - r1
      - r5
      - sp
      offset:
      - [2, r9] # Both 2 ; with (B, B) there are no new cases
      - [4, r4] # r4 = 0, but aligned...
      it_len:
#      - 0
#      - 1
      - 2
      - 3
      - 4
      extra:
      - ''
      - b.w .+4  #  bus conflict at decode time
      - ldr.w r3, 2f  # conflict at execute time
      last_pad_w:
      - n
      - w
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set cond = 'eq' if it_len > 0 else '' %}
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
    "ldr.w r5, [{base_reg}, {off_imm}]",
    "ldr.w sp, [{base_reg}, {off_imm}]",
    "ldr.w r5, [{base_reg}, {off_reg}]",
    "ldr.w sp, [{base_reg}, {off_reg}]",
    "ldr.w sp, 2f @ in next ltorg",

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
@{% if offset[0] % 4 == 0 %}
@{% do instruction_set_A.extend([
@    "ldm.w {base_reg}, {{r3, r5}}",
@    "ldm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",
@
@    "stm.w {base_reg}, {{r3, r5}}",
@    "stm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",
@]) %}
@{% endif %}
@

{% set instruction_set_B = [
    "nop.n",
    "add.n {base_reg}, r4",

    "ldr.n r5, =1f+1",
    "ldr.n r5, [{base_reg}, 4]",
    "ldr.w r5, [{base_reg}, 4]",
    "str.n r5, [{base_reg}, 4]",
    "str.w r5, [{base_reg}, 4]",

    "pop.n {{r3, r5}}",
    "push.n {{r3, r5}}",
] %}
@ // missing pop.n {pc}
{% if it_len < 4 %}
{% do instruction_set_B.extend([
    "cmp.n {base_reg}, r5 @ can change flags",
    "b.n .+2",
]) %}
{% endif %}

{% if base_reg2 != sp %}
{% do instruction_set_B.extend([
    "mov{s}.n {base_reg}, #0",
    "ldr.n r5, [{base_reg}, r4]",
    "mul{s}.n {base_reg}, r4, {base_reg}",
    "str.n r5, [{base_reg}, r4]",
]) %}

@{% if offset[0] % 4 == 0 or base_reg1 != base_reg2 %}
@@ LDM cannot have unaligned base
@{% do instruction_set_B.extend([
@    "ldm.w {base_reg}, {{r3, r5}}",
@    "stm.w {base_reg}, {{r3, r5}}",
@]) %}
@{% endif %}

{% endif %}
{% set format_args1 = dict(base_reg=base_reg1, r5_or_non_colliding=(r5 if base_reg1 != r5 else r6), sp_or_non_colliding=(sp if base_reg1 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}
{% set format_args2 = dict(base_reg=base_reg2, r5_or_non_colliding=(r5 if base_reg2 != r5 else r6), sp_or_non_colliding=(sp if base_reg2 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}

{% if last_pad_w == 'n' %}
{% set last_pads = ('nop.n', 'mov.n r3, r3',) %}
{% elif last_pad_w == 'w' %}
{% set last_pads = ('nop.w', 'mov.w r3, r3') %}
{% else %}
{{unreachable()}}
{% endif %}

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

{% for first, second in itertools.product(instruction_set_A if set1 == 'A' else instruction_set_B, (instruction_set_A if set2 == 'A' else instruction_set_B)) %}
{% for cond_2 in ('t', 'e', '') if (not cond_2) == (it_len < 3) or (second.startswith('b') and it_len < 3)  %}
{% for last_pad in last_pads %}

  ldr.w r5, 2f
  ldr.w r10, =3f
  mov.n r1, r5
  mov.n sp, r5

  @ Set flags and use as offset
  movs.n r4, #0

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r2, [r0]
  add.w r3, r3
  {% if cond %}
  i{% if it_len >= 1 %}t{% endif %}{% if it_len >= 2 %}e{% endif %}{% if it_len >= 3 %}{{cond_2}}{% endif %}{% if it_len >= 4 %}e{% endif %}.n {{cond}}
  {% else %}
  nop.n
  {% endif %}
  {% call itify(it_len >= 1 and cond) %}{{ first.format(**format_args1) }}{% endcall %}
  {% call itify(it_len >= 2 and notcond) %}add.w r3, r3{% endcall %}
  {% call itify(cond_2|replace('e', 'ne')|replace('t', 'eq')) %}{{ second.format(s='s' if not cond_2 else '', **format_args2) }}{% endcall %}
  {% call itify(it_len >= 4  and notcond ) %}{{ last_pad }}{% endcall %}
  itt.n {{notcond}}
  {% call itify(notcond) %}add.w r3, r3{% endcall %}
  {% call itify(notcond) %}add.w r3, r3{% endcall %}
  {{extra}}

  @ Get finish time
1: @ see https://ftp.gnu.org/old-gnu/Manuals/gas-2.9.1/html_chapter/as_5.html#SEC48
  ldr.w  r3, [r0]

    @ Save the times and results
   blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
b.w 9f
3:
.word 1b+1 @ address of previous
.word 1b+1 @ address of previous
9:
{% endfor %}
{% endfor %}

b.w 9f
.align 2
2: @ Literal addresses to cell
.word memory_cell
.word memory_cell
@.word memory_cell
.ltorg
9:
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
