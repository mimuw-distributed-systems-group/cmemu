# TESTS GUIDELINES

Hints and tips how to write cmemu tests.

## Goal

The main goal of this guidelines is to describe how the tests should be written to be readable and effective. Format used for writing tests forces to separate configuration part from testing.

## Test structure

*Yaml-o-jinja* (name invented by the cmemu team) is the format used for writing tests. Configuration part is written in YAML format (started with `---` and ended with `...`). Testing part is written in *Jinja2*, so all features from this template engine can be used. Details of the test format are described in [test format specification on GDrive][tests-format-doc].

### Configuration part

* __name__ — Mandatory field to name what this test is testing. Try to write it in one sentence. Used instructions should be written in uppercase, e.g. `LDR`.

* __description__ — Mandatory field to describe with more details what is tested and how. Used instructions should be written in uppercase, e.g. `LDR`.

* __dumped_symbols__ — Names and sizes of the arrays that will be dumped during execution of each configuration. The arrays are not shared between configurations. Allocate only as much memory as needed. If too much memory is allocated (not everything is used), the test will take more time than necessary to generate. If there is allocated not enough memory, the *Experimentation Framework* will hang (check logs for error messages). User can create symbols with sizes in `B` (bytes) or `words` (4-byte items):

  ```
  name1: 100 B
  name2: 200 words
  ```

Information about user defined symbols can be found in [here][tests-format-doc].

* __configurations__ — List of configurations that will be tested. If possible, **try to avoid unnecessary overhead** of generating and then testing greater number of configurations than necessary. For regular tests, i.e. prefer `{% set values = ... %} {% for value in values %} ... {% endfor %}` instead of creating separate configurations for every value. For tests with huge amount of data, put the data in flash and load it in runtime, like in `smull_umull_timing.asm`.

* __product__ — List of dictionaries, where keys are variables and values are possible valuations. For each evaluation of variables defined here, the *Experimentation Framework* will generate new configuration. Use only for making experiments, not for testing. More information in [tests format doc][tests-format-doc].

#### Example configuration

```
name: INSTR instuction test
description: Timing and correctness test of INSTR
dumped_symbols:
  results: 120 words
  times: 120 words
configurations:
- { values: [[1, 2], [2, 3], [3, 4] ... ] }
```

### ASM part

Before writing a new test, it is good to read already existing tests to see how it can be done (prefer recently written tests).

If there is a test that is almost identical to the newly created, probably the best approach would be to add new configurations to it and add new necessary fields, i.e. `testedInstr` with the instruction that is tested. It helps to maintain the tests and reduce copy-pasting.

Example initialization
```
{% device:line_buffer_enabled = True %} 
{% device:write_buffer_enabled = False %} 
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r0, dwt

    b.w tested_code
.thumb_func
end_label:
{% endblock %}
```

Example code with a test
```
{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for (r7Val, r8Val) in values %}
    @ Prepare input values
    ldr.w r7, ={{r7Val}}
    ldr.w r8, ={{r8Val}}

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w r2, [r0, {{CYCCNT}}]

    ...
    # tested code with result in r5
    ...

    @ Get finish time
    ldr.w r3, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
    b.w end_label

save:
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}

    bx.n lr
{% endblock %}
```

## Saving data of the test (result, timing and user defined data)

At the end of instruction testing write:
```
bl.w save
```

Add `save` function (if possible, there are some exceptions from that) with the following example code:
```
save:
  # Code that saves data.
  # r2, r3, r4 are registers used for this example. Different ones can be used.
  # r2 - stores data to save.
  # r3, r4 - temp registers used for operations.
  {{saveValue(name, r2, r3, r4)}} # save any value to allocated "name" array

  bx.n lr
```

When saving times, the difference between end and start of the measurement has to be computed. To do so, `sub.w` should be used. In early days of the emulator, when `sub.w` had been unsupported, the `subs.n` was used instead. However, it sets flags - and this is an unwanted side effect and requires specific order of instructions.

