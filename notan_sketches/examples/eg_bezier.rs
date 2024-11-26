use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::mathutils::mid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::WHITE;

#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub from: Vec2,
    pub to: Vec2,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    // let from = vec2(300.0, 300.0);
    // let to = vec2(600.0, 100.0);
    // let ctrl = vec2(400.0, 200.0);
    // let ctrl = mid(from, to);

    let from = vec2(
        rng.gen_range(0.0..work_size.x),
        rng.gen_range(0.0..work_size.y),
    );
    let to = vec2(
        rng.gen_range(0.0..work_size.x),
        rng.gen_range(0.0..work_size.y),
    );

    State {
        rng,
        work_size,
        from,
        to,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    let ctrl = vec2(
        state
            .rng
            .gen_range(state.from.x.min(state.to.x)..state.from.x.max(state.to.x)),
        state
            .rng
            .gen_range(state.from.y.min(state.to.y)..state.from.y.max(state.to.y)),
    );
    draw.path()
        .move_to(state.from.x, state.from.y)
        .quadratic_bezier_to((ctrl.x, ctrl.y), (state.to.x, state.to.y))
        .color(Color::ORANGE)
        .stroke(10.0);

    gfx.render(draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(
            // let win_config = get_common_win_config().high_dpi(true).size(
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

    set_html_bgcolor(CLEAR_COLOR);

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .touch_as_mouse(false)
        // .event(event)
        // .update(update)
        .draw(draw)
        .build()
}
