use notan::draw::*;
use notan::log;
use notan::math::{Vec2, Vec3};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::emotion_bg_visualizer::visualizers::tiled_shaders::shader::{
    TileShaderBundle as Tile, FRAG,
};
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


#[derive(AppState)]
struct State {
    pub pipeline: Pipeline,
    pub tiles: Vec<Tile>,
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();

    let mut tiles: Vec<Tile> = vec![];
    for tile_color in TILE_COLORS.iter() {
        tiles.push(Tile::new(gfx, &WORK_SIZE, tile_color, &BG_COLOR));
    }
    State { pipeline, tiles }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, CLEAR_COLOR);

    for tile in state.tiles.iter_mut() {
        tile.draw_filled(gfx, &state.pipeline);
        // tile.srt.draw_filled(
        //     gfx,
        //     &state.pipeline,
        //     vec![&tile.common_ubo, &tile.tile_colors_ubo],
        // );
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
