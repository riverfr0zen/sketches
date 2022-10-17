/// `caterpoellar` is not a fractal app, just a simple idea to familiarize myself with Notan
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const CP_BODY_W: f32 = WORK_SIZE.x / 10.0;
// const CP_BODY_H: f32 = WORK_SIZE.x / 10.0;
// const CP_ROWS: f32 = 10.0;
// const CP_COLS: f32 = 10.0;
const CP_ROWS: f32 = 5.0;
const CP_COLS: f32 = 4.0;
const CP_BODY_W: f32 = WORK_SIZE.x / CP_COLS;
const CP_BODY_H: f32 = WORK_SIZE.y / CP_ROWS;
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

struct BodySegment {
    color: Color,
    pos: Vec2,
}


// #[derive(AppState, Default)]
#[derive(AppState)]
struct State {
    cp_head_pos: Vec2,
    cp_speed: f32,
    cp_direction: Direction,
    cp_reversing: bool,
    cp_next_row: f32,
    cp_spawn_seg_at: Vec2,
    cp_spawned_segs: Vec<BodySegment>,
}


impl Default for State {
    fn default() -> Self {
        Self {
            // cp_head_pos: vec2(CP_BODY_W, CP_BODY_H),
            // cp_head_pos: vec2(0.0, 0.0),
            cp_head_pos: vec2(0.0, CP_BODY_H),
            cp_speed: 1.0,
            cp_direction: Direction::RIGHT,
            cp_reversing: false,
            cp_next_row: 1.0,
            cp_spawn_seg_at: vec2(0.0, 0.0),
            cp_spawned_segs: Vec::new(),
        }
    }
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(State::default)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}

fn spawn_body_segment(state: &mut State) {
    log::debug!("new seg at {} {}", state.cp_head_pos.x, state.cp_head_pos.y);

    if !state
        .cp_spawned_segs
        .iter()
        .any(|seg| seg.pos == state.cp_head_pos)
    {
        log::debug!("new seg");
        state.cp_spawned_segs.push(BodySegment {
            color: Color::YELLOW,
            pos: state.cp_head_pos,
        });
    }
}


fn update_head_movement(state: &mut State) {
    // log::debug!("{}, {}", state.cp_next_row, state.cp_reversing);

    if state.cp_head_pos.x > 0.0
        && state.cp_head_pos.x < WORK_SIZE.x
        && state.cp_head_pos.x % (WORK_SIZE.x / CP_COLS) == 0.0
    {
        spawn_body_segment(state);
    }

    match &state.cp_direction {
        Direction::RIGHT => {
            state.cp_head_pos.x += state.cp_speed;

            if state.cp_head_pos.x > WORK_SIZE.x {
                if state.cp_reversing {
                    state.cp_direction = Direction::UP;
                    state.cp_next_row -= 1.0;
                } else {
                    state.cp_direction = Direction::DOWN;
                    state.cp_next_row += 1.0;
                }
            }
        }
        Direction::DOWN => {
            state.cp_head_pos.y += state.cp_speed;

            // log::debug!(
            //     "{}, {}, {}",
            //     state.cp_head_pos.y,
            //     state.cp_next_row,
            //     CP_BODY_H * state.cp_next_row
            // );
            if state.cp_head_pos.y > CP_BODY_H * state.cp_next_row {
                if state.cp_head_pos.x < 0.0 {
                    state.cp_direction = Direction::RIGHT;
                }
                if state.cp_head_pos.x > WORK_SIZE.x {
                    state.cp_direction = Direction::LEFT;
                }
            }
        }
        Direction::UP => {
            state.cp_head_pos.y -= state.cp_speed;

            if state.cp_head_pos.y < CP_BODY_H * state.cp_next_row {
                if state.cp_head_pos.x < 0.0 {
                    state.cp_direction = Direction::RIGHT;
                }
                if state.cp_head_pos.x > WORK_SIZE.x {
                    state.cp_direction = Direction::LEFT;
                }
            }
        }
        Direction::LEFT => {
            state.cp_head_pos.x -= state.cp_speed;

            if state.cp_head_pos.x < 0.0 {
                if state.cp_reversing {
                    state.cp_direction = Direction::UP;
                    state.cp_next_row -= 1.0;
                } else {
                    state.cp_direction = Direction::DOWN;
                    state.cp_next_row += 1.0;
                }
            }
        }
    }

    if state.cp_next_row > CP_ROWS - 2.0 {
        state.cp_reversing = true;
    }

    if state.cp_next_row < 2.0 {
        state.cp_reversing = false;
    }
}


fn update(state: &mut State) {
    update_head_movement(state);
}


fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    // XXX this is being drawn but not showing up because it is erased in the next draw.
    // Need an array to store body part positions (and colors?) so that they can be drawn
    // on every draw.
    for seg in state.cp_spawned_segs.iter() {
        draw.ellipse((seg.pos.x, seg.pos.y), (CP_BODY_W, CP_BODY_H))
            .fill()
            .color(seg.color);
        draw.ellipse((seg.pos.x, seg.pos.y), (CP_BODY_W, CP_BODY_H))
            .color(Color::BLUE)
            .stroke(CP_STROKE);
    }

    draw.ellipse(
        (state.cp_head_pos.x, state.cp_head_pos.y),
        // (CP_BODY_W, CP_BODY_H),
        (CP_HEAD_W, CP_HEAD_H),
    )
    // .color(Color::BLUE)
    // .stroke(CP_STROKE)
    .color(Color::ORANGE)
    .fill();


    // draw to screen
    gfx.render(&draw);
}
