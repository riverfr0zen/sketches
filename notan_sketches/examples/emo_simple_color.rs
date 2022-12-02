use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan::text::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::fs;


macro_rules! EMOCAT_OUTPUT_FILE {
    () => {
        "assets/wilde01.json"
    };
}
// const WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
const WORK_SIZE: Vec2 = ScreenDimensions::RES_4K;
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


fn scale_font(default_size: f32) -> f32 {
    if WORK_SIZE.x == ScreenDimensions::RES_1K.x {
        return default_size * 1.5;
    }
    if WORK_SIZE.x == ScreenDimensions::RES_2K.x {
        return default_size * 2.0;
    }
    if WORK_SIZE.x == ScreenDimensions::RES_4K.x {
        return default_size * 3.0;
    }
    return default_size * 1.0;
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, true, CLEAR_COLOR);

    let mut textbox_width = WORK_SIZE.x * 0.75;
    draw.text(&state.font, &state.emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(Color::PURPLE)
        .size(scale_font(60.0))
        .max_width(textbox_width)
        .position(WORK_SIZE.x * 0.5 - textbox_width * 0.5, WORK_SIZE.y * 0.4)
        .h_align_left()
        .v_align_middle();

    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.font, &format!("By {}", state.emodoc.author))
        .alpha_mode(BlendMode::OVER)
        .color(Color::BLACK)
        .size(scale_font(30.0))
        .max_width(textbox_width)
        .position(
            WORK_SIZE.x * 0.5 - textbox_width * 0.5,
            title_bounds.y + title_bounds.height + WORK_SIZE.y * 0.1,
        )
        .h_align_left()
        .v_align_middle();


    // draw to screen
    gfx.render(&draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_2K.x as i32,
        // ScreenDimensions::RES_2K.y as i32,
        ScreenDimensions::DEFAULT.x as i32,
        ScreenDimensions::DEFAULT.y as i32,
    );

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
