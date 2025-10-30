---
name: Check for conflict between instructions reading registers in Execute and AGU.
description: |
    Cortex-M3 has supposedly two register bank read ports: A and B.
    Moreover, data may come from FieldExtractor (from instr), or be forwarded from LDR or ALU+MUL results.

    STR (register), MLA and UMLAL need to read additional register(s - for UMLAL) from the reg bank/fwd logic.
    But AGU might need the same buses at the same time. A few options are conceivable:
    1. Those instructions delay AGU (or AGU always at the end cycle?)
    2. A/B/FieldExtractor/DataForwarder may use a separate routing (note: no way to check a conflict for DataForwarder?)
    3. The effect are not noticeable during normal execution (last cycle always is read-free).
      - Unless considering single-cycle STR (internal write buffer)

    Conclusions:
    Is seems that 3 is the case. Moreover, skipped instruction doesn't need the read in execute.
    From other tests, however, we know that LDR result is forwarded to AGU, but LSU not (critical path?).
dumped_symbols:
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations:
    - {code: flash, lbEn: True, instrs: ["ldr.n", "str.n"],  it: False}
    - {code: flash, lbEn: False, instrs: ["ldr.n", "str.n"], it: False}
    - {code: flash, lbEn: True, instrs: ["ldr.n", "str.n"],  it: True }
    - {code: flash, lbEn: False, instrs: ["ldr.n", "str.n"], it: True }
    - {code: sram, lbEn: True, instrs: ["ldr.n"], it: False}
    - {code: sram, lbEn: True, instrs: ["str.n"], it: False}
    - {code: sram, lbEn: True, instrs: ["ldr.n"], it: True }
    - {code: sram, lbEn: True, instrs: ["str.n"], it: True }
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}


@ Register assignment
@ r0 - counter
@ r1 - free, destroyed in save
@ r2 - counter-start
@ r3 - counter-end (can be used earlier)
@ r1 - free, destroyed in save
@ r5 - memory cell addr (unchanged!)
@ r6 - result (trash)
@ r7 - always 4
@ r8 - always 0
@ r9 - xstall = always set to 2
@ r10 - xstall changing
@ r11 - alt store for LR (can be changed for push/pop if needed)
@ r12 - after-test func (unchanged!)
@ sp, lr - reserved

{% macro itify(repl) %}
{% if repl.strip() == "" %}
{{ caller() }}
{% else %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}

{% set cond, notcond = ('eq', 'ne') if it else (' ', ' ') %}


{% set save_func_reg = "r12" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r5, =memory_cell

    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"),] %}
        ldr.w r0, dwt
        add.w r0, {{counter}}
        ldr.w {{save_func_reg}}, ={{save_func}}

        bl.w tested_code
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
    mov.w r11, lr @ Store LR
    movs.w r8, #0
    mov.w r7, #4
{% for ex_reg_reader in [
    'mla.w {rex2}, {rd1}, {rd2}, {rex1}',
    'umlal.w {rex1}, {rex2}, {rd1}, {rd2}',
    'str.w {rex1}, [{rd1}, {rd2}]',
 ] %}
{% for same_reg in (True, False) %}
{% for offset in ('#4', r7) %}
{% for pad in ('', 'nop.n', 'nop.w', 'add.w r6, 0') %}
{% for x_cycles1 in range(1, {'flash': 8, 'gpram': 4, 'sram': 7}[code]) %}
{% for instr in instrs %}
  {% set x_loader1, x_word_exec1 = n_x_cycles(x_cycles1, "r10", "r9") %}

  @ Prepare register contents
  {{x_loader1}}

   cmp.n r8, r8 @ set flags
  .align 3
  isb.w   @ Clear PIQ

  @ Get start time
  {% if not it %}
  ldr.w  r2, [r0]
  {% elif it %}
  ldr.n  r2, [r0]
  itet.n eq
  {% endif %}
  {% call itify(cond) %}{{x_word_exec1}}{% endcall %}

  @ NOTE: UMLAL is designed to preserve rex1
  {% call itify(notcond) %}{{ex_reg_reader.format(rex1=r5 if same_reg else r9, rex2=r6, rd1=r5, rd2=r8) }}{% endcall %}
  {% call itify(cond) %}{{instr}} r6, [r5, {{offset}}]{% endcall %}

  {{pad}}
  @ Get finish time
  ldr.w  r3, [r0]
  blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  bx.n r11

.thumb_func
save_times_and_flags:
  mrs.w r1, apsr
  sub.w r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
  bx.n lr

.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr


.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{{section('sram')}}
.align 2
memory_cell:
.space 32
{% endblock %}
