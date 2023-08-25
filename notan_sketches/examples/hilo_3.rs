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
use palette::{FromColor, Hsv, Shade, Srgb};


// const CLEAR_COLOR: Color = Color::WHITE;
const CLEAR_COLOR: Color = Color::BLACK;
// const STRIP_STROKE: f32 = 2.0;
const STRIP_STROKE: f32 = 5.0;
// The vertical interval between each strip. If the STRIP_HEIGHT is greater than STRIP_INTERVAL, then strips will overlap
// const STRIP_INTERVAL: f32 = 0.05;
const STRIP_INTERVAL: f32 = 0.1;
// const STRIP_HEIGHT: f32 = 0.05;
const STRIP_HEIGHT: f32 = 0.08;
const SEG_WIDTH: f32 = 0.2;
const DISPLACEMENT_POS_STEP: f32 = 10.0;
const DISPLACEMENT_RANGE: f32 = 0.5;
const MONOCHROME: bool = false;
const PALETTE: [Color; 21] = [
    colors::PEACOCK,
    colors::AEGEAN,
    colors::AZURE,
    colors::CERULEAN,
    colors::STONE,
    colors::OCHRE,
    colors::OLIVE,
    colors::SAFFRON,
    colors::BANANA,
    colors::LAGUNA,
    colors::SACRAMENTO,
    colors::SEAWEED,
    colors::PICKLE,
    colors::LIME,
    colors::EMERALD,
    colors::PICKLE,
    colors::GRAYPURP,
    colors::MAHOGANY,
    colors::CARMINE,
    colors::SCARLET,
    colors::SALMON,
];
// const PALETTE: [Color; 3] = [
//     colors::PEACOCK,
//     colors::LIME,
//     colors::SALMON,
// ];
// const PALETTE: [Color; 3] = [colors::PEACOCK, colors::SEAWEED, colors::MAHOGANY];
// Neon
// const PALETTE: [Color; 3] = [
//     Color::new(1.0, 0.37, 0.0, 1.0),
//     Color::new(0.8, 1.0, 0.0, 1.0),
//     Color::new(0.74, 0.07, 1.0, 1.0),
// ];


struct Segment {
    from: Vec2,
    to: Vec2,
    ctrl: Vec2,
    ctrl2: Vec2,
}


struct Strip {
    segs: Vec<Segment>,
    color: Color,
    stroke_color: Color,
}


#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub seg_width: f32,
    pub strip_interval: f32,
    pub strip_height: f32,
    pub cursor: Vec2,
    pub strips: Vec<Strip>,
    pub displacement_pos: f32,
    pub displacement_dir: enums::Direction,
    pub paused: bool,
}


fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    let cursor = Vec2::new(0.0, 0.0);

    let seg_width = SEG_WIDTH * work_size.x;
    let strip_interval = STRIP_INTERVAL * work_size.y;
    let strip_height = STRIP_HEIGHT * work_size.y;

    State {
        rng,
        work_size,
        seg_width,
        strip_interval,
        strip_height,
        cursor,
        strips: vec![],
        displacement_pos: 0.0,
        displacement_dir: enums::Direction::Down,
        paused: false,
    }
}


fn add_strip(state: &mut State) {
    let color = choose_color();
    let stroke_color = Srgb::new(color.r, color.g, color.b);
    let mut stroke_color = Hsv::from_color(stroke_color);
    match state.rng.gen_bool(0.5) {
        true => {
            // log::info!("darken");
            stroke_color = stroke_color.darken(0.5);
        }
        false => {
            // log::info!("lighten");
            // Lighten factor intentionally more than darken above
            stroke_color = stroke_color.lighten(0.9);
        }
    }
    let stroke_color = Srgb::from_color(stroke_color);

    let mut strip = Strip {
        segs: vec![],
        color: color,
        stroke_color: Color::new(stroke_color.red, stroke_color.green, stroke_color.blue, 1.0),
    };
    while state.cursor.x < state.work_size.x {
        let from = vec2(state.cursor.x, state.cursor.y);

        state.cursor.x += state.seg_width;
        let to = vec2(state.cursor.x, state.cursor.y);

        let middle = mid(from, to);
        let ctrl = mid(from, middle);
        let ctrl2 = mid(middle, to);
        strip.segs.push(Segment {
            from,
            to,
            ctrl,
            ctrl2,
        });
    }
    state.strips.push(strip);
    state.cursor.x = 0.0;
}


