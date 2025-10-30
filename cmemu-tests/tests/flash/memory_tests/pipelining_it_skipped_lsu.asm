---
name: Check the address phase timings after skipped instructions, which would incur AGU RAW conflict.
description: >
    Assuming AHB_CONST_CTRL if an address was exposed on DCode bus, the transfer will finish before interrupt handling.
    Assuming DNOTITRANS, Cortex-M3 internally waits I/DCode in case of conflicts.
    We can use this knowledge to properly map the correct position of address generation upon skipped RAW conflict.
    Example code would include:
    ite eq (false)
    ldreq r1, [r2], 4      D   X (S)
    ldreq r3, [r2]             D/AGU? X:DA?
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
      lbEn: [True, False]
      width: [
        w,
#        n
      ]
      cond: ['eq', 'ne']
      reg_dep: [true, false]
      post_instr: ["ldr", "str", "add", "nop"]
      pre_instrs: [
        ["ldr", "str"],
#        ["add", "adds"],
      ]
      data: [0, 1, 2]
      second_offset: ["r7", "#0"]
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - delay-addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - ldr base addr,
@ r6 - ldr destination;
@ r7 - possible register-offset (always 0?)
@ r5 - pad add-s counter
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
{% set addr_1, delay_addr = [
    ("sram_chunk",  "sram_chunk",),
    ("sram_chunk+2","sram_chunk",),
    ("sram_chunk+3","sram_chunk",),
][data] %}

{% set set_data_value = addr_1.startswith("sram") or addr_1.startswith("gpram") %}

{% set notcond = {
    "eq": "ne",
    "ne": "eq",
}[cond] %}

{% block tested_code  %}

{% for pad in (
       [2, 'add.w r5, r11'],
       [1, 'nop.w; add.w r5, r11'],
       ) %}
{% set pad_pos, pad_instr = pad %}
{% for reg_off_instr in pre_instrs %}
{% for cycles in systick_cycles_range %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    isb.w

    @ Read start CYCCNT value
    ldr.n r1, [r0, {{CYCCNT}}]
    @ Fully recreate the conditions from benchmark_out/definitive_lsu_reg_offset-partial12.1830633
    adds.w r5, r11 @ Set flags here
    ldr.n r3, [r2, 0]
    itete.n {{ cond }}
    {% if "ldr" in pre_instrs %}
    {{reg_off_instr}}{{cond}}.w r3, [{{ r4 if reg_dep else r6}}], 0x4
    {% else %}
    {{reg_off_instr}}{{cond}}.w {{ r4 if reg_dep else r6}}, 0x4
    {% endif %}
    @ The Decode should happen at the same time as "virtual" X(S) of the previous
    {% if post_instr == "add" %}
        {{post_instr}}{{notcond}}.{{width}} r3, r4, {{second_offset}}
    {% elif post_instr == "nop" %}
        {{post_instr}}{{notcond}}.{{width}}
    {% else %}
        {{post_instr}}{{notcond}}.{{width}} r3, [r4, {{second_offset}}]
    {% endif %}
    add{{cond}}.n r5, r2
    add{{notcond}}.w r5, r11

{% if pad_pos == 1 %}
    {{pad_instr}}
{% endif %}

    @ Read end CYCCNT value
    ldr.n r2, [r0, {{CYCCNT}}]

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
{% endfor %}
{% endblock tested_code %}

{% block after_tested_code %}
save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r3, r7)}}
    {{saveValue('base_addr', r4, r2, r3)}}
    {{saveValue('second_loaded_after', r3, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    @ Prepare address for ldm
    ldr.w r4, ={{addr_1}}
    ldr.w r2, ={{delay_addr}} @ r2 is stacked

{% if set_data_value %}
    @ restore original values
    ldr.w r3, [r4, 12]
    str.w r3, [r4]
    str.w r3, [r4, 4]
@    ldr.w r3, [r2, 8]
@    str.w r3, [r2]
{% endif %}
    @ Clear destination registers
    @ EVFLAGS is 0, otherwise we won't see its result
    mov.w r6, {{ '0x42' if reg_dep else r4 }}
    mov.w r7, 0x0 @ offset in the ldr reg-reg

    mov.w r11, #1
{% endcall %}

    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r11) %}
    {{saveValue('r6_dest_loaded', r3, r2, r3)}}
    {{saveValue('r4_addr_value', r4, r2, r3)}}

{% if set_data_value %}
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
