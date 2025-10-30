# vim:ft=arm
#ifndef CODE
@ This may be import-configured
#define CODE .data.exec
#endif
#include "asm_test_prelude.asm"

#define VIMS_BASE 0x40034000
#define VIMS_O_CTL 0x4
#define VIMS_O_STAT 0x0
#define VIMS_STAT_MODE_CHANGING  0x8
#define IDCODE_LB_DIS_SHIFT 5
#define LB_EN 0x0
#define LB_DIS (1<<IDCODE_LB_DIS_SHIFT)

.macro switch_lb to=LB_EN
mov.w r5, \to
str.w r5, [r4, #VIMS_O_CTL]

ldm r0, {r6-r12}
bl spin_wait

ldr.w r5, [r4, #VIMS_O_STAT]
cmp r5, \to
bne err
.endm

@ Executing from flash while enabling/disabling seems to be allowed, but tricky for emu.
.section CODE, "xa" @progbits
.global _start
.thumb_func
_start:
movs r0, 0

ldr.w r4, =VIMS_BASE
ldr.w r1, [r0]

@ Enable the line buffer
switch_lb LB_EN

@ Disable the line buffer
switch_lb LB_DIS

@ Enable again in case the previous was already enabled
switch_lb LB_EN

b _exit

.thumb_func
spin_wait:
ldr.w r2, [r4, #VIMS_O_STAT]
tst   r2, VIMS_STAT_MODE_CHANGING
bne   spin_wait
bx    lr

.ltorg

.global err
.thumb_func
err:
ldr r0, =PANIK_ADDR
str r2, [r0]
b   .
