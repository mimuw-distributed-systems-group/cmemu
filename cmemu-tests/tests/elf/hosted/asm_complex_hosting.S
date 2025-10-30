# vim:ft=arm
.cpu cortex-m3
.align	1
.syntax unified
.thumb
.fpu softvfp

#include "semihosting.h"

.section .gpram.exec, "wxa" @progbits

.global main
.thumb_func
main:
.weak _start
.thumb_func
_start:
nop.n

mov r0, 'a'
@ldr r5, =UNIX_SEC
@ldr r0, [r5, 0]
@ldr r5, =STDOUT_ADDR
@str r0, [r5, 0]

bl stack_test
ldr r4, =message

@ uncomment to read from stdin
@ bl load_user_msg
@ ldr r4, =user_msg

bl print_r4_16B


b.w _exit

movw r0, #:lower16:loop
movt r0, #:upper16:loop
blx r0

@ TODO: allow running without VTOR?

.global _exit
.thumb_func
_exit:

movw r0, #:lower16:EXIT_ADDR
movt r0, #:upper16:EXIT_ADDR
mov r1, 42
str r1, [r0]
b.w .


print_r4:
ldr r5, =STDOUT_ALIAS
print_loop:
ldrb r0, [r4], +1
cbz r0, end_message
strb r0, [r5]
b print_loop
end_message:
bx lr

print_r4_16B:
ldr r5, =STDOUT_ALIAS
print_loop2:
ldm r4!, {r0, r1, r2, r3}
stm r5!, {r0, r1, r2, r3}
teq r3, 0
bne print_loop2
end_message2:
bx lr

load_user_msg:
ldr r5, =user_msg
ldr r4, =STDIN_ADDR
load_loop:
ldrb r0, [r4]
cmp r0, #0xa @ newlinw
beq end_load
strb r0, [r5], +1
b load_loop
end_load:
mov r0, 0
strb r0, [r5], +1
bx lr

stack_test:
push {r4, r5, lr}
pop {r4, r5, pc}


@.string "AAAA"
.align 3
message: .string "Hello from asm! This is a long message!"
.align 2, 0x20
.space 16, 0


.align 3
.thumb_func
loop:
movw r1, 0x1000
movt r1, 0x0000
movw r2, 0x1111
movt r2, 0x1111
udiv r3, r2, r1
tst.n r3, r3
bne loop
bx lr



.section "data.eeek", "wa" @progbits
.align 3
user_msg: .string "This is a placeholder"
.space 16, 0
