# Quick start

## Prerequisites

To compile a program to run in the emulator, you will need the `arm-none-eabi` toolchain:
```bash
sudo apt install gcc-arm-none-eabi
```

To run the emulator, you will need Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To build the viewer for cycle-by-cycle execution logs, you will need `npm`:
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
nvm install --lts
```

To use gdb with the emulator, you will need `arm-none-eabi-gdb` or `gdb-multiarch`:
```bash
sudo apt install gdb-arm-none-eabi || sudo apt install gdb-multiarch
```

## Build a program to run in the emulator

### Prepare your code

Your program can be written in C or assembly. Put the code in `cmemu-elf-loader/live_playground`, in a `.c` or `.S` file respectively. C programs can use some libc functions.

Several examples are provided:
- `libc_playground.c` - uses libc functions and places things in specific memory areas
- `playground.S` - accesses control registers and counters and places things in specific memory areas
- `minimal.S` - a minimal template for assembly code

For more examples, see `cmemu-tests/tests/elf/hosted`. In particular, `cmemu-tests/tests/elf/hosted/Makefile` shows how to build code in a different directory.

### Build your code

```bash
cd cmemu-elf-loader/live_playground
make
```

If you make changes to your code in `cmemu-elf-loader/live_playground` after building it, it will be automatically rebuilt when you run the emulator.

## Run a program in the emulator

```bash
cargo run --release -p cmemu -- cmemu-elf-loader/live_playground/libc_playground.elf -- arg1 arg2
```

When the software is a binary image of the microcontroller's Flash (`.flash`):

```bash
cargo run --release -p cmemu -- some.flash
```

## Run a program and visualize a cycle-by-cycle execution log

### Generate the log file

```bash
cargo run --release -p cmemu --features cycle-debug-logger -- --cycle-debug-log-file cdl.json cmemu-elf-loader/live_playground/libc_playground.elf -- arg1 arg2
```

### Build and run the log viewer

```bash
cd cdl-viewer/src
npm install
npm run build

NODE_OPTIONS=--openssl-legacy-provider npm run start
```

Then go to http://localhost:3000/ (it should open automatically) and upload the generated log file (`cdl.json` in this example). Tip: you can click the cycle/address header to jump between non-empty cells in that column/row.

## Generate a highly detailed microarchitectural log

```bash
RUST_LOG=trace cargo run -p cmemu -- cmemu-elf-loader/live_playground/libc_playground.elf -- arg1 arg2 2>log.txt
```
(you can enable different log levels for different modules, e.g. `RUST_LOG=info,cmemu_lib::common::new_ahb=trace,cmemu_lib::component::vims=debug`)

## Use GDB with the emulator

```bash
cargo run --release -p cmemu --features gdb -- cmemu-elf-loader/live_playground/libc_playground.elf --gdb-here
```

For more instructions, see `cmemu-gdb/README.md`.

# Useful dev commands

## Running flash tests

* `cargo run -p cmemu-flash-test -- flash_new::tmp::dev_test_1` or
  `cargo run -p cmemu-flash-test -- tests/flash_new/tmp/dev_test.tzst.1`
  -- as above, but runs given flash test from `*.tzst` archive.

## Style & tests (aka CI)

* For running the checks (all tests, lints, code formatting) consult
  the `./poor-man-ci.sh` script.
  
  ***
  Please, check styling and tests (`./poor-man-ci.sh`) by yourself before commiting changes.
  ***

## Tips & tricks

* Configure logging system with `RUST_LOG` environment variable few of possibile values are: `trace`, `debug`, `info`, `warn`, `error`.

* And use `RUST_BACKTRACE=1` environment variable to allow printing backtraces.

* Nearly all CMemu binaries have `--help` option.
