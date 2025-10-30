# Confeature

**Confeature** (pronounced with a long **iː**, a portmanteau of *config* and *feature*)
is a flexible configuration/(hyper-)parametrisation utility for your high-performance Rust code,
seamlessly bridging compile-time and run-time constants.

Confeature takes the middle ground between cargo features (conditional compilation) and a typical runtime configuration
parsing (e.g., INI files),
while having a distinct scope from both of them.

At its simplest, with confeature you specify configurable parameters (sometimes called *confeagurables*),
which turn into compile-time constants – which in turn are optimized out by the compiler, including dead-code
elimination.
Confeature gives you flexibility to, without changing any code [1],
relax the confeagurables to runtime-initialized constants aka immutable statics,
while preserving as much performance as possible.

When to use confeature?

- You are writing a high-performance library code that has some hyperparameters (usually extracted as constants) and:
    - would like to do a hyperparameter space search – you can either compile each parametrisation anew, determine it at
      runtime, or a mixture of these two;
    - expose some settings to end-users (note: crates cannot configure other crates), for instance: implementation of
      BLAS primitives;
    - provide fine-grained tracing without **any** overhead of disabled traces.

- You have a codebase with a lot of constants and want to easily manage or fiddle with them. E.g.,
    - A game may have a multitude of parameters:
        - simulated physics usually benefits a lot from compile-time optimization of configuration,
        - multitude of entities' properties: monsters' health, armor; angle of diffraction for ghosts, etc.
    - All kinds of simulators.

- You write an ugly scientific code and employ commenting out lines to manage *trial and error* experiments.
    - With confeature you can put such code behind a bazillion of conditional-compilation flags.

## Confeagurable modes

In the standalone build (the only one right now), a configurable is accessible by dereferencing  (``*``. ``impl Deref``)
a "constant" defined in a generated module. Depending on the ``mode`` of the parameter, it may actually generate either:

1. A ``const``,
2. A binary-patchable ``static``,
3. A lazy-``static`` initialized at runtime (at startup with the *static_init* crate for performance).

The **mode** of a confeagurable determine when a parameter may be set, and corollary, performance implications:

1. **fixed** - marks the default value from the configuration spec as no longer modifiable (e.g. use case: best result
   of hyperparams search),
2. **comptime** - requires that a final value is known at compile time (generates consts, in the future usable by build
   script and proc-macros),
3. **patchable** - provides the absolute best performance, while still allowing a modification (through binary-patching
   the executable),
4. **mixed** - if a configuration is present at built time, this may optimize to ``comptime``, otherwise behaves
   like ``runtime``,
5. **anytime** - three levels of configuration may override the prior: the default from configuration spec, compile-time
   value and runtime value (always lazy static),
6. **runtime** - this mode allows to deliberately ignore the compile-time parameters in the above.

## Confeagurables

Each confeagurable lives in a, possibly nested, namespace.
Currently, there is a support for booleans, (optional) integers and enums, `&'static str`, and structs.
Currently, only environment variables are used for passing values of the parameters.
With the exception of special-kind booleans, which are based on a `cfg!()` block.
Note: runtime ``&'static str`` comes from leaking memory with `String::leak`.
The *mode* of a configurable depends on its annotations, while each namespace can set a default one or (further)
restrict it.

## Python package

// TODO!
Confeature has a dedicated Python package with support for:

- Generating an **s: skopt.Space** for your hyperparameters.
- Building / executing cargo binary with the parameters given by e.g., ``skopt.Optimizer(s).ask()``, including
- support for binary-patching ELF executables.

## Flavors

Confeature envisions two modes of operation. The first one, **standalone**, generates a self-contained module.
It may either be generated manually (`cargo install confeature -F binary`) or by a build
script (`confeature::builder::standalone_from_env()` to automatically regenerate after changes).
In either case, there is no further build-time interaction with **confeature**.

