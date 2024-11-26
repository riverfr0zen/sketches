use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    ScreenDimensions,
};

const CLEAR_COLOR: Color = Color::from_rgb(0.14, 0.13, 0.03);
const BUILDING_MIN_WIDTH: f32 = 0.1;
const BUILDING_MAX_WIDTH: f32 = 0.3;
const BUILDING_MAX_HEIGHT_RATIO: f32 = 2.0;
const BUILDING_MIN_HEIGHT_RATIO: f32 = 16.0;

pub struct Destination {
    x: f32,
    y: f32,
    speed: f32,
}

pub struct Shifty {
    dest: Destination,
}

pub struct Building;

struct BuildingSpec {
    pub min_width: f32,
    pub max_width: f32,
}

impl BuildingSpec {
    fn from_work_size(work_size: &Vec2) -> Self {
        Self {
            min_width: work_size.x * BUILDING_MIN_WIDTH,
            max_width: work_size.x * BUILDING_MAX_WIDTH,
        }
    }
}

#[derive(AppState)]
struct State {
    pub rng: Random,
    pub work_size: Vec2,
    pub building_spec: BuildingSpec,
    pub pipeline: Pipeline,
    pub ubo: Buffer,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    let work_size = get_work_size_for_screen(app, gfx);

    let pipeline = create_shape_pipeline(gfx, Some(&FRAGMENT)).unwrap();

    let ubo = gfx
        .create_uniform_buffer(1, "Time")
        .with_data(&[0.0])
        .build()
        .unwrap();

    State {
        rng,
        work_size: work_size,
        building_spec: BuildingSpec::from_work_size(&work_size),
        pipeline,
        ubo,
    }
}

// language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;
    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;
    layout(set = 0, binding = 1) uniform Time {
        float u_time;
    };
    void main() {
        color = vec4(abs(sin(u_time)),0.0,0.0,1.0);
    }
"#
};

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, state.work_size, false, CLEAR_COLOR);

    draw.ellipse((100.0, 100.0), (100.0, 80.0))
        .fill_color(Color::ORANGE)
        .fill();

    // add custom pipeline for shapes
    draw.shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.ubo);

    draw.rect((200.0, 200.0), (300.0, 300.0))
        .fill_color(Color::GRAY)
        .fill();
    // remove custom pipeline
    draw.shape_pipeline().remove();
    gfx.render(draw);

    let u_time = app.timer.elapsed_f32();
    log::debug!("{}", u_time);
    gfx.set_buffer_data(&state.ubo, &[u_time]);
}

// fn draw_skyline_layer(draw: &mut Draw, state: &mut State) {
//     let remaining_space = draw.width();

//     while remaining_space > 0.0 {
//         // debug!("{:?}, {:?}", available_space, remaining_space);

//         let building_width = if remaining_space > BUILDING_MAX_WIDTH {
//             state
//                 .rng
//                 .gen_range(state.building_spec.min_width..state.building_spec.max_width)
//         } else {
//             // It should correctly be `remaining_space`, but iirc there used to be
//             // some kind of error before. Re-instate BUILDING_MAX_WIDTH if error recurrs.
//             // BUILDING_MAX_WIDTH
//             remaining_space
//         };
//         let building_height = 200.0;
//         // let building_height = rng.gen_range(building_min_height..building_max_height);
//         log::debug!("{} {}", building_width, building_height);
//     }
// }

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(
            // let win_config = get_common_win_config().high_dpi(true).size(
            // ScreenDimensions::RES_4KISH.x as i32,
            // ScreenDimensions::RES_4KISH.y as i32,
            // ScreenDimensions::RES_HDPLUS.x as i32,
            // ScreenDimensions::RES_HDPLUS.y as i32,
            ScreenDimensions::RES_1080P.x as u32,
            ScreenDimensions::RES_1080P.y as u32,
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
        .touch_as_mouse(false)
        // .event(event)
        // .update(update)
        .draw(draw)
        .build()
}
