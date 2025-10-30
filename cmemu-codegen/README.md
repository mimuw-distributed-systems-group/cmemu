# cmemu-codegen is build-only dependency for cmemu-lib

This is an integrated place to do code generation.
Rust (cargo) project has two places for code generation:

- build.rs which is a single file script
- proc-macros, which needs a separate crate with a dedicated type

These two have a bit different scope:

- build.rs can access the whole codebase, but cannot modify it directly -- instead it produces new files
- proc-macros make local changes to the code as seen by the compiler

In cmemu, we use both approaches.
For instance, cross-component handlers calling is done by an event queue, where:

- build.rs collects all the handlers and builds a dispatcher code
- proc-macros rewrite the methods to pack their parameters in a struct

With cmemu-codegen, both these locations are thin wrappers calling code located here.

## Note

This crate's functions are public, but should be considered an internal implementation detail as a whole.
