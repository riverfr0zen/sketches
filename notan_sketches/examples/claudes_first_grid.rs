// Refactored version of claudes_first.rs using the grid utilities module
// This demonstrates the dramatic reduction in boilerplate code!

use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::gridutils::Grid;
use notan_sketches::shaderutils::{
    create_hot_shape_pipeline, CommonData, ShaderReloadManager, ShaderRenderTexture,
};
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, ScreenDimensions,
};
use palette::{Darken, FromColor, Hsv, Lighten, Srgb};
use std::f32::consts::PI;

const ROWS: u32 = 10;
const COLS: u32 = 10;
const MAX_CHILD_CIRCLES: u32 = 3;
const DARKEN_MAX: f32 = 0.2;
const LIGHTEN_MAX: f32 = 0.3;
const CHILD_RADIUS_MOD_MAX: f32 = 0.5;
const CHILD_RADIUS_MOD_MIN: f32 = 0.125;
const GRID_STROKE: f32 = 5.0;

#[cfg(not(debug_assertions))]
const FRAG: ShaderSource =
    notan::include_fragment_shader!("examples/assets/shaders/tile_blend.frag.glsl");

#[derive(Clone)]
struct ChildCircle {
    angle: f32,
    radius: f32,
    color: Color,
}

// Cell data - now using NORMALIZED coordinates (0.0-1.0)!
#[derive(Clone)]
struct CellData {
    position: Vec2, // Normalized 0-1 within cell - resolution independent!
    color: Color,
    bg_color: Color,
    children: Vec<ChildCircle>,
}

#[uniform]
#[derive(Copy, Clone)]
struct TileGridInfo {
    pub grid_size: Vec2,
}

fn vary_color(color: Color, rng: &mut Random) -> Color {
    let srgb = Srgb::new(color.r, color.g, color.b);
    let mut hsv = Hsv::from_color(srgb);

    if rng.random_bool(0.5) {
        hsv = hsv.darken(rng.random_range(0.0..DARKEN_MAX));
    } else {
        hsv = hsv.lighten(rng.random_range(0.0..LIGHTEN_MAX));
    }

    let result = Srgb::from_color(hsv);
    Color::new(result.red, result.green, result.blue, color.a)
}

#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>, // ✨ All cell data in one place!
    circle_radius: f32,
    palette: PalettesSelection,
    bg_palette: PalettesSelection,
    show_grid: bool,
    // Shader-related fields
    pipeline: Pipeline,
    common_ubo: Buffer,
    tile_colors_ubo: Buffer,
    tile_grid_info_ubo: Buffer,
    srt: ShaderRenderTexture,
    tile_colors_dirty: bool,
    #[cfg(debug_assertions)]
    hot_mgr: ShaderReloadManager,
}

// Helper to generate cell data
fn generate_cell_data(
    _row: u32,
    _col: u32,
    bounds: notan::math::Rect,
    rng: &mut Random,
    circle_radius: f32,
    palette: &PalettesSelection,
    bg_palette: &PalettesSelection,
) -> CellData {
    // Calculate normalized position with margin to keep circle inside cell
    let cell_width = bounds.width;
    let cell_height = bounds.height;
    let margin_x = circle_radius / cell_width;
    let margin_y = circle_radius / cell_height;

    let position = vec2(
        rng.random_range(margin_x..(1.0 - margin_x)),
        rng.random_range(margin_y..(1.0 - margin_y)),
    );

    let color = colors::Palettes::choose_color(palette);
    let bg_color = colors::Palettes::choose_color(bg_palette);

    // Generate child circles
    let num_children = rng.random_range(0..=MAX_CHILD_CIRCLES);
    let mut children = Vec::new();
    for _ in 0..num_children {
        let angle = rng.random_range(0.0..(2.0 * PI));
        let child_radius = rng.random_range(
            (circle_radius * CHILD_RADIUS_MOD_MIN)..(circle_radius * CHILD_RADIUS_MOD_MAX),
        );
        let child_color = vary_color(color, rng);
        children.push(ChildCircle {
            angle,
            radius: child_radius,
            color: child_color,
        });
    }

    CellData {
        position,
        color,
        bg_color,
        children,
    }
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);

    // Calculate circle radius
    let tile_width = work_size.x / COLS as f32;
    let tile_height = work_size.y / ROWS as f32;
    let circle_radius = (tile_width.min(tile_height) / 2.0) * 0.8;
    log::info!("Circle radius: {}", circle_radius);

    // Choose palettes
    let palette: PalettesSelection = rng.random();
    log::info!("Circle Palette: {:?}", palette);

    let mut bg_palette: PalettesSelection = rng.random();
    while format!("{:?}", bg_palette) == format!("{:?}", palette) {
        bg_palette = rng.random();
    }
    log::info!("Background Palette: {:?}", bg_palette);

    // ✨ Create grid with all cell data in ONE place using normalized coordinates!
    let grid = Grid::builder(ROWS, COLS, work_size)
        .with_cell_data(|row, col, bounds, rng| {
            generate_cell_data(row, col, bounds, rng, circle_radius, &palette, &bg_palette)
        })
        .build(&mut rng);

    // Initialize shader pipeline
    #[cfg(not(debug_assertions))]
    let pipeline = create_shape_pipeline(gfx, Some(&FRAG)).unwrap();
    #[cfg(debug_assertions)]
    let pipeline =
        create_hot_shape_pipeline(gfx, "examples/assets/shaders/tile_blend.frag.glsl").unwrap();

    let common_data = CommonData::new(0.0, work_size);
    let common_ubo = gfx
        .create_uniform_buffer(1, "Common")
        .with_data(&common_data)
        .build()
        .unwrap();

    // Create tile colors uniform buffer from grid data
    let mut tile_colors_flat: Vec<f32> = Vec::with_capacity(ROWS as usize * COLS as usize * 4);
    for cell in grid.cells() {
        tile_colors_flat.push(cell.data.bg_color.r);
        tile_colors_flat.push(cell.data.bg_color.g);
        tile_colors_flat.push(cell.data.bg_color.b);
        tile_colors_flat.push(cell.data.bg_color.a);
    }

    let tile_colors_ubo = gfx
        .create_uniform_buffer(2, "TileColors")
        .with_data(tile_colors_flat.as_slice())
        .build()
        .unwrap();

    let grid_info = TileGridInfo {
        grid_size: vec2(COLS as f32, ROWS as f32),
    };

    let tile_grid_info_ubo = gfx
        .create_uniform_buffer(3, "TileGridInfo")
        .with_data(&grid_info)
        .build()
        .unwrap();

    let srt = ShaderRenderTexture::new(gfx, work_size.x, work_size.y);

    State {
        rng,
        work_size,
        grid,
        circle_radius,
        palette,
        bg_palette,
        show_grid: false,
        pipeline,
        common_ubo,
        tile_colors_ubo,
        tile_grid_info_ubo,
        srt,
        tile_colors_dirty: false,
        #[cfg(debug_assertions)]
        hot_mgr: ShaderReloadManager::default(),
    }
}

