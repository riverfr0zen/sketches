use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan::text::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};
use palette::{FromColor, Hsv, Mix, RgbHue, Srgb};
use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::fs;


macro_rules! EMOCAT_OUTPUT_FILE {
    () => {
        // "assets/wilde01.json"
        "assets/the_stagger.json"
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


struct EmoColor {
    emotion: String,
    sentiment: Sentiment,
    hsv: Hsv,
}


enum Sentiment {
    POSITIVE,
    NEGATIVE,
    NEUTRAL,
}


fn get_emotion_sentiment(emotion: &str) -> Sentiment {
    match emotion {
        "fear" => Sentiment::NEGATIVE,
        "anger" => Sentiment::NEGATIVE,
        "anticipation" => Sentiment::NEUTRAL,
        "trust" => Sentiment::POSITIVE,
        "surprise" => Sentiment::NEUTRAL,
        "sadness" => Sentiment::NEGATIVE,
        "disgust" => Sentiment::NEGATIVE,
        "joy" => Sentiment::POSITIVE,
        _ => Sentiment::NEUTRAL,
    }
}


/// Returns color mapped to the emotion provided based on Plutchik color wheel here:
/// http://shelleycrick.com/how-color-affects-emotions/
fn get_mapped_color_plutchik(emotion: &str) -> Hsv {
    match emotion {
        "fear" => Hsv::new(RgbHue::from_degrees(88.0), 1.0, 0.59),
        "anger" => Hsv::new(RgbHue::from_degrees(350.0), 1.0, 0.72),
        "anticipation" => Hsv::new(RgbHue::from_degrees(21.0), 1.0, 0.96),
        "trust" => Hsv::new(RgbHue::from_degrees(69.0), 1.0, 0.72),
        "surprise" => Hsv::new(RgbHue::from_degrees(136.0), 0.98, 0.50),
        "sadness" => Hsv::new(RgbHue::from_degrees(206.0), 1.0, 0.85),
        "disgust" => Hsv::new(RgbHue::from_degrees(300.0), 1.0, 0.24),
        "joy" => Hsv::new(RgbHue::from_degrees(55.0), 1.0, 0.91),
        _ => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 0.0),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 1.0),
    }
}


/// Returns color mapped to the emotion provided based on the art therapy color
/// associations here:
/// http://www.arttherapyblog.com/online/color-meanings-symbolism
fn get_mapped_color_therapy(emotion: &str) -> Hsv {
    match emotion {
        "fear" => Hsv::new(RgbHue::from_degrees(60.0), 0.8, 1.0),
        "anger" => Hsv::new(RgbHue::from_degrees(5.0), 0.93, 1.0),
        // Loosely interpreting anticipation to be green
        "anticipation" => Hsv::new(RgbHue::from_degrees(95.0), 0.72, 0.69),
        "trust" => Hsv::new(RgbHue::from_degrees(224.0), 0.99, 1.0),
        // Loosely interpreting surprise as violet
        "surprise" => Hsv::new(RgbHue::from_degrees(286.0), 0.99, 0.69),
        "sadness" => Hsv::new(RgbHue::from_degrees(224.0), 0.99, 1.0),
        // Cannot find an equivalent, so just going to return gray
        "disgust" => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        "joy" => Hsv::new(RgbHue::from_degrees(36.0), 0.99, 0.98),
        _ => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 0.0),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 1.0),
    }
}


/// Returns colors & sentiment mapped to the emotion provided
fn get_mapped_emocolor(emotion: &str, mapping_func: &dyn Fn(&str) -> Hsv) -> EmoColor {
    match emotion {
        "fear" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "anger" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "anticipation" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
        "trust" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::POSITIVE,
            hsv: mapping_func(emotion),
        },
        "surprise" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
        "sadness" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "disgust" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "joy" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::POSITIVE,
            hsv: mapping_func(emotion),
        },
        _ => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
    }
}


/// Simple Color Model. See README for description.
fn get_simple_color_for_emo(analysis: &EmocatTextAnalysis) -> Color {
    let scores = &mut analysis.results.nrclex.clone();
    // log::debug!("Scores before {:?}", scores);

    let positive_pos = scores.iter().position(|s| s.marker == "positive").unwrap();
    let positive_sentiment = scores.remove(positive_pos);
    let negative_pos = scores.iter().position(|s| s.marker == "negative").unwrap();
    let negative_sentiment = scores.remove(negative_pos);
    log::debug!(
        "positive: {}, negative: {}",
        positive_sentiment.score,
        negative_sentiment.score
    );

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
        let mapping_func = get_mapped_color_plutchik;
        let emocolors: Vec<EmoColor> = top_emotions
            .iter()
            .map(|&s| get_mapped_emocolor(&s.marker, &mapping_func))
            .collect();
        // Start with a neutral gray
        if emocolors.len() > 1 {
            let mut final_color = get_mapped_emocolor("", &mapping_func).hsv;
            for emocolor in emocolors.iter() {
                log::debug!("Before mix: {:?}", final_color);
                let sentiment_value: f32 = match &emocolor.sentiment {
                    Sentiment::POSITIVE => positive_sentiment.score,
                    Sentiment::NEGATIVE => negative_sentiment.score,
                    Sentiment::NEUTRAL => positive_sentiment.score.max(negative_sentiment.score),
                };
                // The sentiment values don't often seem to go beyond 0.5, so I'm modifying the
                // mix factor a little. Must test later with more examples of text.
                let mix_factor = sentiment_value * 2.0;
                log::debug!(
                    "Emotion: {}, Sentiment value: {}, Mix_factor: {}",
                    emocolor.emotion,
                    sentiment_value,
                    mix_factor
                );
                // final_color = final_color.mix(&emocolor.hsv, 0.5);
                final_color = final_color.mix(&emocolor.hsv, mix_factor);
                log::debug!("After mix: {:?}", final_color);
            }
            let color = Srgb::from_color(final_color);
            return Color::from_rgb(color.red, color.green, color.blue);
        } else {
            let color = Srgb::from_color(emocolors[0].hsv);
            return Color::from_rgb(color.red, color.green, color.blue);
        }
    }
    Color::GRAY
}


fn update(app: &mut App, state: &mut State) {
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
