# Shader System

Custom shader utilities in `src/shaderutils.rs`:

- **ShaderRenderTexture**: Wrapper for rendering with custom shaders
  - Supports up to 5 uniform buffers (hacky implementation noted in code)
  - Provides `draw()` and `draw_filled()` methods
- **create_hot_shape_pipeline()**: Creates shader pipelines from file paths
- **ShaderReloadManager**: Hot reload support for shader development (frame-based polling)
- **CommonData uniform**: Standard time and resolution uniforms for shader effects

**Uniform Buffer Binding Index Note**: In Notan, uniform buffer binding indices in Rust code start at **1**, not 0. Even though shaders use `layout(binding = 0)`, the first parameter to `create_uniform_buffer()` should be `1`. Subsequent uniforms use indices 2, 3, 4, etc. See `color_points_shader.rs` and `erratic_wave_shader.rs` for examples.

**Coordinate System Note**: When working with Notan's shader render textures, use normalized coordinates (`gl_FragCoord.xy / u_resolution`) directly without manual Y-flipping. The `RenderTexture` system already handles coordinate transformations internally - manual Y-flips will cause misalignment.