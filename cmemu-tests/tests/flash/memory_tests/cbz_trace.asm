---
name: Tracing weird behavior of cbz instructions
description: >
    See the sibling test for ldrs first!
dumped_symbols:
  first_wrote: auto
  second_wrote: auto
  first_write_duped: auto
  second_write_duped: auto
  full_test_cnt: auto
  # from template
  pad_adds: auto
  cyc_at_int_entry: auto
  cpi_at_int_entry: auto
  lsu_at_int_entry: auto
  return_addr_off: auto
  return_addr_raw: auto
configurations: []
product:
    - code: [flash]
      lbEn: [True, False]
#      lbEn: [True]
      align_pad: ["", "add.n r5, r11", "add.w r5, r11"]
#      align_pad: [""]
      stallers1: [[0], [1], [2], [3], [4]]
#      stallers1: [[0], [1]]
      dist: [1, 3, 4]
#      dist: [3]
      pre_pad: ["", "nop.n", "adds.n r3, r3, r3", "add.n r3, r11", "adds.n r2, r2, r2", "tst.n r3, r3",
        "tst.n r2, r2", "movs.n r2, 1", "movs.n r3, 1", "cbz.n r0, .+4", "cbz.n r3, .+4", "b.n .+2", "b.w .+4", "beq.n .+4"]
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - tmp reg, r3 - tmp  + cdz base
@ r4 - x_cyc reg 2
@ r5 - pad add-s counter
@ r6, r7 - x_cyc main arg
@ r8 - CPICNT at start + tmp otherwise
@ r9 - LSUCNT at start + tmp otherwise
@ r10 - systick addr to start ticking
@ r11 - tmp in handler, otherwise adder counter
@ r12 - test start addr (stacked)
@ r13 (lr) - needed to jump to saving and return from exception

{% extends "bases/systick_trace.asm.tpl" %}

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}

{% set systick_cycles_range = dict(flash=range(2, 46), sram=range(2, 26), gpram=range(2, 16))[code] %}
{% set adds_after_load = 20 if code == "flash" else 10 %}


{% block tested_code %}
{% for staller_1_cyc  in stallers1 %}
{% set i = loop.index %}
  {% set _, staller_1_exec = n_x_cycles(staller_1_cyc, "r6", "r4") %}

{% for x_cycles in range(5) %}
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r7", "r4") %}

{% for cycles in systick_cycles_range %}
    {% set label = uniq_label("cbz") %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    bl.w prepare_{{i}}_{{x_cycles}}

    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    isb.w
    {{ align_pad }} @ make pad after isb to offset execute vs fetch requests
    @ having it first is meaningful for flash line-buffer testing


    @ Read start CYCCNT value
    ldr.w r1, [r0, {{CYCCNT}}]

    {{ staller_1_exec }}

    {{ pre_pad }}
    cbnz.n r3, {{label}}
    add{{ '.w' if '.n' in pre_pad else 's.n'}} r5, r4

    .rept {{dist-1}}
    add.w r5, r11
    .endr

    {{ label }}:
    {{ x_word_exec }}

    @ Read end CYCCNT value
    ldr.w r2, [r0, {{CYCCNT}}]

@ Do some more adds in case the interrupt comes after the load
.rept {{adds_after_load}}
    add.w r5, r11
.endr

    sub.w r2, r1
    bl save
    {{inc_auto_syms() }}
{% endfor %}
{% endfor %}
{% endfor %}

{% endblock tested_code %}

{% block after_tested_code %}
{% for staller_1_cyc  in stallers1 %}
{% set i = loop.index %}
{% for x_cycles in range(5) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r6", "r4") %}
  {% set staller_1_loader, _ = n_x_cycles(staller_1_cyc, "r7", "r4") %}

  {{ staller_1_loader }}
  {{ x_loader }}
  bx.n lr
{% endfor %}
{% endfor %}

save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r3, r4)}}
    {{saveValue('first_write_duped', r7, r2, r3)}}
    {{saveValue('second_write_duped', r6, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    @ Adds source to check after/before int executed
    mov.w r11, #1
{% endcall %}
    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r11) %}
    {{saveValue('first_wrote', r7, r2, r3)}}
    {{saveValue('second_wrote', r6, r2, r3)}}
    mov.w r11, #0
{% endcall %}
{% endblock handler %}
