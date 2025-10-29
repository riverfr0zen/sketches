# Render Texture Antialiasing

## Problem

When capturing images from render textures using `RenderTexture::to_file()`, the resulting images had visible aliasing (jagged edges) even though the main window had multisampling enabled with `.set_multisampling(8)`.

**Root Cause**: Notan (v0.13.0) does not support multisampling (MSAA) on render textures. The `RenderTextureBuilder` API has no methods for setting sample counts or enabling MSAA. Multisampling can only be set on the main window framebuffer, not on off-screen render targets.

## Solutions Considered

### Option A: Request MSAA Support from Notan

**Pros:**
- Would be the "proper" solution
- Would work automatically without workarounds

**Cons:**
- Not available in current Notan version
- Would require waiting for upstream changes
- No guarantee it would be implemented

**Decision:** Not viable for immediate needs.

---

### Option B: Supersampling + Image Crate Downsampling

Render to a larger texture (e.g., 2x), then use the `image` crate to downsample before saving.

**Pros:**
- Smaller final file sizes
- Professional-quality antialiasing
- Image crate works in WASM

**Cons:**
- Complex implementation requiring different code paths for native vs WASM
- `RenderTexture::to_file()` is Notan-specific; the `image` crate needs raw pixel data
- In WASM, would require JavaScript interop via `wasm-bindgen` to trigger downloads from raw bytes
- The `image` crate and Notan's `to_file()` are separate systems that don't integrate easily

**Decision:** Too complex for the benefit gained. The WASM file-saving interop would add significant complexity.

---

### Option C: Supersampling with Hybrid Downsampling (CHOSEN SOLUTION)

Render to a larger texture (e.g., 2x or 4x). On native, automatically downsample before final save. On WASM, save the larger image.

**Pros:**
- ✅ Excellent antialiasing quality through supersampling
- ✅ Smaller file sizes on native (automatic downsampling)
- ✅ Sketches don't need to know about downsampling - it's automatic
- ✅ Works in WASM (saves full supersampled image for browser download)
- ✅ Backward compatible - existing code continues to work
- ✅ Uses high-quality Lanczos3 filter for downsampling

**Cons:**
- Requires temporary file write/read on native (negligible performance impact)
- WASM gets larger files (but that's acceptable for browser downloads)
- Adds `image` crate dependency for native builds

**Decision:** ✅ **Selected** - Best balance of quality, file size, and cross-platform compatibility.

## Implementation

### 1. Modified `CapturingTexture` (notan_sketches/src/utils.rs)

Added supersampling support while maintaining backward compatibility:

```rust
pub struct CapturingTexture {
    pub render_texture: RenderTexture,
    pub capture_to: String,
    pub capture_interval: f32,
    pub last_capture: f32,
    pub capture_lock: bool,
    pub num_captures: u32,
    pub supersample_factor: f32,  // NEW: e.g., 2.0 for 2x
}

impl CapturingTexture {
    fn create_render_texture(
        gfx: &mut Graphics,
        work_size: &Vec2,
        bgcolor: Color,
        supersample_factor: f32,
    ) -> RenderTexture {
        let width = (work_size.x * supersample_factor) as u32;
        let height = (work_size.y * supersample_factor) as u32;
        let render_texture = gfx
            .create_render_texture(width, height)
            .with_filter(TextureFilter::Linear, TextureFilter::Linear)  // Important!
            .build()
            .unwrap();
        // ... clear and return
    }

    // Backward compatible - defaults to 1.0 (no supersampling)
    pub fn new(...) -> Self {
        Self::new_with_supersample(..., 1.0)
    }

    // New constructor with supersample control
    pub fn new_with_supersample(
        gfx: &mut Graphics,
        work_size: &Vec2,
        bgcolor: Color,
        capture_to: String,
        capture_interval: f32,
        supersample_factor: f32,
    ) -> Self {
        // Creates texture at work_size * supersample_factor
    }
}
```

**Key points:**
- `TextureFilter::Linear` is crucial for good quality when scaling
- Backward compatible via `new()` defaulting to 1.0x
- Supersample factor is stored for potential future use

### 2. Updated Example (bobas-nightmare.rs)

```rust
if state.capture_next_draw {
    // Use 2x supersampling for better antialiasing in captures
    let supersample_factor = 2.0;
    let mut capture = CapturingTexture::new_with_supersample(
        gfx,
        &state.work_size,
        BG_COLOR,
        format!("renders/bobas-nightmare/{}", state.current_seed),
        0.0,
        supersample_factor,
    );

    // Render the existing draw to the supersampled texture
    // Notan automatically scales the draw commands to the larger texture
    gfx.render_to(&capture.render_texture, &state.draw);

    capture.capture(app, gfx);
    log::info!("Capture completed with {}x supersampling", supersample_factor);
    state.capture_next_draw = false;
}
```

**Key insights:**
- No need to redraw at a different scale! The existing `state.draw` can be rendered directly to the larger texture - Notan handles the coordinate transformation automatically.
- Downsampling happens automatically in `capture()` on native platforms - sketches don't need to know about it
- On WASM, the full supersampled image is provided for browser download (downsampling would require complex JavaScript interop)

## Implementation Details

### Native Platforms

On native (Linux, Windows, macOS), `capture()` automatically:
1. Saves the supersampled render texture to a temporary file
2. Loads it with the `image` crate
3. Downsamples using Lanczos3 filter (high quality)
4. Saves the downsampled version to the final filepath
5. Removes the temporary file

This happens transparently - sketches just call `capture()` and get antialiased, reasonably-sized files.

### WASM Platform

On WASM, `capture()` simply:
1. Saves the full supersampled render texture
2. Triggers browser download via Notan's built-in mechanism

The larger file size is acceptable for browser downloads, and avoids the complexity of JavaScript interop for pixel manipulation.

## Results

- **Quality**: Significant improvement in edge quality - smooth antialiased edges instead of jagged lines
- **File size (native)**: Same as original work_size thanks to automatic downsampling
- **File size (WASM)**: Approximately 4x larger for 2x supersampling, acceptable for browser downloads
- **Performance**: Minimal impact - temp file I/O during capture is negligible
- **Cross-platform**: Works on native Linux, Windows, macOS, and WASM targets

## Recommendations

### Supersample Factors

- **1x**: No antialiasing (current window render quality with multisampling)
- **2x**: Good balance - 4x file size, excellent quality improvement
- **4x**: Maximum quality - 16x file size, diminishing returns
- **1.5x**: Conservative option - 2.25x file size, moderate improvement

**Recommended**: Use 2x for most cases.

### When to Use Supersampling

- ✅ Final artwork captures for printing or portfolio
- ✅ High-quality screenshots for sharing
- ✅ When edge quality is critical (geometric art, line art)
- ❌ Quick iteration/testing captures (use 1x)
- ❌ When file size is critical (use 1x, or downsample offline)

### Future Considerations

If Notan adds native MSAA support for render textures in the future:
1. Check if `RenderTextureBuilder` gains a `.with_samples()` or similar method
2. If available, that would be preferable to supersampling (better performance, similar quality)
3. The current supersampling approach can remain as a fallback for higher quality

## Related Files

- Implementation: `notan_sketches/src/utils.rs` (CapturingTexture)
- Example usage: `notan_sketches/examples/bobas-nightmare.rs`
- Can be applied to: `notan_sketches/examples/radial_pointillist.rs` and other examples using CapturingTexture

## References

- Notan documentation: https://docs.rs/notan/0.13.0/notan/
- Notan GitHub: https://github.com/Nazariglez/notan
- Related discussion: This solution was developed when investigating multisampling support for render textures (October 2025)