### Saving flags

Flags should be saved in tests from `data_processing` category or if test uses flags and sets them.

To save flags, first define symbol `flags` in `dumped_symbols` section.

If the test checks flags, set them explicitly to desired initial value before each measurement ("Get start (...)"). It could be done in the following way

```
@ Clear flags
mov.w r5, #0
msr.w apsr_nzcvq, r5
```

To save flags we use

```
mrs.w r5, apsr

{{saveValue('flags', r5, r3, r4)}}
```

### Saving counters

To save counters, first define symbols for each counter, e.g.:
```
cpicnts: 24 words
```

Add function to save results:

```
save_cpicnt:
    sub.w r2, r3, r2 @ Start counter value is in r2, finish counter values is in r3
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}
    
    bx.n lr
```

#### The first choice approach

When test doesn't require many registers (3 registers are spare) following approach should be used.

1. In `{% block code %}` section add:

    ```
    @ Loop over all counters to save.
    {% for counter, save_func in [(CPICNT, "save_cpicnt")] %}         
    mov.w r1, {{counter}}
    ldr.w r2, ={{save_func}}

    bl.w tested_code
    {% endfor %}
    ```

2. At the beginning of `tested_code`, add:

    ```
    tested_code:
        @ Save where to return after test.
        mov r10, lr
    ```

3. Read counter value before and after tested code:
    ```
    @ Read start counter value
    ldr.w r2, [r0, r1]

    TESTED CODE

    @ Get end counter value
    ldr.w r3, [r0, r1]
    blx.n r2
    ```

4. At the end of the `tested_code`, add:

    ```
        @ Return to counters loop.
        bx.n r10
    ```

Instead of `r1, r2, r10` registers, any 3 free registers could be used.

Remember to add `.thumb_func` directive to each save function, otherwise branch to them will cause `UsageFault` exception.

To see full example, go to: `cmemu-tests/tests/flash/instructions/memory/ldr_imm.asm`

#### The second choice approach

Use this approach, when 3 free registers can't be find.

1. In `tested_code` add loop over counters and functions to save them:
    ```
    {% for counter, save_func in [(CPICNT, "save_cpicnt")] %}
    ```

2. Read counter value before and after tested code:
    ```
    @ Read start counter value
    ldr.w r2, [r0, {{counter}}]

    TESTED CODE

    @ Get end counter value
    ldr.w r3, [r0, {{counter}}]
    bl.w {{save_func}}
    ```

For more detailed example look into `cmemu-tests/tests/flash/instructions/memory/ldr_reg.asm`.

### When specific counters should be saved?

* __lsucnt__ and __cpicnt__ - save when testing branches and memory access
* __foldcnt__ - save when `IT` instruction is used

## List of test cases

One of the techniques used for testing cases that differ only in values, but have the same code structure is creating list of these test cases. It could be applied in the following way:

```
{# 
    Description of test case tuple.
#}
{% set test_cases = [
    ((Optional) Test case name, val1, val2, ...),
    ...
] %}
```

If name of each test case is the same as variable with e.g. address of register to test and reference to the documenation is required, consider this approach:

```
@ Description of tested_variable.
{% set tested_variable = ... %}

{# 
    Description of test case tuple.
#}
{% set test_cases = [
    (tested_variable, val1, val2, ...),
    ...
] %}
```

Example:

```
{#
    Description of a tuple: (
     name of test case,
     register addres,
     values to write,
    )
#}
{% set test_cases = [
  ("vtor", "0xE000ED08", [0x00, 0x01, 0xff]),
  ...
] %}
```

In the `tested_code` it can be used like this:

```
tested_code:
{% for var1, var2, .. in tested_cases %}
```

## Repeating tested instruction

Repeting execution of the same instruction is one of the techniques used in tests. The reason for this is to check more cases and also to test time increase after each repetition.
 
