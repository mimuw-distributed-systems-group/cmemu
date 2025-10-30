# Hosted tests

Tests located under `tests/elf/hosted` are self-contained with sources,
and depend only on cmemu build files (`cmemu-elf-loader/cmemu-target`)
and a cross-compiler (`arm-none-eabi-gcc`).
If you pass `CMEMU__LIVE_TESTS=1` environment variable, `cargo test` will
even automatically regenerate the binaries by running `make` for you.

Note: if you want to interactively play with C/ASM programs and have them
rebuilt when you do `cargo run`, head over to `cmemu-elf-loader/live_playground`.

A *cmemu hosted* applications is built solely for execution with cmemu,
and looks like a typical userspace binary.
CMEmu is acting as an operating system loading the ELF file segments
into the correct memory and provide simple way of interacting with the environment.
**cmemu-hosting** is a simple "system call" implementation using memory-mapped addresses.
For instance, by writing to `0xFEED_0004`, the application may request the emulator to exit.
In particular, this allows running code on the emulator with virtually no startup overhead,
or compiling and executing userspace programs, which use libc for I/O.

The apps here link against default libc (newlib) and are **not usable** on real hardware.
