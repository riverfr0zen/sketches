use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
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

    // void main() {
    //     vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
    //     float y = st.x;
    //     vec3 xcolor = vec3(y);
    //     // Plot a line
    //     float pct = plot(st);
    //     xcolor = (1.0-pct)*xcolor+pct*vec3(0.0,1.0,0.0);
    //     color = vec4(xcolor,1.0);
    // }

"#
};


const PLOTFRAG: ShaderSource = notan::fragment_shader! {
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

    // void main() {
    //     vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
    //     float y = st.x;
    //     vec3 xcolor = vec3(y);
    //     // Plot a line
    //     float pct = plot(st);
    //     xcolor = (1.0-pct)*xcolor+pct*vec3(0.0,1.0,0.0);
    //     color = vec4(xcolor,1.0);
    // }

"#
};


pub struct ShaderRenderTexture {
    pub rt: RenderTexture,
}

impl ShaderRenderTexture {
    fn new(gfx: &mut Graphics, width: f32, height: f32) -> Self {
        let rt = gfx
            .create_render_texture(width as _, height as _)
            .build()
            .unwrap();
        Self { rt }
    }

    fn draw<F>(&mut self, gfx: &mut Graphics, pipeline: &Pipeline, ubos: Vec<&Buffer>, draw_fn: F)
    where
        F: Fn(&mut Draw),
    {
        let rt_draw = &mut self.rt.create_draw();

        for ubo in ubos.iter() {
            rt_draw
                .shape_pipeline()
                .pipeline(&pipeline)
                .uniform_buffer(&ubo);
        }
        draw_fn(rt_draw);
        rt_draw.shape_pipeline().remove();
        gfx.render_to(&self.rt, rt_draw);
    }
}


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    // pub pipeline2: Pipeline,
    pub clr_ubo: Buffer,
    pub clr_ubo2: Buffer,
    pub common_ubo: Buffer,
    pub rt: RenderTexture,
    pub rt2: RenderTexture,
    pub srt: ShaderRenderTexture,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&COLOR_FRAG)).unwrap();
    // let pipeline2 = create_shape_pipeline(gfx, Some(&FRAGMENT)).unwrap();

    let clr_ubo = gfx
        .create_uniform_buffer(1, "ColorVals")
        .with_data(&[RED_VAL, GREEN_VAL])
        .build()
        .unwrap();

    let clr_ubo2 = gfx
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

    let rt = gfx
        .create_render_texture(WORK_SIZE.x as _, WORK_SIZE.y as _)
        .build()
        .unwrap();

    let rt2 = gfx
        .create_render_texture(WORK_SIZE.x as _, WORK_SIZE.y as _)
        .build()
        .unwrap();

    let srt = ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y);

    State {
        pipeline,
        // pipeline2,
        clr_ubo,
        clr_ubo2,
        common_ubo,
        rt,
        rt2,
        srt,
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let rt_draw = &mut state.rt.create_draw();

    // rt with clr_ubo
    rt_draw
        .shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.clr_ubo)
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.common_ubo);

    rt_draw
        .rect((0.0, 0.0), (state.rt.width(), state.rt.height()))
        .fill_color(Color::GRAY)
        .fill();

    rt_draw.shape_pipeline().remove();

    gfx.render_to(&state.rt, rt_draw);


    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::BLUE);
    draw.image(&state.rt)
        .position(50.0, 100.0)
        .size(200.0, 200.0);

    draw.image(&state.rt)
        .position(300.0, 100.0)
        .size(300.0, 200.0);


    draw.image(&state.rt)
        .position(650.0, 100.0)
        .size(600.0, 200.0);


    // Switch to rt2 with clr_ubo2
    let rt_draw = &mut state.rt2.create_draw();
    rt_draw
        .shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.clr_ubo2)
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.common_ubo);

    rt_draw
        .rect((0.0, 0.0), (state.rt2.width(), state.rt2.height()))
        .fill_color(Color::GRAY)
        .fill();

    rt_draw.shape_pipeline().remove();

    gfx.render_to(&state.rt2, rt_draw);


    draw.image(&state.rt2)
        .position(50.0, 400.0)
        .size(200.0, 200.0);


    state
        .srt
        .draw(gfx, &state.pipeline, vec![&state.clr_ubo], |srtdraw| {
            srtdraw
                .rect((0.0, 0.0), (srtdraw.width(), srtdraw.height()))
                .fill_color(Color::GRAY)
                .fill();
        });


    draw.image(&state.srt.rt)
        .position(50.0, 700.0)
        .size(200.0, 200.0);


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