To use this technique, first, create variable `repetitions`. Chosen value should assure a good test coverage and in the same time avoid unnecessarily large number of configurations. In practice, it means that `5+` could be a reasonable value. 

The next step is to test all possible repetitions:
```
tested_code:
{% for reps in range(repetitions) %}
    ...
    {% for _ in range(reps) %}
        measured instruction
    {% endfor %}
    ...
{% endfor %}
```

## Memory types

There are 3 main types of memory that we test: `gpram`, `sram` and `flash`. Specifying that the section of code should be in one of these memory types is done by writing
```
{{ section(memory) }}
```
where `memory` is one of the memory types. Code will be put in the section specified by the `memory` value, starting from this place in the test (all of these builtin functions can be found in `ms371763/task_handler/_tasks/builtin/functions.py`).

If you want to test configurations in different memories, add `code: "gpram"/"sram"/"flash"` in your configuration and write `{{ section(code) }}` at the beginning of the tested code.

## Constants declaration used in test with `{% set ... %}`

Defining constants for test, e.g. values to use, can be put inside `{% set ... %}` declaration. To keep the whole configuration of the test in one place, insert `set` block after `{% extends "asm.s.tpl" %}`, but before `{% block code %}`. It separates configuration part from the actual test.

### Example

```
{% extends "asm.s.tpl" %}

{% set values = [1, 2, 3] %}
{% set useful_const = 42 %}

{% block code %}

```

## Same state of processor before each measurement

To make sure that processor has the same state before each measurement, clear the pipeline, align instructions and clear line buffer. 

* Clearing the pipeline makes PIQ state more or less the same across measurements, so it is easier to analyze the results. It helps to not depend on whether measurement starts with full or empty PIQ.
* Aligning instructions makes sure that they are executed in the same way for each measurement. More specifically, before an instruction is decoded, all its bytes must be already fetched. Fetch requests only a single *aligned* word at a time. Thus, if a wide instruction (e.g. `ADD.W`) isn't aligned, fetching all its bytes takes two transfers instead of a one, so decode phase can be stalled introducing a bubble in PIQ and thus increasing execution time.
* Clearing line buffer prevents situation when for the first measurement a transfer to flash is done and its result is cached in the line buffer, while for the next measurements the cached value is read directly from the line buffer. Since the flash is slow, the difference in timing is visible.

To clear the pipeline before test and align the code, use:
```
@ Align and clear PIQ
.align 4
isb.w
```

* `.align 4` alignes code to 16 bytes. Size of the line buffer is 8 bytes, so aligned instructions fit into it.

* `isb.w` flushes the instruction pipeline. Instructions that follows the `isb.w` will be fetched again.

