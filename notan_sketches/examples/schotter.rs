///
/// Recreation of (and variations on) "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
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
const STROKE_WIDTH: f32 = 4.0;
// Rotation increment in degrees
// const RAND_STEP: f32 = 0.22;
const RAND_STEP: f32 = 0.022;
// const RAND_STEP: f32 = 0.0022;
// Soften random effect for translation
// const DAMPEN: f32 = 0.45;
// const DAMPEN: f32 = 0.045;
const DAMPEN: f32 = 4.5;
// Reds Dark-Light
const GRAYPURP: Color = Color::new(0.29, 0.26, 0.36, 1.0);
const MAHOGANY: Color = Color::new(0.26, 0.05, 0.04, 1.0);
const CARMINE: Color = Color::new(0.59, 0.0, 0.09, 1.0);
const SCARLET: Color = Color::new(1.0, 0.14, 0.0, 1.0);
const SALMON: Color = Color::new(0.98, 0.5, 0.45, 1.0);
// Yellows
const OLIVE: Color = Color::new(0.5, 0.5, 0.0, 1.0);
const SAFFRON: Color = Color::new(0.98, 0.54, 0.09, 1.0);
const BANANA: Color = Color::new(1.0, 0.88, 0.21, 1.0);


// Visualization modifier
enum VizMod {
    BASIC,
    SOLID,
}


fn _create_box_texture(gfx: &mut Graphics, tile_size: f32, vizmod: VizMod) -> Texture {
    let rt = gfx
        .create_render_texture(tile_size as i32, tile_size as i32)
        .build()
        .unwrap();

    let tile_size = tile_size as f32;
    let mut draw = gfx.create_draw();
    draw.set_size(tile_size, tile_size);
    match vizmod {
        VizMod::SOLID => {
            draw.rect((0.0, 0.0), (tile_size, tile_size))
                .fill_color(Color::WHITE)
                .fill()
                // .stroke_color(Color::BLACK)
                // .stroke_color(Color::new(0.5, 0.5, 0.5, 1.0))
                .stroke_color(Color::new(0.8, 0.8, 0.8, 1.0))
                .stroke(STROKE_WIDTH);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
        _ => {
            draw.clear(Color::TRANSPARENT);
            draw.rect((0.0, 0.0), (tile_size, tile_size))
                .color(Color::BLACK)
                .stroke(STROKE_WIDTH);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
    }
}


fn create_basic_box_texture(gfx: &mut Graphics, tile_size: f32) -> Texture {
    _create_box_texture(gfx, tile_size, VizMod::BASIC)
}


fn create_solid_box_texture(gfx: &mut Graphics, tile_size: f32) -> Texture {
    _create_box_texture(gfx, tile_size, VizMod::SOLID)
}


#[derive(AppState)]
pub struct State {
    pub display_height: f32,
    pub display_width: f32,
    pub hpadding: f32,
    pub vpadding: f32,
    pub tile_size: f32,
    pub box_texture: Texture,
    pub rng: Random,
    pub freeze: bool,
    pub rand_step: f32,
    pub cols: u8,
    pub rows: u8,
}


impl State {
    fn reframe(num_rows: u8, num_cols: u8) -> (f32, f32, f32, f32, f32) {
        let display_height: f32;
        let tile_size: f32;
        let display_width: f32;
        let hpadding: f32;
        let vpadding: f32;

        if num_rows > num_cols {
            display_height = WORK_SIZE.y - PADDING * 2.0;
            tile_size = display_height / num_rows as f32;
            display_width = tile_size * num_cols as f32;
            hpadding = (WORK_SIZE.x - display_width) * 0.5;
            vpadding = PADDING;
        } else {
            display_width = WORK_SIZE.x - PADDING * 2.0;
            tile_size = display_width / num_cols as f32;
            display_height = tile_size * num_rows as f32;
            vpadding = (WORK_SIZE.y - display_height) * 0.5;
            hpadding = PADDING;
        }
        (display_width, tile_size, display_height, vpadding, hpadding)
    }

    fn _new(gfx: &mut Graphics, box_texture_fn: &dyn Fn(&mut Graphics, f32) -> Texture) -> Self {
        // let display_height: f32;
        // let tile_size: f32;
        // let display_width: f32;
        // let hpadding: f32;
        // let vpadding: f32;

        let (display_width, tile_size, display_height, vpadding, hpadding) =
            Self::reframe(ROWS, COLS);
        // if ROWS > COLS {
        //     display_height = WORK_SIZE.y - PADDING * 2.0;
        //     tile_size = display_height / ROWS as f32;
        //     display_width = tile_size * COLS as f32;
        //     hpadding = (WORK_SIZE.x - display_width) * 0.5;
        //     vpadding = PADDING;
        // } else {
        //     display_width = WORK_SIZE.x - PADDING * 2.0;
        //     tile_size = display_width / COLS as f32;
        //     display_height = tile_size * ROWS as f32;
        //     vpadding = (WORK_SIZE.y - display_height) * 0.5;
        //     hpadding = PADDING;
        // }

        let box_texture = box_texture_fn(gfx, tile_size);
        let (rng, seed) = get_rng(None);
        log::debug!("seed: {}", seed);
        Self {
            display_height: display_height,
            display_width: display_width,
            hpadding: hpadding,
            vpadding: vpadding,
            tile_size: tile_size,
            box_texture: box_texture,
            rng: rng,
            freeze: false,
            rand_step: RAND_STEP,
            cols: COLS,
            rows: ROWS,
        }
    }

    fn new_basic(gfx: &mut Graphics) -> Self {
        Self::_new(gfx, &create_basic_box_texture)
    }


    fn new_solid(gfx: &mut Graphics) -> Self {
        Self::_new(gfx, &create_solid_box_texture)
    }
}


fn draw_basic(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    if !state.freeze {
        let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::WHITE);

        // Cumulative rotation value
        let mut rand_sum = 0.0;

        for row in 0..state.rows {
            rand_sum += (row + 1) as f32 * state.rand_step;
            for col in 0..state.cols {
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);
                let xpos = col as f32 * state.tile_size + state.hpadding + (rand_val * DAMPEN);
                let ypos = row as f32 * state.tile_size + state.vpadding + (rand_val * DAMPEN);
                // let xpos = col as f32 * state.tile_size + state.hpadding;
                // let ypos = row as f32 * state.tile_size + state.vpadding;
                draw.image(&state.box_texture)
                    .position(xpos, ypos)
                    // Need to rotate from the center of the image, which doesn't seem to be the
                    // default.
                    .rotate_from(
                        (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
                        rand_val,
                    )
                    .size(state.tile_size, state.tile_size);
            }
        }

        gfx.render(&draw);
        state.freeze = true;
        // log::debug!("fps: {}", app.timer.fps().round());
    }
}


fn draw_solid(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    if !state.freeze {
        // let mut draw = get_draw_setup(gfx, WORK_SIZE, true, MAHOGANY);
        let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::GRAY);

        // Cumulative rotation value
        let mut rand_sum = 0.0;

        // for row in 0..state.rows {
        //     rand_sum += (row + 1) as f32 * state.rand_step;
        //     for col in 0..state.cols {
        //         let rand_val = state.rng.gen_range(-rand_sum..rand_sum);

        //         let mut xpos = col as f32 * state.tile_size + state.hpadding;
        //         let mut ypos = row as f32 * state.tile_size + state.vpadding;

        //         draw.image(&state.box_texture)
        //             .position(xpos, ypos)
        //             // Need to rotate from the center of the image, which doesn't seem to be the
        //             // default.
        //             // .rotate_from(
        //             //     (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
        //             //     rand_val,
        //             // )
        //             // .color(Color::RED);
        //             .color(Color::new(0.5, 0.0, 0.01, 1.0))
        //             .size(state.tile_size, state.tile_size);

        //         xpos += rand_val * DAMPEN;
        //         ypos += rand_val * DAMPEN;
        //         draw.image(&state.box_texture)
        //             .position(xpos, ypos)
        //             // Need to rotate from the center of the image, which doesn't seem to be the
        //             // default.
        //             .rotate_from(
        //                 (xpos + state.tile_size * 0.5, ypos + state.tile_size * 0.5),
        //                 rand_val,
        //             )
        //             .size(state.tile_size, state.tile_size);
        //     }
        // }

        for row in 0..state.rows {
            rand_sum += (row + 1) as f32 * (state.rand_step * 0.05);
            for col in 0..state.cols {
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);

                let mut xpos = col as f32 * state.tile_size + state.hpadding;
                let mut ypos = row as f32 * state.tile_size + state.vpadding;

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
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);

                let mut xpos = col as f32 * state.tile_size + state.hpadding;
                let mut ypos = row as f32 * state.tile_size + state.vpadding;

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
                    .size(state.tile_size, state.tile_size);
            }
        }


        gfx.render(&draw);
        state.freeze = true;
        // log::debug!("fps: {}", app.timer.fps().round());
    }
}


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
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);

                let mut xpos = col as f32 * state.tile_size + state.hpadding;
                let mut ypos = row as f32 * state.tile_size + state.vpadding;

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
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);

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


