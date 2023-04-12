use notan::draw::*;
use notan::log;
use notan::math::{Vec2, Vec3, Vec4};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::shaderutils::ShaderRenderTexture;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, set_html_bgcolor, ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::BLUE;
const BG_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);
const TILE_COLOR1: Color = Color::from_rgb(0.043, 0.525, 0.756);
// const TILE_COLORS: [Color; 6] = [
//     colors::AEGEAN,
//     colors::BANANA,
//     colors::SAFFRON,
//     colors::SALMON,
//     colors::OLIVE,
//     colors::CARMINE,
// ];
// const WORK_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;


// Based on https://thebookofshaders.com/05/
// language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;
    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    // layout(set = 0, binding = 0) uniform TileColors {
    //     vec3 tile_color;
    //     vec3 bg_color;
    // };

    layout(set = 0, binding = 0) uniform Common {
        float u_time;
        float u_resolution_x;
        float u_resolution_y;
    };

    layout(set = 0, binding = 0) uniform TileColors {
        float tile_color_r;
        float tile_color_g;
        float tile_color_b;
        float bg_color_r;
        float bg_color_g;
        float bg_color_b;
    };


    // Plot a line on Y using a value between 0.0-1.0
    float plot(vec2 st) {    
        return smoothstep(0.02, 0.0, abs(st.y - st.x));
    }

    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        
        // float pct = distance(st,vec2(0.5));
        // float pct = 0.5-distance(st,vec2(0.5));
        float pct = 1.0-distance(st,vec2(0.5));

        vec3 tile_color = vec3(tile_color_r, tile_color_g, tile_color_b);
        vec3 bg_color = vec3(bg_color_r, bg_color_g, bg_color_b);

        // vec3 xcolor = mix(bg_color, tile_color, pct);
        vec3 xcolor = mix(bg_color, tile_color, pct * abs(sin(u_time)));


        // Plot a line
        color = vec4(xcolor, 1.0);
    }
"#
};


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub common_ubo: Buffer,
    pub tile_colors_ubo: Buffer,
    pub srt: ShaderRenderTexture,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();


    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&[0.0, WORK_SIZE.x, WORK_SIZE.y])
        .build()
        .unwrap();

    let tile_colors_ubo = gfx
        .create_uniform_buffer(2, "TileColors")
        // .with_data(&[TILE_COLOR1, BG_COLOR])
        .with_data(&[
            TILE_COLOR1.r,
            TILE_COLOR1.g,
            TILE_COLOR1.b,
            BG_COLOR.r,
            BG_COLOR.g,
            BG_COLOR.b,
        ])
        .build()
        .unwrap();


    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        common_ubo,
        tile_colors_ubo,
        srt,
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, CLEAR_COLOR);

    state.srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![&state.common_ubo, &state.tile_colors_ubo],
    );


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

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);

    set_html_bgcolor(CLEAR_COLOR);

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
