---
name: Finding why tests with last narrow pipelineable instruction in IT sometimes fail
description: >

dumped_symbols:
  times: auto
  cyc_in_it: auto
  flags: auto
  foldcnts: 1024 B
  cpicnts: 1024 B
  lsucnts: 1024 B
configurations: []
product:
    - code: [flash ]
      lbEn: [False, True]
      first_it:
        - t
        - e
        - ''
...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - reset alias to r0
@ r3 - final counter

@ r4 - always 0 (used in saving)
@ r5 - memory cell
@ r6,
@ r7 - ~0 (all ones: -1)
@ r8 - staller 3
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
{% set it = 'pl' %}
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
   ldr.w r11, =test_lr
   str.w lr, [r11]

{% for x_cycles1 in (1,) %}
  {% set x_loader1, x_word_exec1 = n_x_cycles(x_cycles1, "r10", "r9") %}
{% for b_instr in ('b.n .+2', 'nop.n', 'add.n r4, r4', 'ldr.n r5, [r6]') %}
{% for b_in_it, bcond in itertools.product((True, False), (cond, notcond,)) if (b_in_it or 'b.' in b_instr) and (bcond == notcond or 'nop' in b_instr) %}

@ same for  'ldr.w sp, [r4]', 'ldr.w r5, [r5, r4]', 'adds.w r5, r5'
@ Only number of cycles matter here
{% for dummy_instr in ('ldr.w r6, [r1]', 'add.w r5, r5', 'add.n r5, r5; add.n r5, r5', 'mla.w r5, r4, r6, r5', 'umull.w r5, r4, r5, r4', 'nop.w', 'add.w r5, r5; add.w r5, r5') %}
@{% for x_cycles2 in (range(5, 9) if not lbEn else (range(1, 5) if x_cycles1 == 0 else range(4, 8))) %}
{% for x_cycles2 in (range(7, 11) if not lbEn else (range(1, 5) if x_cycles1 == 0 else range(5, 9))) %}
{% for x3_cycles in (1, 2, 3, 4) %}

  {% set x_loader2, x_word_exec2 = n_x_cycles(x_cycles2, "r11", "r9", load_2=False) %}
  {% set x3_loader, x3_word_exec = n_x_cycles(x3_cycles, "r8", "r9", load_2=False) %}
  {% set jump_label = uniq_label("bx_n") %}
  {{ x_loader1 }}
  {{ x_loader2 }}
  {{ x3_loader }}
    movs.n r4, #0
    mvns.n r7, r4
    mov.n r6, r4
    ldr.w r1, =memory_cell
    mov.n r5, r4
    mov.n sp, r4

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
  {% set it_cond = notcond if first_it == 'e' else cond %}
  {% set it_instr -%}
  i{{'t'*dummy_instr.count('.') if first_it else ''}}{{'e' if first_it == 'e' else 't'}}{{ {True: 't', False: 'e'}[bcond == it_cond] if b_in_it else ''}}.n {{it_cond}}
  {% endset %}

  {{ x3_word_exec }}
  {{ it_instr if first_it else '' }} @ Extra instr in IT
  {% call itify({'t': cond, 'e': notcond, '': ''}[first_it]) %}{{dummy_instr}}{% endcall %}
  {{ it_instr if not first_it else '' }} @ Alternatively, extra instr before it
  ldr{{cond}}.w r6, [r0]
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
