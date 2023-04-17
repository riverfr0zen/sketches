use notan::draw::*;
use notan::log;
use notan::math::{Vec2, Vec3};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::shaderutils::{
    create_hot_shape_pipeline, CommonData, ShaderReloadManager, ShaderRenderTexture,
};
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, set_html_bgcolor, ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::BLUE;
const COLOR1: Color = colors::BANANA;
const COLOR2: Color = colors::SALMON;

// const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;

#[uniform]
#[derive(Copy, Clone)]
struct ColorSourceUniform {
    color: Vec3,
    pos: Vec2,
}

struct ColorSource {
    uniform: ColorSourceUniform,
    ubo: Buffer,
    created: f32,
}

impl ColorSource {
    fn reset(&mut self, created: f32) {
        self.created = created;
    }

    fn update(&mut self, time_since_init: f32) {
        if time_since_init - self.created > 5.0 {
            log::debug!("color source update step");
            self.uniform.pos.x += 0.01;
            self.uniform.pos.y += 0.01;
            self.reset(time_since_init);
        }
    }
}


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub common_ubo: Buffer,
    pub bg_color_ubo: Buffer,
    pub color1: ColorSource,
    pub color2: ColorSource,
    pub srt: ShaderRenderTexture,
    pub hot_mgr: ShaderReloadManager,
}

fn prep_ubos(
    gfx: &mut Graphics,
    common_data: CommonData,
    bg_color: Color,
    color1_uniform: ColorSourceUniform,
    color2_uniform: ColorSourceUniform,
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
        .with_data(&color1_uniform)
        .build()
        .unwrap();

    let color2_ubo = gfx
        .create_uniform_buffer(4, "ColorSource2")
        .with_data(&color2_uniform)
        .build()
        .unwrap();

    (common_ubo, bg_color_ubo, color1_ubo, color2_ubo)
}


fn init(gfx: &mut Graphics) -> State {
    let pipeline =
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/color_points.frag.glsl").unwrap();
    let common_data = CommonData::new(0.0, WORK_SIZE);
    let color1_uniform = ColorSourceUniform {
        color: Vec3::new(COLOR1.r, COLOR1.g, COLOR1.b),
        pos: Vec2::new(0.2, 0.8),
    };
    let color2_uniform = ColorSourceUniform {
        color: Vec3::new(COLOR2.r, COLOR2.g, COLOR2.b),
        pos: Vec2::new(0.5, 0.5),
    };

    let (common_ubo, bg_color_ubo, color1_ubo, color2_ubo) = prep_ubos(
        gfx,
        common_data,
        colors::AEGEAN,
        color1_uniform,
        color2_uniform,
    );

    let color1 = ColorSource {
        uniform: color1_uniform,
        ubo: color1_ubo,
        created: 0.0,
    };

    let color2 = ColorSource {
        uniform: color2_uniform,
        ubo: color2_ubo,
        created: 0.0,
    };

    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        common_ubo,
        bg_color_ubo,
        color1,
        color2,
        srt,
        hot_mgr: ShaderReloadManager::default(),
    }
}


fn update(app: &mut App, state: &mut State) {
    state.hot_mgr.update();
    state.color1.update(app.timer.time_since_init());
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
            state.color1.ubo,
            state.color2.ubo,
        ) = prep_ubos(
            gfx,
            common_data,
            colors::AEGEAN,
            state.color1.uniform,
            state.color2.uniform,
        );
    }

    state.srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![
            &state.common_ubo,
            &state.bg_color_ubo,
            &state.color1.ubo,
            &state.color2.ubo,
        ],
    );


    draw.image(&state.srt.rt)
        .position(0.0, 0.0)
        .size(WORK_SIZE.x, WORK_SIZE.y);


    gfx.render(draw);

    gfx.set_buffer_data(&state.common_ubo, &common_data);
    gfx.set_buffer_data(&state.color1.ubo, &state.color1.uniform);
    gfx.set_buffer_data(&state.color2.ubo, &state.color2.uniform);
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
