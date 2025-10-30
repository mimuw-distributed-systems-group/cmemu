---
name: Assuming AHB_CONST_CTRL & DNOTITRANS, check the behavior of interrupted pipelineing with conflict
description: >
    Assuming AHB_CONST_CTRL if an address was exposed on DCode bus, the transfer will finish before interrupt handling.
    Assuming DNOTITRANS, Cortex-M3 internally waits I/DCode in case of conflicts.
    Some other tests (namely ldr_reg) show that a) sometimes ldr-s don't pipeline if decode is after address-phase
    b) but sometimes the do, while DCode is waitstated.
    In the flash/flash case, the ldrs have to be pipelined in order to reduce the timing by 5 cycles.
dumped_symbols:
  first_loaded: auto
  second_loaded: auto
  first_loaded_after: auto
  second_loaded_after: auto
  pad_adds: auto
  cyc_at_int_entry: auto
  cpi_at_int_entry: auto
  lsu_at_int_entry: auto
  full_test_cnt: auto
  return_addr_off: auto
  return_addr_raw: auto
configurations: []
product:
    - code: [sram, flash, gpram]
      lbEn: [True, False]
      data: [flash, sram, gpio, gpram]
      width: [w, n]
      pad: [
       [1, 'add.n r5, r11'],
       [2, 'add.n r5, r11'],
       [1, 'add.w r5, r11'],
       [2, 'add.w r5, r11'],
      ]
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - second ldr addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - first ldr addr, r5 - pad add-s counter
@ r6, r7 - ldr's destination; r7 also tmp for cnt
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

{% set sysbus_addresses = {
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}
{% set addr_1 = {"flash": "flash_chunk", "sram": "sram_chunk", "gpram": "gpram_chunk", "gpio": sysbus_addresses["GPIO::DIN31_0"]}[data] %}
{% set addr_2 = {"flash": "flash_chunk+4", "sram": "sram_chunk+4", "gpram": "gpram_chunk+4", "gpio": sysbus_addresses["GPIO::EVFLAGS"]}[data] %}

{% set pad_pos, pad_instr = pad %}

{% block tested_code  %}
{% for align_pad in ("", "nop.w", "nop.n") %}
{% for cycles in systick_cycles_range %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    {{ align_pad }}
    isb.w

    @ Read start CYCCNT value
    ldr.w r1, [r0, {{CYCCNT}}]
    add.w r5, #1
    @ The first ldr cannot pipeline anyway and it have to loose arbitration with Fetch at the beginning
    ldr.{{width}} r6, [r4, 0]
    @ The second ldr comes too late, but still has to win with next Fetch
    ldr.{{width}} r7, [r2, 0]

{% if pad_pos == 1 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

    @ Read end CYCCNT value
    ldr.w r2, [r0, {{CYCCNT}}]

{% if pad_pos == 2 %}
@ Do some more adds in case the interrupt comes after the load
.rept {{adds_after_load}}
    add.w r5, r11
.endr
{% endif %}


    @ Save number of cycles until ldm done
    sub.w r2, r1
    bl save
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endblock tested_code %}

{% block after_tested_code %}
save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r3, r4)}}
    {{saveValue('first_loaded_after', r6, r2, r3)}}
    {{saveValue('second_loaded_after', r7, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    @ Prepare address for ldm
    ldr.w r4, ={{addr_1}}
    ldr.w r2, ={{addr_2}} @ r2 is stacked

{% if data in ('sram', 'gpram') %}
    @ restore original values
    ldr.w r3, [r4, 8]
    str.w r3, [r4]
    ldr.w r3, [r2, 8]
    str.w r3, [r2]
{% endif %}
    @ Clear destination registers
    @ EVFLAGS is 0, otherwise we won't see its result
    mov.w r6, 0x42
    mov.w r7, 0x42

    mov.w r11, #1
{% endcall %}

    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r11) %}
    {{saveValue('first_loaded', r6, r2, r3)}}
    {{saveValue('second_loaded', r7, r2, r3)}}

{% if data in ('sram', 'gpram') %}
    mov.w r11, 0x13
    str.w r11, [r4, 0]
    str.w r11, [r4, 4]
{% endif %}

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
