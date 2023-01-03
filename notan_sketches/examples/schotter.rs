///
/// Recreation of "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);
const COLS: u8 = 12;
const ROWS: u8 = 22;
// const COLS: u8 = 22;
// const ROWS: u8 = 12;
// Minimum padding
const PADDING: f32 = 10.0;


#[derive(AppState)]
pub struct State {
    pub display_height: f32,
    pub display_width: f32,
    pub hpadding: f32,
    pub vpadding: f32,
    pub tile_size: f32,
    pub box_texture: Texture,
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

        let box_texture = create_texture(gfx, tile_size);
        Self {
            display_height: display_height,
            display_width: display_width,
            hpadding: hpadding,
            vpadding: vpadding,
            tile_size: tile_size,
            box_texture: box_texture,
        }
    }
}

fn create_texture(gfx: &mut Graphics, tile_size: f32) -> Texture {
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
        .stroke(4.0);
    // draw.rect((2.0, 2.0), (tile_size - 4.0, tile_size - 4.0))
    //     .color(Color::BLACK)
    //     .stroke(4.0);
    gfx.render_to(&rt, &draw);
    rt.take_inner()
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::WHITE);

    for col in 0..COLS {
        for row in 0..ROWS {
            draw.image(&state.box_texture)
                .position(
                    col as f32 * state.tile_size + state.hpadding,
                    row as f32 * state.tile_size + state.vpadding,
                )
                .size(state.tile_size, state.tile_size);
        }
    }


    // draw to screen
    gfx.render(&draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    // notan::init()
    // notan::init_with(init)
    notan::init_with(State::new)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
