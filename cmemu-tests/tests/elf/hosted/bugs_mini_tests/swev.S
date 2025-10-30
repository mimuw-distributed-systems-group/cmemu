# vim:ft=arm
#ifndef CODE
@ This may be import-configured
#define CODE .text.exec
#endif
#include "asm_test_prelude.asm"

#define EVENT_BASE 0x40083000
#define EVENT_O_SWEV 0xf00
@ IRQ for SWEV0
#define EVENT_O_CPUIRQSEL27 0x6c
#define SWEV_IRQ 27

#define NVIC_BASE 0xe000e000
#define NVIC_O_ICER0 0x180
#define NVIC_O_ISPR0 0x200

.section CODE, "xa" @progbits
.global _start
.thumb_func
_start:
movs r0, 0
movs r1, 1

ldr.w r4, =EVENT_BASE
ldr.w r7, =NVIC_BASE

@ Disable the IRQ
movs r2, (1<<SWEV_IRQ)
str.w r2, [r7, NVIC_O_ICER0]
@ Make sure write is done
ldr.w r2, [r7, NVIC_O_ICER0]

@ Trigger SWEV0
str.w r1, [r4, EVENT_O_SWEV]
@ Make sure the store landed
ldr.w r2, [r4, EVENT_O_SWEV]

@ Check if SWEV0 is pending
ldr.w r2, [r7, NVIC_O_ISPR0]
tst.w r2, (1<<SWEV_IRQ)
bne _exit
b err

.ltorg

@ TODO: setup a handler

.global err
.thumb_func
err:
ldr r0, =PANIK_ADDR
str r2, [r0]
b   .
