# vim:ft=arm
#define CODE .data.exec
#include "asm_test_prelude.asm"

.global _start
.thumb_func
_start:
nop.n
b sram

.section .data.exec, "wxa" @progbits
.thumb_func
sram:
.align 3
    mov.w r2, 0x40000000
    mov.w r1, 0x00030000
    orr.w r2, r1
    mov.w r1, 0x00004000
    orr.w r2, r1 @ r2 == address of VIMS_STAT
    nop.n
    nop.n
    nop.n
    ldr.n r4, sram_addr
    nop.n
    nop.n
    @ RegistrationBuffer was in Registered and got Deny
    ldr.n r3, [r2]
    nop.n
    nop.n
    nop.n
    b.w _exit

.align 3
sram_addr: .word 0x20000000


.ltorg
