use notan::draw::*;
use notan::extra::FpsLimit;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::emotion::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};
use palette::{FromColor, Hsl, Hsv, LinSrgb, Mix, RgbHue, Srgb};
use serde::{Deserialize, Serialize};
// use serde_json::{Result as JsonResult, Value};
use std::fs;


// See details at https://stackoverflow.com/a/42764117
const EMOCAT_DOCS: [&'static str; 9] = [
    include_str!("assets/lb_bronte01.json"),
    include_str!("assets/lb_dickinson01.json"),
    include_str!("assets/lb_dickinson02.json"),
    include_str!("assets/lb_howe01.json"),
    include_str!("assets/lb_hughes01.json"),
    include_str!("assets/lb_teasdale01.json"),
    include_str!("assets/wilde01.json"),
    include_str!("assets/lb_whitman01.json"),
    include_str!("assets/the_stagger.json"),
];


const CLEAR_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;
const DYNAMIC_TEXT_COLOR: bool = false;
const STARTING_MIX_FACTOR: f32 = 0.0;
// const MIX_RATE: f32 = 0.001;
// const MIX_RATE: f32 = 0.0001;
const MIX_RATE: f32 = 0.00001;
// const MIX_RATE: f32 = 0.000001;
const COLOR_COMPARISON_PRECISION: f32 = 3.0;
const MAX_FPS: u8 = 240;


// #[derive(PartialEq)]
enum View {
    HOME,
    READ,
}


struct ReadingViewState {
    doc_index: usize,
    analysis: usize,
}


#[derive(AppState)]
struct State {
    view: View,
    emodocs: Vec<EmocatOutputDoc>,
    reading: ReadingViewState,
    font: Font,
    title_font: Font,
    simple_color: Color,
    bg_color: Color,
    bg_color_mix_factor: f32,
    text_color: Color,
    dynamic_text_color: bool,
}


fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!(
            // "./assets/fonts/Ubuntu-B.ttf"
            // "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
            "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.spaced.ttf"
        ))
        .unwrap();

    let title_font = gfx
        .create_font(include_bytes!(
            "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
        ))
        .unwrap();

    let emodocs: Vec<EmocatOutputDoc> = EMOCAT_DOCS
        .iter()
        .map(|&doc| serde_json::from_str(doc).expect("Could not open emocat document"))
        .collect();

    let state = State {
        view: View::HOME,
        emodocs: emodocs,
        reading: ReadingViewState {
            doc_index: 0,
            analysis: 0,
        },
        font: font,
        title_font: title_font,
        simple_color: CLEAR_COLOR,
        bg_color: CLEAR_COLOR,
        bg_color_mix_factor: STARTING_MIX_FACTOR,
        text_color: TITLE_COLOR,
        dynamic_text_color: DYNAMIC_TEXT_COLOR,
    };
    state
}


/// Scale the font according to the current work size. Quite simple right now,
/// probably lots of room for improving this.
///
/// These return values were decided by comparing sizes on my own setup. Needs testing
/// across devices.
///
/// @TODO: What about portrait dimensions?
fn scale_font(default_size: f32, work_size: Vec2) -> f32 {
    if work_size.x >= ScreenDimensions::RES_1080P.x && work_size.x < ScreenDimensions::RES_1440P.x {
        // log::debug!("1080p");
        return default_size * 2.25;
    }
    if work_size.x >= ScreenDimensions::RES_1440P.x && work_size.x < ScreenDimensions::RES_4K.x {
        // log::debug!("1440p");
        return default_size * 3.0;
    }
    if work_size.x >= ScreenDimensions::RES_4K.x {
        // log::debug!("4k");
        return default_size * 4.5;
    }
    // log::debug!("Default.");
    return default_size * 1.0;
}


