use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2, Vec4};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::colors::Palettes;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::enums;
use notan_sketches::mathutils::mid;
#[cfg(not(debug_assertions))]
use notan_sketches::shaderutils::create_shape_pipeline;
#[cfg(debug_assertions)]
use notan_sketches::shaderutils::{create_hot_shape_pipeline, ShaderReloadManager};
use notan_sketches::shaderutils::{CommonData, ShaderRenderTexture};
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};
use palette::{Darken, FromColor, Hsv, Lighten, Srgb};
use std::ops::RangeInclusive;

// const CLEAR_COLOR: Color = Color::WHITE;
const CLEAR_COLOR: Color = Color::BLACK;
// const STRIP_STROKE: f32 = 2.0;
const STRIP_STROKE: f32 = 5.0;

#[cfg(not(debug_assertions))]
const FRAG: ShaderSource =
    notan::include_fragment_shader!("examples/assets/shaders/horizontal_city.frag.glsl");
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
    shader_rt: Option<ShaderRenderTexture>,
    shader_curve_ubo: Option<Buffer>,
}

// Uniform data for curve warping (8 Vec4 = 32 samples)
const CURVE_SAMPLES: usize = 32;

#[uniform]
#[repr(C)]
#[derive(Copy, Clone)]
struct CurveData {
    // Using 8 separate Vec4s instead of an array to avoid std140 limitations
    s0: Vec4,
    s1: Vec4,
    s2: Vec4,
    s3: Vec4,
    s4: Vec4,
    s5: Vec4,
    s6: Vec4,
    s7: Vec4,
    strip_y: f32,      // Base Y position of the strip
    strip_height: f32, // Height of the strip
    num_samples: f32,  // Number of valid samples
    _padding: f32,     // Alignment padding
    bg_color: Vec4,    // Background color of the strip (rgba)
}

#[derive(Debug)]
pub struct GenSettings {
    pub seg_width: f32,
    pub strip_interval: f32,
    pub strip_height: f32,
    pub displacement_pos_step: f32,
    pub displacement_range: f32,
    pub palette: colors::PalettesSelection,
    pub clear_color: Color,
}

impl GenSettings {
    fn default(work_size: &Vec2) -> Self {
        let seg_width = 0.2 * work_size.x;
        let strip_interval = 0.1 * work_size.y;
        let strip_height = 0.08 * work_size.y;
        let displacement_pos_step: f32 = 10.0;
        let displacement_range: f32 = 0.3;
        let palette = colors::PalettesSelection::All;
        let clear_color = CLEAR_COLOR;

        Self {
            seg_width,
            strip_interval,
            strip_height,
            displacement_pos_step,
            displacement_range,
            palette,
            clear_color,
        }
    }

    fn randomize(rng: &mut Random, work_size: &Vec2) -> Self {
        // default = Self::default(&work_size)
        let seed;
        (*rng, seed) = get_rng(None);
        log::info!("Seed: {}", seed);

        let seg_width = rng.gen_range(SEG_WIDTH) * work_size.x;
        let strip_interval = rng.gen_range(STRIP_INTERVAL) * work_size.y;
        let strip_height = rng.gen_range(STRIP_HEIGHT) * work_size.y;
        let displacement_pos_step = rng.gen_range(DISPLACEMENT_POS_STEP);
        let displacement_range = rng.gen_range(DISPLACEMENT_RANGE);
        let palette: PalettesSelection = rng.gen();
        // let clear_color = Palettes::choose_color(&clear_palette);
        let clear_color = match rng.gen_bool(0.5) {
            true => Palettes::choose_color(&palette),
            false => {
                let clear_palette: PalettesSelection = rng.gen();
                Palettes::choose_color(&clear_palette)
            }
        };

        Self {
            seg_width,
            strip_interval,
            strip_height,
            displacement_pos_step,
            displacement_range,
            // clear_color: Palettes::choose_color(&clear_palette),
            palette,
            clear_color,
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
    pub shader_pipeline: Pipeline,
    pub shader_ubo: Buffer,
    #[cfg(debug_assertions)]
    pub hot_mgr: ShaderReloadManager,
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

    // Initialize shader pipeline
    #[cfg(not(debug_assertions))]
    let shader_pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();
    #[cfg(debug_assertions)]
    let shader_pipeline =
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/horizontal_city.frag.glsl")
            .unwrap();

    let shader_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&CommonData::new(0.0, work_size))
        .build()
        .unwrap();

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
        shader_pipeline,
        shader_ubo,
        #[cfg(debug_assertions)]
        hot_mgr: ShaderReloadManager::default(),
    }
}

/// Get the distance of the strip from the displacement
fn get_displacement_distance(strip: &Strip, displacement_pos: &f32, work_size: &Vec2) -> f32 {
    (strip.segs[0].from.y - displacement_pos).abs() / work_size.y
}

