---
published: '2023-03-14T13:19:17.673173-04:00'
---

A variation of the [Sierpinski Gasket](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle) fractal. The fractal is a basic example of a self-similar set -- a mathematically generated pattern that repeats on every iteration, infinitely.

**Controls:**
* Press the 'Up' Key or Swipe Up to increase number of iterations
* Press the 'Down' Key or Swipe Down to decrease iterations
* Press the 'R' Key or Swipe Left to reset

**Variation**

This variation shows how some very simple changes (such as slightly changing the positions of some triangle vertices) can result in this "bushy" outcome after iterating.

Here are some code diffs illustrating said changes:

```
< let a = vec2(WORK_SIZE.x / 2.0, 0.0);
< let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
< let c = vec2(0.0, WORK_SIZE.y);
---
> // variation (/ 2.0 -> / 3.0)
> let a = vec2(WORK_SIZE.x / 3.0, 0.0);
> let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
> // variation (+ 20.0)
> let c = vec2(0.0, WORK_SIZE.y + 20.0);
```

```
<         let b2 = vec2(b.x, b.y);
---
>         // variation (+ 10.0)
>         let b2 = vec2(b.x, b.y + 10.0);
```

[See full source on Github](https://github.com/riverfr0zen/sketches/blob/main/notan_sketches/src/fractals/sierpinski.rs).