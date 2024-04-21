# rust
Rust штуки


## run a single workspace

```
cargo run -p WORKSPACE
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