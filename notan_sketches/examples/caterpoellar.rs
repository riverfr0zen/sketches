use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const CP_BODY_W: f32 = WORK_SIZE.x / 10.0;
// const CP_BODY_H: f32 = WORK_SIZE.x / 10.0;
const CP_COLS: f32 = 10.0;
const CP_ROWS: f32 = 8.0;
const CP_MAX_SEGS: usize = CP_COLS as usize * CP_ROWS as usize;
// const CP_MAX_SEGS: usize = 10;
// const CP_COLS: f32 = 5.0;
// const CP_ROWS: f32 = 4.0;
const CP_BODY_W: f32 = WORK_SIZE.x / CP_COLS;
const CP_BODY_H: f32 = WORK_SIZE.y / CP_ROWS;
const CP_HEAD_W: f32 = CP_BODY_W + 20.0;
const CP_HEAD_H: f32 = CP_BODY_H + 20.0;
// const CP_STROKE: f32 = 1.0;
const CP_SPEED: f32 = 1.0;


#[derive(Clone, Copy)]
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
    direction: Direction,
    // visible: bool,
}


// #[derive(AppState, Default)]
#[derive(AppState)]
struct State {
    cp_head_pos: Vec2,
    cp_speed: f32,
    cp_direction: Direction,
    cp_reversing: bool,
    cp_next_row: f32,
    cp_spawned_segs: Vec<BodySegment>,
    cp_seg_texture: Texture,
    cp_seg_texture_hflip: Texture,
    cp_colors: Vec<Color>,
    rng: Random,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/seg.png"))
            .build()
            .unwrap();
        let texture_hflip = gfx
            .create_texture()
            .from_image(include_bytes!("assets/seg_hflip.png"))
            .build()
            .unwrap();

        let (rng, seed) = get_rng(None);
        log::debug!("seed: {}", seed);
        Self {
            // cp_head_pos: vec2(CP_BODY_W, CP_BODY_H),
            // cp_head_pos: vec2(0.0, 0.0),
            cp_head_pos: vec2(0.0, CP_BODY_H),
            cp_speed: CP_SPEED,
            cp_direction: Direction::RIGHT,
            cp_reversing: false,
            cp_next_row: 1.0,
            cp_spawned_segs: Vec::new(),
            cp_seg_texture: texture,
            cp_seg_texture_hflip: texture_hflip,
            cp_colors: vec![Color::YELLOW, Color::RED, Color::BLUE, Color::GREEN],
            rng: rng,
        }
    }
}


fn init(gfx: &mut Graphics) -> State {
    let state = State::new(gfx);
    state
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}

fn spawn_body_segment(state: &mut State) {
    // let col = (state.cp_head_pos.x / CP_BODY_W) as usize;
    // let row = (state.cp_head_pos.y / CP_BODY_H) as usize;

    // log::debug!(
    //     "At {} {} / {} {}",
    //     state.cp_head_pos.x,
    //     state.cp_head_pos.y,
    //     col,
    //     row
    // );

    state.cp_spawned_segs.push(BodySegment {
        // color: Color::YELLOW,
        color: state.cp_colors[state.rng.gen_range(0..state.cp_colors.len())],
        pos: state.cp_head_pos,
        direction: state.cp_direction,
    });
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
            if state.cp_head_pos.y >= CP_BODY_H * state.cp_next_row {
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

            if state.cp_head_pos.y <= CP_BODY_H * state.cp_next_row {
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


fn manage_num_segs(state: &mut State) {
    // log::debug!("{} {}", state.cp_spawned_segs.len(), CP_MAX_SEGS);
    if state.cp_spawned_segs.len() > CP_MAX_SEGS {
        state.cp_spawned_segs.rotate_left(1);
        state.cp_spawned_segs.pop();
    }
}


fn update(state: &mut State) {
    manage_num_segs(state);
    update_head_movement(state);
}


fn draw_seg(draw: &mut Draw, seg: &BodySegment, texture: &Texture) {
    // draw.ellipse((seg.pos.x, seg.pos.y), (CP_BODY_W, CP_BODY_H))
    //     .fill()
    //     .color(seg.color);

    // draw.ellipse((seg.pos.x, seg.pos.y), (CP_BODY_W, CP_BODY_H))
    //     .color(Color::BLUE)
    //     .stroke(CP_STROKE);

    draw.image(texture)
        .position(seg.pos.x - CP_BODY_W, seg.pos.y - CP_BODY_H)
        .size(CP_BODY_W * 2.0, CP_BODY_H * 2.0)
        .color(seg.color);
}

fn draw(
    // app: &mut App,
    gfx: &mut Graphics,
    state: &mut State,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::OLIVE);

    for seg in state.cp_spawned_segs.iter() {
        match seg.direction {
            Direction::LEFT => {
                draw_seg(&mut draw, seg, &state.cp_seg_texture_hflip);
            }
            _ => {
                draw_seg(&mut draw, seg, &state.cp_seg_texture);
            }
        }
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

    // log::debug!("fps: {}", app.timer.fps().round());
}
