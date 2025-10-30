---
name: We identified that conditional branches (outside it) after mul(s).n (either in it or not) behaves differently
description: >
    Note: this is a compact version of `memory_tests/branch_after_narrow_mul.asm`

    Branches within it (encodings T2 and T4) work with the current model.
    However, conditional branches (T1 and T3) seem like they could become "execute time" in this situation.
    It is because of a critical path of flags? It seems to work after wide mul, which never sets flags.

    The example flow in question looks like this:
    .align 2
    it.n              X
    mul.n(whatever)   ID  D  X
    beq.n(whatever)   ID  .  ?  X?  <- not fetching?

    The issue is only observed with side effect of stalling the fetching, by conflicting beq fetch.

dumped_symbols:
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations:
- { code: flash, lbEn: True, mul_width: n, pad_branch: False, b_in_it: False }
- { code: flash, lbEn: True, mul_width: n, pad_branch: True,  b_in_it: False }
- { code: flash, lbEn: True, mul_width: w, pad_branch: False, b_in_it: False }
- { code: flash, lbEn: True, mul_width: n, pad_branch: False, b_in_it: True }
- { code: flash, lbEn: True, mul_width: n, pad_branch: True,  b_in_it: True }
- { code: flash, lbEn: True, mul_width: w, pad_branch: False, b_in_it: True }
...


@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - delay-addr + tmp + after test cnt, r3 - tmp
@ r4 - ldr base addr,
@ r5 - pad add-s counter
@ r6 - xstall reg
@ r7 - mul destination;
@ r8 - tmp otherwise
@ r9 - tmp otherwise
@ r10 - save function
@ r11 - otherwise adder counter
@ r12 - counters loop (lr backup)
@ r13 (lr) - needed to jump to saving and return from exception

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address

{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    ldr.w r0, dwt
    add.w r0, {{counter}}
    ldr.w r10, ={{save_func}}

    bl.w    tested_code
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
    @ save where to return after test
    mov.w r12, lr
    mov.w r4, 0
    mov.w r5, 1

{% for pre_align in [
      '',
      'adds.w r5, r11',
      ] %}
{% for b_width in [
        'w',
        'n',
      ] %}
{% for mul_cond in [
         '', 'ne', 'eq',
         'lt'
      ] if mul_cond or not b_in_it %} @# lt hackily introduces non-skipped ldr which hides skipped mul
{% for b_cond in
      (["eq", "ne", ""] if not b_in_it else
       (["eq", "ne"] if mul_cond != 'lt' else ["lt", "ge"]))
 %}
{% for lc1_width in [
      'n',
      'w',
      ] %}
{% for n_stall in [
      1,
      4,
      8,
      ] %}

    {% set x_load, x_exec = n_x_cycles(n_stall, r6, r11) %}
    {% set end_label = uniq_label('end_label') %}
    @ Clear destination registers
    mov.w r6, 0x42 @ mul result
    mov.w r7, #1
    @ r11 = 2
    {{x_load}}

    @ start the main test
    .align 3
    isb.w
    {{ pre_align }}

    @ Read start CYCCNT value
    ldr.{{lc1_width}} r1, [r0]
    {{x_exec}}

    {% if not pre_align or mul_cond %}
    adds.n r5, r7 @ Set flags here
    {% if mul_cond == 'lt' %} @ put ldr
    ite{{('t' if b_cond=='ge' else 'e') if b_in_it else ''}}.n ge
    ldrge.n r6, [r4]
    {% elif mul_cond %}
    it{{('t' if b_cond==mul_cond else 'e') if b_in_it else ''}}.n {{ mul_cond }}
    {% else %}
    nop.n
    {% endif %}
    {% endif %}

    mul{{'s' if mul_width == 'n' and not mul_cond}}{{mul_cond}}.{{mul_width}} r7, r4, r7
    b{{b_cond}}.{{b_width}} {{end_label}}


{% if pad_branch %}
.rept 3
    add.w r5, r11
.endr
{% endif %}

{{end_label}}:
    @ Read end CYCCNT value
    ldr.n r2, [r0]

  {{ inc_auto_syms() }}
    @ Save test results
    blx.n r10
{% endfor %}
{% endfor %}
{% endfor %}
{{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}


    @ Return to counters loop
    bx.n r12


.ltorg

.align 4
.thumb_func
save_time_flags_and_result:
    sub.w r1, r2, r1
    mrs.w r9, apsr

    {{saveValue("times", r1, r3, r8)}}
    {{saveValue("flags", r9, r3, r8)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r1, r2, r1
    and.w r1, r1, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r1, r3, r8)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r1, r2, r1
    and.w r1, r1, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r1, r3, r8)}}

    bx.n lr

{% endblock %}
