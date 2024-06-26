---
published: '2023-03-16T11:38:17.885926-04:00'
---

To display controls, click or tap on the sketch.

This is a variation on the [Sierpinski Gasket](sierpinski_gasket.html) fractal. By changing the positions of the triangle vertices in the simple manner shown below, a pretty interesting "spiralling" effect is achieved: 

```
fn vary_triangle(a: Vec2, b: Vec2, c: Vec2) -> (Vec2, Vec2, Vec2) {
    (
        vec2(a.x * 1.2, a.y * 1.0),
        vec2(b.x * 1.0, b.y * 0.8),
        vec2(c.x * 1.0, c.y * 1.0),
    )
}
```

The `vary_triangle()` function is applied on each iteration.

[See full source on Github](https://github.com/riverfr0zen/sketches/blob/main/notan_sketches/src/fractals/sierpinski.rs).