use notan::draw::*;
use notan::log;
use notan::math::{vec2, Rect, Vec2};
use notan::prelude::*;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::gridutils::Grid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, ScreenDimensions,
};

const MAX_ROWS: u32 = 20;
const MAX_COLS: u32 = 20;
const GRID_STROKE: f32 = 5.0;


#[derive(Debug)]
struct Tooth {
    start: Vec2,
    mid: Vec2,
    end: Vec2,
}

#[derive(Debug)]
struct CellData {
    teeth: Vec<Tooth>, // Teeth stored in normalized coordinates (0.0 to 1.0)
}

/// Grid example with cell-specific data (teeth) stored for performance.
/// This approach is better when you have complex calculations that would be
/// expensive to recompute on every draw.
#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>,
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

    // Grid with cell data containing teeth
    let grid = Grid::builder(rows, cols, work_size)
        .with_cell_data(|_row, _col, bounds, rng| generate_cell_data(bounds, rng))
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
            .with_cell_data(|_row, _col, bounds, rng| generate_cell_data(bounds, rng))
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


fn generate_cell_data(_bounds: Rect, rng: &mut Random) -> CellData {
    let mut teeth: Vec<Tooth> = vec![];

    // The height and width of the tooth if situated upright
    let max_height = 0.4;
    let min_height = 0.10;
    let tooth_width = 0.1;
    let padding = 0.05;

    for i in 2..10 {
        let tooth_height = rng.gen_range(min_height..max_height);
        // Bottom teeth (stored in normalized coordinates)
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, 1.0 - tooth_height);
        let start = vec2(boundary - tooth_width, 1.0 - padding);
        let end = vec2(boundary, 1.0 - padding);
        teeth.push(Tooth { start, mid, end });

        // Top teeth
        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, tooth_height);
        let start = vec2(boundary - tooth_width, padding);
        let end = vec2(boundary, padding);
        teeth.push(Tooth { start, mid, end });

        // Left teeth (horizontal)
        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(tooth_height, boundary - 0.05);
        let start = vec2(padding, boundary - tooth_width);
        let end = vec2(padding, boundary);
        teeth.push(Tooth { start, mid, end });

        // Right teeth (horizontal)
        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(1.0 - tooth_height, boundary - 0.05);
        let start = vec2(1.0 - padding, boundary - tooth_width);
        let end = vec2(1.0 - padding, boundary);
        teeth.push(Tooth { start, mid, end });
    }

    CellData { teeth }
}

fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.needs_redraw {
        state.draw = get_draw_setup(gfx, state.work_size, false, Color::WHITE);
        for cell in state.grid.cells() {
            // Draw "gums" - outer layer
            state
                .draw
                .rect(
                    (cell.offset.x, cell.offset.y),
                    (cell.bounds.width, cell.bounds.height),
                )
                .color(Color::new(0.9, 0.2, 0.1, 1.0));

            // Draw "mouth" - inner layer
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

            // Draw teeth from cell data (pre-generated in normalized coords, converted to pixels here)
            let color = Color::new(1.0, 1.0, 0.8, 1.0);
            for tooth in &cell.data.teeth {
                let start_px = cell.to_px(tooth.start);
                let mid_px = cell.to_px(tooth.mid);
                let end_px = cell.to_px(tooth.end);

                state
                    .draw
                    .triangle(
                        (start_px.x, start_px.y),
                        (mid_px.x, mid_px.y),
                        (end_px.x, end_px.y),
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
