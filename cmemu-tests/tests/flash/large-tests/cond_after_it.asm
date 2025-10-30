---
name: Test behavior of conditional branches after it.
description: >
    Theoretically, it should be tested within  definitive_ldr*, with it_len=3, but some errors were missed.
    It's possible the issue is visible only from single-cycle memory.
    This is an elaboration around code such as:

        itt le                  @ <- can both execute or skip ?
        ldrhle  r4, cell_139    @ <- does it must be an lsu instr here?
        mulle   r7, r3          @ is it about flags settings? being second-part narrow?
        bmi jump_back           @  <- has to be conditional, but may either fail or succeed

dumped_symbols:
  times: auto
  lsucnts: auto
#  foldcnts: auto
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
#      it_len:
##      - 0
#      - 1
#      - 2
##      - 3
##      - 4
      it_base:
#      - t
#      - te
#      - tt
      - tte
      - ttt
      - tet
      - tee
      cond:
      - eq
      - ne
      - ''
      set1:
      - A
#      - B
      set2:
      - A
      - B
      extra:
      - ''
      - nop.n
      - b.n .+2  #  bus conflict at decode time
      - ldr.n r3, 2f  # conflict at execute time

      last_pad:
      - nop.n
      - nop.w
      - mov{s}.n r3, 0
#      - mov{s}.n {base_reg}, #0
      - mov.n r3, r3
      - mov.n sp, r3
#      - cmp.n sp, r5
#      - add.n sp, 4
      - add{s}.n r3, r3
#      - mul{s}.n {base_reg}, r4, {base_reg}
#      - mul.w {base_reg}, r4, {base_reg}
#      - mla.w {base_reg}, r3, {base_reg}, r4
      - mul{s}.n r3, r4, r3
      - mul.w r3, r4, r3
      - mla.w r3, r3, r3, r4
      - mov.w r3, r3
      - cmp.n r5, {base_reg}
      #- b.n 1f
      #- mov.n pc, lr
      - mrs.w r5, apsr
      - msr.w   apsr, r5
      # NOTE: r5 may be invalid!
      #- ldr.n r5, [sp]
      #- ldr.n r5, [r1]
      #- str.n r5, [sp]
      #- pop.n {{r3, r5}}
      #- ldr.n r5, [{base_reg}, r4]
      #- str.n r5, [{base_reg}, r4]
      #- push.n {{r3, r5}}
      #- ldm.n r5, {{r5, r7}}
      #- ldm.n r5!, {{r3, r7}}
      branch_cond:
      - eq
      - ne
      - t
      - e
      - ''
      post_it:
      # what if inside it?
#      - beq.n 1f
#      - bne.n 1f
#      - cbz.n r5, 1f
#      - cbnz.n r5, 1f
#      - beq.w 1f
      - b.n 1f
      - b.w 1f
# TODO: cannot be cond outside it
#      - bx.n lr
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% pyset it_len = len(it_base) %}
@{% set cond = 'eq' if it_len > 0 else '' %}
@{% set set1, set2 = 'A', 'B' %}

@ Register assignment:
@ r0, r2, - Counter
@ r1 - possible base offset
@ r3 - x_staller, then trash, then final counter
@ r4 - always 0 (used in saving)
@ r5 - possible base offset
@ r6 - alternative trash
@ r7 - free to use
@ r8 -
@ r9 - staller's 2
@ r10 - holds address of address with "1f+1" (case end; ldr rx, ==label)
@ r11 - auxiliary (used during navigation between counters loop and tested code)
@ r12 - after-test func (unchanged!)
@ r13 (sp) - possible base offset
@ lr - navigation, may be used in test

{% set save_func_reg = "r12" %}

{% block code %}
    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (LSUCNT, "save_lsucnt")] %}
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
@ The loaded value into r5,sp,base_reg2 is always a correct address!
@  "ldr.w sp, 2f @ in next ltorg",
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
{% if offset[0] % 4 == 0 %}
{% do instruction_set_A.extend([
    "ldm.w {base_reg}, {{r3, r5}}",
    "ldm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",

    "stm.w {base_reg}, {{r3, r5}}",
    "stm.w {base_reg}!, {{r3, {r5_or_non_colliding} }}",
]) %}
{% endif %}


{% set instruction_set_B = [
    "nop.n",
    "add.n {base_reg}, r4",
    "cmp.n {base_reg}, r5 @ can change flags",

    "ldr.n r5, =1f+1",
    "ldr.n r5, [{base_reg}, 4]",
    "ldr.w r5, [{base_reg}, 4]",
    "str.n r5, [{base_reg}, 4]",
    "str.w r5, [{base_reg}, 4]",

    "pop.n {{r3, r5}}",
    "push.n {{r3, r5}}",
] %}
@ // missing pop.n {pc}
@{% if it_len < 4 %}
@{% do instruction_set_B.extend([
@    "b.n .+2",
@]) %}
@{% endif %}

{% if base_reg2 != sp %}
{% do instruction_set_B.extend([
    "mov{s}.n {base_reg}, #0",
    "ldr.n r5, [{base_reg}, r4]",
    "mul{s}.n {base_reg}, r4, {base_reg}",
    "str.n r5, [{base_reg}, r4]",
    'mrs.w r5, apsr',
    'msr.w apsr, r5',
]) %}

