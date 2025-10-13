use notan::draw::*;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init()
        .add_config(win_config)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics) {
    // Set up draw with scaling projection (aspect_fit = false)
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    // Draw blue circle in the center
    draw.circle(100.0)
        .position(WORK_SIZE.x / 2.0, WORK_SIZE.y / 2.0)
        .color(Color::BLUE);

    // Render to screen
    gfx.render(&draw);
}
