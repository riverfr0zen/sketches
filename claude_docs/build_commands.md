# Build Commands

## IMPORTANT: Working Directory

**ALWAYS change to the `notan_sketches` directory before running any cargo commands:**

```bash
cd notan_sketches
```

All cargo commands (build, test, run) must be executed from within the `notan_sketches` directory, NOT from the parent `sketches` directory. Running cargo from the wrong directory will result in "could not find Cargo.toml" errors.

## Running Native Examples

```bash
cd notan_sketches
cargo run --example <example_name>
# Example: cargo run --example sierpinski_gasket
```

## Running Tests

```bash
cd notan_sketches

# Run all tests
cargo test

# Run tests for a specific module
cargo test gridutils

# Run a specific test file
cargo test --test gridutils_test
```

## Building Examples

```bash
cd notan_sketches

# Build a single example
cargo build --release --example <example_name>

# Build all examples
cargo build --release --examples
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
