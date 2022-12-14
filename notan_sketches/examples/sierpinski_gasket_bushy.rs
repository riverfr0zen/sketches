use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::fractals::sierpinski::{draw_bushy_gasket, State};
use notan_sketches::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);


fn update(app: &mut App, state: &mut State) {
    // if app.keyboard.is_down(KeyCode::W) {
    //     state.y -= MOVE_SPEED * app.timer.delta_f32();
    // }
    if app.keyboard.was_pressed(KeyCode::Up) {
        state.max_depth += 1;
        log::debug!("state.max_depth increased: {}", state.max_depth);
    }

    if app.keyboard.was_pressed(KeyCode::Down) && state.max_depth > 0 {
        state.max_depth -= 1;
        log::debug!("state.max_depth decreased: {}", state.max_depth);
    }

    if app.keyboard.was_pressed(KeyCode::R) {
        state.max_depth = State::default().max_depth;
        log::debug!("state.max_depth reset: {}", state.max_depth);
    }
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::BLACK);

    // variation (/ 2.0 -> / 3.0)
    let a = vec2(WORK_SIZE.x / 3.0, 0.0);
    let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
    // variation (+ 20.0)
    let c = vec2(0.0, WORK_SIZE.y + 20.0);
    draw_bushy_gasket(&mut draw, state, a, b, c, 0);

    // draw to screen
    gfx.render(&draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let mut win_config = get_common_win_config();
    win_config = win_config.title("sierpinski gasket (bushy)");

    // notan::init()
    // notan::init_with(init)
    notan::init_with(State::default)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}
