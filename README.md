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