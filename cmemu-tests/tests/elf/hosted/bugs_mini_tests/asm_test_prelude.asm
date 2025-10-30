# vim:ft=arm
.cpu cortex-m3
.align	1
.syntax unified
.thumb
.fpu softvfp

#include "semihosting.h"
#ifndef CODE
#define CODE .data.exec
#endif

.section CODE.exit, "xa" @progbits
.thumb_func
_exit:
ldr r0, =EXIT_ADDR
mov r1, 42
str r1, [r0]
b .

.ltorg

.section CODE, "xa" @progbits
@ your test, define _start as .thumb_func
