---
name: Assuming AHB_CONST_CTRL & DNOTITRANS, check the behavior of interrupted pipelineing of str with conflict
description: >
    See the sibling test for ldrs first!
    Assuming AHB_CONST_CTRL if an address was exposed on DCode bus, the transfer will finish before interrupt handling.
    Assuming DNOTITRANS, Cortex-M3 internally waits I/DCode in case of conflicts.
    Some other tests (namely ldr_reg) show that a) sometimes ldr-s don't pipeline if decode is after address-phase
    b) but sometimes the do, while DCode is waitstated.
    In the flash/flash case, the ldrs have to be pipelined in order to reduce the timing by 5 cycles.
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
      align_pad: ["", "add.n r5, r11", "add.w r5, r11"]
      stallers1: [[1], [2]]
      stallers2: [[4],[5] ]
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - x_cyc reg, r3 - tmp
@ r4 - x_cyc reg 2, r5 - pad add-s counter
@ r6, r7 - Second loader
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
{% set stallers = itertools.product(stallers1, stallers2) %}
{% for (staller_1_cyc, staller_2_cyc)  in stallers %}
{% set i = loop.index %}
@  {% set _, staller_1_exec = n_x_cycles(staller_1_cyc, "r4", "r5") %}
  {% set _, staller_2_exec = n_x_cycles(staller_2_cyc, "r6", "r7") %}

{% for x_cycles in range(5) %}
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r4", "r2") %}

{% for cycles in systick_cycles_range %}

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

    add.w r5, r11
    {{ staller_2_exec }}

    b.w .+4
    add.w r5, r11
    add.w r5, r11

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
{% set stallers = itertools.product(stallers1, stallers2) %}
{% for (staller_1_cyc, staller_2_cyc)  in stallers %}
{% set i = loop.index %}
{% for x_cycles in range(5) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r4", "r2") %}
  @{% set staller_1_loader, _ = n_x_cycles(staller_1_cyc, "r4", "r5") %}
  {% set staller_2_loader, _ = n_x_cycles(staller_2_cyc, "r6", "r7") %}
@  {{ staller_1_loader }}
  {{ staller_2_loader }}
  {{ x_loader }}
  bx.n lr
{% endfor %}
{% endfor %}

save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r3, r7)}}
    {{saveValue('first_write_duped', r4, r2, r3)}}
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
    {{saveValue('first_wrote', r4, r2, r3)}}
    {{saveValue('second_wrote', r6, r2, r3)}}
    mov.w r11, #0
{% endcall %}
{% endblock handler %}
