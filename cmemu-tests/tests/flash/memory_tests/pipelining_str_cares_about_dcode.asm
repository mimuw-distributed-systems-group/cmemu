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
      data: [sram, gpio, gpram]
      width: [w, n]
      pad: [[0, 'add.n r5, r11'], [1, 'add.n r5, r11'], [2, 'add.n r5, r11'],  [2, 'add.n r2, r11'], [0, 'add.w r5, r11'],  [1, 'add.w r5, r11'], [2, 'add.w r5, r11'], [2, 'add.w r2, r11']]
...

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - second ldr addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - first ldr addr, r5 - pad add-s counter
@ r6, r7 - str data source; r7 also tmp for cnt
@ r8 - CPICNT at start + tmp otherwise
@ r9 - LSUCNT at start + tmp otherwise
@ r10 - systick addr to start ticking
@ r11 - tmp in handler, otherwise adder counter
@ r12 (above) - test start addr (stacked)
@ r13 (lr) - needed to jump to saving and return from exception

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}

{% extends "asm.s.tpl" %}

@ [ARM-ARM] B3.3.2
{% set syst_csr = 'E000E010'|int(base=16) %}
{% set syst_rvr = 'E000E014'|int(base=16) %}
{% set syst_cvr = 'E000E018'|int(base=16) %}

@ [ARM-ARM] B3.3.2
{% set syst_csr = 'E000E010'|int(base=16) %}
{% set syst_rvr = 'E000E014'|int(base=16) %}
{% set syst_cvr = 'E000E018'|int(base=16) %}
@ [AMR-ARM] Table B3-4
{% set icsr = 'E000ED04'|int(base=16) %}
@ [ARM-ARM] B3.2.4
{% set clear_systick_pending_value = 2 ** 25 %}

{% set registers = [
    syst_csr,
    syst_rvr,
    syst_cvr,
] %}

{% set adds_between_ticking_start_and_load = 8 if code == "flash" else 4 %}

{% set systick_cycles_range = dict(flash=range(2, 46), sram=range(2, 26), gpram=range(2, 16))[code] %}
{% set adds_after_load = 20 if code == "flash" else 10 %}