{% if offset[0] % 4 == 0 or base_reg1 != base_reg2 %}
@ LDM cannot have unaligned base
{% do instruction_set_B.extend([
    "ldm.w {base_reg}, {{r3, r5}}",
    "stm.w {base_reg}, {{r3, r5}}",
]) %}
{% endif %}

{% if base_reg2 != sp and (offset[0] % 4 == 0 or base_reg1 != base_reg2) %}
@ LDM cannot have unaligned base
{% do instruction_set_B.extend([
    "ldm.n {base_reg}!, {{ {r5_or_non_colliding}, r7 }}",
    "stm.n {base_reg}!, {{ {r5_or_non_colliding}, r7 }}",
]) %}
{% endif %}

{% endif %}
{% set format_args1 = dict(base_reg=base_reg1, r5_or_non_colliding=(r5 if base_reg1 != r5 else r6), sp_or_non_colliding=(sp if base_reg1 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}
{% set format_args2 = dict(base_reg=base_reg2, r5_or_non_colliding=(r5 if base_reg2 != r5 else r6), sp_or_non_colliding=(sp if base_reg2 != sp else r5),
                          off_imm=offset[0], off_reg=offset[1]) %}



{% python def itify(cond, caller)  %}{% raw %}
    """cond is either a condition, None - no output / '' - no condition"""
    if cond is None or cond is False:
        return ''
    cond = cond.strip()
    instr = caller if isinstance(caller, str) else caller()
    if cond == '':
        return instr
    return instr.replace('.n', cond + '.n').replace('.w', cond + '.w')
{% endraw %}{% endpython %}


{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
   ldr.w r11, =test_lr
   str.w lr, [r11]
   mov.w r9, #2 @ For n-X-cycles

{% set notcond = {'eq': 'ne', 'ne': 'eq', '': ''}[cond] %}
{% set ns = namespace() %}
{% python %}{% raw %}
te_map = {'t': cond, 'e': notcond, ' ': None}
b_in_it = branch_cond in set('te')
it_last = branch_cond if b_in_it else ' '
ns.real_bcond = te_map[branch_cond] if b_in_it else branch_cond

# its -> 4-string of 't', 'e', ' '
# conds -> 4-tuple of matching cond, notcond, '' or None (=gaps)
# TODO: simple approach first!
ns.its = (it_base + it_last + '    ')[:4]
#{% pyset priority = [2, 3, 1, 0] %}
#@if b_in_it:
#@    priority = [2, 1, 3, 0]
#@else:
priority = [1, 0, 2, 3]

#{% pyset priority = [1, 2, 0, 3] %} @ <- version without last_pad
conds = [te_map[t] for t in ns.its]
ns.conds = [conds[sum(pp < it_len for pp in priority[:i])] if p < it_len else None for i, p in enumerate(priority)]
ns.firsts = ('NOTTHERE',) if ns.conds[0] is None else instruction_set_A if set1 == 'A' else instruction_set_B
{% endraw %}{% endpython %}
@ IDEA: add "export" field to generate Jinja Assign nodes from the block
{% pyset conds = ns.conds %}
{% pyset real_bcond = ns.real_bcond %}


{% for first in ns.firsts %}
{% for second in (instruction_set_A if set2 == 'A' else instruction_set_B) %}
{% for align in ('add.w r3, r3', 'mov.n r3, r3', 'add.w r3, r3; mov.n r3, r3') %}
{% for x_cycles in ((9,) if first != 'NOTTHERE' else (0, 1, 4, 6, 7, 8))  %}
  {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r8", "r9", load_2=False, compact=True) %}

  ldr.w lr, =1f+1
  ldr.w r5, 2f
  ldr.w r10, =3f
  mov.n r1, r5
  mov.n sp, r5
  {{ x_word_load }}


  @ Set flags and use as offset
  movs.n r4, #0

  @ XXX: we neeed stallers!
  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r2, [r0]
  {{ x_word_exec }}
  {{align}}
  {% if cond %}
  i{{ns.its}} {{cond}}
  {% else %}
  nop.n
  {% endif %}
  {% call itify(conds[0]) %}{{ first.format(s='s' if not conds[0] else '', **format_args1) }}{% endcall %}
@  {% if conds[1] != None %}{% call itify(conds[1]) %}add.w r3, r3{% endcall %}{% endif %}
  {% call itify(conds[1]) %}{{ second.format(s='s' if not conds[1] else '', **format_args2) }}{% endcall %}
  {% call itify(conds[2]) %}{{ last_pad.format(s='s' if not conds[2] else '', **format_args2) }}{% endcall %}
@  {% if conds[3] != None %}{% call itify(conds[3]) %}{{post_it.format(s='', **format_args2)}}{% endcall %}{% endif %}
  {% call itify(real_bcond) %}{{post_it.format(s='', **format_args2)}}{% endcall %}
  {{extra}}

  @ Get finish time
1: @ see https://ftp.gnu.org/old-gnu/Manuals/gas-2.9.1/html_chapter/as_5.html#SEC48
  ldr.w  r3, [r0]

    @ Save the times and results
   blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
b.n 9f
.align 2
3:
.word 1b+1 @ address of previous
.word 1b+1 @ address of previous
2: @ Literal addresses to cell
.word memory_cell
.word memory_cell
@.word memory_cell
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

@.thumb_func
@save_foldcnt:
@  sub.w r2, r3, r2
@  and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
@  {{saveValue("foldcnts", r2, r3, r4)}}
@  b.w restore_order @ tail-call


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
