# hilo_smoove Shader Integration

This document describes the shader integration implementation in `hilo_smoove.rs`, which randomly applies the `horizontal_city` shader to animated bezier-curve strips.

## Architecture Overview

Each `Strip` can optionally have its own shader applied via a `shader_rt: Option<ShaderRenderTexture>` field. This per-strip approach allows:
- Multiple strips with shaders to render without interfering with each other
- Individual alpha values per shader strip
- Future flexibility for strip-specific shader parameters or different shaders per strip

## Implementation Details

### Strip Structure
```rust
pub struct Strip {
    segs: Vec<Segment>,
    color: Color,
    stroke_color: Color,
    alpha: f32,
    last_distance: f32,
    displaced: bool,
    shader_rt: Option<ShaderRenderTexture>,     // Each strip gets its own render texture
    shader_curve_ubo: Option<Buffer>,           // Per-strip curve data uniform buffer
}
```

### Shader Assignment
Shaders are randomly assigned when strips are created in `add_strip()`:
- 30% probability per strip (`state.rng.gen_bool(0.3)`)
- Each shader strip gets its own `ShaderRenderTexture` created at strip creation time
- Assignment is re-randomized when shuffle is triggered (pressing 'r' or auto-shuffle)

### Rendering Approach
1. **Shader strips** are rendered by:
   - Drawing the strip's bezier path shape into its own `ShaderRenderTexture` as a white fill
   - The shader is applied to this white shape mask
   - The resulting texture is drawn to the screen with the strip's alpha value

2. **Non-shader strips** are rendered directly using standard draw commands with fill colors

### Key Functions

**`add_strip(state: &mut State, gfx: &mut Graphics)`**
- Creates strips with random shader assignment
- Instantiates `ShaderRenderTexture` for shader strips (dimensions match work_size)
- Creates per-strip curve uniform buffer for shader warping

**`sample_curve(strip: &Strip, work_size: Vec2, strip_height: f32) -> CurveData`**
- Samples the strip's Bezier curve at 32 regular intervals across the width
- Uses cubic Bezier interpolation to calculate y-position at each sample point
- Stores normalized y-offsets from the base position for shader use
- Returns `CurveData` struct packed into 8 Vec4 fields (to satisfy std140 uniform constraints)

**`draw_shader_strip(...)`**
- Samples the current curve state using `sample_curve()` and updates the curve uniform buffer
- Draws the strip's bezier path into the strip's shader render texture
- Fills the path with `Color::WHITE` as a mask
- Passes both common uniforms (time, resolution) and curve data to shader
- Renders the shader texture to screen with the strip's alpha

**`draw_strip(...)`**
- Standard rendering for non-shader strips
- Uses fill colors and stroke

## Shader Curve Warping

The shader patterns warp along the Bezier curves rather than remaining static:

### Curve Data Transmission
- Each shader strip has a `shader_curve_ubo` buffer containing curve profile data
- The `CurveData` struct contains:
  - 8 Vec4 fields (`s0` through `s7`) storing 32 curve samples (4 per Vec4)
  - `strip_y`: normalized base Y position of the strip
  - `strip_height`: normalized height of the strip
  - `num_samples`: number of valid samples (32)
- Structure uses individual Vec4 fields instead of arrays to work around std140 uniform buffer limitations

### Shader Implementation (horizontal_city.frag.glsl)
1. **`get_sample(int idx)`** - Unpacks a single sample from the Vec4 fields
2. **`get_curve_offset(float x_norm)`** - Interpolates curve offset at a given x position
3. **Pattern warping** - The shader adjusts texture coordinates: `st.y = st.y - curve_offset`

This makes the horizontal scrolling pattern follow the undulating waves of each strip's Bezier curve.

### Update Frequency
- Curve data is sampled and updated every frame in `draw_shader_strip()`
- As the Bezier control points animate during displacement, the shader pattern warps accordingly
- Creates organic, flowing visual integration between the strips and shader effects

## Global Shader Resources

While each strip has its own render texture and curve uniform buffer, some resources are shared globally in `State`:
- `shader_pipeline: Pipeline` - Loaded from `horizontal_city.frag.glsl`
- `shader_ubo: Buffer` - Common uniforms (u_time, u_resolution)

These are shared because:
- The same shader is used for all shader strips (currently)
- Time and resolution are global parameters
- Reduces resource overhead

## Future Extensibility

The per-strip `shader_rt` design allows for future enhancements:
- Different shaders for different strips (would need per-strip pipeline)
- Strip-specific shader parameters (would need per-strip uniform buffers)
- Dynamic shader switching based on strip properties or events
- Shader-based strip effects (e.g., displacement-triggered shader changes)

## Performance Considerations

- Each shader strip creates a full-screen `RenderTexture` (work_size dimensions)
- With 30% probability and ~20 strips on screen, this means ~6 render textures active
- Each render texture is cleared and redrawn every frame
- Future optimization: Could crop render textures to strip bounding boxes

## Differences from Single Render Texture Approach

**Previous (broken) approach:**
- Single global `ShaderRenderTexture` in `State`
- All shader strips drawn into same texture
- Only the last strip drawn was visible (texture cleared each draw call)
- Could not apply individual alpha values per strip

**Current approach:**
- Per-strip `ShaderRenderTexture`
- Each shader strip is independent
- Individual alpha values work correctly
- Slightly higher memory usage but necessary for correctness

## Related Files

- **Shader**: `examples/assets/shaders/horizontal_city.frag.glsl`
- **Test example**: `examples/test_horizontal_city_shader.rs`
- **Main implementation**: `notan_sketches/examples/hilo_smoove.rs`