The second one (WIP) will benefit from cooperation between the build script and proc-macros.
For instance, it will enable using *comptime* confeagurables to steer conditional-compilation (``#[cfg]``).

```NOTE
In the playground repo use: cargo run --manifest-path <git-root>/mm319369/confeature/Cargo.toml -F binary
In cmemu-framework workspace directory use:
cargo run --manifest-path ../../mm319369/confeature/Cargo.toml -F binary -- --cap comptime --manifest-path cmemu-lib -d never
```

# Example usage

**Note**: some details in this section are likely to change. But the principle format and usage will remain the same.

First, you need to create a `confeature.yml` file in your crate's root (next to `Cargo.toml`).
This YAML should be a dictionary representing confeagurable namespaces with the key value being its name.
Within this file, unless specified otherwise, all keys starting with a dot (`.`) are reserved.
The keys should be considered case-insensitive, whereas the case has little importance, as identifiers are recased to
match the Rust style.
There are two noteworthy special keys: `.confeature` containing extra directives to confeature and `.meta` which may be
any mapping, as it's not processed further.
Let's look at an example:

```yaml
my_crate:
  div_not_mul:
    doc: Use division instead of multiplication in an example found later in this README
    type: bool
    default: false
  # This uses a shortcut notation
  answer: 42

my_crate_blas:
  .confeature:
    doc: This namespace allows selecting implementation of low-level primitives
    mode_cap: comptime # Enforce all confeagurables of this namespace to be known at compile time
  inverted_sqrt_impl:
    doc: Select the implementation of ``1 / sqrt(x)``
    type: enum
    default: Rust
    mode: fixed # Not reconfigurable
    variants:
      - Rust
      - X86
      - Quake
```

We defined two namespaces containing a total of three confeagurables: `my_crate::div_not_mul`, `my_crate::answer`,
and `my_crate_blas::inverted_sqrt_impl`.
We can generate a standalone module by invoking `confeature` binary in the crate's root directory.
It will place a `confeature.rs` module inside `src/`, but this may be customized. This file contains modules named after
your namespaces.
Inside them, each configurable will have a corresponding upper-cased const/static field. Now, let's take a look at the
fields one by one.

The dictionary located at `my_crate.div_not_mul` defined a boolean confeagurable `my_crate::div_not_mul`, which is false
by default.
As a result, `confeature.rs` contains a `my_crate::DIV_NOT_MUL` item, which may be read in your code
with `*crate::confeature::my_crate::DIV_NOT_MUL`.
The generated documentation of this item has the `doc` attribute included, along with other useful information, for
instance,
that it may be configured with a `MY_CRATE__DIV_NOT_MUL` environment variable.

The second confeagurable located at `my_crate.answer` uses a shorthand in-line notation to define a configurable integer
with default value of 42. Likewise, it may be accessed with `*confeature::my_crate::ANSWER` and set
with `MY_CRATE__ANSWER` environment variable at runtime.
Neither of the confeagurables specified a *mode*, thefore a default one will be applied (typically **mixed**).

The third confeagurable is located under a separate namespace with item `my_crate_blas::INVERTED_SQRT_IMPL`.
The `my_crate_blass` module documentation will include the provided value, likewise to the generated item.
Here, we defined a new enum type with three possible variants. With **standalone** flavor it may be used as follows:

```rust 
 fn inv_sqrt(x: f32) -> f32 {
    use crate::confeature::my_crate_blas::{InvertedSqrtImplValues, INVERTED_SQRT_IMPL};
    const _: () = {
        let _assert_flag_is_comptime = *INVERTED_SQRT_IMPL;
    };
    match *INVERTED_SQRT_IMPL {
        InvertedSqrtImplValues::Rust => x.sqrt().recip(),
        InvertedSqrtImplValues::X86 => {  // see confeature-example project for a proper definition
            use std::arch::x86_64::*;
            unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
        }
        InvertedSqrtImplValues::Quake => {
            todo!("...")
        }
    }
}
```

This namespace enforces all confeagurables to have at most the *comptime* mode, thus being fully resolved at compile
time.
The definition of `inverted_sqrt_impl`, however, restricts the mode even further to prevent outside changes.
Namespaces might be nested and each layer (including the crate-specific top-level) may define a default mode and a mode
cap.
The default mode is inherited in the namespace tree and can be changed at any level, including the field as illustrated
above.
On the other hand, `mode_cap` may be only tightened, thus allowing for simple global restriction of the runtime impact.
As a specific case, you may make the generated `confeature.rs` module to only contain trivial constants (no extra macros
involved) with
including the following code at the top of `confeature.yml`:

```yml
.confeature:
  # Make this file the only source of truth.
  mode_cap: fixed
```

Set the `CONFEATURE_DEBUG` environment variable to see a summary of final values as seen during runtime of your program.

See an example project (`confeature-example`) configuration and rust files for a more complete overview.

## Using in your crate

Start with adding the following to your dependencies:

```toml
[dependencies]
confeature = { path = "../confeature", features = ["runtime"] }
# For support mode >= mixed:
# This is a necessary leak of abstraction from confeature, as we cannot generate code that uses this crate
# (even if we pull it with the above), as it is not imported (put to extern) otherwise. And re-exposing doesn't work either.
static_init = { version = "1.0.3", optional = false }
```

to automatically rebuild generated module, add this to your build dependencies:

```toml 
[build-dependencies]
confeature = { path = "../confeature", features = ["comptime"] }
```

put in your `build.rs` script:

```rust
use confeature::builder;

fn main() {
    // ... other stuff ...
    builder::self_contained_from_build_script();
}
```

and finally somewhere in your `src` dir:

```rust
// You can choose any name and visibility.
::confeature::include_out_dir! {pub(crate) mod confeagurables;}
```

Alternatively, if you want to manually regenerate the file with the `confeature` binary,
you may store its parameters in `package.metadata.confeature` array inside `Cargo.toml` as below:

```toml
[package.metadata.confeature]
mode_cap = "runtime"
debug_mode = "debug_assertions"
generated_file_path = "src/confeature.rs"
```

# FAQ

## Confeature vs cargo features / runtime config files

The goal of cargo features is vastly different: the focus is on an additive set of flags to partition a crate's code
into optional fragments, which may require further dependencies or longer compilation time.
Confeature is not a tool for intra-crate configuration or dependencies – in fact, a crate cannot directly influence the
configuration of another.
Confeature is closer to a plain-old config file approach but with focus on high-performance hot code that may benefit
from all the modern compiler optimizations.

In short, when you would like to use cargo features for their performance (conditional code generation etc.),
but shouldn't (because it doesn't fit the design), use **confeature**.

