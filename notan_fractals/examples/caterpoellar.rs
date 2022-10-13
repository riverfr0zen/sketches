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
const CP_STROKE: f32 = 1.0;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    // UP_LEFT,
    // UP_RIGHT,
    // DOWN_LEFT,
    // DOWN_RIGHT,
}


// #[derive(AppState, Default)]
#[derive(AppState)]
struct State {
    cp_head_pos: (f32, f32),
    cp_speed: f32,
    cp_direction: Direction,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // cp_head_pos: (CP_HEAD_W, CP_HEAD_H),
            cp_head_pos: (CP_BODY_W, CP_BODY_H),
            cp_speed: 0.1,
            cp_direction: Direction::LEFT,
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
        .update(update)
        .build()
}


fn update_head_movement(state: &mut State) {
    if state.cp_head_pos.0 > WORK_SIZE.x {
        state.cp_direction = Direction::DOWN;
    }

    match &state.cp_direction {
        Direction::LEFT => {
            state.cp_head_pos = (state.cp_head_pos.0 + state.cp_speed, state.cp_head_pos.1);
        }
        Direction::DOWN => {
            state.cp_head_pos = (state.cp_head_pos.0, state.cp_head_pos.1 + state.cp_speed);
        }
        _ => (),
    }
}


fn update(state: &mut State) {
    update_head_movement(state);
}


fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, Color::WHITE);

    draw.ellipse(state.cp_head_pos, (CP_BODY_W, CP_BODY_H))
        .color(Color::BLUE)
        .stroke(CP_STROKE)
        .color(Color::ORANGE)
        .fill();

    // draw to screen
    gfx.render(&draw);
}
