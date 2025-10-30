# cmemu-tests

This crate contains end-to-end tests of cmemu.
That is: loading a program, executing it, and verification of the results.

The main test types are:

- **flash tests** – standardized flash image paired with saved parts of memory
  after execution on a *real* device.
  They are generated with CMGen (Toolie/Benchmark) from assembly snippet and
  put extreme focus on results reproducibility.

  Read more about writing these in [the testing guidelines](../TESTS-GUIDELINES.md).
- **elf tests** are free-form, focusing on an end-user experience with cmemu.
  The assertions are customized in code, for provided ELF files,
  which may range from minimal hosted binaries to real-world networking apps.

## ELF Tests Guidelines

All elf tests MUST be compiled with debug information and be documented
how to obtain their sources (for a debugger) and regenerate the binary.

In particular, compile with `-g -ggdb3 -gz`,
which will attach (compressed) verbose debugging information,
without impacting code generation.
If optimization level is unimportant, prefer `-Og`.

It must be clear, what was the target ABI/machine for each binary.
In particular:

- whether it can run natively on a board?
- is the target a simple hosted cmemu / dual-entrypoint / native app?
- is the binary in any way aware of cmemu (e.g., by using cmemu-hosting ABI)?
- does the cmemu entrypoint manage loading RAM segments?
- what cmemu features are required?

### Targets

To elaborate, as mentioned above, applications can be compiled against
distinct *targets*.

A *native application* is compiled for a board (e.g., a CherryMote)
and use (almost) exclusively true hardware interfaces of the chip.
Such tests exercise the ability of cmemu to run full programs.
However, several constant parts are quite lengthy: namely
startup code (configuration of analog devices, etc.) or
using UART for communication (thousands of cycles per character).

A *cmemu hosted* applications is built solely for execution with cmemu,
and looks like a typical userspace binary.
CMEmu is acting as an operating system loading the ELF file segments
into the correct memory and provide simple way of interacting with the environment.
**cmemu-hosting** is a simple "system call" implementation using memory-mapped addresses.
For instance, by writing to `0xFEED_0004`, the application may request the emulator to exit.
In particular, this allows running code on the emulator with virtually no startup overhead,
or compiling and executing userspace programs, which use libc for I/O.

A middle ground approach — **dual-entrypoint** — executes different startup code
on the device and on the emulator, and merges at the *important* fragment.
*Flash tests* are the prime example of this technique:
since we mock most of the special analog registers, we can skip them anyway.
With `cmemu-elf-loader` this trick is typically realized by having different
*elf entrypoint* and *Reset Vector* value:

- The value at reset vector (at `0x0000_0004` for cc2650) are used for the CPU
  as an entry-point.
- Elf entrypoint (inside an ELF header, which is not present on the device) may be different.
  `cmemu-elf-loader`, by default, starts the program from this entrypoint (if it is present),
  instead of the reset vector.

### Hosted Tests

Tests located under `tests/elf/hosted` are self-contained with sources,
and depend only on cmemu build files (`cmemu-elf-loader/cmemu-terget`)
and a cross-compiler (`arm-none-eabi-gcc`).
If you pass `CMEMU__LIVE_TESTS=1` environment variable, `cargo test` will
even automatically regenerate the binaries by running `make` for you.

Note: if you want to interactively play with C/ASM programs and have them
rebuilt when you do `cargo run`, head over to `cmemu-elf-loader/live_playground`.
