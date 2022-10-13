/// `caterpoellar` is not a fractal app, just a simple idea to familiarize myself with Notan
use notan::draw::*;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const CP_BODY_W: f32 = WORK_SIZE.x / 10.0;
const CP_BODY_H: f32 = WORK_SIZE.x / 10.0;
const CP_HEAD_W: f32 = CP_BODY_W + 50.0;
const CP_HEAD_H: f32 = CP_BODY_H + 50.0;


// #[derive(AppState, Default)]
#[derive(AppState)]
struct State {
    cp_head_pos: (f32, f32),
}

impl Default for State {
    fn default() -> Self {
        Self {
            // cp_head_pos: (CP_HEAD_W, CP_HEAD_H),
            cp_head_pos: (CP_BODY_W, CP_BODY_H),
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(State::default)
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE);

    // // Our resolution bounds
    // draw.rect((0.0, 0.0), WORK_SIZE.into())
    //     .color(Color::ORANGE)
    //     .stroke(10.0);

    // draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));

    draw.ellipse(state.cp_head_pos, (CP_BODY_W, CP_BODY_H));

    // draw to screen
    gfx.render(&draw);

    state.cp_head_pos = (state.cp_head_pos.0 + 1.0, state.cp_head_pos.1);
}
