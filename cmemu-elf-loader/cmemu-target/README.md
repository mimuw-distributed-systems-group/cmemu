# Build files for executing cross-compiled binaries

This is to check that elfs compiled for ARM-v7M can be executed by the emulator.
In contrast to executing an image compiled for CC2650, there is no operating system boot code.
In particular, this allows for testing certain problematic cases while running the emulator only for a handful of
cycles (i.e., no Benchmark overhead).

You will need to have an ``arm-none-eabi-gcc`` compiler to cross-compile the binaries. I suggest using
the [gcc10 release from ARM](https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2).

Important files in this directory:
<dl>
<dt>Makefile.base</dt><dd>The binaries dependencies are described there. There are ``raw_targets`` that only link to ``cc26xx.S``, ``stdlib_targets`` that links with libc. </dd>
<dt>cc26xx.lds</dt><dd>This is a linker script that tells GCC what sections and segments to use. It is compatible with whip6 linker script, but adds support libc-specific sections.</dd>
<dt>cc26xx.S</dt><dd>This is a minimal glue file required to generate valid ARMv7-M raw binaries.</dd>
<dt>newlib_funs.c</dt><dd>Syscalls implementation for Newlib (libc) that use the memory-mapped cmemu-hosting feature of cmemu.</dd>
</dl>
