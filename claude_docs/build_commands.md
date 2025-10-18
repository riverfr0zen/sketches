# Build Commands

## Running Native Examples

```bash
cd notan_sketches
cargo run --example <example_name>
# Example: cargo run --example sierpinski_gasket
```

## Building for WASM

Build and bind WASM modules for web deployment:

```bash
cd notan_sketches
cargo build --release --example <example_name> --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/<example_name>.wasm
```

WASM outputs are placed in `notan_sketches/www/wasms/` with corresponding HTML files in `notan_sketches/www/`.

## Dependency Management

```bash
# Update dependencies (run in both notan_sketches and notan_touchy)
cargo update

# Check what wasn't updated
cargo update --verbose
```

Always test both native and WASM builds after updating dependencies.
