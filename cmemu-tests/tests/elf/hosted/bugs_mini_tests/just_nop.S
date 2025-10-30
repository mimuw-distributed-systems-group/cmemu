# vim:ft=arm
#define CODE .data.exec
#include "asm_test_prelude.asm"

.global _start
.thumb_func
_start:
nop.n
b _exit
