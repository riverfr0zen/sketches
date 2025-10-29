use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors::{Palettes, PalettesSelection};
use notan_sketches::gridutils::{CellContext, Grid};
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, ScreenDimensions,
};

const MAX_ROWS: u32 = 20;
const MAX_COLS: u32 = 20;
const GRID_STROKE: f32 = 5.0;
const SHADOW_COLOR: Color = Color::new(0.25, 0.25, 0.25, 0.25);


#[derive(Debug)]
struct Tooth {
    start: Vec2,
    mid: Vec2,
    end: Vec2,
}

/// Simple grid example with no cell-specific data. Drawing only happens if `needs_redraw` is true,
/// and drawing persists by keeping the Draw in the AppState.
///
/// Hence, we don't need any cell-specific data to persist the drawing (note that in other sketches,
/// cell-specific data may still be needed to persist non-drawing related elements).
///
/// This kind of setup is enough for static (no animation) sketches where the next redraw does not depend
/// on the previous draw details.
///
/// KNOWN ISSUES: Unfortunately, one of the caveats of this simpler setup is that the grid overlay does not
/// disappear immediately once toggled off, and only disappears on the next redraw. The alternative would be
/// to use 2 draws and render the state.draw into the other one on every call to draw(), as done in
/// `radial_pointillist.rs`.
#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    // Simple grid with no cell-specific data
    grid: Grid<bool>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    draw: Draw,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    // // Choose a color palette
    let palette: PalettesSelection = rng.gen();
    log::info!("Palette: {:?}", palette);

    let rows = rng.gen_range(1..MAX_ROWS);
    let cols = rng.gen_range(1..MAX_COLS);

    // Very simple grid with no cell data
    let grid = Grid::builder(rows, cols, work_size)
        // .with_cell_data(|row, col, bounds, rng| generate_cell_data(row, col, bounds, rng, &palette))
        .with_cell_data(|row, col, bounds, rng| false)
        .build(&mut rng);

    log::info!("Created {}x{} grid", rows, cols);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");

    let draw = get_draw_setup(gfx, work_size, false, Color::WHITE);

    State {
        rng,
        work_size,
        grid,
        palette,
        show_grid: false,
        needs_redraw: true,
        draw,
    }
}

fn update(app: &mut App, state: &mut State) {
    // R key - redraw
    if app.keyboard.was_pressed(KeyCode::R) {
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Choose new palette
        state.palette = state.rng.gen();
        log::info!("Palette: {:?}", state.palette);

        // Create a new grid with different size
        let rows = state.rng.gen_range(1..MAX_ROWS);
        let cols = state.rng.gen_range(1..MAX_COLS);

        state.grid = Grid::builder(rows, cols, state.work_size)
            // .with_cell_data(|row, col, bounds, rng| generate_cell_data(row, col, bounds, rng, &palette))
            .with_cell_data(|row, col, bounds, rng| false)
            .build(&mut state.rng);

        log::info!("Created {}x{} grid", rows, cols);


        state.needs_redraw = true;
    }

    // G key - toggle grid overlay
    if app.keyboard.was_pressed(KeyCode::G) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid overlay: {}", state.show_grid);
    }
}


fn get_cell_teeth<T>(cell: CellContext<T>, rng: &mut Random) -> Vec<Tooth> {
    let mut teeth: Vec<Tooth> = vec![];
    // log::info!("{}, {}, {:?}", cell.row, cell.col, cell.bounds);

    // The height and width of the tooth if situated upright
    let max_height = 0.4;
    let min_height = 0.10;
    let width = 0.1;
    let padding = 0.05;


    for i in 2..10 {
        let height = rng.gen_range(min_height..max_height);
        // Bottom teeth
        let boundary: f32 = i as f32 / 10.0;
        // let mid = (boundary * 0.5, height);
        // let start = (boundary - width, 1.0);
        // let end = (boundary, 1.0);
        let mid = cell.to_px(vec2(boundary - 0.05, 1.0 - height));
        // let start = cell.to_px(vec2(boundary - width, 1.0));
        let start = cell.to_px(vec2(boundary - width, 1.0 - padding));
        let end = cell.to_px(vec2(boundary, 1.0 - padding));
        teeth.push(Tooth { start, mid, end });

        // Top teeth
        let height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        // let mid = (boundary * 0.5, height);
        // let start = (boundary - width, 1.0);
        // let end = (boundary, 1.0);
        let mid = cell.to_px(vec2(boundary - 0.05, height));
        let start = cell.to_px(vec2(boundary - width, padding));
        let end = cell.to_px(vec2(boundary, padding));
        teeth.push(Tooth { start, mid, end });
    }

    teeth
}

fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.needs_redraw {
        state.draw = get_draw_setup(gfx, state.work_size, false, Color::WHITE);
        for cell in state.grid.cells() {
            // Draw "gums"
            state
                .draw
                .rect(
                    (cell.offset.x, cell.offset.y),
                    (cell.bounds.width, cell.bounds.height),
                )
                .color(Color::new(0.9, 0.2, 0.1, 1.0));

            // Draw "gums"
            let padding = cell.norm_size(vec2(0.05, 0.05));
            state
                .draw
                .rect(
                    (
                        cell.offset.x + padding.x * 0.5,
                        cell.offset.y + padding.y * 0.5,
                    ),
                    (
                        cell.bounds.width - padding.x,
                        cell.bounds.height - padding.y,
                    ),
                )
                .color(Color::new(0.7, 0.2, 0.1, 1.0));

            let teeth = get_cell_teeth(cell, &mut state.rng);
            // log::info!("{:?}", teeth);

            // let color = Palettes::choose_color(&state.palette);
            let color = Color::new(1.0, 1.0, 0.8, 1.0);
            for tooth in teeth {
                state
                    .draw
                    .triangle(
                        (tooth.start.x, tooth.start.y),
                        (tooth.mid.x, tooth.mid.y),
                        (tooth.end.x, tooth.end.y),
                    )
                    .stroke(5.0)
                    .stroke_color(Color::BLACK)
                    .fill_color(color)
                    .fill();
            }
        }
        state.needs_redraw = false;
    }

    if state.show_grid {
        state
            .grid
            .draw_overlay(&mut state.draw, Color::GREEN, GRID_STROKE);
    }

    gfx.render(&state.draw);
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_multisampling(8)
        .set_size(
            // ScreenDimensions::RES_1080P.x as u32,
            // ScreenDimensions::RES_1080P.y as u32,
            // ScreenDimensions::RES_4K.x as u32,
            // ScreenDimensions::RES_4K.y as u32,
            ScreenDimensions::RES_4KISH.x as u32,
            ScreenDimensions::RES_4KISH.y as u32,
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
