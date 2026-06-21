# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Cargo workspace of independent Rust learning/practice crates ("Rust штуки"). Each crate under `crates/*` is self-contained with its own edition and dependencies — there is no shared library code or cross-crate dependency. Comments are frequently in Russian and Spanish; this is expected, not a problem to "fix."

Crates: `api` (axum HTTP server), `geometry`, `playground` (proc-macro + assorted bins), `problems` (algorithm exercises), `regexes`, `snake`, `wasm` (wasm-bindgen cdylib), `web3-basic` (mini Substrate-style blockchain runtime).

## Commands

Everything is scoped per crate with `-p <crate>`. Substitute the crate name (`playground`, `web3-basic`, etc.).

```bash
cargo build -p <crate>
cargo run   -p <crate>                      # runs the crate's default bin
cargo test  -p <crate>                       # all tests in a crate
cargo test  -p <crate> <module_or_fn_name>   # filter by substring
cargo test  -p <crate> module::tests::test_name   # exact path
cargo test  -p <crate> -- --nocapture        # show stdout
cargo test  -p <crate> -- --list             # list tests without running

cargo watch -x "run -p <crate>"              # auto-rerun on change
```

Format (rustfmt config: `max_width = 150`, `use_small_heuristics = "Max"`):

```bash
find crates -name "*.rs" -exec rustfmt {} +
```

## Multiple binaries in `playground`

`playground` holds many `main()` files. How a binary is discovered depends on where it lives:

- **Directly in `src/bin/*.rs`** (e.g. `bin_runner.rs`, `derive_macro.rs`, `operations_that_move.rs`): auto-discovered by Cargo. Run with `cargo run -p playground --bin <filename_no_ext>`.
- **In a subdirectory of `src/bin/`** (e.g. `src/bin/lifetimes/example_1.rs`): NOT auto-discovered. It must have an explicit `[[bin]]` entry in `crates/playground/Cargo.toml` giving its `name` and `path`. Add such an entry when creating a new bin in a subdirectory.

`bin_runner.rs` is an arg-dispatched runner: it `match`es on `args[1]` to call a function from the `fp` module. Run a specific example with `cargo run -p playground --bin bin_runner <example_name>` (the VS Code "Debug current bin with args" launch config prompts for this arg).

Note: `playground` is also `proc-macro = true` and exports the `Hello` derive macro (`src/lib.rs`). `derive_macro.rs` consumes it via `use playground::Hello;`.

## Test conventions

Two patterns coexist; match whichever the surrounding crate uses:

1. **Sibling `_test.rs` file** (used in `problems`, `regexes`): the implementation file declares
   ```rust
   #[cfg(test)]
   #[path = "./foo_test.rs"]
   mod foo_test;
   ```
   and `foo_test.rs` does `use super::*;`. Tests live in a separate file but compile as a submodule.
2. **Inline `mod tests`** (used in `web3-basic`): a `#[cfg(test)] mod tests { use super::*; ... }` block at the bottom of the source file.

## web3-basic architecture

A miniature blockchain runtime modeled on the Substrate/Polkadot "pallet" pattern. Understanding it requires reading several files together:

- Each pallet module (`system.rs`, `balances.rs`) defines a `Config` **trait** of associated types plus a generic `Pallet<T: Config>` struct holding that pallet's state. State logic is written against the associated types (e.g. `T::AccountId`, `T::Balance`), kept abstract via trait bounds from the `num` crate (`Zero`, `One`, `AddAssign`).
- `main.rs` defines the concrete `Runtime` struct that owns one of each `Pallet`, picks concrete types in the `types` module (`AccountId = String`, `Balance = u128`, …), and wires them up by `impl`ing every pallet's `Config` for `Runtime`.
- `support.rs` holds runtime-wide primitives shared across pallets: `Block`, `Header`, `Extrinsic`, `DispatchResult`, and the `Dispatch` trait.

When adding a pallet: define its `Config` + `Pallet<T>`, add the field to `Runtime`, and `impl <pallet>::Config for Runtime`.
