# Important Notes

## Code Formatting

**IMPORTANT**: All Rust code edits must be formatted using nightly rustfmt.

After editing any Rust files, always run:
```bash
cargo +nightly fmt
```

This applies the unstable formatting features defined in `.rustfmt.toml` (max_width=100, blank_lines_upper_bound=2), matching the rust-analyzer configuration in `sketches.code-workspace`.

## Notan Version Synchronization

When updating Notan, **always update both** `notan_sketches` and `notan_touchy` packages to the same version to avoid incompatibilities.

## wasm-bindgen Version Mismatch

If you encounter schema version mismatch errors during WASM builds:

```
rust wasm file schema version: 0.2.95
   this binary schema version: 0.2.92
```

Update wasm-bindgen-cli: `cargo install wasm-bindgen-cli`

## Font Line Spacing

Notan doesn't yet support line spacing configuration. Modified fonts with custom line spacing are in `notan_sketches/examples/assets/fonts/`, created using FontForge.

## Local Cargo Configuration

`.cargo/config.toml` is gitignored. Add local build optimizations there (e.g., `jobs = 2` for resource-constrained systems).

## Outstanding Issues

### emo_bg_visualizer (RESOLVED)

Had egui-related compilation issues after the 0.13.0 upgrade. Fixed by updating to new egui 0.31.1 API.

### Mobile Rendering (Pixel 8a)

Notan apps may be choppy or crash on Pixel 8a with the native renderer due to memory issues with RenderTextures. **Workaround**: Switch browser to ANGLE renderer in device developer settings (Settings > System > Developer Options > ANGLE Preferences > select browser > choose "angle"). This is a device/driver limitation, not fixable in code.
