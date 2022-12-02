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


/// Scale the font according to the current work size. Quite simple right now,
/// probably lots of room for improving this.
///
/// These return values were decided by comparing sizes on my own setup. Needs testing
/// across devices.
///
/// @TODO: What about portrait dimensions?
fn scale_font(default_size: f32, work_size: Vec2) -> f32 {
    if work_size.x >= ScreenDimensions::RES_1080P.x && work_size.x < ScreenDimensions::RES_1440P.x {
        log::debug!("1080p");
        return default_size * 2.25;
    }
    if work_size.x >= ScreenDimensions::RES_1440P.x && work_size.x < ScreenDimensions::RES_4K.x {
        log::debug!("1440p");
        // Right now 2K is same as 1K because widths are the same
        return default_size * 3.0;
    }
    if work_size.x >= ScreenDimensions::RES_4K.x {
        log::debug!("4k");
        return default_size * 4.5;
    }
    log::debug!("Default.");
    return default_size * 1.0;
}


fn get_work_size(gfx: &Graphics) -> Vec2 {
    return vec2(gfx.device.size().0 as f32, gfx.device.size().1 as f32);
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let work_size = get_work_size(gfx);
    let mut draw = get_draw_setup(gfx, work_size, true, CLEAR_COLOR);

    let mut textbox_width = work_size.x * 0.75;
    draw.text(&state.font, &state.emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(Color::PURPLE)
        .size(scale_font(60.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
        .h_align_left()
        .v_align_middle();

    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.font, &format!("By {}", state.emodoc.author))
        .alpha_mode(BlendMode::OVER)
        .color(Color::BLACK)
        .size(scale_font(30.0, work_size))
        .max_width(textbox_width)
        .position(
            work_size.x * 0.5 - textbox_width * 0.5,
            title_bounds.y + title_bounds.height + work_size.y * 0.1,
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
        // ScreenDimensions::RES_1080P.x as i32,
        // ScreenDimensions::RES_1080P.y as i32,
        ScreenDimensions::DEFAULT.x as i32,
        ScreenDimensions::DEFAULT.y as i32,
    );

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