{% set sysbus_addresses = {
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}

@ Writing 0 to EVFLAGS should have no effect
{% set addr_1 = {"flash": "flash_chunk", "sram": "sram_chunk", "gpram": "gpram_chunk", "gpio": sysbus_addresses["GPIO::EVFLAGS"]}[data] %}
{% set second_addr_offset = 0 if data == "gpio" else 4 %}

{% set pad_pos, pad_instr = pad %}

{% block code %}
    @ Save all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r1, [r1]
        str.w r1, [r0, #{{offset}}]
    {% endfor %}

    @ Zero all registers
    {% for reg in registers %}
        ldr.w r0, ={{reg}}
        mov.w r1, #0
        str.w r1, [r0]
    {% endfor %}

    {{setExceptionHandler(15, "SysTick_Handler", r0, r1)}}

    @ Save previous value of VTOR
    ldr.w r1, ={{NVIC.VTOR}}
    ldr.w r2, [r1]
    ldr.w r3, =prev_vtor_value
    str.w r2, [r3]

    @ Set VTOR to a vector table in flash
    ldr.w r2, =flash_vector_table
    str.w r2, [r1]

    @ Prepare registers for tests
    ldr.w r0, dwt

    b.w tested_code

.thumb_func
end_label:
    @ Restore VTOR value
    ldr.w r3, =prev_vtor_value
    ldr.w r2, [r3]
    ldr.w r1, ={{NVIC.VTOR}}
    str.w r2, [r1]

    @ Restore all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r2, [r0, #{{offset}}]
        str.w r2, [r1]
    {% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
tested_code:
{% for align_pad in ("", "nop.w", "nop.n") %}
{% for cycles in systick_cycles_range %}

    mov.w r1, #{{cycles}}

    bl.w initialize

    @ Align and clear PIQ
    .align 3
    adr.w r12, .
    {{ 'isb.w' }} @ not-a-test

    @ Start ticking
    str.w r3, [r10]

    @ this should be repeatable
    @ Early read CYCCNT value
    ldr.w r1, [r0, {{CYCCNT}}]
    {{ assert_aligned(2) }}
    ldr.n r3, [r0, {{LSUCNT}}]
    ldr.n r7, [r0, {{CPICNT}}]

    mov.n r8, r7 @ CPI
    mov.n r9, r3 @ LSU

    @ Give time to process interrupt
{% for _ in range(adds_between_ticking_start_and_load) %}
    adds.n r5, #1
{% endfor %}

    @ start the main test
    {{ assert_aligned(3) }}
    {{ align_pad }}
    isb.w

{% if pad_pos == 0 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

    @ Read start CYCCNT value
    ldr.w r1, [r0, {{CYCCNT}}]
    @ The first ldr cannot pipeline anyway and it have to loose arbitration with Fetch at the beginning
    @ XXX: did I saw anywhere than 0-offset strs are faster?
    str.{{width}} r6, [r4, 0]
    @ The second ldr comes too late, but still has to win with next Fetch
    str.{{width}} r7, [r2, 0]

{% if pad_pos == 1 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

    @ Read end CYCCNT value
    ldr.w r2, [r0, {{CYCCNT}}]

@ Do some more adds in case the interrupt comes after the load
{% if pad_pos == 2 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

    @ Save number of cycles until ldm done
    sub.w r2, r1
    bl save
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}

    b.w end_label

save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r6, r7)}}


    ldr.w r6, [r4, 0]
    @ XXX: this is code management of constants  here
    ldr.w r7, [r4, {{ second_addr_offset }}]
    {{saveValue('first_write_duped', r6, r2, r3)}}
    {{saveValue('second_write_duped', r7, r2, r3)}}
    bx.n lr

initialize:
    ldr.w r10, ={{syst_csr}}
    ldr.w r8, ={{syst_cvr}}
    ldr.w r7, ={{syst_rvr}}

    @ Clear CSR
    mov.w r3, #0
    str.w r3, [r10]

    @ Write cycle to reload
    str.w r1, [r7]

    @ Clear current counter
    str.w r1, [r8]

    @ Prepare address for ldm
    ldr.w r4, ={{ addr_1 }}
    ldr.w r2, ={{ addr_1 }} + {{second_addr_offset }} @ r2 is stacked

{% if data in ('sram', 'gpram') %}
    @ restore original values
    ldr.w r3, [r4, 8]
    str.w r3, [r4]
    ldr.w r3, [r2, 8]
    str.w r3, [r2]
{% endif %}

    @ Prepare register for adds
    mov.w r5, #0

    @ Clear destination registers
    @ EVFLAGS is 0, otherwise we won't see its result
    mov.w r6, 0x42
    mov.w r7, 0x42

    mov.w r11, #1

    @ Prepare register to enable ticking and pend interrupt
    mov.w r3, #3

    bx.n lr

.align 4
.thumb_func
.type	SysTick_Handler, %function
SysTick_Handler:
    @ Read interrupt CYCCNT, CPICNT and LSUCNT to low registers to not stall if possible
    {{ assert_aligned(3) }}
    ldr.n r2, [r0, {{CYCCNT}}]
    ldr.n r3, [r0, {{CPICNT}}] @ this won't be stalled further than being first
    ldr.w r11, [r0, {{LSUCNT}}] @ we need a whole word anyway

    @ Save number of cycles until interrupt entered
    sub.w r8, r3, r8 @ CPICNT
    sub.w r9, r11, r9 @ LSUCNT
    sub.w r11, r2, r1 @ CYCCNT
    and.w r8, 0xff
    and.w r9, 0xff

    {{saveValue('cyc_at_int_entry', r11, r2, r3)}}
    {{saveValue('cpi_at_int_entry', r8, r2, r3)}}
    {{saveValue('lsu_at_int_entry', r9, r2, r3)}}

    @ Save number of ADDs that have been executed before interrupt
    {{saveValue('pad_adds', r5, r2, r3)}}

    ldr.w r6, [r4, 0]
    ldr.w r7, [r4, {{ second_addr_offset }}]
    {{saveValue('first_wrote', r6, r2, r3)}}
    {{saveValue('second_wrote', r7, r2, r3)}}

    @ Our code runs on Process stack - get return addr
    mrs r8, PSP
    ldr.w r8, [r8, 0x18]
    {{saveValue('return_addr_raw', r8, r2, r3)}}
    sub.w r8, r12 @ start addr
    {{saveValue('return_addr_off', r8, r2, r3)}}

    @ Mutate r4 hahaha, to load something else
@    add.w r4, 8

    @ Clear CSR
    ldr.w r3, ={{syst_csr}}
    mov.w r2, #0
    str.w r2, [r3]

    @ Clear pending interrupt if necessary
    ldr.w r2, ={{icsr}}
    mov.w r3, #{{clear_systick_pending_value}}
    str.w r3, [r2]

{% if data in ('sram', 'gpram') %}
    @ Override memory to notice retransmission
    mov.w r11, 0x1337
    str.w r11, [r4, 0]
    str.w r11, [r4, {{ second_addr_offset }} ]
{% endif %}

    mov.w r11, #0
    @ Change stored values to observe source overriding
    mov.w r6, 0x456
    mov.w r7, 0x123
    bx.n lr

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

{{ section('sram') }}
.align 3
regs:
.rept 3
    .word 0x0
.endr

prev_vtor_value: .word 0

{{ section('sram') }}
.align 2
.word 0
sram_chunk:
.word 0xd00de00f
.word 0x900d1337
.word 0xd00de00f
.word 0x900d1337
.word 0x1337dead
.word 0xd00dad41

{{ section("flash") }}
.align 3
flash_chunk:
.word 0xfaceb00c
.word 0x90091317
.word 0xdeafbead
.word 0x7f7f7f7f

{{ section("gpram") }}
.align 3
gpram_chunk:
.word 0xee113355
.word 0x12345678
.word 0xee113355
.word 0x12345678
.word 0x90abcdef
.word 0x7f7f7f7f

{{ section(code) }}
.align 9
flash_vector_table:
.word _estack
.word ResetISR + 1
.word NMIFaultHandler + 1
.word HardFaultISR + 1
.word MPUFaultHandler + 1
.word BusFaultHandler + 1
.word UsageFaultHandler + 1
.word 0
.word 0
.word 0
.word 0
.word SVCallFaultHandler + 1
.word DebugFaultHandler + 1
.word 0
.word PendSVISR + 1
.word SysTick_Handler + 1
.rept 34
.word 0
.endr

{% endblock %}