fn calc_displacement_factor(seg_y_pos: &f32, displacement_pos: &f32, work_size: &Vec2) -> f32 {
    // Return a displacement factor based on the vertical distance of the segment from displacement_pos
    let distance = (seg_y_pos - displacement_pos).abs() / work_size.y;
    if distance > 0.0 && distance < DISPLACEMENT_RANGE {
        // return 1.0 - distance;
        return (1.0 - distance.log(4.0)) * 0.5;
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


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::P) {
        state.paused = !state.paused;
        log::debug!("pause toggled");
    }
}


fn update_strip(
    strip: &mut Strip,
    displacement_pos: f32,
    strip_interval: f32,
    work_size: &Vec2,
    rng: &mut Random,
) {
    let mut y_displacement_factor: f32 = -0.1;
    let mut y_displacement: f32 = 0.0;
    for seg in strip.segs.iter_mut() {
        if y_displacement_factor < 0.0 {
            y_displacement_factor =
                calc_displacement_factor(&seg.from.y, &displacement_pos, &work_size);
            // y_displacement = strip_interval * 0.5 * y_displacement_factor;
            y_displacement = strip_interval * y_displacement_factor;
            // log::debug!(
            //     "dpos: {}, y {}, ydf {}, yd {}",
            //     state.displacement_pos,
            //     seg.from.y,
            //     y_displacement_factor,
            //     y_displacement,
            // );
        }
        let middle = mid(seg.from, seg.to);
        // seg.ctrl.x = rng.gen_range(seg.from.x.min(seg.to.x)..seg.from.x.max(seg.to.x));
        // seg.ctrl.y = rng.gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement);
        seg.ctrl.x = rng.gen_range(seg.from.x.min(middle.x)..seg.from.x.max(middle.x));
        seg.ctrl.y = rng.gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement);

        seg.ctrl2.x = rng.gen_range(middle.x.min(seg.to.x)..middle.x.max(seg.to.x));
        seg.ctrl2.y = rng.gen_range(seg.from.y - y_displacement..seg.from.y + y_displacement);
    }
}


fn draw_strip(draw: &mut Draw, strip: &mut Strip, ypos: f32, strip_height: f32) {
    let path = &mut draw.path();
    path.move_to(0.0, ypos);


    for seg in strip.segs.iter_mut() {
        path.cubic_bezier_to(
            (seg.ctrl.x, seg.ctrl.y),
            (seg.ctrl2.x, seg.ctrl2.y),
            (seg.to.x, seg.to.y),
        );
    }
    path.line_to(
        strip.segs.last().unwrap().to.x,
        strip.segs.last().unwrap().to.y + strip_height,
    );
    for seg in strip.segs.iter_mut().rev() {
        path.cubic_bezier_to(
            (seg.ctrl2.x, seg.ctrl2.y + strip_height),
            (seg.ctrl.x, seg.ctrl.y + strip_height),
            (seg.from.x, seg.from.y + strip_height),
        );
    }
    path.stroke_color(strip.stroke_color)
        .stroke(STRIP_STROKE)
        .fill_color(strip.color)
        .fill()
        .close();
}


fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);


    // Cursor for testing  w/ a single line
    // if state.strips.len() == 0 {
    //     state.cursor.y = 300.0;
    //     add_strip(state);
    // }
    // Cursor for all lines
    if state.cursor.y < state.work_size.y + state.strip_interval {
        add_strip(state);
        state.cursor.y += state.strip_interval;
    }


    for strip in state.strips.iter_mut() {
        if !state.paused {
            update_strip(
                strip,
                state.displacement_pos,
                state.strip_interval,
                &state.work_size,
                &mut state.rng,
            );
        }

        draw_strip(draw, strip, strip.segs[0].from.y, state.strip_height);
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

    let win_config = win_config.title("hilo_strips.displacement");
    set_html_bgcolor(CLEAR_COLOR);

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .touch_as_mouse(false)
        // .event(event)
        .update(update)
        .draw(draw)
        .build()
}
