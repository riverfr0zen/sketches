# Package Structure

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
