use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::{get_common_win_config, get_draw_setup};

// const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);


// // #[derive(AppState, Default)]
// #[derive(AppState)]
// struct State {}


// fn update(state: &mut State) {
//     manage_num_segs(state);
//     update_head_movement(state);
// }


fn draw(
    // app: &mut App,
    gfx: &mut Graphics,
    // state: &mut State,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    let mut depth = 2.0;
    // depth == 1
    draw.triangle(
        (WORK_SIZE.x / 2.0, 0.0),
        (WORK_SIZE.x, WORK_SIZE.y),
        (0.0, WORK_SIZE.y),
    )
    .color(Color::BLACK)
    .fill();

    // depth == 2
    draw.triangle(
        (WORK_SIZE.x / 2.0, 0.0),
        (WORK_SIZE.x, WORK_SIZE.y / depth),
        (0.0, WORK_SIZE.y / depth),
    )
    .color(Color::BLACK)
    .fill();


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
