use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);


// // #[derive(AppState, Default)]
// #[derive(AppState)]
// struct State {}


// fn update(state: &mut State) {
//     manage_num_segs(state);
//     update_head_movement(state);
// }


fn get_vec2_midpoint(vec_a: Vec2, vec_b: Vec2) -> Vec2 {
    vec2((vec_a.x + vec_b.x) / 2.0, (vec_a.y + vec_b.y) / 2.0)
}


fn draw_gasket(draw: &mut Draw, a: Vec2, b: Vec2, c: Vec2, curr_depth: usize, max_depth: usize) {
    if max_depth == 0 {
        draw.triangle((a.x, a.y), (b.x, b.y), (c.x, c.y))
            .color(Color::BLACK)
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
        if curr_depth + 1 == max_depth {
            draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
                .color(Color::BLACK)
                .fill();

            draw.triangle((a2.x, a2.y), (b2.x, b2.y), (c2.x, c2.y))
                .color(Color::BLACK)
                .fill();

            draw.triangle((a3.x, a3.y), (b3.x, b3.y), (c3.x, c3.y))
                .color(Color::BLACK)
                .fill();
        } else {
            draw_gasket(draw, a1, b1, c1, curr_depth + 1, max_depth);
            draw_gasket(draw, a2, b2, c2, curr_depth + 1, max_depth);
            draw_gasket(draw, a3, b3, c3, curr_depth + 1, max_depth);
        }
    }
}


fn draw(
    // app: &mut App,
    gfx: &mut Graphics,
    // state: &mut State,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    let a = vec2(WORK_SIZE.x / 2.0, 0.0);
    let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
    let c = vec2(0.0, WORK_SIZE.y);
    draw_gasket(&mut draw, a, b, c, 0, 3);

    // depth == 0
    // let a = vec2(WORK_SIZE.x / 2.0, 0.0);
    // let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
    // let c = vec2(0.0, WORK_SIZE.y);
    // draw.triangle((a.x, a.y), (b.x, b.y), (c.x, c.y))
    //     .color(Color::BLACK)
    //     .fill();

    // depth == 1
    // let mid_ab: Vec2 = get_vec2_midpoint(a, b);
    // let mid_ac: Vec2 = get_vec2_midpoint(a, c);

    // draw.triangle((a.x, a.y), (mid_ab.x, mid_ab.y), (mid_ac.x, mid_ac.y))
    //     .color(Color::BLACK)
    //     .fill();


    // draw.triangle((mid_ab.x, mid_ab.y), (b.x, b.y), (a.x, b.y))
    //     .color(Color::BLACK)
    //     .fill();


    // draw.triangle((mid_ac.x, mid_ac.y), (c.x, c.y), (a.x, c.y))
    //     .color(Color::BLACK)
    //     .fill();

    // draw to screen
    gfx.render(&draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    // notan::init_with(init)
    notan::init()
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
