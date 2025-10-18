# Target-Specific Dependencies

The codebase uses conditional compilation for native vs WASM:

- **Native**: Uses `rand` and `uuid` with `rng-rand` features for fast randomness
- **WASM**: Uses `web-sys` and `uuid` with `js` feature for browser compatibility