fn add_strip(state: &mut State, gfx: &mut Graphics) {
    let color = colors::Palettes::choose_color(&state.gen.palette);
    let stroke_color = Srgb::new(color.r, color.g, color.b);
    let mut stroke_color = Hsv::from_color(stroke_color);
    match state.rng.gen_bool(0.5) {
        true => {
            // log::info!("darken");
            stroke_color = stroke_color.darken(0.2);
        }
        false => {
            // log::info!("lighten");
            // Lighten factor intentionally more than darken above
            stroke_color = stroke_color.lighten(0.5);
        }
    }
    let stroke_color = Srgb::from_color(stroke_color);

    let use_shader = state.rng.gen_bool(0.3); // 30% chance of using shader
    let (shader_rt, shader_curve_ubo) = if use_shader {
        log::debug!("Strip at y={} will use shader", state.cursor.y);
        let rt = Some(ShaderRenderTexture::new(
            gfx,
            state.work_size.x,
            state.work_size.y,
        ));
        let curve_ubo = Some(
            gfx.create_uniform_buffer(2, "CurveData")
                .with_data(&CurveData {
                    s0: Vec4::ZERO,
                    s1: Vec4::ZERO,
                    s2: Vec4::ZERO,
                    s3: Vec4::ZERO,
                    s4: Vec4::ZERO,
                    s5: Vec4::ZERO,
                    s6: Vec4::ZERO,
                    s7: Vec4::ZERO,
                    strip_y: 0.0,
                    strip_height: 0.0,
                    num_samples: CURVE_SAMPLES as f32,
                    _padding: 0.0,
                    bg_color: Vec4::new(color.r, color.g, color.b, color.a),
                })
                .build()
                .unwrap(),
        );
        (rt, curve_ubo)
    } else {
        (None, None)
    };

    let mut strip = Strip {
        segs: vec![],
        color: color,
        stroke_color: Color::new(stroke_color.red, stroke_color.green, stroke_color.blue, 1.0),
        alpha: state.rng.gen_range(0.2..1.0),
        last_distance: 0.0,
        displaced: false,
        shader_rt,
        shader_curve_ubo,
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

fn generate_strips(state: &mut State, gfx: &mut Graphics, refresh: bool) {
    if refresh {
        state.strips = vec![];
        state.cursor = Vec2::new(0.0, 0.0);
    }

    // Cursor for testing  w/ a single line
    // if state.strips.len() == 0 {
    //     state.cursor.y = 300.0;
    //     add_strip(state, gfx);
    // }
    // Cursor for all lines
    if state.cursor.y < state.work_size.y + state.gen.strip_interval {
        add_strip(state, gfx);
        state.cursor.y += state.gen.strip_interval;
    }
}

fn shuffle(state: &mut State, gfx: &mut Graphics) {
    state.shuffle_counter = 0;
    state.gen = GenSettings::randomize(&mut state.rng, &state.work_size);
    generate_strips(state, gfx, true);
    log::debug!("{:#?}", state.gen);
}

fn update(app: &mut App, state: &mut State) {
    #[cfg(debug_assertions)]
    state.hot_mgr.update();

    if app.keyboard.was_pressed(KeyCode::P) {
        state.paused = !state.paused;
        log::debug!("pause toggled");
    }

    if app.keyboard.was_pressed(KeyCode::R) {
        state.shuffle_counter = SHUFFLE_PERIOD; // Trigger shuffle in draw
    }

    if app.keyboard.was_pressed(KeyCode::D) {
        state.show_displacement_pos = !state.show_displacement_pos;
    }

    if app.keyboard.was_pressed(KeyCode::S) {
        state.auto_shuffle = !state.auto_shuffle;
        log::debug!("shuffle toggled");
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

fn draw_strip(draw: &mut Draw, strip: &Strip, ypos: f32, strip_height: f32) {
    let path = &mut draw.path();
    path.move_to(0.0, ypos);

    for seg in strip.segs.iter() {
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
    for seg in strip.segs.iter().rev() {
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
    path.close()
        .stroke_color(strip.stroke_color)
        .stroke(STRIP_STROKE)
        .fill_color(strip.color)
        .fill()
        .alpha(strip.alpha);
}

// Sample the Bezier curve at regular intervals and return y-offsets
fn sample_curve(strip: &Strip, work_size: Vec2, strip_height: f32) -> CurveData {
    let mut samples = [0.0; CURVE_SAMPLES];
    let base_y = strip.segs[0].from.y;
    let step = work_size.x / (CURVE_SAMPLES as f32);

    for i in 0..CURVE_SAMPLES {
        let x = i as f32 * step;

        // Find which segment this x falls into
        let mut y = base_y;
        for seg in &strip.segs {
            if x >= seg.from.x && x <= seg.to.x {
                // Interpolate along the cubic bezier curve
                let t = (x - seg.from.x) / (seg.to.x - seg.from.x);

                if USE_CUBIC_BEZIER {
                    // Cubic bezier formula: (1-t)^3 * P0 + 3(1-t)^2 * t * P1 + 3(1-t) * t^2 * P2 + t^3 * P3
                    let t2 = t * t;
                    let t3 = t2 * t;
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;
                    let mt3 = mt2 * mt;

                    y = mt3 * seg.from.y
                        + 3.0 * mt2 * t * seg.ctrl.y
                        + 3.0 * mt * t2 * seg.ctrl2.y
                        + t3 * seg.to.y;
                } else {
                    // Quadratic bezier formula: (1-t)^2 * P0 + 2(1-t) * t * P1 + t^2 * P2
                    let t2 = t * t;
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;

                    y = mt2 * seg.from.y + 2.0 * mt * t * seg.ctrl.y + t2 * seg.to.y;
                }
                break;
            }
        }

        // Store the normalized offset from base position
        samples[i] = (y - base_y) / work_size.y;
    }

    CurveData {
        s0: Vec4::new(samples[0], samples[1], samples[2], samples[3]),
        s1: Vec4::new(samples[4], samples[5], samples[6], samples[7]),
        s2: Vec4::new(samples[8], samples[9], samples[10], samples[11]),
        s3: Vec4::new(samples[12], samples[13], samples[14], samples[15]),
        s4: Vec4::new(samples[16], samples[17], samples[18], samples[19]),
        s5: Vec4::new(samples[20], samples[21], samples[22], samples[23]),
        s6: Vec4::new(samples[24], samples[25], samples[26], samples[27]),
        s7: Vec4::new(samples[28], samples[29], samples[30], samples[31]),
        strip_y: base_y / work_size.y,
        strip_height: strip_height / work_size.y,
        num_samples: CURVE_SAMPLES as f32,
        _padding: 0.0,
        bg_color: Vec4::new(strip.color.r, strip.color.g, strip.color.b, strip.color.a),
    }
}

fn draw_shader_strip(
    draw: &mut Draw,
    gfx: &mut Graphics,
    strip: &mut Strip,
    shader_pipeline: &Pipeline,
    shader_ubo: &Buffer,
    strip_height: f32,
    work_size: Vec2,
) {
    let ypos = strip.segs[0].from.y;

    // Sample the curve and update the curve uniform buffer
    if let Some(curve_ubo) = &strip.shader_curve_ubo {
        let curve_data = sample_curve(strip, work_size, strip_height);
        gfx.set_buffer_data(curve_ubo, &curve_data);
    }

    if let Some(shader_rt) = &mut strip.shader_rt {
        let ubos = if let Some(curve_ubo) = &strip.shader_curve_ubo {
            vec![shader_ubo, curve_ubo]
        } else {
            vec![shader_ubo]
        };

        shader_rt.draw(gfx, shader_pipeline, ubos, |shader_draw| {
            let path = &mut shader_draw.path();
            path.move_to(0.0, ypos);

            for seg in strip.segs.iter() {
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
            for seg in strip.segs.iter().rev() {
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
            path.close().fill_color(strip.color).fill();
        });

        // Draw the shader texture
        draw.image(&shader_rt.rt)
            .position(0.0, 0.0)
            .size(work_size.x, work_size.y)
            .alpha(strip.alpha);
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    // Check if we need to shuffle before drawing
    if state.auto_shuffle && state.shuffle_counter >= SHUFFLE_PERIOD {
        shuffle(state, gfx);
    }

    let draw = &mut get_draw_setup(gfx, state.work_size, false, state.gen.clear_color);

    generate_strips(state, gfx, false);

    // Update shader uniform
    let u_time = app.timer.elapsed_f32();
    let common_data = CommonData::new(u_time, state.work_size);

    #[cfg(debug_assertions)]
    if state.hot_mgr.needs_reload() {
        match create_hot_shape_pipeline(gfx, "examples/assets/shaders/horizontal_city.frag.glsl") {
            Ok(pipeline) => state.shader_pipeline = pipeline,
            Err(err) => log::error!("{}", err),
        }

        state.shader_ubo = gfx
            .create_uniform_buffer(1, "Common")
            .with_data(&common_data)
            .build()
            .unwrap();
    }

    gfx.set_buffer_data(&state.shader_ubo, &common_data);

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

        if strip.shader_rt.is_some() {
            draw_shader_strip(
                draw,
                gfx,
                strip,
                &state.shader_pipeline,
                &state.shader_ubo,
                state.gen.strip_height,
                state.work_size,
            );
        } else {
            draw_strip(draw, strip, strip.segs[0].from.y, state.gen.strip_height);
        }
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
    let win_config = get_common_win_config().set_high_dpi(true).set_size(
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
    let win_config = get_common_win_config().set_high_dpi(true);

    let win_config = win_config.set_title("hilo_strips.smoove");
    set_html_bgcolor(colors::MAHOGANY);

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
