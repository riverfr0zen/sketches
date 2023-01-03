///
/// Recreation of "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng, ScreenDimensions};

// const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
// const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
const COLS: u8 = 12;
const ROWS: u8 = 22;
// const COLS: u8 = 22;
// const ROWS: u8 = 12;
// Minimum padding
const PADDING: f32 = 50.0;
const STROKE_WIDTH: f32 = 4.0;


fn create_box_texture(gfx: &mut Graphics, tile_size: f32) -> Texture {
    let rt = gfx
        .create_render_texture(tile_size as i32, tile_size as i32)
        .build()
        .unwrap();

    let tile_size = tile_size as f32;

    let mut draw = gfx.create_draw();
    draw.set_size(tile_size, tile_size);
    draw.clear(Color::TRANSPARENT);
    draw.rect((0.0, 0.0), (tile_size, tile_size))
        .color(Color::BLACK)
        .stroke(STROKE_WIDTH);

    gfx.render_to(&rt, &draw);
    rt.take_inner()
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
}


impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let display_height: f32;
        let tile_size: f32;
        let display_width: f32;
        let hpadding: f32;
        let vpadding: f32;

        if ROWS > COLS {
            display_height = WORK_SIZE.y - PADDING * 2.0;
            tile_size = display_height / ROWS as f32;
            display_width = tile_size * COLS as f32;
            hpadding = (WORK_SIZE.x - display_width) * 0.5;
            vpadding = PADDING;
        } else {
            display_width = WORK_SIZE.x - PADDING * 2.0;
            tile_size = display_width / COLS as f32;
            display_height = tile_size * ROWS as f32;
            vpadding = (WORK_SIZE.y - display_height) * 0.5;
            hpadding = PADDING;
        }

        let box_texture = create_box_texture(gfx, tile_size);
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
        }
    }
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    if !state.freeze {
        let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::WHITE);

        // Rotation increment in degrees
        // let rand_step: f32 = 0.22;
        let rand_step: f32 = 0.022;
        // let rand_step: f32 = 0.0022;
        // Soften random effect for translation
        let dampen: f32 = 0.45;
        // let dampen: f32 = 0.045;
        // let dampen: f32 = 1.5;
        // Cumulative rotation value
        let mut rand_sum = 0.0;

        for row in 0..ROWS {
            rand_sum += (row + 1) as f32 * rand_step;
            for col in 0..COLS {
                let rand_val = state.rng.gen_range(-rand_sum..rand_sum);
                let xpos = col as f32 * state.tile_size + state.hpadding + (rand_val * dampen);
                let ypos = row as f32 * state.tile_size + state.vpadding + (rand_val * dampen);
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


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .high_dpi(true)
        .size(WORK_SIZE.x as i32, WORK_SIZE.y as i32);

    // notan::init()
    // notan::init_with(init)
    notan::init_with(State::new)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}
