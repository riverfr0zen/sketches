use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan::text::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::fs;


macro_rules! EMOCAT_OUTPUT_FILE {
    () => {
        "assets/wilde01.json"
    };
}
const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const EMOCAT_ANALYSIS: &str = "examples/assets/wilde01.json";


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
            // "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
            "./assets/fonts/Ubuntu-B.ttf"
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
    let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::GRAY);
    // print!("{}", state.emodoc.title);

    let mut text = gfx.create_text();
    text.add(&state.emodoc.title)
        .font(&state.font)
        .color(Color::PURPLE)
        .size(60.0)
        .max_width(WORK_SIZE.x * 0.5)
        .position(WORK_SIZE.x * 0.25, 200.0)
        .h_align_left()
        .v_align_middle();

    draw.text(&state.font, &state.emodoc.title)
        .color(Color::PURPLE)
        .size(60.0)
        .max_width(WORK_SIZE.x * 0.5)
        .position(WORK_SIZE.x * 0.25, 400.0)
        .h_align_left()
        .v_align_middle();

    print!("{:?}", draw.last_text_bounds());

    // draw to screen
    gfx.render(&draw);
    // gfx.render(&text);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

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
