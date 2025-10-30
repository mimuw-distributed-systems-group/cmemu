# CMEmu-GDB: a debugger server and more

`cmemu-gdb` allows for powerful inspection of the emulated program, the SoC, and CMEmu itself.
It is a gdb (stub) server and allows for workflows just like you would be connecting to an embedded target
through, for instance, OpenOCD.
One notable difference is interaction with the server through the `monitor` command:
we're not emulating OpenOCD here, but expose a general-purpose command line to interact with CMEmu mid-execution.

`cmemu-gdb` is a separate crate from `cmemu` and `cmemu_lib`, therefore all employed interfaces are generic and public.

<div class="warning">
Paths and exact commands are up for change, so if the examples don't work take a while to resolve the current name.
</div>

## Basic (generic) usage

First, you should be aware of two related log targets (for use with `RUST_LOG`:

- `cmemu_gdb=trace` will output single-line summary of request from the gdb client to our server,
- `gdbstub=trace` will output raw communication packers.

### Connecting

To enable `gdb-server` for the main binary, you need to enable the `gdb` feature and use the `--gdb` flag like:
`cargo run --bin cmemu -F gdb -- [binary_path] --gdb [port_or_socket]`. For instance:

```shell
cargo run --bin cmemu -F gdb -- cmemu-elf-loader/test_files/mandelbrot.elf  --gdb 3333
```

will wait for gdb connection on port 3333 before proceeding to the execution:

```text
Waiting for a GDB connection on tcp:127.0.0.1:3333...
You can connect using:
target remote tcp:127.0.0.1:3333

For a quickstart, just run on this machine:
arm-none-eabi-gdb -iex "target remote tcp:127.0.0.1:3333"
```

You can run the gdb client for *arm-none-eabi* target however you like,
but the simplest way is to just copy the suggested command and run it in the same working directory as cmemu.
Notice you don't even need to pass the binary to `arm-none-eabi-gdb`, as it will ask the server for it.

If you use `gdb-multiarch` instead, use
```bash
gdb-multiarch -iex "set architecture armv7"
```
to get rid of a `warning: Architecture rejected target-supplied description` error.

As a convenience, you can use `--gdb-here` to spawn `arm-none-eabi-gdb` (or `gdb-multiarch`) in the same terminal
(as a child of cmemu connected through a temporary unix socket).
There are pros and cons of this approach.
It allows you to rapidly iterate and recompile `cmemu` without much of a hassle,
but on the other hand, the log output would break gdb's TUI layout.
You can also pass arguments to the gdb using `--gdb-here=ARGS`,
just remember about quoting/escaping spaces.
For instance, to automatically start execution, you may pass `--gdb-here='-ex c'`.

### Breakpoints and inspection

Upon connection, the emulator is initialized, but not a single cycle was simulated.
You can use commands like `break`, `step`, `list`, `disass`, `info` to do the usual thing.

Let's continue our example and set up a breakpoint at the main function
(note: `*main` means first **instruction**)
and continue there:

```gdb 
break *main
continue
```

At this point, you might hit the default timeout and see:

```text
Program received signal SIGXCPU, CPU time limit exceeded.
```

You can inspect the program, or just ignore the limit and `c(ontinue)`.
You can hit Ctrl-C at any point to interrupt the program and return to the debugger.
If you have the commands trace enabled, you might have already noticed that gdb removes and re-inserts breakpoints
when giving back control.

```text
 TRACE cmemu_gdb::gdb                        > Breakpoint hit for 000004f8
 TRACE cmemu_gdb::gdb                        > Dumping all registers
 TRACE cmemu_gdb::gdb                        > Removing a breakpoint on 0x4f8 Thumb16
 TRACE cmemu_gdb::gdb                        > Reading memory at 0x4f8/4b
 
Breakpoint 1, main (k=0) at mandelbrot.c:1
```

You can inspect the program at this point (for instance with `list`), or continue to the next source line with `s(tep)`.
You can freely inspect the memory, for instance,
see the local variables `info locals` or registers with `info registers`:

```gdb
(gdb) print k
$1 = 1
(gdb) info locals
i = 4.38000011
j = -0.368399829
r = -1.96000004
x = 1
y = -15
(gdb) info reg
r0             0x0                 0
r1             0x41300000          1093664768
r2             0x82                130
r3             0x0                 0
r4             0xbebc9ee7          3200032487
r5             0x408c28f6          1082927350
r6             0x3e0af9c6          1040906694
r7             0x1                 1
r8             0x3f800000          1065353216
r9             0xc1700000          3245342720
r10            0xbffae148          3220889928
r11            0x0                 0
r12            0xc3351f36          3275038518
sp             0x20004fe0          0x20004fe0
lr             0xc7f               3199
pc             0x5aa               0x5aa <main+178>
xPSR           0x21000000          [ C T exc=0 ]
msp            0x20004fe0          0x20004fe0
psp            0x0                 0x0 <_text>
ITSTATE        0x0                 0
primask        0x0                 [ prio=not set ]
basepri        0x0                 0
faultmask      0x0                 [ prio=not set ]
control        0x0                 [ nPRIV=priv SPSEL=MSP ]
(gdb) x/16x $sp
0x20004fe0:	0x00000000	0x00000000	0x00000000	0x00000000
0x20004ff0:	0x00000000	0x00000000	0x00000000	0x000004d3
0x20005000:	Cannot access memory at address 0x20005000
```

### Watchpoints and stepping

Continuing with our running example, let's continue until the `_write` function implementing a stub syscall.

```gdb
(gdb) b _write
Breakpoint 2 at 0x6a4: file newlib_funs.c, line 74.
(gdb) c
Continuing.

Breakpoint 2, _write (file=1, ptr=0x200008b8 ":", len=1) at newlib_funs.c:74
74	int _write(int file, char *ptr, int len) {
(gdb) bt
#0  _write (file=1, ptr=0x200008b8 ":", len=1) at newlib_funs.c:74
#1  0x00001b64 in _write_r ()
#2  0x00000dca in __sflush_r ()
#3  0x00000e54 in _fflush_r ()
#4  0x000012fe in _fwalk_reent ()
#5  0x000005bc in main (k=<optimized out>) at mandelbrot.c:2
(gdb) info frame
Stack level 0, frame at 0x20004f88:
 pc = 0x6a4 in _write (newlib_funs.c:74); saved pc = 0x1b64
 called by frame at 0x20004f98
 source language c.
 Arglist at 0x20004f88, args: file=1, ptr=0x200008b8 ":", len=1
 Locals at 0x20004f88, Previous frame's sp is 0x20004f88
(gdb)
```

Remember, you can always display the assembly of the current function with `disass(embly)` or use `layout asm`.
You can use `step-instruction` (`si`) to step over a single instruction.
When doing so, it is useful to automatically display the current instruction with `display/i $pc`.
Let's single-step the program until it does a store to memory (hint: Enter just repeats the last command):

```gdb
(gdb) si
0x000006c0	91	        *dest_alias++ = *ptr++;
1: x/i $pc
=> 0x6c0 <_write+28>:	strb.w	lr, [r3], #1
```

We can check the destination address with:

```
(gdb) info reg r3
r3             0xfeed1000          4276948992
```

however, you won't be able to access the special memory-mapped register right now:

```gdb
(gdb) p *$r3
Cannot access memory at address 0xfeed1000
(gdb) set *(char*)$r3 = 'a'
Cannot access memory at address 0xfeed1000
```

this is because the debugger is in the **non-invasive** mode.
The mode is designed to not require cooperation from fragile modules.
As such, you can only read and write *ordinary* memory
(without affecting caches; as exposed through the emulator API),
and cannot change the registers.

**Watchpoints** can be used to set "a breakpoint" on memory modification / access by the Load-Store Unit.
(You need `cmemu_lib` with `"cycle-debug-logger"` feature which should be turned on by default.)
Let's return to the entry of `_write`, remove this breakpoint, and set a watchpoint at the argument location:

```gdb
(gdb) c # continue to have a clear value of ptr
Continuing.

Breakpoint 2, _write (file=1, ptr=0x200008b8 ".", len=1) at newlib_funs.c:74
74	int _write(int file, char *ptr, int len) {
(gdb) watch -l *ptr
Hardware watchpoint 3: -location *ptr
(gdb) del 2
(gdb) c
Continuing.
^C
Program received signal SIGINT, Interrupt.
```

You might have noticed in the trace console that the watchpoint was hit numerous times.
As it turns out, gdb continues the execution if the value didn't change (or it **cannot** access that memory).
If that's not what you want, there are two options:

- Set an access watchpoint with `awatch` or an extra read watchpoint (`rwatch`).
- Configure the server to report a trap instead of write watchpoints by running `monitor write-watchpoint-trap`.
  You can call `monitor lsu-request` to check what access triggered the *trap*.

This allows us to set a write watchpoint at any address.
If you want to watch a range of addresses, you need to tell GDB to watch a phony array
(this is a single range in the backend).

```gdb
(gdb) monitor write-watchpoint-trap
(gdb) del 3
(gdb) watch -l *(uint8_t[0x2000]*) 0xfeed1000 # our "semi_hosting" range
Hardware watchpoint 4: -location *(uint8_t[0x2000]*) 0xfeed1000
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
0x000006c0 in _write (file=1, ptr=0x200008b9 "", len=1) at newlib_funs.c:91
91	        *dest_alias++ = *ptr++;
(gdb) monitor lsu-request
Some(
    [Single Write 1 @ feed1000 prot=DPBC],
)
```

## Advanced (cmemu-specific) usage

As our running example, we will be investigating a panic in the emulator.
We can reliably trigger it using the "semi_hosting" API exercised by `cmemu-elf-loader/test_files/panic.elf`.
Catching a panic requires that cmemu is compiled with the `panic=abort` feature (e.g., choose `--profile test`).

### The monitor

The part actually managing the execution and interaction with the program under test is called a **debug monitor**.
GDB has a free-form command for interacting with the monitor through the *gdb stub server*: `monitor` (just `mo` works).
The monitor command in `cmemu-gdb` has a clap-based interface.
(Hint: just like in gdb, you can use only a prefix of a command name.)
You can see the help by executing:

```gdb
(gdb) monitor help
CMemu debug monitor interface
[..]

Usage: monitor <COMMAND>

Commands:
  guest                  Commands related to the simulation (guest) [aliases: g]
  [...]
  help                   Print this message or the help of the given subcommand(s)
```

If you need to check the current status of cmemu, your one-stop-shop is `monitor status`:

```gdb
(gdb) mo status
cmemu-lib built with CDL: true
cmemu-gdb built with CDL: true
watchpoints supported: true
[...]
Timeout cycles: 133
Virtual time: ~260ns
```

### Timeout

As mentioned above, `cmemu-gdb` respects the timeout option of the `cmemu` binary.
It will report the `SIGXCPU` signal, *"CPU time limit exceeded"*.
You can use `monitor guest` subcommands to manage it:

```gdb
(gdb) monitor guest cycle-time
13
(gdb) monitor guest virtual-time
Timepoint { picoseconds: 260412 }
Roughly 260ns
(gdb) monitor guest timeout # also print the current value
Current timeout: Some(133)
(pass a value to change)
(gdb) monitor guest timeout 500
Current timeout: Some(133)
(gdb) monitor g timeout 0 # disable the timeout
Current timeout: Some(500)
(gdb) mo g timeout
Current timeout: None
(pass a value to change)
```

### Catching a panic

Let's `c(ontinue)` with the execution till the emulator panics:

```gdb
(gdb) c
Continuing.

Program received signal SIGKILL, Killed.
0x00000626 in abort () at newlib_funs.c:131
131	    HWREG(PANIK_ADDR) = 222;

```

In the cmemu console, you will also see the regular backtrace and hints from `cmemu-gdb`.
You are now in **post-mortem** analysis mode.
The emulator panicked while simulating a cycle, and you cannot "recover," but we may still investigate it:

```gdb
(gdb) mo cycle
553
(gdb) disass
Dump of assembler code for function abort:
   0x00000620 <+0>:	push	{r3, lr}
   0x00000622 <+2>:	movs	r0, #222	; 0xde
   0x00000624 <+4>:	ldr	r3, [pc, #4]	; (0x62c <abort+12>)
=> 0x00000626 <+6>:	str	r0, [r3, #0]
   0x00000628 <+8>:	bl	0x614 <_exit>
   0x0000062c <+12>:	movs	r0, r0
   0x0000062e <+14>:	cdp2	0, 14, cr15, cr13, cr15, {2}
End of assembler dump.
(gdb) p/x $r3
$1 = 0xfeed0000
(gdb) info frame
Stack level 0, frame at 0x20004ff8:
 pc = 0x626 in abort (newlib_funs.c:131); saved pc = 0x508
 called by frame at 0x20005000
 source language c.
 Arglist at 0x20004ff0, args: 
 Locals at 0x20004ff0, Previous frame's sp is 0x20004ff8
 Saved registers:
  r3 at 0x20004ff0, lr at 0x20004ff4
(gdb) backtrace
#0  0x00000626 in abort () at newlib_funs.c:131
#1  0x00000508 in main () at panic.c:6
```

You have several options to proceed:

- attempting to resume the execution or quitting the debugger will terminate cmemu normally (albeit with an error),
- you can re-throw the panic to continue unwinding with `monitor cmemu resume-unwind`,
- note the cycle number (`monitor cycle`) and rerun to [investigate](#Cycle-stepping)
  – you may try `monitor cmemu attempt-reset` but starting a new process is the most reproducible,
- attach a debugger to **cmemu binary**:

#### Debugging cmemu itself

`monitor cmemu` provides several subcommands useful to locate the emulator in the operating system like
`monitor cmemu pid` or `monitor cmemu path`.
This information should be enough to attach a debugger to the cmemu process.
However, on most Linux distributions attaching a debugger to an unrelated process is restricted.
Is some settings, a process can explicitly allow another process (or any process) to act as a debugger.
This operation is triggered with `monitor cmemu debug-me`.
Read the command's help message for details.

```gdb
(gdb) mo cmemu pid
2868406
(gdb) mo cmemu emulator-exe
[...]/cmemu-framework/target/debug/cmemu
(gdb) mo cmemu debug-me
Attach to: 2868406
NOTE: don't use the debugger of the guest while halted, as it will time out!
```

### Cycle-stepping

While the gdb remote protocol has a concept of [stepping by a clock cycle][cycle-step-packet],
there is no native support in the UI.
Our trick is to switch the behavior of `step-instruction` to step cycles instead with: `monitor step-cycle`.
Breakpoints on a cycle number may be set with `monitor break-cycle`.

[cycle-step-packet]: https://sourceware.org/gdb/current/onlinedocs/gdb.html/Packets.html#cycle-step-packet

We can continue investigation of our running example after restarting the emulator:

```gdb
(gdb) monitor break-cycle 545  # earlier than the panic
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
abort () at newlib_funs.c:130
130	void __attribute__((noreturn)) abort() {
(gdb) mo cycle
545
(gdb) mo step-cycle  # si => step-cycle
(gdb) si
130	void __attribute__((noreturn)) abort() {
(gdb) x/i $pc
=> 0x620 <abort>:	push	{r3, lr}
(gdb) si
abort () at newlib_funs.c:130
130	void __attribute__((noreturn)) abort() {
(gdb) # Enter
130	void __attribute__((noreturn)) abort() {
(gdb) 
abort () at newlib_funs.c:131
131	    HWREG(PANIK_ADDR) = 222;
(gdb) x/i $pc
=> 0x622 <abort+2>:	movs	r0, #222	; 0xde
(gdb) mo cycle
549
(gdb) mo step-cycle off  # turn back to step-instruction
```

Some inspection commands are particularly useful when coupled with cycle-stepping:
`monitor agu`, `monitor lsu-request`, etc.

### Managing CDL

The `monitor cdl` subcommand allows you to manage the recording and dumping of Cycle Debug Logger data.
For example, you can record and dump only a short fragment of the execution.

```gdb
(gdb) mo step-cycle 
(gdb) mo cdl set-file /tmp/cdl.json
(gdb) mo cdl start
(gdb) si 10  # step N=10 cycles
(gdb) mo cdl stop
(gdb) mo cdl dump
Dumped to /tmp/cdl.json
(gdb) mo cdl unset  # clear the file path (cdl will dump on exit)
```

### Managing the logger

An ability to dynamically select the logging output allows executing millions of cycles
before turning on detailed microarchitectural logs.
The log specification is managed by `monitor log` subcommands.
The logging backend, flexi-logger, supports pushing temporary specifications and building new ones.
A log specification can come from:

- a string (`monitor log push/add spec [SPEC]`) with a format like the `RUST_LOG` variable,
- the actual `RUST_LOG` variable: `... env`,
- a toml file (`... path [FILE]`) -- see `cmemu-framework/log-presets/` for examples,
- or be an empty one: `monitor log push clear`.

The interface enables us to tailor the log verbosity to our needs, for instance:

```gdb
(gdb) mo log push env  # push temporary specification from RUST_LOG onto the stack
(gdb) mo log add file log-presets/core-trace.toml  # add rules from the file
(gdb) si
# The log was a bit too verbbose
(gdb) mo log show  # full debug display of the builder
Some(LogSpecBuilder { module_filters: {None: Info, Some("cmemu_gdb"): Trace, Some("cmemu_lib::component::core"): Trace, Some("cmemu_lib::component::quartz"): Trace, Some("gdbstub"): Debug, Some("cmemu_lib::engine"): Debug} })
(gdb) mo log show-spec  # a string that can be passed to RUST_LOG/add spec
info, cmemu_lib::component::quartz = trace, cmemu_lib::component::core = trace, cmemu_lib::engine = debug, cmemu_gdb = trace, gdbstub = debug
(gdb) mo log dup  # push a copy of the logger on the stack and narrow some rules
(gdb) mo log add spec cmemu_lib::component::core::lsu::ahb_trace=debug,cmemu_lib::component::core::fetch::transfers::ahb_trace=debug
(gdb) si  # yeah, that just right!
(gdb) mo log show-toml  # you can save this somewhere
global_level = 'info'
# [...]

[modules]
'cmemu_lib::component::core::fetch::transfers::ahb_trace' = 'debug'
'cmemu_lib::component::core::lsu::ahb_trace' = 'debug'
'cmemu_lib::component::quartz' = 'trace'
# [...]
(gdb) mo log pop  # pop from the spec stack
(gdb) mo log pop  # back to the original logger spec
```

Note: the builder (`... add ...`) is only available to modify the recently pushed specification.

### Using presets

In particular, storing the logger configuration interacts well with scripting the GDB itself:
you can use the `-x FILE` GDB argument to execute GDB commands before dropping to its shell.
For instance, you can use a [script to halt and enable verbose logs](gdb-presets/contiki_test-verbose_sleep)
after a Contiki application enters sleep to streamline your debugging sessions, by running CMEmu with:

```text
--gdb-here='-x cmemu-gdb/gdb-presets/contiki_test-verbose_sleep' 
```

Feel free to submit new useful log presets and gdb scripts!

## Debugging Flash Tests

You can debug flash-tests with `cmemu-gdb`, even though their `.elf` is long gone!
`cmemu-gdb` will recreate a stub ELF symbols file from whatever information is available in the `.dump`.
Then, it will set up a correct entrypoint (`emulator_main`)
and traps at the point of actual test start (`emulator_cdl_start`) and test end (`emulator_exit`).
You just need to compile `cmemu-flash-test` with the `gdb` feature and append launch parameters as previously.
Commands related to `cmemu-flash-test` are available under `monitor flash-test`.

```text
(gdb) monitor flash-test help
Commands related to a flash-test-case

Usage: monitor flash-test <COMMAND>

Commands:
  symbols-file   Print the path to a generated symbols file
  configuration  Get configuration of the test
  info           Display summary about the test
  howto          Tell me how to print/watch memory
  dump           Show saved dump
  compare        Compare memory state with the saved dump
  help           Print this message or the help of the given subcommand(s)
```

As for another example, let's investigate a regression in the `misc/agu_double.asm` test, as reported by `cargo test`:

```text
failures:
    flash::misc::agu_double_0

test result: FAILED. [...]
```

To investigate the mismatch in `times` symbol, we can launch `cmemu-flash-test` with this name:

```shell
cargo run --bin cmemu-flash-test -F gdb -- flash::misc::agu_double_0  --gdb 3333
```

You may notice that gdb magically displays the current *function*:
the mentioned stub file is minimal and should work fine for GDB 10.
You can display an overview of the test with `monitor flash-test info`:

```text
(gdb) mo flash info
Configuration: {'code': 'flash', 'lbEn': False}
Test path: /home/.../cmemu-framework/cmemu-flash-test/../cmemu-tests/tests/flash/misc/agu_double.tzst.0
Fake symbols ELF: /tmp/cmemu-test-symsofCE2R.elf

Saved symbols:
  times at 0x20000200: 1080 bytes
  flags at 0x20000658: 1080 bytes
  ...
```

Before we proceed, let's run the emulator entrypoint till the actual test.
Otherwise, initializing the memories would needlessly trigger our watchpoints.

```gdb
(gdb) c
Continuing.

Program received signal SIGEMT, Emulation trap.
0x00005480 in emulator_cdl_start ()
```

Remember, you can use `info functions` and `info variables` to list available symbols for these.
All the symbols are actually untyped, as we long-lost such information.
GDB even ignores the symbol size field in ELFs.

### Working with memory

You can use `monitor flash-test dump <symbol_name> [memory_format]` to show the saved contents of flash-test symbols
specified for comparison with the emulator.
Use `monitor flash-test howto` to learn how to correctly print memory contents and set up watchpoints:

```gdb
(gdb) mo flash howto
GDB doesn't know the type of our saved symbols,
so you need to cast them explicitly.
After casting, you may use `print` or `watch` commands.
Cast to an array works for 'print', but hardware watch needs more tricks.
You can use labels like '__Ltrigger_*' for breakpoints.
We've set traps at test start/end for you.

Symbol times:
print/d (char[1080]) times
watch -l *(char[1080]*) &times
watch -l *(int[270]*) &times

Symbol flags:
...
(gdb) print/d (char[1080]) times
$1 = {0 <repeats 1080 times>}
```

Remember to use only hardware watchpoints (`-l` option),
as without debugging information GDB will resort to single-stepping the target.

Let's `c(ontinue)` till the end of the test.
We can use `monitor flash-test compare <symbol_name> [memory_format]` to compare the current symbol contents
with the saved reference value.

```text
(gdb) monitor flash-test compare times
13, 18, 18, 18, 18, 18, 21, 27, 27, 27, 27, 27, 18, 18, 18, 18, 19, 20, 20, 20, 23, 23, 23, 23,
20, 26, 26, 26, 26, 23, 13 != 16, 18, 18, 18 != 19, 18 != 20, 18 != 21, 21, 27, 27, 27, 27, 27,
18, 18 != 21, 18 != 21, 18 != 21, 19 != 22, 20 != 23, 20 != 23, 20 != 23, 23, 23, 23 != 24, 23 != 25,
...
First error at item 30 (offset +78). Watch for it with:
watch -l *((int*) &times + 30)
```

There is no magic command for trapping the first wrong answer,
but we got a hint how to set a right watchpoint.
Let's restart the test and set the watchpoint after initialization:

```gdb
(gdb) c
Continuing.

Program received signal SIGEMT, Emulation trap.
0x00005480 in emulator_cdl_start ()
(gdb) watch -l *((int*) &times + 30)
Hardware watchpoint 1: -location *((int*) &times + 30)
(gdb) c
Continuing.

Hardware watchpoint 1: -location *((int*) &times + 30)

Old value = 0
New value = 13
0x0000531c in __Lbuiltin_saveValue_2 ()
```

The expected value was 16, but we saved 13.
After hitting a watchpoint in a "save" helper, we can use standard facilities to see where in the test we are:

```gdb
(gdb) bt
#0  0x0000531c in __Lbuiltin_saveValue_2 ()
#1  0x00000bba in __Ltrigger_30 ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
(gdb) frame 1
#1  0x00000bba in __Ltrigger_30 ()
(gdb) disass
Dump of assembler code for function __Ltrigger_30:
   0x00000ba0 <+0>:	isb	sy
   0x00000ba4 <+4>:	ldr.w	r2, [r0, #4]
   0x00000ba8 <+8>:	udiv	r8, r8, r9
   0x00000bac <+12>:	mov	r5, r4
   0x00000bae <+14>:	ldr.w	r7, [r5]
   0x00000bb2 <+18>:	ldr.w	r3, [r0, #4]
   0x00000bb6 <+22>:	bl	0x52b8 <save>
=> 0x00000bba <+26>:	mov.w	r1, #0
   0x00000bbe <+30>:	mov.w	r3, #16
```

As the parameters are set up before the *trigger* marker symbol at `isb.w`,
it is easier to scroll the assembly up with TUI (`layout asm`).

In our example, we find that `r5` has an interesting value:

```gdb
(gdb) info reg r5
r5             0x400220c0          1073881280
```

We can ask the emulator to tell us something more about the address with:

```gdb
(gdb) mo guest name 0x400220c0
GPIO::DIN31_0
# This doesn't work at the moment:
(gdb) info symbol 0x400220c0
No symbol matches 0x400220c0.
```

And now I remember that the last merge introduced an actual implementation of GPIO…

### Test Debug

By the way, this particular example could've been identified faster with tools in `mm319369/test_debug`:

```text
$ cd .../cmemu-framework
$ python ../../mm319369/test_debug/eval_all.py -C . -o test_results/  -j 20  cmemu-tests/tests/flash/misc/agu_double.tzst
...
Configuration #0 {'code': 'flash', 'lbEn': False} status: FAIL [151/270] times
data.ok      False  True
data.symbol             
flags                270
results              270
results2             270
times          151   119

All done! Failed 1 of 1 configurations!
Final status: 151 failed cases of 1080
Results dataframe written to:    test_results/agu_double.1-931409.parquet

$ python ../../mm319369/test_debug/explain.py  --patience 300 test_results/agu_double.1-931409.parquet
```

## TODO

There are a few features on the roadmap:

- supporting modification of registers (in particular, this allows for target-side evaluation of functions),
- supporting accessing any memory-address from the Debug interface,
- supporting "reverse execution" (this is a GDB concept!) by walking CDL time-frames,
- decode test parameters in flash-tests