fn event(state: &mut State, event: Event) {
    match event {
        Event::WindowResize { .. } => {
            log::debug!("Release freeze due to resize...");
            state.freeze = false;
        }
        _ => {}
    }
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        state.freeze = false;
        log::debug!("Freeze released");
    }
}


fn update_anim(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        state.freeze = false;
        log::debug!("Freeze released");
    }

    let incr = (app.timer.time_since_init().sin().abs() * 10.0) as u8;
    state.rows = ROWS + incr * 8;
    state.cols = COLS + incr * 4;
    (
        state.display_width,
        state.tile_size,
        state.display_height,
        state.vpadding,
        state.hpadding,
    ) = State::reframe(state.rows, state.cols);

    // log::debug!("{:?}", app.timer.time_since_init() % 4.00);
    // let loop_length: i32 = 4;
    // let time_in_loop = app.timer.time_since_init() as i32 % loop_length;
    // if time_in_loop < loop_length / 2 {
    //     log::debug!("time_in_loop: FWD {} / {}", time_in_loop, time_in_loop);
    //     // state.cols += time_in_loop as u8;
    // } else if time_in_loop > loop_length / 2 {
    //     let rev_time = loop_length - time_in_loop;
    //     log::debug!("time_in_loop: REV {} / {}", rev_time, time_in_loop);
    //     // state.cols -= rev_time as u8;
    // }
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .high_dpi(true)
        .size(WORK_SIZE.x as i32, WORK_SIZE.y as i32);

    // // Basic reproduction
    // notan::init_with(State::new_basic)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update)
    //     .draw(draw_basic)
    //     .build()

    // // Solid variant 1
    // notan::init_with(State::new_solid)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update)
    //     .draw(draw_solid)
    //     .build()

    // // Solid variant 2
    // notan::init_with(State::new_solid)
    //     .add_config(log::LogConfig::debug())
    //     .add_config(win_config)
    //     .add_config(DrawConfig) // Simple way to add the draw extension
    //     .event(event)
    //     .update(update)
    //     .draw(draw_solid2)
    //     .build()

    // Solid variant 2 animated
    notan::init_with(State::new_solid)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .event(event)
        .update(update_anim)
        .draw(draw_solid2_anim)
        .build()
}