You can use cargo facilities to pass parameters to confeature by including the `env` section in
your [`config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) file. For instance:

```toml
[env]
MY_CRATE__DIV_NOT_MUL = "true"
```

## Compiler Explorer or didn't happen!

This is a stub section. For a code like the below:

```rust 
use crate::confeagurables::my_crate::DIV_NOT_MUL;

fn div_or_mul(x: i32) {
    if *DIV_NOT_MUL { x / x } else { x * x }
}

fn test() -> i32 {
    black_box(div_or_mul(11))
}
```

compiled with ``opt-level=2``:

1. With a ``const`` would optimize down to either ``fn test() -> i32 {1}`` or ``fn test() -> i32 {122}``.
2. With a patchable ``static`` would optimize down
   to ``static DIV_NOT_MUL = default; fn test() -> i32 { if std::ptr::read_volatile(&DIV_NOT_MUL) {1} else {122}}``.
3. With a lazy ``static`` would in principle optimize to (pseudocode)
    ```rust
    static DIV_NOT_MUL: AtomicOptionBool = None;
    fn parse_DIV_NOT_MUL() -> bool {std::env::var("DIV_NOT_MUL").ok().and_then(|s| s.parse().unwrap()).unwrap()}
    fn __run_before_main__() { DIV_NOT_MUL.set_with(parse_DIV_NOT_MUL); }
    fn test() -> i32 { 
      let div_not_mul = DIV_NOT_MUL.get_or_insert_with(parse_DIV_NOT_MUL);
      if div_not_mul {1} else {122}
    }
    ```
   where ``parse_DIV_NOT_MUL`` is likely inlined.

[1] Some features require to know the constant at compile time, for instance, ``const`` functions.
