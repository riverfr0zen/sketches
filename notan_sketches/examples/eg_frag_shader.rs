use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};
use palette::named::GREEN;

const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const RED_VAL: f32 = 1.0;
const GREEN_VAL: f32 = 0.5;


// language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;
    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    layout(set = 0, binding = 1) uniform ColorVals {
        float rVal;
        float gVal;
    };

    layout(set = 0, binding = 2) uniform Common {
        float u_time;
        vec2 u_resolution;
    };

    void main() {
        vec2 st = gl_FragCoord.xy / u_resolution;
        // vec2 st = gl_FragCoord.xy / vec2(1920, 1080);
        // color = vec4(rVal, gVal, 0.0, 1.0);
        // color = vec4(rVal, gVal, abs(sin(u_time)), 1.0);
        color = vec4(st.x, st.y, 0.0, 1.0);
    }
"#
};

#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub clr_ubo: Buffer,
    pub common_ubo: Buffer,
    pub rect_size: Vec2,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAGMENT)).unwrap();

    let clr_ubo = gfx
        .create_uniform_buffer(1, "ColorVals")
        .with_data(&[RED_VAL, GREEN_VAL])
        .build()
        .unwrap();

    let (width, height) = gfx.device.size();
    let common_ubo = gfx
        .create_uniform_buffer(2, "Common")
        .with_data(&[0.0, width as f32, height as f32])
        .build()
        .unwrap();

    State {
        pipeline,
        clr_ubo,
        common_ubo,
        rect_size: Vec2::new(300.0, 200.0),
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLACK);


    // add custom pipeline for shapes
    draw.shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.clr_ubo)
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.common_ubo);


    draw.rect((100.0, 100.0), (state.rect_size.x, state.rect_size.y))
        .fill_color(Color::GRAY)
        .fill();

    draw.rect((100.0, 350.0), (300.0, 200.0))
        .fill_color(Color::GRAY)
        .fill();

    draw.shape_pipeline().remove();


    gfx.render(draw);

    let u_time = app.timer.time_since_init();
    let (width, height) = gfx.device.size();
    log::debug!("t {} w {} h {}", u_time, width, height);
    gfx.set_buffer_data(&state.common_ubo, &[u_time, width as f32, height as f32]);
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

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        // .event(event)
        // .update(update)
        .draw(draw)
        .build()
}
