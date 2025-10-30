# vim:ft=arm
@ Executing from flash while enabling/disabling seems to be **allowed**, but tricky for cmemu.
#define CODE .text.exec
#define WAITED 0
#include "cache_en.S"
