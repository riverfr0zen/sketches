///
/// Recreation of "Schotter" (c.1965) by George Nees.
///
/// Based on http://www.artsnova.com/Nees_Schotter_Tutorial.html
///
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Rect, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);
const COLS: u8 = 12;
const ROWS: u8 = 22;
const PADDING: f32 = 50.0;


#[derive(AppState)]
pub struct State {
    pub box_texture: Texture,
}


impl State {
    fn new(gfx: &mut Graphics) -> Self {
        // let display_bounds = Rect {
        //     x: PADDING,
        //     y: PADDING,
        //     width: WORK_SIZE.x - .0,
        //     height: 10.0,
        // };
        let box_texture = create_texture(gfx);
        Self {
            box_texture: box_texture,
        }
    }
}

fn create_texture(gfx: &mut Graphics) -> Texture {
    let tile_size = 50;
    let rt = gfx
        .create_render_texture(tile_size, tile_size)
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
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    draw.image(&state.box_texture)
        .position(100.0, 100.0)
        .size(50.0, 50.0);


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
