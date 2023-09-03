use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::enums;
use notan_sketches::mathutils::mid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};
use palette::{FromColor, Hsv, Shade, Srgb};
use std::ops::RangeInclusive;


// const CLEAR_COLOR: Color = Color::WHITE;
const CLEAR_COLOR: Color = Color::BLACK;
// const STRIP_STROKE: f32 = 2.0;
const STRIP_STROKE: f32 = 5.0;
// The vertical interval between each strip. If the STRIP_HEIGHT is greater than STRIP_INTERVAL, then strips will overlap
// const STRIP_INTERVAL: f32 = 0.05;
const STRIP_INTERVAL: RangeInclusive<f32> = 0.02..=0.4;
// const STRIP_HEIGHT: f32 = 0.05;
const STRIP_HEIGHT: RangeInclusive<f32> = 0.02..=0.2;
const SEG_WIDTH: RangeInclusive<f32> = 0.02..=0.4;
const SEG_CTRL_STEP: f32 = 0.01;
const SEG_CTRL_BSTEP: f32 = 0.005;
const DISPLACEMENT_POS_STEP: RangeInclusive<f32> = 0.5..=20.0;
const DISPLACEMENT_RANGE: RangeInclusive<f32> = 0.1..=0.5;
// The number of displacement cycles before shuffling settings
const SHUFFLE_PERIOD: u8 = 2;
const VARY_HORIZONTAL: bool = true;
const USE_CUBIC_BEZIER: bool = true;


pub struct Segment {
    from: Vec2,
    to: Vec2,
    ctrl: Vec2,
    ctrl_to: Vec2,
    ctrl2: Vec2,
    ctrl2_to: Vec2,
}


pub struct Strip {
    segs: Vec<Segment>,
    color: Color,
    stroke_color: Color,
    alpha: f32,
    last_distance: f32,
    displaced: bool,
}


#[derive(Debug)]
pub struct GenSettings {
    pub seg_width: f32,
    pub strip_interval: f32,
    pub strip_height: f32,
    pub displacement_pos_step: f32,
    pub displacement_range: f32,
    pub palette: colors::PalettesSelection,
}

impl GenSettings {
    fn default(work_size: &Vec2) -> Self {
        let seg_width = 0.2 * work_size.x;
        let strip_interval = 0.1 * work_size.y;
        let strip_height = 0.08 * work_size.y;
        let displacement_pos_step: f32 = 10.0;
        let displacement_range: f32 = 0.3;
        let palette = colors::PalettesSelection::All;

        Self {
            seg_width,
            strip_interval,
            strip_height,
            displacement_pos_step,
            displacement_range,
            palette,
        }
    }

    fn randomize(rng: &mut Random, work_size: &Vec2) -> Self {
        // default = Self::default(&work_size)
        let seg_width = rng.gen_range(SEG_WIDTH) * work_size.x;
        let strip_interval = rng.gen_range(STRIP_INTERVAL) * work_size.y;
        let strip_height = rng.gen_range(STRIP_HEIGHT) * work_size.y;
        let displacement_pos_step = rng.gen_range(DISPLACEMENT_POS_STEP);
        let displacement_range = rng.gen_range(DISPLACEMENT_RANGE);
        let palette: PalettesSelection = rng.gen();

        Self {
            seg_width,
            strip_interval,
            strip_height,
            displacement_pos_step,
            displacement_range,
            palette,
        }
    }
}


#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub cursor: Vec2,
    pub strips: Vec<Strip>,
    pub displacement_pos: f32,
    pub displacement_dir: enums::Direction,
    pub show_displacement_pos: bool,
    pub paused: bool,
    pub auto_shuffle: bool,
    pub shuffle_counter: u8,
    pub gen: GenSettings,
}

enum Position {
    Neutral,
    Above,
    Below,
}


fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    let cursor = Vec2::new(0.0, 0.0);


    State {
        rng,
        work_size,
        cursor,
        strips: vec![],
        displacement_pos: 0.0,
        displacement_dir: enums::Direction::Down,
        show_displacement_pos: false,
        paused: false,
        auto_shuffle: true,
        shuffle_counter: 0,
        gen: GenSettings::default(&work_size),
    }
}


/// Get the distance of the strip from the displacement
fn get_displacement_distance(strip: &Strip, displacement_pos: &f32, work_size: &Vec2) -> f32 {
    (strip.segs[0].from.y - displacement_pos).abs() / work_size.y
}


