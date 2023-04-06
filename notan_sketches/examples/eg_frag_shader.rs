use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::shaderutils::ShaderRenderTexture;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};

// const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
const RED_VAL: f32 = 1.0;
const GREEN_VAL: f32 = 0.5;


// language=glsl
const COLOR_FRAG: ShaderSource = notan::fragment_shader! {
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
        float u_resolution_x;
        float u_resolution_y;
    };

    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        // color = vec4(rVal, gVal, 0.0, 1.0);
        // color = vec4(rVal, gVal, abs(sin(u_time)), 1.0);
        // color = vec4(rVal, gVal, st.y, 1.0);
        color = vec4(rVal, gVal, st.y, abs(sin(u_time)));
    }
"#
};


const PLOT_FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;
    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    layout(set = 0, binding = 2) uniform Common {
        float u_time;
        float u_resolution_x;
        float u_resolution_y;
    };

    // Plot a line on Y using a value between 0.0-1.0
    float plot(vec2 st) {    
        return smoothstep(0.02, 0.0, abs(st.y - st.x));
    }

    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        float y = st.x;
        vec3 xcolor = vec3(y);
        // Plot a line
        float pct = plot(st);
        xcolor = (1.0-pct)*xcolor+pct*vec3(0.0,1.0,0.0);
        color = vec4(xcolor,1.0);
    }
"#
};


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub pipeline2: Pipeline,
    pub red_green_ubo: Buffer,
    pub blue_green_ubo: Buffer,
    pub common_ubo: Buffer,
    pub common_ubo2: Buffer,
    pub red_green_srt: ShaderRenderTexture,
    pub blue_green_srt: ShaderRenderTexture,
    pub plot_srt: ShaderRenderTexture,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let pipeline2 = create_shape_pipeline(gfx, Some(&PLOT_FRAG)).unwrap();
    let pipeline = create_shape_pipeline(gfx, Some(&COLOR_FRAG)).unwrap();

    let red_green_ubo = gfx
        .create_uniform_buffer(1, "ColorVals")
        .with_data(&[RED_VAL, GREEN_VAL])
        .build()
        .unwrap();

    let blue_green_ubo = gfx
        .create_uniform_buffer(1, "ColorVals")
        .with_data(&[RED_VAL - 0.5, GREEN_VAL + 0.5])
        .build()
        .unwrap();


    let (width, height) = gfx.device.size();

    let common_ubo = gfx
        .create_uniform_buffer(2, "Common")
        .with_data(&[0.0, width as f32, height as f32])
        .build()
        .unwrap();

    let common_ubo2 = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&[0.0, width as f32, height as f32])
        .build()
        .unwrap();

    let red_green_srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);
    let blue_green_srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);
    let plot_srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        pipeline2,
        red_green_ubo,
        blue_green_ubo,
        common_ubo,
        common_ubo2,
        red_green_srt,
        blue_green_srt,
        plot_srt,
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLUE);

    // srt with red_green_ubo & common_ubo
    // state.red_green_srt.draw(
    //     gfx,
    //     &state.pipeline,
    //     vec![&state.red_green_ubo, &state.common_ubo],
    //     |srtdraw| {
    //         srtdraw
    //             .rect((0.0, 0.0), (srtdraw.width(), srtdraw.height()))
    //             .fill_color(Color::GRAY)
    //             .fill();
    //     },
    // );

    state.red_green_srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![&state.red_green_ubo, &state.common_ubo],
    );


    draw.image(&state.red_green_srt.rt)
        .position(50.0, 50.0)
        .size(100.0, 100.0);

    draw.image(&state.red_green_srt.rt)
        .position(200.0, 50.0)
        .size(300.0, 100.0);

    draw.image(&state.red_green_srt.rt)
        .position(550.0, 50.0)
        .size(600.0, 100.0);


    // blue_green_srt with blue_green_ubo & common_ubo
    state.blue_green_srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![&state.blue_green_ubo, &state.common_ubo],
    );

    draw.image(&state.blue_green_srt.rt)
        .position(1200.0, 50.0)
        .size(100.0, 100.0);


    // srt with red_green_ubo & common_ubo again
    state.red_green_srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![&state.red_green_ubo, &state.common_ubo],
    );

    draw.image(&state.red_green_srt.rt)
        .position(1350.0, 50.0)
        .size(100.0, 100.0);


    // plot_srt with common_ubo2
    state
        .plot_srt
        .draw_filled(gfx, &state.pipeline2, vec![&state.common_ubo2]);

    draw.image(&state.plot_srt.rt)
        .position(1500.0, 50.0)
        .size(100.0, 100.0);


    gfx.render(draw);

    let u_time = app.timer.time_since_init();
    gfx.set_buffer_data(&state.common_ubo, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
    gfx.set_buffer_data(&state.common_ubo2, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
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
