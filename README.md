# rust
Rust штуки


## run a single workspace

```
cargo run -p WORKSPACE
```
## run a single file in workspace
[see](https://www.reddit.com/r/rust/comments/tiaor0/comment/i1d8lj4/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
```
cargo run -p WORKSPACE --bin NAME_OF_FILE
```

### run binaries in subdirectories
By default, Cargo looks for binaries in `src/bin/` directory. To run binaries from subdirectories, you need to configure them in `Cargo.toml`:

```toml
[[bin]]
name = "NAME_OF_BIN"
path = "src/bin/lifetimes/NAME_OF_BIN.rs"
```

Then run with:
```
cargo run -p  WORKSPACE --bin NAME_OF_BIN
```
## run a single test

```
 cargo test -p WORKSPACE
```
## watch (auto-rerun on change)

Uses [bacon](https://dystroy.org/bacon/) — a background code checker with a TUI. Not built into Cargo, install it once:

```bash
cargo install --locked bacon
```

> Previously this was `cargo watch`, which was archived by its maintainer in 2024 and is no longer maintained.

Anything after `--` is appended to the job's cargo command, so workspace flags like `-p` and `--bin` go there.

### watch a single workspace

```bash
bacon run -- -p WORKSPACE          # cargo run -p WORKSPACE
bacon check -- -p WORKSPACE        # cargo check -p WORKSPACE (default job)
bacon clippy -- -p WORKSPACE
bacon test -- -p WORKSPACE
```

### watch a single file in workspace

```bash
bacon run -- -p WORKSPACE --bin NAME_OF_FILE
```

### useful keys inside the TUI

| key | action |
| --- | --- |
| `c` | switch to clippy |
| `t` | switch to test |
| `d` | open rust doc |
| `s` | toggle summary mode |
| `w` | toggle line wrapping |
| `?` | all shortcuts |
| `q` | quit |

List available jobs with `bacon --list-jobs`. Run without the TUI using `bacon --headless <job> -- -p WORKSPACE`.

## format all files

```bash
find crates -name "*.rs" -exec rustfmt {} +
```

**find crates**: This command searches for files in the `crates` directory. </br>
**-name "*.rs"**: This option specifies that you are looking for files with the `.rs` extension (Rust source files). </br>
**-exec rustfmt {} +**: This part executes `rustfmt` on each of the found files. The `{}` is a placeholder for the found file names, and the `+` at the end allows `find` to pass multiple files to `rustfmt` at once, which is more efficient.


## run a single test

```bash
# Run all tests in workspace
cargo test -p WORKSPACE

# Run tests from specific module
cargo test -p WORKSPACE MODULE_NAME

# Run specific test function
cargo test -p WORKSPACE TEST_FUNCTION_NAME

# Run tests with full path
cargo test -p WORKSPACE module::tests::test_name

# List all tests without running them
cargo test -p WORKSPACE -- --list

# Run tests with detailed output
cargo test -p WORKSPACE -- --nocapture
```

**Examples:**
```bash
# Run all system module tests
cargo test -p web3-basic system

# Run specific test
cargo test -p web3-basic init_system

# Run with verbose output
cargo test -p web3-basic system -- --nocapture
```