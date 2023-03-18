---
published: '2023-03-14T13:19:17.673173-04:00'
---

To display controls, click or tap on the sketch.

This is a variation on the [Sierpinski Gasket](sierpinski_gasket.html) fractal. This variation shows how some very simple changes (such as slightly changing the positions of some triangle vertices) can result in this "bushy" outcome after iterating.

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