fn add_strip(state: &mut State) {
    let color = colors::Palettes::choose_color(&state.gen.palette);
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
        alpha: state.rng.gen_range(0.2..1.0),
        last_distance: 0.0,
        displaced: false,
    };
    while state.cursor.x < state.work_size.x {
        let from = vec2(state.cursor.x, state.cursor.y);

        state.cursor.x += state.gen.seg_width;
        let to = vec2(state.cursor.x, state.cursor.y);

        let middle = mid(from, to);
        let ctrl = mid(from, middle);
        let ctrl2 = mid(middle, to);
        strip.segs.push(Segment {
            from,
            to,
            ctrl,
            ctrl_to: ctrl,
            ctrl2,
            ctrl2_to: ctrl2,
        });
    }
    strip.last_distance =
        get_displacement_distance(&strip, &state.displacement_pos, &state.work_size);
    state.strips.push(strip);
    state.cursor.x = 0.0;
}


fn move_displacement(state: &mut State) {
    if state.displacement_pos <= 0.0 {
        state.displacement_dir = enums::Direction::Down;
    }

    if state.displacement_pos > state.work_size.y {
        state.displacement_dir = enums::Direction::Up;
    }

    match state.displacement_dir {
        enums::Direction::Up => state.displacement_pos -= state.gen.displacement_pos_step,
        enums::Direction::Down => state.displacement_pos += state.gen.displacement_pos_step,
        _ => (),
    }
}


fn generate_strips(state: &mut State, refresh: bool) {
    if refresh {
        state.strips = vec![];
        state.cursor = Vec2::new(0.0, 0.0);
    }

    // Cursor for testing  w/ a single line
    // if state.strips.len() == 0 {
    //     state.cursor.y = 300.0;
    //     add_strip(state);
    // }
    // Cursor for all lines
    if state.cursor.y < state.work_size.y + state.gen.strip_interval {
        add_strip(state);
        state.cursor.y += state.gen.strip_interval;
    }
}

fn shuffle(state: &mut State) {
    state.shuffle_counter = 0;
    state.gen = GenSettings::randomize(&mut state.rng, &state.work_size);
    generate_strips(state, true);
    log::debug!("{:#?}", state.gen);
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::P) {
        state.paused = !state.paused;
        log::debug!("pause toggled");
    }

    if app.keyboard.was_pressed(KeyCode::R) {
        shuffle(state);
    }

    if app.keyboard.was_pressed(KeyCode::D) {
        state.show_displacement_pos = !state.show_displacement_pos;
    }

    if app.keyboard.was_pressed(KeyCode::S) {
        state.auto_shuffle = !state.auto_shuffle;
        log::debug!("shuffle toggled");
    }

    if state.auto_shuffle && state.shuffle_counter >= SHUFFLE_PERIOD {
        shuffle(state);
    }
}


