use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::shaderutils::{
    create_hot_shape_pipeline, CommonData, ShaderReloadManager, ShaderRenderTexture,
};
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};

const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;

#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub common_ubo: Buffer,
    pub srt: ShaderRenderTexture,
    #[cfg(debug_assertions)]
    pub hot_mgr: ShaderReloadManager,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline =
        // create_hot_shape_pipeline(gfx, "examples/assets/shaders/horizontal_city.frag.glsl")
        //     .unwrap();
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/erratic_wave.frag.glsl")
            .unwrap();

    let common_data = CommonData::new(0.0, WORK_SIZE);
    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&common_data)
        .build()
        .unwrap();

    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        common_ubo,
        srt,
        #[cfg(debug_assertions)]
        hot_mgr: ShaderReloadManager::default(),
    }
}

fn update(state: &mut State) {
    #[cfg(debug_assertions)]
    state.hot_mgr.update();
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    let u_time = app.timer.elapsed_f32();
    let common_data = CommonData::new(u_time, WORK_SIZE);

    #[cfg(debug_assertions)]
    if state.hot_mgr.needs_reload() {
        match create_hot_shape_pipeline(gfx, "examples/assets/shaders/horizontal_city.frag.glsl") {
            Ok(pipeline) => state.pipeline = pipeline,
            Err(err) => log::error!("{}", err),
        }

        state.common_ubo = gfx
            .create_uniform_buffer(1, "Common")
            .with_data(&common_data)
            .build()
            .unwrap();
    }

    state
        .srt
        .draw_filled(gfx, &state.pipeline, vec![&state.common_ubo]);

    // Draw shader in a rectangle at the center
    let rect_size = 800.0;
    let x = (WORK_SIZE.x - rect_size) / 2.0;
    let y = (WORK_SIZE.y - rect_size) / 2.0;

    draw.image(&state.srt.rt)
        .position(x, y)
        .size(rect_size, rect_size);

    gfx.render(draw);
    gfx.set_buffer_data(&state.common_ubo, &common_data);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().set_high_dpi(true).set_size(
        ScreenDimensions::RES_1080P.x as u32,
        ScreenDimensions::RES_1080P.y as u32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().set_high_dpi(true);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}