To read more when to clear line buffer, see: [Reset line buffer](#reset-line-buffer).

## Reducing number of configurations

Unnecessarily large number of configurations is undesired. First, it significantly increases time of generating .tzst archive, and later it introduces unnecessary overhead to the tests (mostly initliazation: loading new flash & emulator_main). Also fewer configurations results with shorter test execution time. To reduce number of configurations, try to find values in them, which can be extracted and iterated over all defined variants. In the following example, `ldrInstr` and `addInstr` could be such values:

```
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", ldrInstr: "ldr.w", addInstr: "add.w" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", ldrInstr: "ldr.w", addInstr: "adds.n" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", ldrInstr: "ldr.n", addInstr: "add.w" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", ldrInstr: "ldr.n", addInstr: "adds.n" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", ldrInstr: "ldr.w", addInstr: "add.w" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", ldrInstr: "ldr.w", addInstr: "adds.n" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", ldrInstr: "ldr.n", addInstr: "add.w" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", ldrInstr: "ldr.n", addInstr: "adds.n" }
```

could be changed into

```
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2" }

...

{% set instructions = [
    ("ldr.w", "add.w"),
    ("ldr.w", "adds.n"),
    ("ldr.n", "add.w"),
    ("ldr.n", "adds.n"),
] %}

...

{% for ldrInstr, addInstr in instruction %}

tested_code

{% endfor %}
```

### What to do when code doesn't fit into a memory?

By reducing number of configurations, size of the code increases. It is possible that it won't fit into a memory. To solve that, configurations that don't fit into one kind of memory can be splitted into parts. 

Example:

```
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2" }
- { code: "flash", addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "flash", addr: "sram", regA: "r2", regB: "r3", regC: "r2" }

...

{% set instructions = [
    ("ldr.w", "add.w"),
    ("ldr.w", "adds.n"),
    ("ldr.n", "add.w"),
    ("ldr.n", "adds.n"),
] %}
```

Let's assume that the first and the second configurations don't fit into `gpram` memory. They can be splitted in the following way:

```
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", gpram_part: 0 }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r2", regC: "r3", gpram_part: 1 }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", gpram_part: 0 }
- { code: "gpram", addr: "sram", regA: "r2", regB: "r3", regC: "r2", gpram_part: 1 }
- { code: "flash", addr: "sram", regA: "r2", regB: "r2", regC: "r3" }
- { code: "flash", addr: "sram", regA: "r2", regB: "r3", regC: "r2" }

...

{% set instructions = [
    ("ldr.w", "add.w"),
    ("ldr.w", "adds.n"),
    ("ldr.n", "add.w"),
    ("ldr.n", "adds.n"),
] %}

{% if code == "gpram" %}
    {% if gpram_part == 0 %}
        {% set instructions = instructions[:2] %}
    {% elif gpram_part == 1 %}
        {% set instructions = instructions[2:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% endif %}
```

Then, one might want to reduce the number of configurations even further: over the tuple of variables `(regA, regB, regC)`.

Note that if there is a meaningful reason to keep a greater number of smaller configurations, then that is fine. One example is temporarily keeping a greater number of smaller configuration, because a test fail and it is significantly easier to debug it with smaller configurations.

## Naming block of configurations

If it is possible to name a block of configurations, it would be helpful for readers to understand what is tested and it would encourage to think about edge cases or other test paths. Suggested way of naming block

```
PREVIOUS BLOCK OF CONFIGURATIONS

# NAME OF THE BLOCK
BLOCK 

NEXT BLOCK OF CONFIGURATIONS
```

Blocks of configurations should be separated by new line.

## Enabling line buffer

Tests that access `flash` memory during the measurement, either to fetch code or to read or to write data, should be run on both cases: line buffer enabled and disabled. In that case, add to all configurations this variable:

```
lbEn: True/False
```
and then write:
```
{% device:line_buffer_enabled = lbEn %}
```

If all tests are executed from other memory types (`gpram` or `sram`), __line buffer__ should be explicitly set to `True`. Because of that tests will load faster. It is result of faster test's prolog execution. *emulator_main* copy code and data from `flash` into `gpram` and `sram` (RAM initialization) and initialize `bss` in the prolog of a program. To read more about how this part of emulator works, visit __Emulator_main()__ section in [tests format doc][tests-format-doc].

### Exception

If you know what you are doing, you can run tests from `gpram` and `sram` with disabled line buffer, but a reason why it is done should be provided in the comment.

### Resetting line buffer

If reading code or data from `flash` is done during a measurement, reset line buffer cache before each measurement. It can be done by i.e. reading from an address 0.

```
mov.w r7, #0
ldr.w r2, [r7]
```

The exception to this rule is when the first use of the line buffer during a measurement is a cache miss. For example, a longer piece of code is being executed from flash and data bus does not request flash.

## Enabling write buffer

By the time the write buffer is implemented, we always disable the write buffer. 

The suggestion for a future usage is that if `STR` instruction is tested, both options should be checked, otherwise write buffer should be enabled.

## Verifying initial state

In some cases, initial state of registers/memory is assumed, i.e. value of the `CONTROL` register. To make sure that these assumptions are emulated, dump required values before measurements.

## Instruction width

Instruction width has to be always explicitly specified (`instr.w` or `instr.n`). It will assure the length of generated instruction.

## `cmemu-test-table` mechanism

This mechanism generates a matrix. Columns represent memories where the code is executed from during a measurement. Analogously, rows represent memories where the data is loaded from and stored to. This matrix is generated for each test that is listed in `cmemu-test-table/test_env_mapping.yaml`. It checks how many configurations passed, sums the results and puts them in the table. Example:

```
 RESULT TABLE (passed/failed):
        data\code        |          gpram          |          sram           |          flash          
          gpram          |          83/14          |         106/6           |         105/29          
          sram           |         147/4           |          64/38          |         209/3           
          flash          |          69/5           |          50/24          |          92/6           
          none           |          95/4           |          90/10          |         213/5           
      flash, gpram       |           5/1           |           3/3           |           5/1           
       flash, sram       |           6/0           |           1/5           |           6/0           
       gpram, sram       |          12/0           |           6/6           |          12/0 
```

It could be found in GitHub repository: `Actions` -> workflow `Test table` -> pick one of results -> the table is inside the log.

After creating a new test, insert it to the `cmemu-test-table/test_env_mapping.yaml` if it reads data during measurement (between loads of the DWT counters).

## Using pseudoinstructions

Be cautious when using pseudoinstruction (i.e. `ldr r0, =mem`) and be sure to know what code it is assembled to. Avoid in measured code (between loads of the DWT counters).

[List of pseudoinstructions](https://developer.arm.com/documentation/dui0489/i/arm-and-thumb-instructions/pseudo-instructions?lang=en).

### What does happen when `ldr r0, =value` is used?

It loads `value` to the `r0` register. This value is stored in the literal pool and assembler generates PC-relative `LDR` instruction that reads this value. Literal pool has to be reachable from the load. To place pool closer, `.ltorg` directive can be used. It forces to put the pool in the place of directive (potentially multiple copies can be created). The literal pool can be placed in code section, however the data inside should *not* be executed, so you should branch over the pool.

Visit [ARM documentation](https://www.keil.com/support/man/docs/armasm/armasm_dom1361289875065.htm) for more details about `LDR` pseudoinstruction.

## Using `.thumb_func` and `.type function_name, %function` directives

These directives are used only when they are required, i.e. without them a test won't compile. They are connected with functions which addresses are loaded to make a jump to them.

## Alternative to `.ltorg` directive

When `ldr` instruction can't reach literal pool, `movt` and `movw` could be considered instead of adding `.ltorg` directive. It doesn't increase size of code as much as `.ltorg`. This aspect is important when reducing number of configurations.

Example of usage:
```
movw.w r5, #:lower16:memory
movt.w r5, #:upper16:memory

...

memory: .word 0x42
```

## Tips & tricks

* Analogously to `bl.w save`, sometimes it might be beneficial to do the same with initialization: `bl.w initialize`. It is a rare case, however if initialization is long and the same for multiple iterations (i.e. independent of enclosing jinja loop), you might consider extracting the code to a separate function.

* Constants can be loaded with `ldr r0, =0x123` (with [one exception](#using-pseudoinstruction)). Note that some instructions (i.e. [mov.w](https://static.docs.arm.com/ddi0403/ed/DDI0403E_d_armv7m_arm.pdf?_ga=2.135389233.1803240997.1558210989-200364969.1558210989#E11.BABCDEDI)) use [`ThumbExpandImm`](https://static.docs.arm.com/ddi0403/ed/DDI0403E_d_armv7m_arm.pdf?_ga=2.135389233.1803240997.1558210989-200364969.1558210989#G9.4953544) to operate on constants, so only values with the format defined there can be used, i.e. `0xda00da00`. You can see examples in `mrs_apsr.asm` and `mov_w_thumb_expand_imm.asm`.

* When nesting loops, think how to make the resulting memory readable in the test runner output.

   An example: iteration over some settings (set some flags, ...) and how many times given code fragment should be repeated. It is best to handle the latter one in the most inner loop, so the output is clustered by settings: `[(some setting start here) 1, 3, 5, 7, ..., (other setting start here) 1, 4, 7, 10, ...]`.

* Check from time to time, that your code has been correctly assembled. You can use *objdump* for this purpose.

* Instead of creating a label containing jinja variables in its name, use new variable constructed with the `format` method (preferred) or format operator: `%`.

    Examples:

    ```
    {% set jump_target_label = 'jump_target_{}_{}'.format(loop1_idx, loop2_idx) %}

    {{jump_target_label}}:
    ```

    or 
    
    ```
    {% set jump_target_label = 'jump_target_%d_%d' % (loop1_idx, loop2_idx) %}

    {{jump_target_label}}:
    ```

    are favored over

    ```
    jump_target_{{loop1_idx}}_{{loop2_idx}}:
    ```

* It's preferred to use `.rept` ARM directive when jinja `for` loop isn't necessary. It might be better for assembler to process such code. This directive could be combined with `.set` that creates symbol with given name and value which can be changed.

    Example:
    ```
    .set counter, 0
    .rept 10
        add.w r1, #counter
        .set counter, counter + 1
    .endr
    ```

    `.rept` directive could be also use to allocate data:
    ```
    dummy_data:
    .rept 10
    .word 0x0
    .endr
    ```

    You can read more about [.rept and .set](https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/useful-assembler-directives-and-macros-for-the-gnu-assembler).

* If there is not enough memory in `gpram/sram`, one can consider to move the code that isn't tested (e.g. save functions) to `flash` memory. It can save some bytes.

## (Very) Simple test example

```
---
name: Simple test of ADD.W
description: >
    Timing and correctness test of few exemplary cases of "ADD" instruction.

    It adds 3 that is declared as jinja variable to values from list.
dumped_symbols:
  results: 50 words # 10 (repetitions) * 5 (values)
  times: 50 words
  flags: 50 words
configurations:
- { code: "gpram", lbEn: True }
- { code: "sram", lbEn: True }
- { code: "flash", lbEn: True }
- { code: "flash", lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 10 %}
{% set three = 3 %}
{% set values = [1, 2, 3, 4, 5] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w r0, dwt

    b.w tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for reps in range(repetitions) %}
{% for value in values %}
    mov.w r8, #{{value}}

    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w r2, [r0, {{CYCCNT}}]

    {% for _ in range(reps) %}
        add.w r7, r8, #{{three}}
    {% endfor %}

    @ Get finish time
    ldr.w r3, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r7, r3, r4)}}
    {{saveValue("flags", r5, r3, r4)}}

    bx.n lr
{% endblock %}

```

## Jinja gotchas

* `set` scoping behaviour - setting variable inside a block or a loop doesn't change variable outside of it. Exception to that is a `if` statement which do not introduce a new scope. Read more about [assignments/scoping](https://jinja.palletsprojects.com/en/2.11.x/templates/#assignments) and [variables](https://jinja.palletsprojects.com/en/2.11.x/templates/#variables).
* `namespace` - allows to propagate changes across scopes.

  Example:
    ```
    {% set ns = namespace(found=false) %}
    {% for i in range(1, 10) %}
        {% if i == 5 %}
            {% set ns.found=true %}
        {% endif %}
    {% endfor %}
    {% if ns.found %}
        Found
    {% endif %}
    ```

## Read more

* [*Experimentation Framework* README](https://github.com/mimuw-distributed-systems-group/playground/tree/master/ms371763/Benchmark)
* [Tests format][tests-format-doc]

[tests-format-doc]: https://docs.google.com/document/d/1JPGmoUOas8HlQ1pMr4WfWjP1_BXhVq5lBEwNy83qDDU/edit?usp=sharing
