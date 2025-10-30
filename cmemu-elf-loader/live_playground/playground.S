# vim:ft=arm
.cpu cortex-m3
.align	1
.syntax unified
.thumb
.fpu softvfp

#include "semihosting.h"
#define CYCCNT 0xe0001004
#define GPIO_DIN 0x400220c0
#define GPIO_EV 0x400220e0
#define GPIO_EV_UNBUF 0x600220e0
#define GPIO_DOE 0x400220d0
#define PRCM_BASE 0x40082000
#define VIMS_CTL 0x40034004
#define CACHE_EN 0x1
#define CACHE_DIS 0x0

#define BIT_BAND_PERIPH_BASE 0x42000000
#define PERIPH_MEM_BASE 0x40000000
#define BIT_BANDED_BIT 2
#define BIT_BAND_PERIPH_ADDR(addr, bit) (BIT_BAND_PERIPH_BASE + ((addr - PERIPH_MEM_BASE) * 32) + (bit * 4))

#define GPIO_EV_BITBANDED BIT_BAND_PERIPH_ADDR(0x400220e0, BIT_BANDED_BIT)

#define BIT_BAND_SRAM_BASE 0x22000000
#define SRAM_MEM_BASE 0x20000000
#define BIT_BAND_SRAM_ADDR(addr, bit) (BIT_BAND_SRAM_BASE + ((addr - SRAM_MEM_BASE) * 32) + (bit * 4))

#define OFFSET_SRAM_MEM 0x600
#define SRAM_MEM 0x20000600
@ XXX: inlined sram_mem address, bcoz linker cannot do such offset
@#define SRAM_BITBANDED sram_mem+0x02000000
#define SRAM_BITBANDED BIT_BAND_SRAM_ADDR(SRAM_MEM, 1)

.align 3
.weak _start
.thumb_func
_start:
ldr.w r0, =DWT_CTRL
ldr r5, =0x40000001
str r5, [r0]
mov r5, 0
b main


@ Use GPRAM or comment-out
@.section .gpram.exec, "wxa" @progbits
.section .text.exec, "xa" @progbits
.global main
.thumb_func
main:
.align 2
@ self-refferential address
ldr.w r2, =sram_2000_2000
@ measure time
ldr.w r0, =CYCCNT
ldr.w r1, [r0]

@@@ Do something funny, like reading half-aligned self-referential addresses over and over again
.irp offset, 1,2,3,4,5,6,7,11,
ldr.w r2, [r2, #\offset * 2]
.endr
@@@

ldr.w r2, [r0]
sub.w r2, r1

.global _exit
.thumb_func
_exit:
@movw r0, #:lower16:EXIT_ADDR
@movt r0, #:upper16:EXIT_ADDR
ldr r0, =EXIT_ADDR
str r2, [r0]

.global emulator_main
emulator_main:
b.w .
.word sram_start_hooray
.word sram_2000_2000
.word sram_2000_4000

.ltorg

.section .bigalignbss.aaaa, "wa" @progbits
.align 20
sram_start_hooray: .word 0


.section .data.aaaa, "wa" @progbits
.align 13
@ An address that can be read with any even shift and get a valid value back!
sram_2000_2000: .word 0x20002000
.rept 16
.short 0x2000
.endr

.section .heap.aaaa, "wa" @progbits
.align 14
sram_2000_4000: .word 0xD00DBEAD

.section .data.exec, "wa" @progbits
@.section .gpram.data, "wxa" @progbits
.align 2
x_sram_mem: .rept 16; .byte 42; .endr
x_sram_mem2: .rept 16; .byte 11; .endr
