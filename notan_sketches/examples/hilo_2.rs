use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan::random::rand::prelude::SliceRandom;
use notan::random::rand::thread_rng;
use notan_sketches::colors;
use notan_sketches::enums;
use notan_sketches::mathutils::mid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::BLACK;
const STRIP_STROKE: f32 = 100.0;
const STRIP_HEIGHT: f32 = 0.05;
const SEG_WIDTH: f32 = 0.2;
const DISPLACEMENT_POS_STEP: f32 = 10.0;
const DISPLACEMENT_RANGE: f32 = 0.2;
const MONOCHROME: bool = false;
// const PALETTE: [Color; 21] = [
//     colors::PEACOCK,
//     colors::AEGEAN,
//     colors::AZURE,
//     colors::CERULEAN,
//     colors::STONE,
//     colors::OCHRE,
//     colors::OLIVE,
//     colors::SAFFRON,
//     colors::BANANA,
//     colors::LAGUNA,
//     colors::SACRAMENTO,
//     colors::SEAWEED,
//     colors::PICKLE,
//     colors::LIME,
//     colors::EMERALD,
//     colors::PICKLE,
//     colors::GRAYPURP,
//     colors::MAHOGANY,
//     colors::CARMINE,
//     colors::SCARLET,
//     colors::SALMON,
// ];
// const PALETTE: [Color; 3] = [
//     colors::PEACOCK,
//     colors::LIME,
//     colors::SALMON,
// ];
const PALETTE: [Color; 3] = [colors::PEACOCK, colors::SEAWEED, colors::MAHOGANY];

struct Segment {
    from: Vec2,
    to: Vec2,
    ctrl: Vec2,
}

struct Strip {
    segs: Vec<Segment>,
    color: Color,
}

#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub seg_width: f32,
    pub strip_height: f32,
    pub cursor: Vec2,
    pub strips: Vec<Strip>,
    pub displacement_pos: f32,
    pub displacement_dir: enums::Direction,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
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
    let mut strip = Strip {
        segs: vec![],
        color: choose_color(),
    };
    while state.cursor.x < state.work_size.x {
        let from = vec2(state.cursor.x, state.cursor.y);

        state.cursor.x += state.seg_width;
        let to = vec2(state.cursor.x, state.cursor.y);

        let ctrl = mid(from, to);
        strip.segs.push(Segment { from, to, ctrl });
    }
    state.strips.push(strip);
    state.cursor.x = 0.0;
}

fn calc_displacement_factor(seg_y_pos: &f32, displacement_pos: &f32, work_size: &Vec2) -> f32 {
    // Return a displacement factor based on the vertical distance of the segment from displacement_pos
    let distance = (seg_y_pos - displacement_pos).abs() / work_size.y;
    if distance > 0.0 && distance < DISPLACEMENT_RANGE {
        return 1.0 - distance;
    }
    // @TODO: Returning zero breaks rng so return a very small value instead.
    // Revisit on whether there's a better approach
    0.00001
}

fn move_displacement(state: &mut State) {
    if state.displacement_pos <= 0.0 {
        state.displacement_dir = enums::Direction::Down;
    }

    if state.displacement_pos > state.work_size.y {
        state.displacement_dir = enums::Direction::Up;
    }

    match state.displacement_dir {
        enums::Direction::Up => state.displacement_pos -= DISPLACEMENT_POS_STEP,
        enums::Direction::Down => state.displacement_pos += DISPLACEMENT_POS_STEP,
        _ => (),
    }
}

fn choose_color() -> Color {
    if !MONOCHROME {
        let mut rng = thread_rng();
        if let Some(color) = PALETTE.choose(&mut rng) {
            return *color;
        }
    }
    Color::BLACK
}

fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    if state.cursor.y < state.work_size.y + state.strip_height {
        add_strip(state);
        state.cursor.y += state.strip_height;
    }

    for strip in state.strips.iter_mut() {
        let mut path = draw.path();
        path.move_to(0.0, strip.segs[0].from.y);

        let mut y_displacement_factor: f32 = -0.1;
        let mut y_displacement: f32 = 0.0;
        for seg in strip.segs.iter_mut() {
            if y_displacement_factor < 0.0 {
                y_displacement_factor = calc_displacement_factor(
                    &seg.from.y,
                    &state.displacement_pos,
                    &state.work_size,
                );
                // y_displacement = state.strip_height * 0.5 * y_displacement_factor;
                y_displacement = state.strip_height * y_displacement_factor;
                // log::debug!(
                //     "dpos: {}, y {}, ydf {}, yd {}",
                //     state.displacement_pos,
                //     seg.from.y,
                //     y_displacement_factor,
                //     y_displacement,
                // );
            }
            seg.ctrl.x = state
                .rng
                .gen_range(seg.from.x.min(seg.to.x)..seg.from.x.max(seg.to.x));
            seg.ctrl.y = state
                .rng
                .gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement);

            path.quadratic_bezier_to((seg.ctrl.x, seg.ctrl.y), (seg.to.x, seg.to.y))
                .color(strip.color)
                .stroke(STRIP_STROKE);
        }
    }

    move_displacement(state);

    gfx.render(draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(
            // let win_config = get_common_win_config().high_dpi(true).size(
            // ScreenDimensions::RES_4KISH.x as i32,
            // ScreenDimensions::RES_4KISH.y as i32,
            // ScreenDimensions::RES_HDPLUS.x as i32,
            // ScreenDimensions::RES_HDPLUS.y as i32,
            ScreenDimensions::RES_1080P.x as u32,
            ScreenDimensions::RES_1080P.y as u32,
            // ScreenDimensions::DEFAULT.x as i32,
            // ScreenDimensions::DEFAULT.y as i32,
        );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);

    let win_config = win_config.set_title("hilo_strips.displacement");
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