fn update_strip(
    strip: &mut Strip,
    displacement_pos: f32,
    displacement_pos_step: f32,
    displacement_range: f32,
    strip_interval: f32,
    work_size: &Vec2,
    rng: &mut Random,
) {
    let distance = get_displacement_distance(strip, &displacement_pos, &work_size);
    let mut do_displacement: bool = false;
    let mut do_return: bool = false;
    // let mut ctrl_step = SEG_CTRL_STEP * work_size.y;
    // let mut ctrl_step = strip_interval / (displacement_range * work_size.y * 2.0);
    // let mut ctrl_step = displacement_pos_step;
    let mut ctrl_step =
        displacement_pos_step * (strip_interval / (displacement_range * work_size.y * 2.0));

    if distance <= displacement_range && !strip.displaced {
        do_displacement = true;
        strip.displaced = true;
    }
    if distance > displacement_range {
        ctrl_step = SEG_CTRL_BSTEP * work_size.y;
        if strip.displaced {
            do_return = true;
            strip.displaced = false;
        }
    }
    let mut prev_seg_loc = Position::Neutral;
    for seg in strip.segs.iter_mut() {
        // Update ctrl targets (ctrl_to)
        if do_displacement {
            let middle = mid(seg.from, seg.to);
            if VARY_HORIZONTAL {
                seg.ctrl_to.x = rng.gen_range(seg.from.x.min(middle.x)..seg.from.x.max(middle.x));
                seg.ctrl2_to.x = rng.gen_range(middle.x.min(seg.to.x)..middle.x.max(seg.to.x));
            } else {
                seg.ctrl_to.x = (seg.from.x + middle.x) / 2.0;
                seg.ctrl2_to.x = (middle.x + seg.to.x) / 2.0;
            }

            match prev_seg_loc {
                Position::Neutral => {
                    seg.ctrl_to.y =
                        rng.gen_range(seg.from.y - strip_interval..seg.from.y + strip_interval);
                    seg.ctrl2_to.y =
                        rng.gen_range(seg.from.y - strip_interval..seg.from.y + strip_interval);

                    if seg.ctrl2_to.y > seg.to.y {
                        prev_seg_loc = Position::Below;
                    } else {
                        prev_seg_loc = Position::Above;
                    }
                }
                Position::Above => {
                    seg.ctrl_to.y = rng.gen_range(seg.from.y..seg.from.y + strip_interval);
                    seg.ctrl2_to.y = rng.gen_range(seg.from.y..seg.from.y + strip_interval);
                    prev_seg_loc = Position::Below;
                }
                Position::Below => {
                    seg.ctrl_to.y = rng.gen_range(seg.from.y - strip_interval..seg.from.y);
                    seg.ctrl2_to.y = rng.gen_range(seg.from.y - strip_interval..seg.from.y);
                    prev_seg_loc = Position::Above;
                }
            }
        }
        if do_return {
            let middle = mid(seg.from, seg.to);
            seg.ctrl_to = mid(seg.from, middle);
            seg.ctrl2_to = mid(middle, seg.to);
        }
        // Update ctrl
        if seg.ctrl.x < seg.ctrl_to.x {
            seg.ctrl.x += ctrl_step;
        }
        if seg.ctrl.x > seg.ctrl_to.x {
            seg.ctrl.x -= ctrl_step;
        }
        if seg.ctrl.y < seg.ctrl_to.y {
            seg.ctrl.y += ctrl_step;
        }
        if seg.ctrl.y > seg.ctrl_to.y {
            seg.ctrl.y -= ctrl_step;
        }

        if seg.ctrl2.x < seg.ctrl2_to.x {
            seg.ctrl2.x += ctrl_step;
        }
        if seg.ctrl2.x > seg.ctrl2_to.x {
            seg.ctrl2.x -= ctrl_step;
        }
        if seg.ctrl2.y < seg.ctrl2_to.y {
            seg.ctrl2.y += ctrl_step;
        }
        if seg.ctrl2.y > seg.ctrl2_to.y {
            seg.ctrl2.y -= ctrl_step;
        }
    }
}


fn draw_strip(draw: &mut Draw, strip: &mut Strip, ypos: f32, strip_height: f32) {
    let path = &mut draw.path();
    path.move_to(0.0, ypos);


    for seg in strip.segs.iter_mut() {
        if USE_CUBIC_BEZIER {
            path.cubic_bezier_to(
                (seg.ctrl.x, seg.ctrl.y),
                (seg.ctrl2.x, seg.ctrl2.y),
                (seg.to.x, seg.to.y),
            );
        } else {
            path.quadratic_bezier_to((seg.ctrl.x, seg.ctrl.y), (seg.to.x, seg.to.y));
        }
    }
    path.line_to(
        strip.segs.last().unwrap().to.x,
        strip.segs.last().unwrap().to.y + strip_height,
    );
    for seg in strip.segs.iter_mut().rev() {
        if USE_CUBIC_BEZIER {
            path.cubic_bezier_to(
                (seg.ctrl2.x, seg.ctrl2.y + strip_height),
                (seg.ctrl.x, seg.ctrl.y + strip_height),
                (seg.from.x, seg.from.y + strip_height),
            );
        } else {
            path.quadratic_bezier_to(
                (seg.ctrl.x, seg.ctrl.y + strip_height),
                (seg.from.x, seg.from.y + strip_height),
            );
        }
    }
    path.stroke_color(strip.stroke_color)
        .close()
        .stroke(STRIP_STROKE)
        .fill_color(strip.color)
        .fill()
        .alpha(strip.alpha);
}


fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    generate_strips(state, false);

    for strip in state.strips.iter_mut() {
        if !state.paused {
            update_strip(
                strip,
                state.displacement_pos,
                state.gen.displacement_pos_step,
                state.gen.displacement_range,
                state.gen.strip_interval,
                &state.work_size,
                &mut state.rng,
            );
        }
        draw_strip(draw, strip, strip.segs[0].from.y, state.gen.strip_height);
    }

    if state.show_displacement_pos {
        draw.path()
            .move_to(0.0, state.displacement_pos)
            .line_to(state.work_size.x, state.displacement_pos)
            .stroke_color(Color::RED)
            .stroke(1.0);
    }

    if !state.paused {
        move_displacement(state);
        if state.displacement_pos <= 0.0 {
            state.shuffle_counter += 1;
        }
    }

    gfx.render(draw);
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    // let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
    let win_config = get_common_win_config().high_dpi(true).size(
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

    let win_config = win_config.title("hilo_strips.smoove");
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
