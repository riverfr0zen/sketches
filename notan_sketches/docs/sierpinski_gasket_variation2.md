---
published: '2023-03-16T11:38:17.885926-04:00'
---

A variation of the [Sierpinski Gasket](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle) fractal. The fractal is a basic example of a self-similar set -- a mathematically generated pattern that repeats on every iteration, infinitely.

**Controls:**
* Press the 'Up' Key or Swipe Up to increase number of iterations
* Press the 'Down' Key or Swipe Down to decrease iterations
* Press the 'R' Key or Swipe Left to reset

**Variation**

By changing the positions of the triangle vertices in the simple manner shown below, a pretty interesting "spiralling" effect is achieved: 

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