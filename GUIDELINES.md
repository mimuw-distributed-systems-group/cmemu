# Guidelines

## General rules related to source code

* Use `rustfmt` with default settings (CI checks it).
  Sometimes it doesn't format the code, so please keep it tidy manually —
  and let it be consistent with the formatting spirit of `rustfmt`.
* Try to make reading code easy: from top downwards without much extra scrolling.
  Reading "Clean Code" by Robert C. Martin is recommended.
  - Sort the code according to "general to specific" rule.
  - Don't mix code on different abstraction level in one function.
  - Try to meaningfully organize the code by extracting modules and functions if it helps.
* To make code easier to understand, name items meaningfully and unambiguously.
  In case of lack of ideas, start describing what the item is, what contains,
  what is its purpose and how it was created.
  Then think about the name again.
* If code indentation "grows too much" to the right, consider making it smaller
  if it makes sense in given situation, i.e. by extracting some bindings
  into outer scope
  ```rust
  let abc = if let Some(abc) = option_abc { abc } else { return }
  ```
  or creating a new method.
* Give items the visibility they should have and not more.
  Visibility can be restricted to any supermodule:
  `pub(super)`, `pub(in some::module)`, `pub(crate)`, `pub`.
* If you need to use `#[allow(...)]`, leave a comment with a reason why it's needed.
  It's useful for others who might later try fix given problem.

### Modules & imports

* Beginning of a file: optional `//! doc string`, then `use some::imports`,
  then define `mod submodules`, then define exports `pub(crate) use MyEnum`.
  - Prefer `use some::module::{self, StructA, method_b}` instead of importing
    every item in its own line from same module.
  - Group `use some::imports` together, without blank lines between them.
    Then, `rustfmt` sorts them all.
* For better readability, avoid chaining too many `super::super`.
  Instead, import some meaningful supermodule:
  ```rust
  use crate::component::core;
  use core::decode::PipelineStepPack;
  use core::instruction::Instruction;
  ```
  Alternatively, if it makes the matter easier, you could consider
  `use super::super as decode`.
  In case of single line import, prefer using full path.
* Macros:
  - `#[macro_use] mod abc;` brings macros from the annotated submodule
    to the scope of current module and its submodules - no `use some_macro;` needed.
  - `#[macro_export]` exports the macro - makes it visible outside the crate.
    Use only when it is the intention.

### Comments

* If something is not obvious (for others, not only the author of the code),
  consider adding `/// doc strings` and `// comments`.
  However, before leaving a comment, check if you can name some variables
  or pieces of code in a better way. Comments tend to outdate.
* If something is based on docs, then leave a comment with a reference.
  - Format `[book] section`, i.e. `[ARM-ARM] B1.4.7`.
    Optionally can be followed with section name or more detailed reference,
    i.e. `[ARM-ARM] B1.4.2, table B1-2 ICI/IT bit allocation in the EPSR`.
  - The code can be also split with banners, i.e.:
    ```rust
    // ----------------------------------------------------------------------------
    // [ARM-ARM] A2.2.1  ARM processor data types and... :: Integer arithmetic
    // ----------------------------------------------------------------------------
    ```
    + For first and last line use either `// -----` or `// =======`
      and they are 79 columns wide like in the preceding example.
    + The description is a single line no longer than 79 columns.
  - List of documentation is available in cmemu-meta repo wiki.

### Assertions

* We all love them. They can save debugging time.
* If there's some assumption, you can `debug_assert!` it.
  You should be careful with regular `assert!`
  which could have performance hit on release build.
* You should never `.unwrap()`. Instead, `.expect("message")`.
  Minimize number of these calls if possible.

### `#[derive(Copy)]`

* Do you really need it? Is it much better to use it?
* Changes move semantics to copy semantics, so it can have negative
  influence on performance in case of unnoticed implicit copies.
* Always check size of the type with [`std::mem::size_of::<Type>()`](
  https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6ab2a23733fc631daa72bb83348e0859).
  Be aware of padding and unobvious compiler optimizations.
* Usage on struct of size (assuming target `x86-64`):
  - up to 8B — it's fine (size of a pointer and cpu register),
  - up to 16B — you should wonder if it's justified.
    If it is, then it's fine (size of a fat pointer),
  - greater than 16B — it shouldn't derive `Copy`.
    If you think it should, you need to have a perfectly legitimate
    explanation and leave a comment.
* Consider future development, especially the fact the structure can
  grow. If that's the case, it's much less preferred to derive `Copy`.


## Components & subcomponents

* They are specific to this project
  (see `engine.rs`, `build.rs` and `cmemu-proc-macros/src/component.rs`).
* In most cases you should use `SeqFlop`.
  Every usage of `CombFlop` must be justified with a comment (see `flop.rs`).
* Preferred order of definitions:
  - `(Sub)Comp::new`
  - `tick` (components) / `tick_assertions` + `tick_extra` (subcomponents)
  - public API — `#[handler]` (components) / `pub(super) fn run_*` (subcomponents),
    in a meaningful order, i.e. how the API is used
  - internals
* Components `tick()` while subcomponents `run_something()`.
* Subcomponents in public API:
  - Should always take a reference to the component first,
    then context if needed, then the rest of arguments.
  - The component parameter name should be based on component name.
  - Take mutable reference to component *if and only if* needed.
  ```rust
  pub(super) fn run_something(bus_matrix: &SC::Component, ctx: &mut Context, ...)
  ```
* Internal types should be defined in appropriate (sub)modules.
  If there's a need to export them outside of the component, i.e. there's a type
  for communication between two components, it should be reexported in the root
  module of the component and no submodules should be accesible from outside.


## Logging

* Start message with uppercase letter.
* Numbers that would be displayed hexadecimally by a debugger
  (i.e. addresses, register values, data on buses) display hexadecimally.
* End sentence with dot if it makes sense (usually info level and higher).
* Levels:
  - `trace` — used to trace execution, saying "hey, I'm here"
  - `debug` — info useful for debugging, usually specific events
  - `info` — information that something normal happened, i.e. "saved log file to (...)"
  - `warn`, `error`
* Sometimes the logging is cut out from the code,
  so don't format arguments eagerly in such cases.
  You can pass `impl Display` for an example.
  
  Note: this applies to formatting errors, too.
  Usually there are `*_with` methods taking closures.
* Note: we have also so called "Cycle Debug Logger" (CDL) tool.
  It's meant to receive debug information with cycle accuracy
  and produce a visualization of program execution.


## Cargo dependencies

* Include major and minor versions *without* patch version, i.e. `thiserror = "1.0"`.
* `cargo update` updates `Cargo.lock` with latest dependency versions
  that meet the constraits.
* If considering adding new crates, use the ones from trusted authors.
  Do not depend on small, not widely used, unmaintained, and/or suspicious crates.
* Avoid multiple versions of crates:
  ```
  $ cargo install cargo-tree
  $ cargo tree -d
  # expect empty output (no duplicates)
  ```
  You might want to check it for all cmemu crates (or at least the binaries)
  and pass some extra flags.
  However, fear not. Clippy should have your back.
  Unless someone reconfigured it not to.
* All direct and indirect dependencies must use either MIT or Apache-2.0 licence.
  It can be verified with:
  ```
  $ cargo install cargo-deny
  # Note: config for the tool is already in the repository.
  $ cargo deny check licenses
  ```


## Other (non-related to source code)

* When collaborating on GitHub and mentioning in an issue/pull request A some comment
  in an issue/pull request B, GitHub creates back reference in B to A. But it does it
  only once per A and doesn't point to specific mention.
  So, if you want to better link the comments in both A and B,
  link from A to B *and* from B to A.
