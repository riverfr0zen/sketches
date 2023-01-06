///
/// Recreation of (and variations on) "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::colors::{
    BANANA, CARMINE, GRAYPURP, MAHOGANY, OLIVE, SAFFRON, SALMON, SCARLET,
};
use notan_sketches::schotter::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng, ScreenDimensions};

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
// Frequency of change in rand_step
const STEP_FREQ: f32 = 0.07;
// Frequency of change cols+rows
const EXPANSION_FREQ: f32 = 0.05;


fn _draw_solid2(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
    freeze_on_render: bool,
) {
    if !state.freeze {
        // let mut draw = get_draw_setup(gfx, WORK_SIZE, true, MAHOGANY);
        let mut draw = get_draw_setup(gfx, WORK_SIZE, true, GRAYPURP);

        // Cumulative rotation value
        let mut rand_sum = 0.0;

        for row in 0..state.rows {
            rand_sum += (row + 1) as f32 * (state.rand_step * 0.05);
            for col in 0..state.cols {
                let mut rand_val = 0.0;
                if rand_sum > 0.0 {
                    rand_val = state.rng.gen_range(-rand_sum..rand_sum);
                }

                let xpos = col as f32 * state.tile_size + state.hpadding;
                let ypos = row as f32 * state.tile_size + state.vpadding;

                draw.image(&state.box_texture)
                    .position(xpos, ypos)
                    // Need to rotate from the center of the image, which doesn't seem to be the
                    // default.
                    .rotate_from(
                        (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
                        rand_val,
                    )
                    // .color(Color::RED);
                    .color(CARMINE)
                    .size(state.tile_size, state.tile_size);
            }
        }

        // Reset rotation value
        rand_sum = 0.0;

        for row in 0..state.rows {
            rand_sum += (row + 1) as f32 * state.rand_step;
            for col in 0..state.cols {
                let mut rand_val = 0.0;
                if rand_sum > 0.0 {
                    rand_val = state.rng.gen_range(-rand_sum..rand_sum);
                }

                let mut xpos = col as f32 * state.tile_size + state.hpadding;
                let mut ypos = row as f32 * state.tile_size + state.vpadding;

                xpos += rand_val * (DAMPEN * 0.1);
                ypos += rand_val * (DAMPEN * 0.1);


                draw.image(&state.box_texture)
                    .position(xpos, ypos)
                    // Need to rotate from the center of the image, which doesn't seem to be the
                    // default.
                    .rotate_from(
                        (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
                        rand_val,
                    )
                    // .color(Color::BLUE)
                    .color(SCARLET)
                    .size(state.tile_size, state.tile_size);


                xpos += rand_val * (DAMPEN * 0.3);
                ypos += rand_val * (DAMPEN * 0.3);


                draw.image(&state.box_texture)
                    .position(xpos, ypos)
                    // Need to rotate from the center of the image, which doesn't seem to be the
                    // default.
                    .rotate_from(
                        (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
                        rand_val,
                    )
                    // .color(Color::GREEN)
                    .color(SAFFRON)
                    .size(state.tile_size, state.tile_size);


                xpos += rand_val * DAMPEN;
                ypos += rand_val * DAMPEN;


                draw.image(&state.box_texture)
                    .position(xpos, ypos)
                    // Need to rotate from the center of the image, which doesn't seem to be the
                    // default.
                    .rotate_from(
                        (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
                        rand_val,
                    )
                    .color(BANANA)
                    .size(state.tile_size, state.tile_size);
            }
        }


        gfx.render(&draw);
        state.freeze = freeze_on_render;
        // log::debug!("fps: {}", app.timer.fps().round());
    }
}


fn draw_solid2(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    _draw_solid2(gfx, state, true);
}


fn draw_solid2_anim(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    _draw_solid2(gfx, state, false);
}


fn update_anim(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        state.freeze = false;
        log::debug!("Freeze released");
    }

    let time_since_init = app.timer.time_since_init();
    let step_mod = ((time_since_init * STEP_FREQ).sin().abs() * 10.0) as u8;
    // Previously _draw_solid2() would crash if state.rand_step was 0.
    // state.rand_step = (step_mod + 1) as f32 * RAND_STEP / 10.0;
    state.rand_step = step_mod as f32 * RAND_STEP / 10.0;
    let expansion_mod = ((time_since_init * EXPANSION_FREQ).sin().abs() * 10.0) as u8;
    state.rows = ROWS + expansion_mod * 8;
    state.cols = COLS + expansion_mod * 4;

    log::debug!(
        "expansion modifier {}, rows: {}, cols: {}, step modifier {}, rand_step: {}",
        expansion_mod,
        state.rows,
        state.cols,
        step_mod,
        state.rand_step,
    );

    (
        state.display_width,
        state.tile_size,
        state.display_height,
        state.vpadding,
        state.hpadding,
    ) = State::reframe(WORK_SIZE, PADDING, state.rows, state.cols);
}


fn init(gfx: &mut Graphics) -> State {
    init_solid(gfx, WORK_SIZE, PADDING, ROWS, COLS, RAND_STEP)
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    draw_solid(gfx, state, WORK_SIZE, DAMPEN, CARMINE)
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .high_dpi(true)
        .vsync(true)
        .size(WORK_SIZE.x as i32, WORK_SIZE.y as i32);

    // // Basic reproduction
    // notan::init_with(init)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update_common)
    //     .draw(draw)
    //     .build()

    // Solid variant 1
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .event(event)
        .update(update_common)
        .draw(draw)
        .build()

    // // Solid variant 2
    // notan::init_with(init_solid)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update_common)
    //     .draw(draw_solid2)
    //     .build()

    // // Solid variant 2 animated
    // notan::init_with(init_solid)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update_anim)
    //     .draw(draw_solid2_anim)
    //     .build()
}
