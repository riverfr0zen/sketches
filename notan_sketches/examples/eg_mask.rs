use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, CapturingTexture, ScreenDimensions,
};


const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;


fn draw(app: &mut App, gfx: &mut Graphics) {
    let mask = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLACK);
    mask.rect((120.0, 120.0), (100.0, 100.0))
        .fill_color(Color::WHITE)
        .fill();

    mask.rect((120.0, 230.0), (100.0, 100.0))
        .fill_color(Color::new(0.5, 0.5, 0.5, 0.5))
        .fill();


    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLACK);
    draw.mask(Some(&mask));
    draw.rect((100.0, 100.0), (400.0, 400.0))
        .fill_color(Color::GREEN)
        .fill();

    gfx.render(draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
        // let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_4KISH.x as i32,
        // ScreenDimensions::RES_4KISH.y as i32,
        // ScreenDimensions::RES_HDPLUS.x as i32,
        // ScreenDimensions::RES_HDPLUS.y as i32,
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);


    notan::init()
        // notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
