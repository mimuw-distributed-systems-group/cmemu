# vim:ft=arm

@ This is a glue file, that allows for barebone compilation on CMEmu/CC2650 with or without stdlib.
@ If you need a true dual-binary, use the target's default startup file (just take care about '_exit' and '_stack_init'


.cpu cortex-m3
.align	1
.syntax unified
.thumb
.fpu softvfp

@ Use:
@ LDFLAGS: -nostdlib -nostdin
@ LDFLAGS: -T LDSCRIPT

@ Vector table (by default at 0 in CC26xx)
.section ".vectors"
.global _estack
.word _estack
.word _start
.word NMIISR
.word HardFaultISR
.word MemManageISR
.word BusFaultISR
.word UsageFaultISR
.rept 3
    .word 0
.endr
.word SVCallISR
.word DebugMonitorISR
.word 0
.word PendSVISR
.word SysTickISR
.rept 256
    .word 0
.endr

.section ".text"
@ Functions required by libstd
.global _exit
.weak _exit
.thumb_func
_exit:
.align 3 @ effective line buffer loop
b.w .

@ Stack pointer is the first entry of VTOR
.global _stack_init
.thumb_func
_stack_init:
bx lr


@ Default implementation of handlers
.weak NMIISR
.weak HardFaultISR
.weak MemManageISR
.weak BusFaultISR
.weak UsageFaultISR
.weak SVCallISR
.weak DebugMonitorISR
.weak PendSVISR
.weak SysTickISR

.thumb_func
HardFaultISR:
    b _exit

.set NMIISR, 0
.equ MemManageISR, HardFaultISR
.equ BusFaultISR, HardFaultISR
.equ UsageFaultISR, HardFaultISR
.equ SVCallISR, HardFaultISR
.equ DebugMonitorISR, HardFaultISR
.equ PendSVISR, HardFaultISR
.equ SysTickISR, HardFaultISR