/// In this application, where font scaling is involved, a work size that matches
/// the window size results in nicer looking fonts. This comes at the expense of
/// not being able to use literal values for sizing shapes and such (not being able
/// to work against a known scale). Instead, one can use fractions of the work size
/// values.
fn get_work_size(gfx: &Graphics) -> Vec2 {
    // If we don't guard against a minimum like this, the app crashes if the window
    // is shrunk to a small size.
    if gfx.device.size().0 as f32 > ScreenDimensions::MINIMUM.x {
        return vec2(gfx.device.size().0 as f32, gfx.device.size().1 as f32);
    }
    ScreenDimensions::MINIMUM
}


/// Return black or white depending on the current background color
///
/// Based on this algorithm:
/// https://stackoverflow.com/a/1855903/4655636
///
fn get_text_color(state: &State) -> Color {
    let luminance: f32;
    if state.dynamic_text_color {
        luminance =
            0.299 * state.bg_color.r + 0.587 * state.bg_color.g + 0.114 * state.bg_color.b / 255.0;
    } else {
        luminance = 0.299 * state.simple_color.r
            + 0.587 * state.simple_color.g
            + 0.114 * state.simple_color.b / 255.0;
    }

    // log::debug!("Luminance {}", luminance);
    if luminance < 0.5 {
        return Color::WHITE;
    }
    Color::BLACK
}

fn round(val: f32, digits: f32) -> f32 {
    // log::debug!("{}, {}", val, (val * 100.0).round() / 100.0);
    // (val * 100.0).round() / 100.0

    let mut multiplier: f32 = 10.0;
    multiplier = multiplier.powf(digits);
    // log::debug!("{}, {}", val, (val * multiplier).round() / multiplier);
    (val * multiplier).round() / multiplier
}


fn update_bg_color(app: &App, state: &mut State) {
    // The mix function used to blend colors below doesn't always end up with the
    // exact floating point numbers of the end color, so comparing with rounded
    // color values instead of comparing the colors directly.
    let precision = COLOR_COMPARISON_PRECISION;
    // log::debug!(
    //     "{}::{}, {}::{}, {}::{}",
    //     round(state.bg_color.r, precision),
    //     round(state.simple_color.r, precision),
    //     round(state.bg_color.g, precision),
    //     round(state.simple_color.g, precision),
    //     round(state.bg_color.b, precision),
    //     round(state.simple_color.b, precision),
    // );
    if round(state.bg_color.r, precision) != round(state.simple_color.r, precision)
        || round(state.bg_color.g, precision) != round(state.simple_color.g, precision)
        || round(state.bg_color.b, precision) != round(state.simple_color.b, precision)
    {
        // log::debug!("Mix factor: {}", state.bg_color_mix_factor);
        let bg_color = Srgb::new(state.bg_color.r, state.bg_color.g, state.bg_color.b);
        let simple_color = Srgb::new(
            state.simple_color.r,
            state.simple_color.g,
            state.simple_color.b,
        );
        let mut bg_color = LinSrgb::from_color(bg_color);
        let simple_color = LinSrgb::from_color(simple_color);
        bg_color = bg_color.mix(&simple_color, state.bg_color_mix_factor);
        let bg_color = Srgb::from_color(bg_color);
        state.bg_color = Color::from_rgb(bg_color.red, bg_color.green, bg_color.blue);
        state.bg_color_mix_factor += MIX_RATE;
    } else {
        state.bg_color_mix_factor = STARTING_MIX_FACTOR;
    }
}


fn update_bg_color_simple(state: &mut State) {
    state.bg_color = state.simple_color.clone();
}