fn update(app: &mut App, state: &mut State) {
    #[cfg(debug_assertions)]
    state.hot_mgr.update();

    if app.keyboard.was_pressed(KeyCode::KeyR) {
        let new_seed = state.rng.random();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Choose new palettes
        state.palette = state.rng.random();
        log::info!("Circle Palette: {:?}", state.palette);

        state.bg_palette = state.rng.random();
        while format!("{:?}", state.bg_palette) == format!("{:?}", state.palette) {
            state.bg_palette = state.rng.random();
        }
        log::info!("Background Palette: {:?}", state.bg_palette);

        // ✨ Regenerate all cell data with ONE method call!
        state
            .grid
            .regenerate_cells(&mut state.rng, |row, col, bounds, rng| {
                generate_cell_data(
                    row,
                    col,
                    bounds,
                    rng,
                    state.circle_radius,
                    &state.palette,
                    &state.bg_palette,
                )
            });

        state.tile_colors_dirty = true;
    }

    if app.keyboard.was_pressed(KeyCode::KeyG) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid toggled: {}", state.show_grid);
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_size(
            ScreenDimensions::RES_1080P.x as u32,
            ScreenDimensions::RES_1080P.y as u32,
        );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().set_high_dpi(true);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    #[cfg(debug_assertions)]
    if state.hot_mgr.needs_reload() {
        match create_hot_shape_pipeline(gfx, "examples/assets/shaders/tile_blend.frag.glsl") {
            Ok(pipeline) => state.pipeline = pipeline,
            Err(err) => log::error!("{}", err),
        }
    }

    // Update tile colors if needed
    if state.tile_colors_dirty {
        let mut tile_colors_flat: Vec<f32> = Vec::with_capacity(ROWS as usize * COLS as usize * 4);
        for cell in state.grid.cells() {
            tile_colors_flat.push(cell.data.bg_color.r);
            tile_colors_flat.push(cell.data.bg_color.g);
            tile_colors_flat.push(cell.data.bg_color.b);
            tile_colors_flat.push(cell.data.bg_color.a);
        }
        gfx.set_buffer_data(&state.tile_colors_ubo, tile_colors_flat.as_slice());
        state.tile_colors_dirty = false;
    }

    // Render shader
    let u_time = app.timer.elapsed_f32();
    let common_data = CommonData::new(u_time, state.work_size);

    state.srt.draw_filled(
        gfx,
        &state.pipeline,
        vec![
            &state.common_ubo,
            &state.tile_colors_ubo,
            &state.tile_grid_info_ubo,
        ],
    );

    let mut draw = get_draw_setup(gfx, state.work_size, false, Color::WHITE);

    draw.image(&state.srt.rt)
        .position(0.0, 0.0)
        .size(state.work_size.x, state.work_size.y);

    // ✨ Draw grid overlay using grid utilities!
    if state.show_grid {
        state
            .grid
            .draw_overlay(&mut draw, Color::GREEN, GRID_STROKE);
    }

    // ✨ Draw circles using grid iterator - NO manual tile_index tracking!
    for cell in state.grid.cells() {
        // Convert normalized position (0-1) to absolute pixels - ONE method call!
        let abs_pos = cell.to_px(cell.data.position);

        // Draw parent circle
        draw.circle(state.circle_radius)
            .position(abs_pos.x, abs_pos.y)
            .color(cell.data.color);

        // Draw child circles
        for child in &cell.data.children {
            let child_x = abs_pos.x + state.circle_radius * child.angle.cos();
            let child_y = abs_pos.y + state.circle_radius * child.angle.sin();
            draw.circle(child.radius)
                .position(child_x, child_y)
                .color(child.color);
        }
    }

    gfx.render(&draw);
    gfx.set_buffer_data(&state.common_ubo, &common_data);
}
