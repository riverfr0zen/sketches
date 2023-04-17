use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::colors;
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
    pub bg_color_ubo: Buffer,
    pub color1_ubo: Buffer,
    pub color2_ubo: Buffer,
    pub srt: ShaderRenderTexture,
    pub hot_mgr: ShaderReloadManager,
}

fn prep_ubos(
    gfx: &mut Graphics,
    common_data: CommonData,
    bg_color: Color,
    color1: Color,
    color2: Color,
) -> (Buffer, Buffer, Buffer, Buffer) {
    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&common_data)
        .build()
        .unwrap();

    let bg_color_ubo = gfx
        .create_uniform_buffer(2, "BgColor")
        .with_data(&[bg_color.r, bg_color.g, bg_color.b])
        .build()
        .unwrap();

    let color1_ubo = gfx
        .create_uniform_buffer(3, "ColorSource1")
        .with_data(&[color1.r, color1.g, color1.b])
        .build()
        .unwrap();


    let color2_ubo = gfx
        .create_uniform_buffer(4, "ColorSource2")
        .with_data(&[color2.r, color2.g, color2.b])
        .build()
        .unwrap();

    (common_ubo, bg_color_ubo, color1_ubo, color2_ubo)
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline =
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/color_points.frag.glsl").unwrap();

    let common_data = CommonData::new(0.0, WORK_SIZE);
    let (common_ubo, bg_color_ubo, color1_ubo, color2_ubo) = prep_ubos(
        gfx,
        common_data,
        colors::AEGEAN,
        colors::BANANA,
        colors::SALMON,
    );

    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        common_ubo,
        bg_color_ubo,
        color1_ubo,
        color2_ubo,
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
        match create_hot_shape_pipeline(gfx, "examples/assets/shaders/color_points.frag.glsl") {
            Ok(pipeline) => state.pipeline = pipeline,
            Err(err) => log::error!("{}", err),
        }

        (
            state.common_ubo,
            state.bg_color_ubo,
            state.color1_ubo,
            state.color2_ubo,
        ) = prep_ubos(
            gfx,
            common_data,
            colors::AEGEAN,
            colors::BANANA,
            colors::SALMON,
        );
    }

    state.srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![
            &state.common_ubo,
            &state.bg_color_ubo,
            &state.color1_ubo,
            &state.color2_ubo,
        ],
    );


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