fn update_read_view(app: &mut App, state: &mut State) {
    let emodoc = &state.emodocs[state.reading.doc_index];

    if app.keyboard.was_pressed(KeyCode::Home) {
        log::debug!("home");
        state.reading.analysis = 0;
        state.simple_color = CLEAR_COLOR;
    }

    if app.keyboard.was_pressed(KeyCode::End) {
        log::debug!("end");
        state.reading.analysis = emodoc.analyses.len() - 1;
        state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
    }


    if app.keyboard.was_pressed(KeyCode::Left) && state.reading.analysis > 0 {
        log::debug!("left");
        state.reading.analysis -= 1;
        if state.reading.analysis > 0 {
            state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
        } else {
            state.simple_color = CLEAR_COLOR;
        }
    }

    if app.keyboard.was_pressed(KeyCode::Right) && state.reading.analysis < emodoc.analyses.len() {
        log::debug!("right");
        state.reading.analysis += 1;
        state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
    }
    // update_bg_color_simple(state);
    update_bg_color(app, state);
    state.text_color = get_text_color(&state);
}


fn update(app: &mut App, state: &mut State) {
    match state.view {
        View::READ => update_read_view(app, state),
        _ => (),
    }
}


fn draw_title(draw: &mut Draw, state: &State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    let mut textbox_width = work_size.x * 0.75;

    draw.text(&state.title_font, &emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(TITLE_COLOR)
        .size(scale_font(60.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
        .h_align_left()
        .v_align_middle();

    // draw.text(&state.font, &state.emodoc.title)
    //     .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
    //     .color(Color::RED)
    //     .size(scale_font(60.0, work_size))
    //     .max_width(textbox_width)
    //     .position(
    //         work_size.x * 0.5 - textbox_width * 0.5 + 1.0,
    //         work_size.y * 0.4 - 1.0,
    //     )
    //     .h_align_left()
    //     .v_align_middle();


    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.title_font, &format!("by {}", emodoc.author))
        .alpha_mode(BlendMode::OVER)
        .color(META_COLOR)
        .size(scale_font(30.0, work_size))
        .max_width(textbox_width)
        .position(
            work_size.x * 0.5 - textbox_width * 0.5,
            title_bounds.y + title_bounds.height + work_size.y * 0.1,
        )
        .h_align_left()
        .v_align_middle();
}


fn draw_paragraph(draw: &mut Draw, state: &State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    let textbox_width = work_size.x * 0.75;

    draw.text(
        &state.font,
        &emodoc.analyses[state.reading.analysis - 1].text,
    )
    .alpha_mode(BlendMode::OVER)
    .color(state.text_color)
    .size(scale_font(32.0, work_size))
    .max_width(textbox_width)
    .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.5)
    .v_align_middle()
    // .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.3)
    // .v_align_top()
    .h_align_left();

    // let title_bounds = draw.last_text_bounds();
}


fn draw_read_view(draw: &mut Draw, state: &State, work_size: Vec2) {
    if state.reading.analysis == 0 {
        draw_title(draw, state, work_size);
    } else {
        draw_paragraph(draw, state, work_size);
    }
}


fn draw_home_view(draw: &mut Draw, state: &State, work_size: Vec2) {
    let mut textbox_width = work_size.x * 0.75;

    let mut menu_item_ypos = work_size.y * 0.1;
    let menu_item_spacing = work_size.y * 0.05;
    for emodoc in state.emodocs.iter() {
        let doc_title = format!("{} by {}", &emodoc.title, &emodoc.author);
        draw.text(&state.title_font, &doc_title)
            .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
            .color(TITLE_COLOR)
            .size(scale_font(24.0, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, menu_item_ypos)
            .h_align_left()
            .v_align_middle();
        let title_bounds = draw.last_text_bounds();
        menu_item_ypos = title_bounds.max_y() + menu_item_spacing;
        // log::debug!("{}", emodoc.title);
    }
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let work_size = get_work_size(gfx);
    let mut draw = get_draw_setup(gfx, work_size, true, state.bg_color);

    match state.view {
        View::READ => draw_read_view(&mut draw, state, work_size),
        _ => draw_home_view(&mut draw, state, work_size),
    }

    // draw to screen
    gfx.render(&draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    // let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
    let win_config = get_common_win_config().high_dpi(true).size(
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);


    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .add_plugin(FpsLimit::new(MAX_FPS))
        .draw(draw)
        .update(update)
        .build()
}
