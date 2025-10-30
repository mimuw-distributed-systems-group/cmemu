# vim:ft=arm
#ifndef CODE
@ This may be import-configured
#define CODE .text.exec
#endif
#include "asm_test_prelude.asm"

#define PRCM_BASE 0x40082000
#define PRCM_O_CLKLOADCTL 0x28
#define PRCM_O_GPIOCLKGR 0x48
#define PRCM_O_GPIOCLKGDS 0x50
#define PRCM_O_PDCTL0PERIPH 0x138
#define PRCM_O_PDSTAT0PERIPH 0x14c

#define GPIO_BASE 0x40022000
#define GPIO_O_DOE31_0 0xd0

.macro spin_wait off val

spin\@:
ldr.w r5, [r4, #\off]
cmp r5, \val
bne spin\@
.endm

@ Executing from flash while enabling/disabling seems to be allowed, but tricky for emu.
.section CODE, "xa" @progbits
.global _start
.thumb_func
_start:
movs r0, 0
movs r1, 1

ldr.w r4, =PRCM_BASE
ldr.w r7, =GPIO_BASE

@ Enable the Periph PD
str.w r1, [r4, PRCM_O_PDCTL0PERIPH]
spin_wait PRCM_O_PDSTAT0PERIPH 1

@ Enable clock (-s for fun)
str.w r1, [r4, PRCM_O_GPIOCLKGDS]
str.w r1, [r4, PRCM_O_GPIOCLKGR]
str.w r1, [r4, PRCM_O_CLKLOADCTL]
spin_wait PRCM_O_CLKLOADCTL 2

@ Ask it for sth
ldr.w r5, [r7, GPIO_O_DOE31_0]
@ Write which may be stuck in a write buffer
str.w r5, [r7, GPIO_O_DOE31_0]

@ Disable clocks
str.w r0, [r4, PRCM_O_GPIOCLKGR]
str.w r1, [r4, PRCM_O_CLKLOADCTL]
spin_wait PRCM_O_CLKLOADCTL 2
str.w r0, [r4, PRCM_O_GPIOCLKGDS]
str.w r1, [r4, PRCM_O_CLKLOADCTL]
spin_wait PRCM_O_CLKLOADCTL 2

@ Disable PD
str.w r0, [r4, PRCM_O_PDCTL0PERIPH]
spin_wait PRCM_O_PDSTAT0PERIPH 0

b _exit

.ltorg

.global err
.thumb_func
err:
ldr r0, =PANIK_ADDR
str r2, [r0]
b   .
