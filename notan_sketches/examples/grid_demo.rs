// Simple grid utilities demonstration without shaders
// This example clearly shows the benefits of using the gridutils module:
// 1. Normalized coordinates (0.0-1.0) for resolution independence
// 2. Clean iteration with no manual index tracking
// 3. Automatic coordinate transformations
// 4. Single unified Grid<T> data structure

use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::gridutils::Grid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, ScreenDimensions,
};

const ROWS: u32 = 8;
const COLS: u32 = 8;
const GRID_STROKE: f32 = 5.0;

// Cell data using NORMALIZED coordinates (0.0-1.0)
// This makes the sketch resolution-independent!
#[derive(Clone)]
struct CellData {
    circle_pos: Vec2, // Normalized 0-1 within cell
    rect_pos: Vec2,   // Normalized 0-1 within cell
    rect_size: Vec2,  // Normalized 0-1 scale
    circle_color: Color,
    rect_color: Color,
    bg_color: Color,
}

#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>, // All cell data in one unified structure!
    palette: PalettesSelection,
    show_grid: bool,
}

// Helper to generate random cell data
fn generate_cell_data(
    _row: u32,
    _col: u32,
    _bounds: notan::math::Rect,
    rng: &mut Random,
    palette: &PalettesSelection,
) -> CellData {
    // All positions and sizes are normalized (0.0-1.0)!
    // The grid utilities will handle conversion to pixels automatically

    let circle_pos = vec2(
        rng.gen_range(0.2..0.8), // Keep away from edges
        rng.gen_range(0.2..0.8),
    );

    let rect_pos = vec2(rng.gen_range(0.1..0.9), rng.gen_range(0.1..0.9));

    let rect_size = vec2(
        rng.gen_range(0.2..0.5), // 20-50% of cell width
        rng.gen_range(0.2..0.5), // 20-50% of cell height
    );

    let circle_color = colors::Palettes::choose_color(palette);
    let rect_color = colors::Palettes::choose_color(palette);
    let bg_color = colors::Palettes::choose_color(palette);

    CellData {
        circle_pos,
        rect_pos,
        rect_size,
        circle_color,
        rect_color,
        bg_color,
    }
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    // Choose a color palette
    let palette: PalettesSelection = rng.gen();
    log::info!("Palette: {:?}", palette);

    // Create grid using the builder pattern
    // Notice how clean this is - no manual loops or index tracking!
    let grid = Grid::builder(ROWS, COLS, work_size)
        .with_cell_data(|row, col, bounds, rng| generate_cell_data(row, col, bounds, rng, &palette))
        .build(&mut rng);

    log::info!("Created {}x{} grid", ROWS, COLS);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");

    State {
        rng,
        work_size,
        grid,
        palette,
        show_grid: false,
    }
}

fn update(app: &mut App, state: &mut State) {
    // R key - regenerate everything with new palette
    if app.keyboard.was_pressed(KeyCode::R) {
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Choose new palette
        state.palette = state.rng.gen();
        log::info!("Palette: {:?}", state.palette);

        // Regenerate all cell data with ONE method call!
        // No need to manually loop, clear vectors, etc.
        state
            .grid
            .regenerate_cells(&mut state.rng, |row, col, bounds, rng| {
                generate_cell_data(row, col, bounds, rng, &state.palette)
            });
    }

    // G key - toggle grid overlay
    if app.keyboard.was_pressed(KeyCode::G) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid overlay: {}", state.show_grid);
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

fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = get_draw_setup(gfx, state.work_size, false, Color::WHITE);

    // Draw backgrounds first
    for cell in state.grid.cells() {
        draw.rect(
            (cell.offset.x, cell.offset.y),
            (cell.bounds.width, cell.bounds.height),
        )
        .color(cell.data.bg_color);
    }

    // Draw rectangles
    for cell in state.grid.cells() {
        let rect_center = cell.norm(cell.data.rect_pos);
        let rect_size = cell.norm_size(cell.data.rect_size);

        // draw.rect() uses the position as top-left corner, not center
        // So rect_center is actually where we want the top-left
        draw.rect((rect_center.x, rect_center.y), (rect_size.x, rect_size.y))
            .color(cell.data.rect_color);
    }

    // Draw circles last
    for cell in state.grid.cells() {
        let circle_pos = cell.norm(cell.data.circle_pos);
        let cell_min_dim = cell.bounds.width.min(cell.bounds.height);
        let circle_radius = cell_min_dim * 0.15;

        draw.circle(circle_radius)
            .position(circle_pos.x, circle_pos.y)
            .color(cell.data.circle_color);
    }

    // Draw grid overlay with ONE method call!
    if state.show_grid {
        state
            .grid
            .draw_overlay(&mut draw, Color::GREEN, GRID_STROKE);
    }

    gfx.render(&draw);
}
