# CMEmu

**CMEmu** aims to be a cycle-exact emulator of an [ARM Cortex-M3][cortex-m3]-based
microcontroller—[Texas Instruments CC2650][ti-cc2650]—synthesized using solely
in-code timing measurements and publicly available documentation.

CMEmu is under active development. While the ultimate goal is to cycle-exactly
model entire microcontrollers, our initial focus was the emulation of user-space
programs. As such, CMEmu codebase currently offers 3 tiers of support (they are
not expressed directly in the source) for various components of the microcontroller
and emulation configurations:

- **Tier 0**: is a cycle-exact model of Cortex-M3 (rev. r2p1), memories, and memory
buses of TI CC2650 necessary to emulate execution of regular user-space programs.
In particular, it models 110 Thumb instructions (arithmetic-logical operations,
memory accesses and branches), execution from Flash or GPRAM, accessing data in
Flash or GPRAM or SRAM, supports the line buffer and Cache to be on or off
(the in-Core store buffer has to be disabled).

- **Tier 1**: is a cycle-accurate model of selected other features of Cortex-M3
and TI CC2650: interrupts, the in-Core store buffer, sleep modes, RTC, PRCM,
GPIO, etc. These are operational but might not be modeled in full yet.
Moreover, emulation of userspace programs of two other chips: Cortex-M4-based
TI CC2652 (limited to the ARMv-7M architecture) and Cortex-M3-based STM32F100
are currently at this tier.

- **Tier 2**: is an experimental model of selected remaining features of TI CC2650
like RF core. These are under development, so they might be only partially operational.


***
The goal of CMEmu is not to reverse engineer the hardware but to build
abstractions that mimic its timing: a model that given a program outputs its
execution time. CMEmu is derived from empirical observations; it is not
based on hardware sources of the device.
***

Therefore, before you use CMEmu, we recommend you read our [research paper presenting CMEmu][cmemu-paper].
It explains, in particular, how the *Tier 0* was derived and what we claim about
its *cycle-exactness*.


## How to use CMEmu?

See [Quick start](QUICK-START.md) to learn how to run CMEemu.

In short, you can use CMEmu to cycle-exactly emulate user-space programs:
compile a program into an ELF (`.elf`), and supply the ELF to CMEmu
– CMEmu will load its sections to target memories and start the execution
from the `_start` symbol.
You can also use CMEmu to emulate software for the microcontroller: compile the
software with an embedded OS ([Contiki-NG for CherryMotes][contiki-ng-1kt] and
[whip6][whip6] should work out of the box) for TI CC2650, generate a binary image
of the microcontroller's Flash (`.flash`), and supply it to CMEmu – CMEmu will
use the image as Flash, and start the execution as the microcontroller does.

Out of the box you can visualize the execution cycle-by-cycle, obtain a highly
detailed microarchitecture-level log, or interactively inspect the emulated
execution from gdb (it is non-intrusive!).


## What is in the repo?

- `cc2650-constants`: constant values generated from SVD,
- `cdl-viewer`: a tool to visualize the cycle-by-cycle execution log (see `README.md` there),
- `cmemu-codegen`: generates Rust code (see `README.md` there),
- `cmemu-common`: stuff shared between multiple crates,
- `cmemu-elf-loader`: loads ELF (`.elf`) into CMEmu for emulation,
- `cmemu-flash-test-lib`: runs flash tests,
- `cmemu-flash-test`: loads a single flash tests (`.tzst`) into CMEmu,
- `cmemu-gdb`: enables inspecting emulated execution with gdb (see `README.md` there),
- `cmemu-lib`: the CMEmu emulator,
- `cmemu-proc-macros`: macros used in the emulator,
- `cmemu-tests`: tests of the emulator (see `README.md` there),
- `cmemu`: "frontend" of the emulator,
- `log-presets`: logging setting for gdb,
- `pretty_flexi_logger`: custom logger for `flexi_logger`,
- `rom`: downloads image of the TI CC2650's ROM (see `README.md` there).


## Authors

CMEmu has been developed by members of the [*MIMUW Distributed Systems Group*][mimuw-distributed-systems-group]:
Maciej Matraszek,
Artur Jamro,
Wojciech Kordalski,
Michalina Sidor,
Bartek Dalak,
Wojciech Ciszewski,
Marek Puzyna,
Daniel Gutowski,
Piotr Karpiński,
Mateusz Banaszek,
Kamil Mykitiuk,
Adam Czajkowski,
Michał Chojnowski,
Kacper Sołtysiak,
Tymoteusz Wiśniewski,
Antoni Żewierżejew,
and Konrad Iwanicki.

*However, we welcome contributions!*


[cortex-m3]: https://www.arm.com/products/silicon-ip-cpu/cortex-m/cortex-m3
[ti-cc2650]: https://www.ti.com/product/CC2650
[cmemu-paper]: https://mimuw-distributed-systems-group.github.io/cmemu/#paper
[contiki-ng-1kt]: https://github.com/mimuw-distributed-systems-group/contiki-ng-1kt
[whip6]: https://github.com/InviNets/whip6-pub
[mimuw-distributed-systems-group]: https://mimuw-distributed-systems-group.github.io
