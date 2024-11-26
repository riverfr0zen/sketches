use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::fractals::sierpinski::{draw_bushy_gasket, event, init, update, State};
use notan_sketches::utils::{get_common_win_config, get_draw_setup, set_html_bgcolor};

// const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);

fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, true, Color::BLACK);

    // variation (/ 2.0 -> / 3.0)
    let a = vec2(WORK_SIZE.x / 3.0, 0.0);
    let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
    // variation (+ 20.0)
    let c = vec2(0.0, WORK_SIZE.y + 20.0);
    draw_bushy_gasket(&mut draw, state, a, b, c, 0);

    state.help_modal.draw(&mut draw, WORK_SIZE);

    // draw to screen
    gfx.render(&draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}

#[notan_main]
fn main() -> Result<(), String> {
    let mut win_config = get_common_win_config().set_high_dpi(true);
    win_config = win_config.set_title("sierpinski gasket (bushy)");
    set_html_bgcolor(Color::BLACK);
    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .touch_as_mouse(false)
        .event(event)
        .draw(draw)
        .update(update)
        .build()
}
