# xtask

This crate provides automation scripts for the repo. They are OS-agnostic as they don't directly use any specific shell. All you need is the `cargo xtask` command.

Beware, these tasks are used in github workflows, so make sure any breaking changes are also reflected in `.github/workflows`.

More about xtask workflow [here](https://github.com/matklad/cargo-xtask). 