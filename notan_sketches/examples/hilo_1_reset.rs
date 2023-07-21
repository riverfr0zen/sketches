use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::mathutils::mid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::WHITE;
const STRIP_HEIGHT: f32 = 0.05;
const SEG_WIDTH: f32 = 0.02;
const UPDATE_STEP: f32 = 0.5;

struct Segment {
    from: Vec2,
    to: Vec2,
    ctrl: Vec2,
}

#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub seg_width: f32,
    pub strip_height: f32,
    pub cursor: Vec2,
    pub strips: Vec<Vec<Segment>>,
    pub displacement_pos: f32,
    pub last_updated: f32,
}

impl State {
    fn reset(&mut self) {
        self.cursor = Vec2::ZERO;
        self.strips = vec![];
    }
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    let cursor = Vec2::ZERO;

    let seg_width = SEG_WIDTH * work_size.x;
    let strip_height = STRIP_HEIGHT * work_size.y;

    State {
        rng,
        work_size,
        seg_width,
        strip_height,
        cursor,
        strips: vec![],
        displacement_pos: 0.0,
        last_updated: 0.0,
    }
}


fn add_strip(state: &mut State) {
    let mut strip: Vec<Segment> = vec![];
    while state.cursor.x < state.work_size.x {
        let from = vec2(state.cursor.x, state.cursor.y);

        state.cursor.x += state.seg_width;
        let to = vec2(state.cursor.x, state.cursor.y);

        // TODO: experiment with moving this around
        let y_displacement_factor = calc_displacement_factor(state);
        let y_displacement = state.strip_height * 0.5 * y_displacement_factor;
        let ctrl = vec2(
            state
                .rng
                .gen_range(state.cursor.x - state.seg_width..state.cursor.x),
            state
                .rng
                .gen_range(state.cursor.y - y_displacement..state.cursor.y + y_displacement),
        );
        strip.push(Segment { from, to, ctrl });
    }
    state.strips.push(strip);
    state.cursor.x = 0.0;
}


fn calc_displacement_factor(state: &mut State) -> f32 {
    // Return a displacement factor that gets larger as we go down the screen
    // 0.001 + state.cursor.y / state.work_size.y

    // Return a displacement factor based on the distance of `state.cursor.y` from `state.displacement_pos`
    0.001 + (state.cursor.y - state.displacement_pos).abs() / state.work_size.y
}

fn move_displacement(state: &mut State) {
    if state.displacement_pos > state.work_size.y {
        state.displacement_pos = 0.0;
    } else {
        state.displacement_pos += 1.0;
        // An interesting setting
        // state.displacement_pos += 500.0;
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    let now = app.timer.time_since_init();
    if now - state.last_updated > UPDATE_STEP {
        log::debug!("t");
        state.reset();
        state.last_updated = now;
    }

    if state.cursor.y < state.work_size.y {
        add_strip(state);
        state.cursor.y += state.strip_height;
    }

    for strip in state.strips.iter() {
        let mut path = draw.path();
        path.move_to(0.0, strip[0].from.y);
        for seg in strip {
            path.quadratic_bezier_to((seg.ctrl.x, seg.ctrl.y), (seg.to.x, seg.to.y))
                .color(Color::ORANGE)
                .stroke(10.0);
        }
    }

    move_displacement(state);

    gfx.render(draw);
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
        // let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_4KISH.x as i32,
        // ScreenDimensions::RES_4KISH.y as i32,
        // ScreenDimensions::RES_HDPLUS.x as i32,
        // ScreenDimensions::RES_HDPLUS.y as i32,
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);

    let win_config = win_config.title("hilostrips");
    set_html_bgcolor(CLEAR_COLOR);

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .touch_as_mouse(false)
        // .event(event)
        // .update(update)
        .draw(draw)
        .build()
}
