use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::shaderutils::ShaderRenderTexture;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, set_html_bgcolor, ScreenDimensions,
};

// const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
// const BGCOLOR: Color = Color::from_rgb(1.0, 0.65, 0.2);

// language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
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

    float plot(vec2 st, float pct, float feather) {
        return smoothstep(pct + feather, pct, st.y);
    }


    float plot2(vec2 st, float pct, float top_feather, float bottom_feather) {
        return smoothstep(pct - bottom_feather, pct, st.y) - smoothstep(pct, pct + top_feather, st.y);
    }
    

    // Divide by 2.0 to scale down y coordinates since display coord system does not have "negative coordinates".
    // After scaling down, compensate for half the wave being in negative y coords by adding 0.5
    // and thus pushing the full sine wave upwards
    float adjusted_sin(float x, float y_shrink, float wave_height) {
        // return sin(x) / 2.0 + wave_height;
        // return sin(x) / 20.0 + wave_height;
        return sin(x) / y_shrink + wave_height;
    }
    

    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        vec3 backgroundColor = vec3(1.0, 0.65, 0.2) * ((1-st.y) * 4.0);
        vec3 waveColor = vec3(0.043, 0.525, 0.756) * (st.y * 0.8);

        float wave_height = 0.5;
        float max_y_shrink = 30.0;
        float min_y_shrink = 10.0;
        float wave_y_timeframe = mod(u_time, max_y_shrink);
        float wave_y_timeframe2x = mod(u_time, (wave_y_timeframe * 2.0));
        float wave_y_shrink = wave_y_timeframe;

        // Using the 2x timeframe to step `wave_y_shrink` "backwards" if we've gone past the
        // single-direction timeframe. This is a technique that can be used to get a 
        // "pendulum" or "back-and-forth" effect from time and modulus.
        if (wave_y_timeframe2x > max_y_shrink) {
            wave_y_shrink = max_y_shrink - wave_y_timeframe;
        }
        if (wave_y_shrink < min_y_shrink) {
            wave_y_shrink = min_y_shrink;
        }

        float y = adjusted_sin(st.x * abs(sin(mod(u_time, 60.0))) * 5.5 + u_time, wave_y_shrink, wave_height);

        // float pct = plot2(st, y, 0.02, 0.02);
        float pct = plot2(st, y, 0.05, 50.0);
        // float pct = plot(st, y, 0.05);

        waveColor = mix(backgroundColor, waveColor, pct);
        color = vec4(waveColor, 1.0);
    }
"#
};


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub common_ubo: Buffer,
    pub srt: ShaderRenderTexture,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();


    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&[0.0, WORK_SIZE.x, WORK_SIZE.y])
        .build()
        .unwrap();

    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        common_ubo,
        srt,
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLACK);

    state
        .srt
        .draw_filled(gfx, &state.pipeline, vec![&state.common_ubo]);


    draw.image(&state.srt.rt)
        .position(0.0, 0.0)
        .size(WORK_SIZE.x, WORK_SIZE.y);


    gfx.render(draw);

    let u_time = app.timer.time_since_init();
    gfx.set_buffer_data(&state.common_ubo, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
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

    set_html_bgcolor(Color::BLACK);

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
