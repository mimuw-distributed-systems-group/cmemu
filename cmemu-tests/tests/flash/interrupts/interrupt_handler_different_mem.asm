---
name: Interrupt latencies with handler in each memory type
description: >
  Measures interrupt latencies and stacking/unstacking timings with all
  combinations of code and handler memory type.
dumped_symbols:
  results: 4 words
  times:   4 words
  exccnts: 4 words
  lsucnts: 4 words
  cpicnts: 4 words
configurations:
- { code: gpram, handler: gpram, lbEn: True }
- { code: gpram, handler: sram, lbEn: True }
- { code: gpram, handler: flash, lbEn: True }
- { code: gpram, handler: flash, lbEn: False }
- { code: sram, handler: gpram, lbEn: True }
- { code: sram, handler: sram, lbEn: True }
- { code: sram, handler: flash, lbEn: True }
- { code: sram, handler: flash, lbEn: False }
- { code: flash, handler: gpram, lbEn: True }
- { code: flash, handler: sram, lbEn: True }
- { code: flash, handler: flash, lbEn: True }
- { code: flash, handler: gpram, lbEn: False }
- { code: flash, handler: sram, lbEn: False }
- { code: flash, handler: flash, lbEn: False }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ Interrupt #30 has (16 + 30) = 46 exception number.
{% set interrupt_30_exc_number = 46 %}

{% block code %}
    {{setExceptionHandler(interrupt_30_exc_number, "Irq30_Handler", r3, r4)}}
    {{enableException(interrupt_30_exc_number, r3, r4)}}
    {{calculateAddressAndValueForSettingExceptionPending(interrupt_30_exc_number, r5, r4)}}

    @ Prepare all registers for tests
    ldr.w r3, dwt

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Reset SP
    mov.w sp, r11

{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
tested_code:
{% for counter, save_func in [(CYCCNT, "save"), (EXCCNT, "save_exccnts"), (CPICNT, "save_cpicnts"), (LSUCNT, "save_lsucnts")] %}
{% for instr in ["add.w", "adds.n"] %}
    @ Used by interrupt handler to save currently dumped counter
    mov r10, {{counter}}
    @ Init ADD value
    mov.w r7, #0
    @ Clear r9 in case an interrupt won't be called
    mov.w r9, #0
    
    @ Read counter
    ldr.w r0, [r3, r10]
    @ Set pending interrupt
    str.w r5, [r4]

    @ Give processor some time to process interrupt
    .rept 8
        {{instr}} r7, #1
    .endr

    @ Read counter
    ldr.w r8, [r3, r10]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    @ Compute entry latency
    sub.w r0, r6, r0
    @ Compute exit latency
    sub.w r6, r8, r6

    {{saveValue("times", r0, r2, r1)}}
    {{saveValue("times", r6, r2, r1)}}
    {{saveValue("results", r7, r2, r1)}}
    {{saveValue("results", r9, r2, r1)}}

    bx.n lr

save_exccnts:
    @ Compute stacking latency
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ EXCCNT is 8-bit wide
    @ Compute unstacking latency
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("exccnts", r0, r2, r1)}}
    {{saveValue("exccnts", r6, r2, r1)}}

    bx.n lr

save_cpicnts:
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ CPICNT is 8-bit wide
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("cpicnts", r0, r2, r1)}}
    {{saveValue("cpicnts", r6, r2, r1)}}

    bx.n lr

save_lsucnts:
    sub.w r0, r6, r0
    and.w r0, r0, 0xFF  @ LSUCNT is 8-bit wide
    sub.w r6, r8, r6
    and.w r6, r6, 0xFF

    {{saveValue("lsucnts", r0, r2, r1)}}
    {{saveValue("lsucnts", r6, r2, r1)}}

    bx.n lr

{{ section(handler) }}
.align 4
.thumb_func
.type	Irq30_Handler, %function
Irq30_Handler:
    ldr.w r6, [r3, r10]

    @ Save number of ADDs that have been executed before interrupt.
    mov.w r9, r7

    bx.n lr

{% endblock %}
