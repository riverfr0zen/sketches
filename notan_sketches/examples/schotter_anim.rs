///
/// Recreation of (and variations on) "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::colors::{BANANA, CARMINE, PEACOCK, SAFFRON, SCARLET};
use notan_sketches::schotter::*;
use notan_sketches::utils::{get_common_win_config, set_html_bgcolor, ScreenDimensions};

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
const RAND_STEP: f32 = 0.22;
// const RAND_STEP: f32 = 0.022;
// const RAND_STEP: f32 = 0.0022;
// const RAND_STEP: f32 = 0.000022;
// Soften random effect for translation
const DAMPEN: f32 = 0.45;
// const DAMPEN: f32 = 0.045;
// const DAMPEN: f32 = 4.5;
// const DAMPEN: f32 = 4500.0;
// Frequency of change in rand_step
// const STEP_FREQ: f32 = 0.07;
const STEP_FREQ: f32 = 1.2;
// const STEP_FREQ: f32 = 2.0;
// Frequency of change cols+rows
// const EXPANSION_FREQ: f32 = 0.05;
// const EXPANSION_FREQ: f32 = 0.5;
const EXPANSION_FREQ: f32 = 0.25;
// The smaller this value, the less displacement occurs during "stable" period
const STABLE_TIME_MOD: f32 = 0.05;

fn init(gfx: &mut Graphics) -> State {
    init_solid(gfx, WORK_SIZE, PADDING, ROWS, COLS, RAND_STEP)
}

fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    draw_solid2_anim(
        // gfx, state, WORK_SIZE, DAMPEN, GRAYPURP, CARMINE, SCARLET, SAFFRON,
        // BANANA,
        gfx, state, WORK_SIZE, DAMPEN, PEACOCK, BANANA, SAFFRON, SCARLET, CARMINE,
    )
}

fn update(app: &mut App, state: &mut State) {
    update_anim(
        app,
        state,
        WORK_SIZE,
        PADDING,
        ROWS,
        COLS,
        RAND_STEP,
        STEP_FREQ,
        EXPANSION_FREQ,
        STABLE_TIME_MOD,
    )
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(WORK_SIZE.x as u32, WORK_SIZE.y as u32)
        .set_title("Schotter (animated)");

    set_html_bgcolor(PEACOCK);

    // Solid variant 2 animated
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}
