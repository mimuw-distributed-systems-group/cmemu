# Background

TI CC 26xx family chips contain a ROM area at 0x1000_0000.
It consists mostly of helper functions for interacting with the on-chip devices -- so-called *driverlib*.
Most of these functions are part of the BSD-licenced *drivelib* C library distributed with the chip's SDK.
This allows deciding during compilation, whether to build a function and place it in Flash or use a prebuilt version
from ROM.
In particular, this allows saving a lot of Flash space,
while letting the producer distribute fixed implementation when needed.

Our tests of the processor Core + memory subsystem *do not require* the ROM,
as we skip over hardware initialization in the emulator.
Other tests may attempt to forcefully use in-Flash implementation of the helper functions.
However, some crucial utilities are present only in ROM (and their source is not in the C library) –
this is so-called *ROM Hard-API* (*HAPI*) defined in ``driverlib/rom.h``.
This is required for code that could make Flash memory unstable for accessing instructions:
e.g., changing the high-frequency clock source, or even programming Flash data.
Moreover, there is a built-in bootloader.
TI distributes this ROM image (20K) with the sources as part of their SDK (`rom/driverlib.elf` and `rom/driverlib.c`),
although they are not covered by BSD, but the whole SDK license ("TI Commercial License").

However, it is not the whole ROM image (115K), as it also contains sparsely documented cryptography implementation
(``driverlib/rom_crypto.h``) used by TI Bluetooth Stack.
A code not based on this stack is not accessing that part of ROM,
and we have only one test suite that actually calls these functions:
evaluation asserting we correctly implement their timings located under``flash/benchmarks/rom_crypto/*``.
You need an actual device to extract the whole image.

If you agree with the SDK Terms & Conditions,
a script in this directory will help you download the public ROM files from plethora of copies at GitHub.
If you need the whole ROM, ask the research team or extract it yourself using a debugger connected to your device.
For instance, with OpenOCD connected to CC2650 just run:

```tcl
dump_image brom.bin 0x10000000 117760
```

# TL;DR

There are two ROM versions which we cannot simply distribute here:

- ``driverlib.elf`` from TI SDK – "public" initial 20 KB of ROM
- ``brom.bin`` full on-device 115 KB with extra crypto, seldom needed

Most *flash* tests don't need a ROM file.
Running a "full binary" starts with initialization from ROM.
