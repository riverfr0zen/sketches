use crate::mathutils::get_vec2_midpoint;
use notan::draw::*;
// use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;


#[derive(AppState)]
pub struct State {
    pub max_depth: usize,
}


impl Default for State {
    fn default() -> Self {
        State { max_depth: 0 }
    }
}


pub fn draw_gasket(
    draw: &mut Draw,
    state: &mut State,
    a: Vec2,
    b: Vec2,
    c: Vec2,
    curr_depth: usize,
) {
    if state.max_depth == 0 {
        draw.triangle((a.x, a.y), (b.x, b.y), (c.x, c.y))
            .color(Color::PURPLE)
            .fill();
    } else {
        let mid_ab: Vec2 = get_vec2_midpoint(a, b);
        let mid_ac: Vec2 = get_vec2_midpoint(a, c);

        let a1 = vec2(a.x, a.y);
        let b1 = vec2(mid_ab.x, mid_ab.y);
        let c1 = vec2(mid_ac.x, mid_ac.y);

        let a2 = vec2(mid_ab.x, mid_ab.y);
        let b2 = vec2(b.x, b.y);
        let c2 = vec2(a.x, b.y);

        let a3 = vec2(mid_ac.x, mid_ac.y);
        let b3 = vec2(c.x, c.y);
        let c3 = vec2(a.x, c.y);
        if curr_depth + 1 == state.max_depth {
            draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
                .color(Color::PURPLE)
                .fill();

            draw.triangle((a2.x, a2.y), (b2.x, b2.y), (c2.x, c2.y))
                .color(Color::RED)
                .fill();

            draw.triangle((a3.x, a3.y), (b3.x, b3.y), (c3.x, c3.y))
                .color(Color::GREEN)
                .fill();
        } else {
            draw_gasket(draw, state, a1, b1, c1, curr_depth + 1);
            draw_gasket(draw, state, a2, b2, c2, curr_depth + 1);
            draw_gasket(draw, state, a3, b3, c3, curr_depth + 1);
        }
    }
}


pub fn draw_bushy_gasket(
    draw: &mut Draw,
    state: &mut State,
    a: Vec2,
    b: Vec2,
    c: Vec2,
    curr_depth: usize,
) {
    if state.max_depth == 0 {
        draw.triangle((a.x, a.y), (b.x, b.y), (c.x, c.y))
            .color(Color::PINK)
            .fill();
    } else {
        let mid_ab: Vec2 = get_vec2_midpoint(a, b);
        let mid_ac: Vec2 = get_vec2_midpoint(a, c);

        let a1 = vec2(a.x, a.y);
        let b1 = vec2(mid_ab.x, mid_ab.y);
        let c1 = vec2(mid_ac.x, mid_ac.y);

        let a2 = vec2(mid_ab.x, mid_ab.y);
        // variation (+ 10.0)
        let b2 = vec2(b.x, b.y + 10.0);
        let c2 = vec2(a.x, b.y);

        let a3 = vec2(mid_ac.x, mid_ac.y);
        let b3 = vec2(c.x, c.y);
        let c3 = vec2(a.x, c.y);
        if curr_depth + 1 == state.max_depth {
            draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
                .color(Color::PINK)
                .fill();

            draw.triangle((a2.x, a2.y), (b2.x, b2.y), (c2.x, c2.y))
                .color(Color::RED)
                .fill();

            draw.triangle((a3.x, a3.y), (b3.x, b3.y), (c3.x, c3.y))
                .color(Color::GREEN)
                .fill();
        } else {
            draw_bushy_gasket(draw, state, a1, b1, c1, curr_depth + 1);
            draw_bushy_gasket(draw, state, a2, b2, c2, curr_depth + 1);
            draw_bushy_gasket(draw, state, a3, b3, c3, curr_depth + 1);
        }
    }
}


fn vary_triangle(a: Vec2, b: Vec2, c: Vec2) -> (Vec2, Vec2, Vec2) {
    (
        vec2(a.x * 1.2, a.y * 1.0),
        vec2(b.x * 1.0, b.y * 0.8),
        vec2(c.x * 1.0, c.y * 1.0),
    )
}


pub fn draw_varied_gasket(
    draw: &mut Draw,
    state: &mut State,
    a: Vec2,
    b: Vec2,
    c: Vec2,
    curr_depth: usize,
) {
    if state.max_depth == 0 {
        let (a1, b1, c1) = vary_triangle(a, b, c);
        draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
            .color(Color::PINK)
            .fill();
    } else {
        let mid_ab: Vec2 = get_vec2_midpoint(a, b);
        let mid_ac: Vec2 = get_vec2_midpoint(a, c);

        let mut a1 = vec2(a.x, a.y);
        let mut b1 = vec2(mid_ab.x, mid_ab.y);
        let mut c1 = vec2(mid_ac.x, mid_ac.y);

        let mut a2 = vec2(mid_ab.x, mid_ab.y);
        let mut b2 = vec2(b.x, b.y);
        let mut c2 = vec2(a.x, b.y);

        let mut a3 = vec2(mid_ac.x, mid_ac.y);
        let mut b3 = vec2(c.x, c.y);
        let mut c3 = vec2(a.x, c.y);

        if curr_depth + 1 == state.max_depth {
            (a1, b1, c1) = vary_triangle(a1, b1, c1);
            (a2, b2, c2) = vary_triangle(a2, b2, c2);
            (a3, b3, c3) = vary_triangle(a3, b3, c3);

            draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
                .color(Color::PINK)
                .fill();

            draw.triangle((a2.x, a2.y), (b2.x, b2.y), (c2.x, c2.y))
                .color(Color::RED)
                .fill();

            draw.triangle((a3.x, a3.y), (b3.x, b3.y), (c3.x, c3.y))
                .color(Color::GREEN)
                .fill();
        } else {
            draw_varied_gasket(draw, state, a1, b1, c1, curr_depth + 1);
            draw_varied_gasket(draw, state, a2, b2, c2, curr_depth + 1);
            draw_varied_gasket(draw, state, a3, b3, c3, curr_depth + 1);
        }
    }
}
