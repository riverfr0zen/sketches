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
    shader_rt: Option<ShaderRenderTexture>,  // Each strip gets its own render texture
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

**`draw_shader_strip(...)`**
- Draws the strip's bezier path into the strip's shader render texture
- Fills the path with `Color::WHITE` as a mask
- Renders the shader texture to screen with the strip's alpha

**`draw_strip(...)`**
- Standard rendering for non-shader strips
- Uses fill colors and stroke

## Global Shader Resources

While each strip has its own render texture, shader pipeline and uniform buffer are shared globally in `State`:
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
