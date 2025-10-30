---
name: Setting SCB registers during exception handling.
description: >
  Dump System Control Block (SCB) registers values in different steps of interrupt execution with multiple interrupts pended.
dumped_symbols:
  pre_exc_regs: 7 words
  in_exc_regs: 14 words
  post_exc_regs: 7 words
configurations:
- { pending_interrupts: 1 }
- { pending_interrupts: 2 }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ [ARM-ARM]B3.2.2
{% set icsr_addr = 'E000ED04'|int(base=16) %}
@ [ARM-ARM]B3.2.2
{% set shcsr_addr = 'E000ED24'|int(base=16) %}

{% set addrs = [
      icsr_addr,
      shcsr_addr
] %}

{% block code %}
    {{setExceptionHandler(16, "Irq0_Handler", r3, r4)}}
    {{setExceptionHandler(17, "Irq1_Handler", r3, r4)}}

    {{enableException(16, r3, r4)}}
    {{enableException(17, r3, r4)}}

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
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
    @ Init ADD value
    mov.w r7, #0

    @ Prepare register for setting interrupts into pending state
    ldr.w r4, ={{NVIC.ISPR0}}

    @ Save scb registers values
    {% for addr in addrs %}
      ldr.w r7, ={{addr}}
      ldr.w r6, [r7]
      {{saveValue("pre_exc_regs", r6, r1, r2)}}
    {% endfor %}

    @ Set pending interrupt

    mov.w r5, #0b{{ "1" * pending_interrupts }}
    str.w r5, [r4]

    @ Give processor some time to process interrupt
    .rept 16
        add.w r7, #1
    .endr

    {% for addr in addrs %}
      ldr.w r7, ={{addr}}
      ldr.w r6, [r7]
      {{saveValue("post_exc_regs", r6, r1, r2)}}
    {% endfor %}

    b.w end_label

.align 4
.thumb_func
.type	Irq0_Handler, %function
Irq0_Handler:
    @ Save scb registers values
    {% for addr in addrs %}
        ldr.w r7, ={{addr}}
        ldr.w r6, [r7]
        {{saveValue("in_exc_regs", r6, r1, r2)}}
    {% endfor %}

    bx.n lr

.align 4
.thumb_func
.type	Irq1_Handler, %function
Irq1_Handler:
    @ Save scb registers values
    {% for addr in addrs %}
        ldr.w r7, ={{addr}}
        ldr.w r6, [r7]
        {{saveValue("in_exc_regs", r6, r1, r2)}}
    {% endfor %}

    bx.n lr

{% endblock %}
