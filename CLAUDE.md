# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust-based creative coding sketches repository containing three main packages:
- **notan_sketches**: Primary sketches package using the Notan game framework (v0.13.0)
- **notan_touchy**: Shared library for Notan-based projects
- **macroquad_fractals**: Experimental fractals using the Macroquad framework

The focus is on generative art, emotion visualization, and interactive graphics experimentation.

## Build Commands

### Running Native Examples

```bash
cd notan_sketches
cargo run --example <example_name>
# Example: cargo run --example sierpinski_gasket
```

### Building for WASM

Build and bind WASM modules for web deployment:

```bash
cd notan_sketches
cargo build --release --example <example_name> --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/<example_name>.wasm
```

WASM outputs are placed in `notan_sketches/www/wasms/` with corresponding HTML files in `notan_sketches/www/`.

### Dependency Management

```bash
# Update dependencies (run in both notan_sketches and notan_touchy)
cargo update

# Check what wasn't updated
cargo update --verbose
```

Always test both native and WASM builds after updating dependencies.

## Architecture

### Package Structure

**notan_sketches** contains:
- **examples/**: 30+ creative coding sketches (e.g., sierpinski_gasket.rs, schotter.rs, erratic_wave_shader.rs)
- **src/lib.rs**: Module exports
- **src/** modules:
  - `emotion.rs`: Text-to-emotion analysis and color mapping models
  - `emotion_bg_visualizer/`: Visualization components for emotion data
  - `colors.rs`: Color utilities
  - `shaderutils.rs`: Custom shader pipeline management and hot reloading
  - `fractals/`: Fractal generation utilities
  - `schotter.rs`: Generative art implementations
  - `utils.rs`: General utilities
  - `mathutils.rs`: Math helpers

**notan_touchy**: Shared library providing common functionality for Notan projects (depends on notan_core and notan_log).

### Key Systems

#### Emotion Visualization System

The emotion system (`src/emotion.rs`) processes text emotion analysis data and maps emotions to colors:

- **EmocatTextAnalysis**: Deserializes emotion analysis JSON from external tools
- **TopEmotionsModel**: Extracts dominant emotions from analysis data
- **Color Mappings**: Two mapping systems available:
  - Plutchik (default): Based on Plutchik's wheel of emotions
  - Therapy: Based on art therapy color associations
- **Methods**: `get_simple_color()`, `get_black_or_white()`, `get_grayscale()`

Used in examples like `emo_bg_visualizer.rs` and `emo_proto_visualizer.rs`.

#### Shader System

Custom shader utilities in `src/shaderutils.rs`:

- **ShaderRenderTexture**: Wrapper for rendering with custom shaders
  - Supports up to 5 uniform buffers (hacky implementation noted in code)
  - Provides `draw()` and `draw_filled()` methods
- **create_hot_shape_pipeline()**: Creates shader pipelines from file paths
- **ShaderReloadManager**: Hot reload support for shader development (frame-based polling)
- **CommonData uniform**: Standard time and resolution uniforms for shader effects

Shader hot reloading has known issues (see Outstanding Issues below).

### Target-Specific Dependencies

The codebase uses conditional compilation for native vs WASM:

- **Native**: Uses `rand` and `uuid` with `rng-rand` features for fast randomness
- **WASM**: Uses `web-sys` and `uuid` with `js` feature for browser compatibility

## Important Notes

### Notan Version Synchronization

When updating Notan, **always update both** `notan_sketches` and `notan_touchy` packages to the same version to avoid incompatibilities.

### wasm-bindgen Version Mismatch

If you encounter schema version mismatch errors during WASM builds:

```
rust wasm file schema version: 0.2.95
   this binary schema version: 0.2.92
```

Update wasm-bindgen-cli: `cargo install wasm-bindgen-cli`

### Font Line Spacing

Notan doesn't yet support line spacing configuration. Modified fonts with custom line spacing are in `notan_sketches/examples/assets/fonts/`, created using FontForge.

### Local Cargo Configuration

`.cargo/config.toml` is gitignored. Add local build optimizations there (e.g., `jobs = 2` for resource-constrained systems).

## Outstanding Issues

### emo_bg_visualizer (RESOLVED)

Had egui-related compilation issues after the 0.13.0 upgrade. Fixed by updating to new egui 0.31.1 API.

### Mobile Rendering (Pixel 8a)

Notan apps may be choppy or crash on Pixel 8a with the native renderer due to memory issues with RenderTextures. **Workaround**: Switch browser to ANGLE renderer in device developer settings (Settings > System > Developer Options > ANGLE Preferences > select browser > choose "angle"). This is a device/driver limitation, not fixable in code.
