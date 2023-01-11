///
/// Recreation of (and variations on) "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::schotter::*;
use notan_sketches::utils::{get_common_win_config, ScreenDimensions};

// const WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
const COLS: u8 = 12;
const ROWS: u8 = 22;
// const COLS: u8 = 48;
// const ROWS: u8 = 88;
// const COLS: u8 = 22;
// const ROWS: u8 = 12;
// Minimum padding
const PADDING: f32 = 50.0;
// Rotation increment in degrees
// const RAND_STEP: f32 = 0.22;
const RAND_STEP: f32 = 0.022;
// const RAND_STEP: f32 = 0.0022;
// const RAND_STEP: f32 = 0.00022;
// Soften random effect for translation
// const DAMPEN: f32 = 0.45;
// const DAMPEN: f32 = 0.045;
const DAMPEN: f32 = 4.5;
// const DAMPEN: f32 = 450.0;


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    draw_basic(gfx, state, WORK_SIZE, DAMPEN)
}


fn init(gfx: &mut Graphics) -> State {
    init_basic(gfx, WORK_SIZE, PADDING, ROWS, COLS, RAND_STEP)
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .high_dpi(true)
        .vsync(true)
        .size(WORK_SIZE.x as i32, WORK_SIZE.y as i32)
        .title("Schotter");

    // Basic reproduction
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .event(event)
        .update(update_common)
        .draw(draw)
        .build()
}
