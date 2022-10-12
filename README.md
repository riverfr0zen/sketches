initial


# Build commands

```
cargo build --release --example eg_notan --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_notan.wasm


cargo build --release --example eg_stretch_resize --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_stretch_resize.wasm

```


