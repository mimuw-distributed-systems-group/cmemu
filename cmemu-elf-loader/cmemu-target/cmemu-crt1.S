# vim:ft=arm
.cpu cortex-m3
.syntax unified
.thumb
.fpu softvfp


@ This is a crt (startup) file for interacting with cmemu hosting, as required by C.
@ Default crt0 is actually missing some stuff we need, so we try to hack into it, by `software_init_hook`.
@ The crt0.o comes from -specs=nosys.specs, so raw targets are free to override _start.
@ TODO: Maybe we can easily replace the crt0 with with defining our own .specs file - do that when someone complains
.section ".text.crt"
#include "semihosting.h"
.global software_init_hook
.thumb_func
software_init_hook:
@ We're prior to calling initializers, after memory setup etc.
@ Register destructors to be called at exit
ldr  r1, .Latexit
cbz  r1, .Lskip_atexit
ldr  r0, .Lfiniarr
blx  r1
.Lskip_atexit:
.global __libc_init_array
bl   __libc_init_array

movs r0, #0
movs r1, #0
ldr  r3, .Lhosting_abi_ver
ldr  r2, [r3], #4
@ Check if ABI is 0 with layout: [abi ver] [argc] [argv] [environ]
cbnz r2, .Lunknown_abi
ldr  r4, .Lenviron
ldm  r3, {r0-r2}  @ argc, argv, environ
str  r2, [r4]

bl   main
bl   exit
.Lunknown_abi:
ldr  r0, .Lpanic
str  r0, [r0]
b    .

.align 2
.global atexit
.weak atexit
.Latexit: .word atexit
.global __libc_fini_array
.weak __libc_fini_array
.Lfiniarr: .word __libc_fini_array

.Lhosting_abi_ver: .word OS_DATA_ABI_VER
@ environ is the libc base addr for getenv & co.
.global environ
.Lenviron: .word environ
.Lpanic: .word PANIK_ADDR
