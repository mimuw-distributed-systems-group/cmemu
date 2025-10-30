# vim:ft=arm
#ifndef CODE
#define CODE .data.exec
#endif
#ifndef WAITED
#define WAITED 1
#endif
#include "asm_test_prelude.asm"

#define VIMS_BASE 0x40034000
#define VIMS_O_CTL 0x4
#define VIMS_O_STAT 0x0
#define VIMS_STAT_MODE_CHANGING  0x8
#define CACHE_EN 0x1
#define CACHE_DIS 0x3


.section .text.exec, "xa" @progbits
@ Some non-linear flow to trigger cache edge-cases
.thumb_func
jumper:
a:
b.w b

c:
b.w d
b.w a

b:
b.w c

d:
bx lr

@ Executing from flash while enabling/disabling seems to be allowed, but tricky for emu.
.section CODE, "xa" @progbits
.global _start
.thumb_func
_start:
ldr.w r4, =VIMS_BASE
ldr.w r1, [r0]

@ Enable the cache
mov.w r5, CACHE_EN
str.w r5, [r4, #VIMS_O_CTL]

#if WAITED
bl spin_wait
bl jumper
#else
bl jumper
bl spin_wait
#endif

ldr.w r5, [r4, #VIMS_O_CTL]
cmp r5, CACHE_EN
bne err

@ Disable cache
mov.w r6, CACHE_DIS
str.w r6, [r4, #VIMS_O_CTL]

#if WAITED
bl spin_wait
bl jumper
#else
bl jumper
bl spin_wait
#endif

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
