use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);

#[derive(AppState)]
struct State {
    circle_pos: Vec2,
}

fn init(_gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    // Generate random position within the work size
    let circle_pos = vec2(
        rng.gen_range(0.0..WORK_SIZE.x),
        rng.gen_range(0.0..WORK_SIZE.y),
    );

    State { circle_pos }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    // Set up draw with scaling projection (aspect_fit = false)
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    // Draw blue circle at random position
    draw.circle(100.0)
        .position(state.circle_pos.x, state.circle_pos.y)
        .color(Color::BLUE);

    // Render to screen
    gfx.render(&draw);
}
