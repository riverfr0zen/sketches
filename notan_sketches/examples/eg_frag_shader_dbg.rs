use notan::draw::*;
use notan::log;
use notan::math::{vec2, vec3, Mat4, Rect, Vec2};
use notan::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys;

const WORK_SIZE: Vec2 = Vec2::new(1920.0, 1080.0);
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

    // Plot a line on Y using a value between 0.0-1.0
    float plot(vec2 st) {
        return smoothstep(0.02, 0.0, abs(st.y - st.x));
    }

    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        // color = vec4(rVal, gVal, 0.0, 1.0);
        color = vec4(rVal, gVal, abs(sin(u_time)), 1.0);
        // color = vec4(rVal, gVal, st.y, 1.0);
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
    pub clr_ubo: Buffer,
    pub common_ubo: Buffer,
    pub common_ubo2: Buffer,
    pub rt: RenderTexture,
    pub rt2: RenderTexture,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let pipeline2 = create_shape_pipeline(gfx, Some(&PLOT_FRAG)).unwrap();
    let pipeline = create_shape_pipeline(gfx, Some(&COLOR_FRAG)).unwrap();

    let (width, height) = gfx.device.size();

    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&[0.0, width as f32, height as f32])
        .build()
        .unwrap();

    let clr_ubo = gfx
        .create_uniform_buffer(2, "ColorVals")
        .with_data(&[RED_VAL, GREEN_VAL])
        .build()
        .unwrap();

    let common_ubo2 = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&[0.0, width as f32, height as f32])
        .build()
        .unwrap();

    let rt = gfx
        .create_render_texture(width as _, height as _)
        .build()
        .unwrap();

    let rt2 = gfx
        .create_render_texture(width as _, height as _)
        .build()
        .unwrap();

    State {
        pipeline,
        pipeline2,
        clr_ubo,
        common_ubo,
        common_ubo2,
        rt,
        rt2,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLUE);

    // Draw rt with first shader
    let rt_draw = &mut state.rt.create_draw();

    rt_draw
        .shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.clr_ubo)
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.common_ubo);

    rt_draw
        .rect((0.0, 0.0), (rt_draw.width(), rt_draw.height()))
        .fill_color(Color::GRAY)
        .fill();

    rt_draw.shape_pipeline().remove();
    gfx.render_to(&state.rt, rt_draw);

    draw.image(&state.rt)
        .position(50.0, 50.0)
        .size(200.0, 200.0);

    // draw second rt with second pipeline to second shader
    let rt2_draw = &mut state.rt2.create_draw();

    rt2_draw
        .shape_pipeline()
        .pipeline(&state.pipeline2)
        // THIS DOESN'T WORK
        .uniform_buffer(&state.common_ubo);
    // Only works if I use a second Buffer
    // .uniform_buffer(&state.common_ubo2);

    rt2_draw
        .rect((0.0, 0.0), (rt2_draw.width(), rt2_draw.height()))
        .fill_color(Color::GRAY)
        .fill();

    rt2_draw.shape_pipeline().remove();
    gfx.render_to(&state.rt2, rt2_draw);

    draw.image(&state.rt2)
        .position(50.0, 300.0)
        .size(200.0, 200.0);

    gfx.render(draw);

    let u_time = app.timer.elapsed_f32();
    gfx.set_buffer_data(&state.common_ubo, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
    gfx.set_buffer_data(&state.common_ubo2, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(WORK_SIZE.x as u32, WORK_SIZE.y as u32);

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().set_high_dpi(true);

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

pub fn get_common_win_config() -> WindowConfig {
    #[cfg(not(target_arch = "wasm32"))]
    return WindowConfig::default().set_resizable(true);

    #[cfg(target_arch = "wasm32")]
    return WindowConfig::default()
        .set_resizable(true)
        .set_maximized(true);
}

/// Set up a Draw with some common basics
pub fn get_draw_setup(
    gfx: &mut Graphics,
    work_size: Vec2,
    aspect_fit: bool,
    clear_color: Color,
) -> Draw {
    let (width, height) = gfx.size();
    let win_size = vec2(width as f32, height as f32);

    let mut draw = gfx.create_draw();
    draw.clear(clear_color);

    if aspect_fit {
        let (projection, _) = get_aspect_fit_projection(win_size, work_size);
        draw.set_projection(Some(projection));
    } else {
        let projection = get_scaling_projection(win_size, work_size);
        draw.set_projection(Some(projection));
    }
    return draw;
}

/// This returns a projection that keeps the aspect ratio while scaling
/// and fitting the content in our window
/// It also returns the ratio in case we need it to calculate positions
/// or manually scale something
///
/// Taken from the following example:
/// https://github.com/Nazariglez/notan/blob/develop/examples/draw_projection.rs
pub fn get_aspect_fit_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, f32) {
    let ratio = (win_size.x / work_size.x).min(win_size.y / work_size.y);

    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let scale = Mat4::from_scale(vec3(ratio, ratio, 1.0));
    let position = vec3(
        (win_size.x - work_size.x * ratio) * 0.5,
        (win_size.y - work_size.y * ratio) * 0.5,
        1.0,
    );
    let translation = Mat4::from_translation(position);
    (projection * translation * scale, ratio)
}

/// Returns a projection for scaling content to the window size WITHOUT maintaining aspect ratio
/// (i.e. content will be stretched to fit window)
///
/// Based on the following example:
/// https://github.com/Nazariglez/notan/blob/develop/examples/draw_projection.rs
pub fn get_scaling_projection(win_size: Vec2, work_size: Vec2) -> Mat4 {
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let scale = Mat4::from_scale(vec3(
        win_size.x / work_size.x,
        win_size.y / work_size.y,
        1.0,
    ));
    projection * scale
}
