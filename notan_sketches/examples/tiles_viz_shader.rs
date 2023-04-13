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
// const BG_COLOR: Color = Color::RED;
// const BG_COLOR: Color = Color::from_rgb(0.02, 1.0, 0.42);
// const BG_COLOR: Color = Color::WHITE;
const BG_COLOR: Color = colors::STONE;
// const TILE_COLOR1: Color = Color::from_rgb(0.043, 0.525, 0.756);
const TILE_COLORS: [Color; 6] = [
    colors::AEGEAN,
    colors::BANANA,
    colors::GRAYPURP,
    colors::SALMON,
    colors::OLIVE,
    colors::CARMINE,
];
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


    void main() {
        vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
        
        // float pct = 0.5-distance(st,vec2(0.5));
        // float pct = 1.0-distance(st,vec2(0.5));
        // float pct = 0.8-distance(st,vec2(0.5));
        float dist = distance(st,vec2(0.5));
        float pct = 0.6-(dist * 1.2);
        if (pct < 0.0) {
            pct = 0.0;
        }

        vec3 tile_color = vec3(tile_color_r, tile_color_g, tile_color_b);
        vec3 bg_color = vec3(bg_color_r, bg_color_g, bg_color_b);

        // vec3 xcolor = mix(bg_color, tile_color, pct);
        vec3 xcolor = mix(bg_color, tile_color, pct * abs(sin(u_time)));

        color = vec4(xcolor, 1.0);
    }
"#
};


struct Tile {
    srt: ShaderRenderTexture,
    common_ubo: Buffer,
    tile_colors_ubo: Buffer,
}

#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    // pub common_ubo: Buffer,
    pub tiles: Vec<Tile>,
    // pub common_ubos: Vec<Buffer>,
    // pub tile_color_ubos: Vec<Buffer>,
    // pub srts: Vec<ShaderRenderTexture>,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();

    let mut tiles: Vec<Tile> = vec![];
    for tile_color in TILE_COLORS.iter() {
        tiles.push(Tile {
            srt: ShaderRenderTexture::new(gfx, WORK_SIZE.x, WORK_SIZE.y),
            common_ubo: gfx
                .create_uniform_buffer(1, "Common")
                .with_data(&[0.0, WORK_SIZE.x, WORK_SIZE.y])
                .build()
                .unwrap(),
            tile_colors_ubo: gfx
                .create_uniform_buffer(2, "TileColors")
                .with_data(&[
                    tile_color.r,
                    tile_color.g,
                    tile_color.b,
                    BG_COLOR.r,
                    BG_COLOR.g,
                    BG_COLOR.b,
                ])
                .build()
                .unwrap(),
        });
    }
    State { pipeline, tiles }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, CLEAR_COLOR);

    for tile in state.tiles.iter_mut() {
        tile.srt.draw_filled(
            gfx,
            &state.pipeline,
            vec![&tile.common_ubo, &tile.tile_colors_ubo],
        );
    }

    let tile_width = WORK_SIZE.x / 3.0;
    let tile_height = WORK_SIZE.y / 2.0;
    draw.image(&state.tiles[0].srt.rt)
        .position(0.0, 0.0)
        .size(tile_width, tile_height);

    draw.image(&state.tiles[1].srt.rt)
        .position(tile_width, 0.0)
        .size(tile_width, tile_height);

    draw.image(&state.tiles[2].srt.rt)
        .position(tile_width * 2.0, 0.0)
        .size(tile_width, tile_height);

    draw.image(&state.tiles[3].srt.rt)
        .position(0.0, tile_height)
        .size(tile_width, tile_height);

    draw.image(&state.tiles[4].srt.rt)
        .position(tile_width, tile_height)
        .size(tile_width, tile_height);

    draw.image(&state.tiles[5].srt.rt)
        .position(tile_width * 2.0, tile_height)
        .size(tile_width, tile_height);


    gfx.render(draw);

    let u_time = app.timer.time_since_init();

    for tile in state.tiles.iter() {
        gfx.set_buffer_data(&tile.common_ubo, &[u_time, WORK_SIZE.x, WORK_SIZE.y]);
    }
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
