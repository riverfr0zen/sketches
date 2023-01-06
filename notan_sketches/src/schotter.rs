use super::utils::{get_common_win_config, get_draw_setup, get_rng, ScreenDimensions};
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;

const STROKE_WIDTH: f32 = 4.0;


// Visualization modifier
pub enum VizMod {
    BASIC,
    SOLID,
}


pub fn _create_box_texture(
    gfx: &mut Graphics,
    tile_size: f32,
    stroke_width: f32,
    vizmod: VizMod,
) -> Texture {
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
                .stroke(stroke_width);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
        _ => {
            draw.clear(Color::TRANSPARENT);
            draw.rect((0.0, 0.0), (tile_size, tile_size))
                .color(Color::BLACK)
                .stroke(stroke_width);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
    }
}


pub fn create_basic_box_texture(gfx: &mut Graphics, tile_size: f32, stroke_width: f32) -> Texture {
    _create_box_texture(gfx, tile_size, stroke_width, VizMod::BASIC)
}


pub fn create_solid_box_texture(gfx: &mut Graphics, tile_size: f32, stroke_width: f32) -> Texture {
    _create_box_texture(gfx, tile_size, stroke_width, VizMod::SOLID)
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
    pub fn reframe(
        work_size: Vec2,
        padding: f32,
        num_rows: u8,
        num_cols: u8,
    ) -> (f32, f32, f32, f32, f32) {
        let display_height: f32;
        let tile_size: f32;
        let display_width: f32;
        let hpadding: f32;
        let vpadding: f32;

        if num_rows > num_cols {
            display_height = work_size.y - padding * 2.0;
            tile_size = display_height / num_rows as f32;
            display_width = tile_size * num_cols as f32;
            hpadding = (work_size.x - display_width) * 0.5;
            vpadding = padding;
        } else {
            display_width = work_size.x - padding * 2.0;
            tile_size = display_width / num_cols as f32;
            display_height = tile_size * num_rows as f32;
            vpadding = (work_size.y - display_height) * 0.5;
            hpadding = padding;
        }
        (display_width, tile_size, display_height, vpadding, hpadding)
    }

    pub fn new(
        gfx: &mut Graphics,
        box_texture_fn: &dyn Fn(&mut Graphics, f32, f32) -> Texture,
        work_size: Vec2,
        padding: f32,
        rows: u8,
        cols: u8,
        rand_step: f32,
    ) -> Self {
        let (display_width, tile_size, display_height, vpadding, hpadding) =
            Self::reframe(work_size, padding, rows, cols);

        let box_texture = box_texture_fn(gfx, tile_size, STROKE_WIDTH);
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
            rand_step: rand_step,
            cols: cols,
            rows: rows,
        }
    }
}


pub fn init_basic(
    gfx: &mut Graphics,
    work_size: Vec2,
    padding: f32,
    rows: u8,
    cols: u8,
    rand_step: f32,
) -> State {
    State::new(
        gfx,
        &create_basic_box_texture,
        work_size,
        padding,
        rows,
        cols,
        rand_step,
    )
}


pub fn init_solid(
    gfx: &mut Graphics,
    work_size: Vec2,
    padding: f32,
    rows: u8,
    cols: u8,
    rand_step: f32,
) -> State {
    State::new(
        gfx,
        &create_solid_box_texture,
        work_size,
        padding,
        rows,
        cols,
        rand_step,
    )
}


pub fn event(state: &mut State, event: Event) {
    match event {
        Event::WindowResize { .. } => {
            log::debug!("Release freeze due to resize...");
            state.freeze = false;
        }
        _ => {}
    }
}


pub fn update_common(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        state.freeze = false;
        log::debug!("Freeze released");
    }
}


pub fn draw_basic(
    gfx: &mut Graphics,
    state: &mut State,
    work_size: Vec2,
    dampen: f32,
    // app: &mut App,
) {
    if !state.freeze {
        let mut draw = get_draw_setup(gfx, work_size, true, Color::WHITE);

        // Cumulative rotation value
        let mut rand_sum = 0.0;

        for row in 0..state.rows {
            rand_sum += (row + 1) as f32 * state.rand_step;
            for col in 0..state.cols {
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);
                let xpos = col as f32 * state.tile_size + state.hpadding + (rand_val * dampen);
                let ypos = row as f32 * state.tile_size + state.vpadding + (rand_val * dampen);
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


pub fn draw_solid(
    gfx: &mut Graphics,
    state: &mut State,
    work_size: Vec2,
    dampen: f32,
    box1_color: Color,
    // app: &mut App,
) {
    if !state.freeze {
        // let mut draw = get_draw_setup(gfx, WORK_SIZE, true, MAHOGANY);
        let mut draw = get_draw_setup(gfx, work_size, true, Color::GRAY);

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
                    .color(box1_color)
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

                xpos += rand_val * dampen;
                ypos += rand_val * dampen;
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
