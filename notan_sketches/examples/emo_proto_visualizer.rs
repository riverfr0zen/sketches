use notan::draw::*;
use notan::extra::FpsLimit;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::emotion::*;
use notan_sketches::emotion_bg_visualizer::get_work_size;
use notan_sketches::emotion_bg_visualizer::ui::scale_font;
use notan_sketches::emotion_bg_visualizer::visualizers::color_transition::ColorTransitionVisualizer;
use notan_sketches::emotion_bg_visualizer::visualizers::tile::TileVisualizer;
use notan_sketches::emotion_bg_visualizer::visualizers::EmoVisualizer;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};


macro_rules! EMOCAT_OUTPUT_FILE {
    () => {
        // "assets/lb_bronte01.json"
        // "assets/lb_dickinson01.json"
        // "assets/lb_dickinson02.json"
        // "assets/lb_howe01.json"
        // "assets/lb_hughes01.json"
        // "assets/lb_teasdale01.json"
        // "assets/wilde01.json"
        "assets/lb_whitman01.json"
        // "assets/the_stagger.json"
    };
}


const CLEAR_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;
const DYNAMIC_TEXT_COLOR: bool = false;
const MAX_FPS: u8 = 240;
// const VISUALIZER: &str = "ColorTransitionVisualizer";
const VISUALIZER: &str = "TileVisualizer";


#[derive(AppState)]
struct State {
    emodoc: EmocatOutputDoc,
    font: Font,
    title_font: Font,
    analysis: usize,
    // visualizer: ColorTransitionVisualizer,
    visualizer: Box<dyn EmoVisualizer>,
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

    let analyses_str = include_str!(EMOCAT_OUTPUT_FILE!());
    let emodoc: EmocatOutputDoc =
        serde_json::from_str(analyses_str).expect("Could not open emocat document");
    let state = State {
        emodoc: emodoc,
        font: font,
        title_font: title_font,
        analysis: 0,
        visualizer: match VISUALIZER {
            "TileVisualizer" => Box::new(TileVisualizer::new(
                CLEAR_COLOR,
                TITLE_COLOR,
                DYNAMIC_TEXT_COLOR,
            )),
            _ => Box::new(ColorTransitionVisualizer::new(
                CLEAR_COLOR,
                TITLE_COLOR,
                DYNAMIC_TEXT_COLOR,
            )),
        },
    };
    state
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Home) {
        log::debug!("home");
        state.analysis = 0;
        state
            .visualizer
            .gracefully_reset(CLEAR_COLOR, TITLE_COLOR, DYNAMIC_TEXT_COLOR);
    }

    if app.keyboard.was_pressed(KeyCode::End) {
        log::debug!("end");
        state.analysis = state.emodoc.analyses.len() - 1;
        state
            .visualizer
            .update_model(&state.emodoc.analyses[state.analysis - 1]);
    }


    if app.keyboard.was_pressed(KeyCode::Left) && state.analysis > 0 {
        log::debug!("left");
        state.analysis -= 1;
        if state.analysis > 0 {
            state
                .visualizer
                .update_model(&state.emodoc.analyses[state.analysis - 1]);
        } else {
            state
                .visualizer
                .gracefully_reset(CLEAR_COLOR, TITLE_COLOR, DYNAMIC_TEXT_COLOR);
        }
    }

    if app.keyboard.was_pressed(KeyCode::Right) && state.analysis < state.emodoc.analyses.len() {
        log::debug!("right");
        state.analysis += 1;
        state
            .visualizer
            .update_model(&state.emodoc.analyses[state.analysis - 1]);
    }
    state.visualizer.update_visualization();
}


fn draw_title(draw: &mut Draw, state: &State, work_size: Vec2) {
    let mut textbox_width = work_size.x * 0.75;


    draw.text(&state.title_font, &state.emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(TITLE_COLOR)
        .size(scale_font(60.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
        .h_align_left()
        .v_align_middle();


    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.title_font, &format!("by {}", state.emodoc.author))
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
        .color(state.visualizer.get_text_color())
        .size(scale_font(32.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.5)
        .v_align_middle()
        .h_align_left();

    // @TODO: Add shadow/highlight effect later, maybe once there is a DrawTextItems trait so that
    // different visualizers can draw text differently (and maybe there is default implementation
    // that draws the basic version)
    //
    // let text_bounds = draw.last_text_bounds();
    // let offset_x = work_size.x * 0.0002;
    // let offset_y = work_size.y * 0.0002;
    // draw.text(&state.font, &state.emodoc.analyses[state.analysis - 1].text)
    //     .alpha_mode(BlendMode::OVER)
    //     .color(Color::WHITE)
    //     .size(scale_font(32.0, work_size))
    //     .max_width(textbox_width)
    //     .position(text_bounds.x + offset_x, text_bounds.y - offset_x);
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let work_size = get_work_size(gfx);
    let draw = &mut get_draw_setup(gfx, work_size, true, CLEAR_COLOR);

    state.visualizer.draw(draw);

    if state.analysis == 0 {
        draw_title(draw, state, work_size);
    } else {
        draw_paragraph(draw, state, work_size);
    }

    // draw to screen
    gfx.render(draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    // let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
    let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_HDPLUS.x as i32,
        // ScreenDimensions::RES_HDPLUS.y as i32,
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
