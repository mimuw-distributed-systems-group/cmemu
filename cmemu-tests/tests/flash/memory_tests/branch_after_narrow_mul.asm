---
name: We identified that conditional branches (outside it) after mul(s).n (either in it or not) behaves differently
description: >
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
  full_test_cnt: auto
  base_addr: auto
  second_loaded_after: auto
  r6_dest_loaded: auto
  r4_addr_value: auto
  # Symbols used by the template
  pad_adds: auto
  cyc_at_int_entry: auto
  cpi_at_int_entry: auto
  lsu_at_int_entry: auto
  return_addr_off: auto
  return_addr_raw: auto
configurations: []
product:
    - code: [
#        sram,
        flash,
#        gpram
        ]
      lbEn: [
      True,
#      False
      ]
      mul_width: [
        w,
        n
      ]
      b_width: [
        w,
        n
      ]
      mul_cond: [
         '', 'ne', 'eq',
         'lt' # lt hackily introduces non-skipped ldr which hides skipped mul
      ]
      n_stall:
      - 1
      - 4
      #- 8
      lc1_width:
      - 'n'
      - 'w'
      pre_align:
      - ''
      - adds.w r5, r11
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - delay-addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - ldr base addr,
@ r5 - pad add-s counter
@ r6 - xstall reg
@ r7 - mul destination;
@ r8 - CPICNT at start + tmp otherwise
@ r9 - LSUCNT at start + tmp otherwise
@ r10 - systick addr to start ticking
@ r11 - tmp in handler, otherwise adder counter
@ r12 (above) - test start addr (stacked)
@ r13 (lr) - needed to jump to saving and return from exception
{% extends "bases/systick_trace.asm.tpl" %}

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}


{% set systick_cycles_range = dict(flash=range(2, 46), sram=range(2, 20), gpram=range(2, 16))[code] %}
{% set adds_after_load = 16 if code == "flash" else 10 %}


{% block tested_code  %}

{% for pad in (
       [2, 'add.w r5, r11'],
       [1, 'add.w r5, r11'],
       ) %}
{% set pad_pos, pad_instr = pad %}
{% for b_cond in
        ["eq", "ne", ""]
 %}
{% for cycles in systick_cycles_range %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    {% set x_load, x_exec = n_x_cycles(n_stall, r6, r11) %}
    {{x_load}}

    {% set jump_label = uniq_label('jump_label') %}
    {% set end_label = uniq_label('end_label') %}
    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    isb.w
    {{ pre_align }}

    @ Read start CYCCNT value
    ldr.{{lc1_width}} r1, [r0, {{CYCCNT}}]
    {{x_exec}}

    {% if not pre_align or mul_cond %}
    adds.n r5, r7 @ Set flags here
    {% if mul_cond == 'lt' %} @ put ldr
    ite.n ge
    ldrge.n r6, [r4]
    {% elif mul_cond %}
    it.n {{ mul_cond }}
    {% else %}
    nop.n
    {% endif %}
    {% endif %}

    mul{{'s' if mul_width == 'n' and not mul_cond}}{{mul_cond}}.{{mul_width}} r7, r4, r7
    b{{b_cond}}.{{b_width}} {{end_label}}


{% if pad_pos == 1 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

{{end_label}}:
    @ Read end CYCCNT value
    ldr.n r2, [r0, {{CYCCNT}}]

{% if pad_pos == 2 %}
@ Do some more adds in case the interrupt comes after the load
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}


    @ Save number of cycles until ldm done
    sub.w r2, r1
    bl save
    {{inc_auto_syms()}}

b.w .+8
{{jump_label}}:
b.w {{end_label}}

{{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endblock tested_code %}

{% block after_tested_code %}
save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('second_loaded_after', r7, r2, r3)}}
    {{saveValue('full_test_cnt', r2, r3, r7)}}
    {{saveValue('base_addr', r4, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    mov.w r4, 0

    @ Clear destination registers
    mov.w r6, 0x42 @ mul result

    mov.w r7, #1
    mov.w r11, #2
{% endcall %}

    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r11) %}
    {{saveValue('r6_dest_loaded', r7, r2, r3)}}
    mov.w r11, #0
{% endcall %}
{% endblock handler %}

{#
.align 2
was_in_cache:
    isb.w
    ldr.n r2, [r0]
    ldr.n r6, [r6]
    ldr.n r3, [r0]
    nop.n
    sub.w r3, r2
    cmp.w r3, #5
    nop.n
    ite.n le
    movle.w r6, #1
    movgt.w r6, #0
    bx.n lr
 #}
