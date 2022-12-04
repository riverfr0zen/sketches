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
        // "assets/the_stagger.json"
    };
}
const CLEAR_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;


#[derive(Serialize, Deserialize, Debug)]
struct EmocatMeta {
    paragraph: u8,
    from_line: u8,
    to_line: u8,
    file: String,
}


// #[derive(Serialize, Deserialize, Debug)]
// struct EmocatAnalyzerResult {
//     fear: f32,
//     anger: f32,
//     anticipation: f32,
//     trust: f32,
//     surprise: f32,
//     positive: f32,
//     negative: f32,
//     sadness: f32,
//     disgust: f32,
//     joy: f32,
// }


#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmocatAnalyzerScore {
    marker: String,
    score: f32,
}


#[derive(Serialize, Deserialize, Debug)]
struct EmocatAnalyzerResults {
    nrclex: Vec<EmocatAnalyzerScore>,
    t2e_repo: Vec<EmocatAnalyzerScore>,
    t2e_demo: Vec<EmocatAnalyzerScore>,
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
    analysis: usize,
    simple_color: Color,
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
        analysis: 0,
        simple_color: CLEAR_COLOR,
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


// Come back to this after getting "top emotions"
// struct EmotionColorPairing {
//     emotion: String,
//     color: Color,
// }


// fn get_plutchik_color_map() -> Vec<EmotionColorPairing> {
//     vec![EmotionColorPairing {
//         emotion: "fear".to_string(),
//         color: Color::RED,
//     }]
// }


/// Simple Color Model. See README for description.
fn get_simple_color_for_emo(analysis: &EmocatTextAnalysis) -> Color {
    let scores = &mut analysis.results.nrclex.clone();
    // log::debug!("Scores before {:?}", scores);

    let positive_pos = scores.iter().position(|s| s.marker == "positive").unwrap();
    let positive_sentiment = scores.remove(positive_pos);
    let negative_pos = scores.iter().position(|s| s.marker == "negative").unwrap();
    let negative_sentiment = scores.remove(positive_pos);

    scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    // log::debug!("Score after {:?}", scores);

    let mut top_emotions: Vec<&EmocatAnalyzerScore> = Vec::new();
    top_emotions.push(&scores[0]);
    for score in scores.iter().skip(1) {
        if score.score == top_emotions[0].score {
            top_emotions.push(&score);
        }
    }
    if top_emotions[0].score > 0.0 {
        log::debug!("Top emotions: {:?}:", top_emotions);
        let second_emo = &scores[1];
        if second_emo.score > 0.0 {
            log::debug!(
                "Second emotion: {}: {}",
                second_emo.marker,
                second_emo.score
            );
        }
    } else {
        log::debug!("No top emotion!");
        // ?? no emotional values
    }

    // XXX TODO after gettting results in a better format!
    CLEAR_COLOR
}


fn update(app: &mut App, state: &mut State) {
    // if app.keyboard.is_down(KeyCode::W) {
    //     state.y -= MOVE_SPEED * app.timer.delta_f32();
    // }

    if app.keyboard.was_pressed(KeyCode::Home) {
        log::debug!("home");
        state.analysis = 0;
        state.simple_color = CLEAR_COLOR
    }

    if app.keyboard.was_pressed(KeyCode::End) {
        log::debug!("end");
        state.analysis = state.emodoc.analyses.len() - 1;
        state.simple_color = get_simple_color_for_emo(&state.emodoc.analyses[state.analysis - 1]);
    }


    if app.keyboard.was_pressed(KeyCode::Left) && state.analysis > 0 {
        log::debug!("left");
        state.analysis -= 1;
        if state.analysis > 0 {
            state.simple_color =
                get_simple_color_for_emo(&state.emodoc.analyses[state.analysis - 1]);
        }
    }

    if app.keyboard.was_pressed(KeyCode::Right) && state.analysis < state.emodoc.analyses.len() {
        log::debug!("right");
        state.analysis += 1;
        state.simple_color = get_simple_color_for_emo(&state.emodoc.analyses[state.analysis - 1]);
    }
}


fn draw_title(draw: &mut Draw, state: &State, work_size: Vec2) {
    let mut textbox_width = work_size.x * 0.75;
    draw.text(&state.font, &state.emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(TITLE_COLOR)
        .size(scale_font(60.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
        .h_align_left()
        .v_align_middle();

    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.font, &format!("by {}", state.emodoc.author))
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
    let textbox_width = work_size.x * 0.75;
    draw.text(&state.font, &state.emodoc.analyses[state.analysis - 1].text)
        .alpha_mode(BlendMode::OVER)
        .color(TITLE_COLOR)
        .size(scale_font(24.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.5)
        .v_align_middle()
        // .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.3)
        // .v_align_top()
        .h_align_left();

    // let title_bounds = draw.last_text_bounds();
}

fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let work_size = get_work_size(gfx);
    let mut draw = get_draw_setup(gfx, work_size, true, state.simple_color);

    if state.analysis == 0 {
        draw_title(&mut draw, state, work_size);
    } else {
        draw_paragraph(&mut draw, state, work_size);
    }

    // draw to screen
    gfx.render(&draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_1080P.x as i32,
        // ScreenDimensions::RES_1080P.y as i32,
        ScreenDimensions::DEFAULT.x as i32,
        ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);


    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}
