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
    bl min_max
    @ Fall through

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

.align 4
.thumb_func
.type min_max, %function
min_max:
    @ Save return address
    mov.w r12, lr

    @ Prepare couner address
    ldr.w r10, =CYCCNT_ADDR

    @ Registers:
    @ r12: return address
    @ r10: counter address
    @ r9: expects counter's start value
    @ r8: expects counter's end value

    ldr r0, =0x0
    ldr r1, =0x0
    ldr r3, =sram_cell

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Reset line buffer
    mov.w  r9, #0
    ldr.w  r8, [r9]

    @ Read counter
    ldr.w r9, [r10]

    @ Reload pipeline
    isb.w @ not a test

    @ The example
    .align 4
    adds.w r0, r1 @ r0 = 0x0 + 0x0 = 0x0
    umull.w r1, r0, r0, r1 @ r0, r1 = 0x0 * 0x0 = 0x0, 0x0
    ldr.w r2, [r1], #4  @ r1 = 0x0 (flash) + 4
    str.w r2, [r3] @ r3 = sram_cell (sram)
    beq.w label
    add.w r4, r2
    add.w r5, r2
    add.w r4, r2
    label:
    add.w r4, r5

    @ Read counter
    ldr.w r8, [r10]

    @ Return
    bx.n  r12

.section .data.min_max, "w" @progbits
sram_cell: .space 4
