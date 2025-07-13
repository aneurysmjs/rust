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
## run a single test

```
 cargo test -p WORKSPACE
```
## watch a single workspace

```
cargo watch -x "run -p WORKSPACE_A"    
cargo watch -x "run -p WORKSPACE_B"
...
```
## watch a single file in workspace

```
cargo watch -x "run -p WORKSPACE --bin NAME_OF_FILE"
```

## format all files

```bash
find crates -name "*.rs" -exec rustfmt {} +
```

**find crates**: This command searches for files in the `crates` directory. </br>
**-name "*.rs"**: This option specifies that you are looking for files with the `.rs` extension (Rust source files). </br>
**-exec rustfmt {} +**: This part executes `rustfmt` on each of the found files. The `{}` is a placeholder for the found file names, and the `+` at the end allows `find` to pass multiple files to `rustfmt` at once, which is more efficient.