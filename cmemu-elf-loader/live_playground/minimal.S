# vim:ft=arm
.cpu cortex-m3
.align	1
.syntax unified
.thumb
.fpu softvfp

#include "semihosting.h"

@ Use GPRAM or comment-out
@.section .gpram.exec, "wxa" @progbits

.align 2
.global _start
.thumb_func
_start:
.global _exit
.thumb_func
_exit:
@movw r0, #:lower16:EXIT_ADDR
@movt r0, #:upper16:EXIT_ADDR
ldr.n r0, =EXIT_ADDR
mov.w r1, 42
str.n r1, [r0]

spin:
b.n spin

@ Stack reservation howto
@.section .highram_reserve.scratch, "w"
@.space 123, 2
