# Drawing Shapes with Shaders

**ShaderRenderTexture.draw()** allows you to draw shapes that have the shader applied to them. The shape acts as a mask for the shader output.

## Basic Usage

```rust
shader_rt.draw(
    gfx,
    &shader_pipeline,
    vec![&shader_ubo],  // Uniform buffers
    |shader_draw| {
        // Draw shapes here - they will have the shader applied
        shader_draw.rect((0.0, 0.0), (100.0, 100.0))
            .fill_color(Color::WHITE)  // Typically use white as mask
            .fill();
    },
);

// Then draw the resulting texture to the screen
draw.image(&shader_rt.rt)
    .position(0.0, 0.0)
    .size(work_size.x, work_size.y);
```

## Drawing Bezier Paths with Shaders

```rust
shader_rt.draw(gfx, &pipeline, vec![&ubo], |shader_draw| {
    let path = &mut shader_draw.path();

    // Start the path
    path.move_to(start_x, start_y);

    // Draw bezier curves
    path.cubic_bezier_to(
        (cp1_x, cp1_y),
        (cp2_x, cp2_y),
        (end_x, end_y)
    );

    // Close and fill - shader is applied to the filled shape
    path.close()
        .fill_color(Color::WHITE)
        .fill();
});
```

## Key Points

- Shapes are typically filled with `Color::WHITE` to act as a full-opacity mask
- The shader output replaces the fill color
- Any shape can be drawn: rectangles, circles, paths, bezier curves, etc.
- See `eg_frag_shader.rs` and `hilo_smoove.rs` for examples
