use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan::text::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::fs;


// enum ScreenDimensions {
//     // DEFAULT(Vec2),
//     // RES_1K(Vec2),
//     // RES_2K(Vec2),
//     // RES_4K(Vec2),
//     DEFAULT = vec2(1920.0, 1080.0),
//     RES_1K(Vec2),
//     RES_2K(Vec2),
//     RES_4K(Vec2),

// }


#[non_exhaustive]
struct ScreenDimensions;

impl ScreenDimensions {
    pub const DEFAULT: Vec2 = vec2(800.0, 600.0);
    pub const RES_1K: Vec2 = vec2(1920.0, 1080.0);
    pub const RES_2K: Vec2 = vec2(2048.0, 1080.0);
    pub const RES_4K: Vec2 = vec2(3840.0, 2160.0);
    pub const RES_4Kish: Vec2 = vec2(3600.0, 1800.0);
}


macro_rules! EMOCAT_OUTPUT_FILE {
    () => {
        "assets/wilde01.json"
    };
}
const WORK_SIZE: Vec2 = ScreenDimensions::RES_4Kish;
const CLEAR_COLOR: Color = Color::GRAY;


#[derive(Serialize, Deserialize, Debug)]
struct EmocatMeta {
    paragraph: u8,
    from_line: u8,
    to_line: u8,
    file: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct EmocatAnalyzerResult {
    fear: f32,
    anger: f32,
    anticipation: f32,
    trust: f32,
    surprise: f32,
    positive: f32,
    negative: f32,
    sadness: f32,
    disgust: f32,
    joy: f32,
}


#[derive(Serialize, Deserialize, Debug)]
struct EmocatAnalyzerResults {
    nrclex: EmocatAnalyzerResult,
    t2e_repo: EmocatAnalyzerResult,
    t2e_demo: EmocatAnalyzerResult,
}


#[derive(Serialize, Deserialize, Debug)]
struct EmocatTextAnalysis {
    text: String,
    meta: EmocatMeta,
    results: EmocatAnalyzerResults,
}


#[derive(Serialize, Deserialize, Debug)]
/// Represents an `emocat` output document
struct EmocatOutputDoc {
    title: String,
    author: String,
    analyses: Vec<EmocatTextAnalysis>,
}


#[derive(AppState)]
struct State {
    emodoc: EmocatOutputDoc,
    font: Font,
}


fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!(
            // "./assets/fonts/Ubuntu-B.ttf"
            "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
        ))
        .unwrap();

    let analyses_str = include_str!(EMOCAT_OUTPUT_FILE!());
    let emodoc: EmocatOutputDoc =
        serde_json::from_str(analyses_str).expect("Could not open emocat document");
    let state = State {
        emodoc: emodoc,
        font: font,
    };
    state
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, true, CLEAR_COLOR);

    let mut textbox_width = WORK_SIZE.x * 0.75;
    draw.text(&state.font, &state.emodoc.title)
        .color(Color::PURPLE)
        .size(60.0)
        .max_width(textbox_width)
        .position(WORK_SIZE.x * 0.5 - textbox_width * 0.5, WORK_SIZE.y * 0.4)
        .h_align_left()
        .v_align_middle();

    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.font, &format!("By {}", state.emodoc.author))
        .color(Color::BLACK)
        .size(40.0)
        .max_width(textbox_width)
        .position(
            WORK_SIZE.x * 0.5 - textbox_width * 0.5,
            title_bounds.y + title_bounds.height + 50.0,
        )
        // .position(WORK_SIZE.x * 0.25, title_bounds.y + title_bounds.height + 50.0)
        .h_align_left()
        .v_align_middle();


    // draw to screen
    gfx.render(&draw);
    // gfx.render(&text);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config()
        .high_dpi(true)
        .size(WORK_SIZE.x as i32, WORK_SIZE.y as i32);

    // notan::init_with(State::default)
    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
