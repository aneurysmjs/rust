# WebAssembly Average Function (Rust → Node.js / Browser)

This project shows how to write a simple Rust function that computes the average of a vector of numbers, compile it to WebAssembly using **wasm-pack**, and use it in **Node.js** or the **browser**.

---

## 🚀 1. Rust Source Code

### 📘 What `crate-type = ["cdylib"]` Does

In your `Cargo.toml`, you specify:

```toml
[lib]
crate-type = ["cdylib"]
```

This tells Cargo to compile your Rust library as a **C-compatible dynamic library**, which is required for WebAssembly.

#### Why `cdylib` is needed for WASM

* It strips away Rust-specific metadata and runtime glue, producing a **leaner, cleaner** output.
* It ensures the crate exports only symbols that `wasm-bindgen` can wrap.
* It avoids including Rust's default runtime, making the `.wasm` file smaller.
* It makes the compiled artifact consumable by non-Rust environments (JavaScript, Python, C, etc.).

Without `cdylib`, the WebAssembly module may include unnecessary Rust internals or fail during the `wasm-bindgen` processing step.

`src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn avg(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}
```

`Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

---

## 📦 2. Installing wasm-pack

```sh
cargo install wasm-pack
```

---

# 🔧 Build Targets Explained

wasm-pack supports several output targets. Here are the ones relevant to Node.js and browsers.

---

# 3️⃣ **Build Target: `nodejs`** (For Node.js only)

This target automatically loads the `.wasm` module **inside the generated JS**.
**No `init()` function is created.**

### Build command:

```sh
wasm-pack build --target nodejs --out-dir pkg-node --out-name wasm_avg
```

### Example use in Node.js

```js
import { avg } from "./pkg-node/wasm_avg.js";

console.log(avg(new Float64Array([10, 20, 30])));
```

### Important

* You **must use `Float64Array`** instead of a normal JS array.
* No need to call `init()`. The wasm module is loaded automatically.

---

# 4️⃣ **Build Target: `bundler`** (For Webpack / Vite / Parcel)

This target expects a JavaScript bundler. It generates JS that imports the WASM file.

### Build command:

```sh
wasm-pack build --target bundler --out-dir pkg-bundler --out-name wasm_avg
```

### Example use (Vite/Webpack/Parcel)

```js
import { avg } from "./pkg-bundler/wasm_avg.js";

console.log(avg(new Float64Array([1, 2, 3])));
```

### Node.js ⚠️ Warning

Node **cannot** directly import `.wasm` when using this target.
You must manually load the wasm file:

```js
import { avg } from "./pkg-bundler/wasm_avg.js";
import fs from "fs";

const wasmBytes = fs.readFileSync("./pkg-bundler/wasm_avg_bg.wasm");
await WebAssembly.instantiate(wasmBytes);

console.log(avg(new Float64Array([1, 2, 3])));
```

---

# 5️⃣ **Build Target: `web`** (Browser-only, no bundler)

Generates pure ES modules for the browser.

### Build command:

```sh
wasm-pack build --target web --out-dir pkg-web --out-name wasm_avg
```

### Example use in browser

```html
<script type="module">
  import { avg } from "./pkg-web/wasm_avg.js";
  console.log(avg(new Float64Array([5, 10, 15])));
</script>
```

---

# 6️⃣ **Using a Custom Output Name**

You can customize the base file name with:

```sh
wasm-pack build --target bundler --out-name my_custom_name
```

This generates files like:

```
pkg/
  my_custom_name.js
  my_custom_name_bg.js
  my_custom_name_bg.wasm
```

You can also customize the output directory:

```sh
wasm-pack build --target nodejs --out-dir dist --out-name wasm_avg
```

---

# 🧩 Summary Table

| Target  | Environment      | Requires `init()`? | Works in Node.js?                | Works in Browser? |
| ------- | ---------------- | ------------------ | -------------------------------- | ----------------- |
| nodejs  | Node.js          | ❌ No               | ✅ Yes                            | ❌ No              |
| bundler | Web bundlers     | ❌ No               | ⚠️ Only with manual WASM loading | ✅ Yes             |
| web     | Web (ES Modules) | ❌ No               | ❌ No                             | ✅ Yes             |

---
