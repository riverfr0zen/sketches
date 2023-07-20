use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::enums;
use notan_sketches::mathutils::mid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::WHITE;
const STRIP_HEIGHT: f32 = 0.05;
const SEG_WIDTH: f32 = 0.02;
const DISPLACEMENT_POS_STEP: f32 = 10.0;

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
    pub displacement_dir: enums::Direction,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    let cursor = Vec2::new(0.0, 0.0);

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
        displacement_dir: enums::Direction::Down,
    }
}


fn add_strip(state: &mut State) {
    let mut strip: Vec<Segment> = vec![];
    while state.cursor.x < state.work_size.x {
        let from = vec2(state.cursor.x, state.cursor.y);

        state.cursor.x += state.seg_width;
        let to = vec2(state.cursor.x, state.cursor.y);

        let ctrl = mid(from, to);
        strip.push(Segment { from, to, ctrl });
    }
    state.strips.push(strip);
    state.cursor.x = 0.0;
}


fn calc_displacement_factor(seg_y_pos: &f32, displacement_pos: &f32, work_size: &Vec2) -> f32 {
    // Return a displacement factor based on the vertical distance of the segment from displacement_pos
    // TODO: revisit this logic to confirm what I'm doing is right with rgds to handling
    // rng sampling exception
    if seg_y_pos > &0.0 {
        return 1.0 - (seg_y_pos - displacement_pos).abs() / work_size.y;
    }
    0.00001
}


fn move_displacement(state: &mut State) {
    if state.displacement_pos > state.work_size.y {
        state.displacement_pos = 0.0;
    } else {
        state.displacement_pos += DISPLACEMENT_POS_STEP;
        // An interesting setting
        // state.displacement_pos += 500.0;
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    if state.cursor.y < state.work_size.y {
        add_strip(state);
        state.cursor.y += state.strip_height;
    }

    // let y_displacement_factor = calc_displacement_factor(state);
    for strip in state.strips.iter_mut() {
        let mut y_displacement_factor: f32 = -0.1;
        let mut y_displacement: f32 = 0.0;
        for seg in strip {
            // TODO NEXT: I think the seg.ctrl should be displaced here. Move ctrl rng code from
            // `add_strip` to here, I think.
            if y_displacement_factor < 0.0 {
                y_displacement_factor = calc_displacement_factor(
                    &seg.from.y,
                    &state.displacement_pos,
                    &state.work_size,
                );
                y_displacement = state.strip_height * 0.5 * y_displacement_factor;
                log::debug!(
                    "dpos: {}, y {}, ydf {}, yd {}",
                    state.displacement_pos,
                    seg.from.y,
                    y_displacement_factor,
                    y_displacement,
                );
            }
            let y_displacement = state.strip_height * 0.5 * y_displacement_factor;
            seg.ctrl = vec2(
                state
                    .rng
                    .gen_range(seg.from.x.min(seg.to.x)..seg.from.x.max(seg.to.x)),
                state
                    .rng
                    .gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement),
            );
            // seg.ctrl.y = state
            //     .rng
            //     .gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement);


            draw.path()
                .move_to(seg.from.x, seg.from.y)
                // .line_to(seg.to.x, seg.to.y)
                .quadratic_bezier_to((seg.ctrl.x, seg.ctrl.y), (seg.to.x, seg.to.y))
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
