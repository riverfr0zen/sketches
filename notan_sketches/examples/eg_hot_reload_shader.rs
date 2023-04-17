use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::shaderutils::{
    create_hot_shape_pipeline, CommonData, ShaderReloadManager, ShaderRenderTexture,
};
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, set_html_bgcolor, ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::BLUE;
// const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub common_ubo: Buffer,
    pub srt: ShaderRenderTexture,
    pub hot_mgr: ShaderReloadManager,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline =
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/plot.frag.glsl").unwrap();

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
        hot_mgr: ShaderReloadManager::default(),
    }
}


fn update(app: &mut App, state: &mut State) {
    state.hot_mgr.update();
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, CLEAR_COLOR);
    let u_time = app.timer.time_since_init();
    let common_data = CommonData::new(u_time, WORK_SIZE);

    if state.hot_mgr.needs_reload() {
        match create_hot_shape_pipeline(gfx, "examples/assets/shaders/plot.frag.glsl") {
            Ok(pipeline) => state.pipeline = pipeline,
            Err(err) => log::error!("{}", err),
        }

        // UBOs here need to be created with the *latest data*, not the initial data we used in init()
        state.common_ubo = gfx
            .create_uniform_buffer(1, "Common")
            .with_data(&common_data)
            .build()
            .unwrap();
    }

    state
        .srt
        .draw_filled(gfx, &state.pipeline, vec![&state.common_ubo]);


    draw.image(&state.srt.rt)
        .position(0.0, 0.0)
        .size(WORK_SIZE.x, WORK_SIZE.y);


    gfx.render(draw);

    gfx.set_buffer_data(&state.common_ubo, &common_data);
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

    set_html_bgcolor(CLEAR_COLOR);

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        // .event(event)
        .update(update)
        .draw(draw)
        .build()
}
