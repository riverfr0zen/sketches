use notan::draw::*;
use notan::log;
use notan::math::{vec2, Rect, Vec2};
use notan::prelude::*;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::gridutils::Grid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, CapturingTexture,
    ScreenDimensions,
};

const MAX_ROWS: u32 = 20;
const MAX_COLS: u32 = 20;
const GRID_STROKE: f32 = 5.0;
const GUMS_COLOR: Color = Color::new(0.9, 0.3, 0.3, 1.0);
const MOUTH_COLOR: Color = Color::new(0.7, 0.2, 0.1, 1.0);
const THROAT_COLOR: Color = Color::new(0.85, 0.27, 0.27, 1.0);
const BG_COLOR: Color = Color::new(0.6, 0.2, 0.2, 1.0);
// Green mouth
// const MOUTH_COLOR: Color = Color::new(0.1, 0.3, 0.1, 1.0);
// const THROAT_COLOR: Color = Color::new(0.05, 0.15, 0.05, 1.0);
// const BG_COLOR: Color = Color::new(0.0, 0.2, 0.0, 1.0);
const TOOTH_COLOR: Color = Color::new(1.0, 1.0, 0.8, 1.0);
// const TOOTH_STROKE: Color = Color::new(6.0, 6.0, 0.4, 1.0);
const TOOTH_STROKE: Color = Color::new(4.0, 4.0, 0.2, 1.0);

// Influence point system constants
const CELLS_PER_INFLUENCE_POINT: f32 = 8.0; // Number of cells per influence point
const BASE_MAX_HEIGHT: f32 = 0.08; // Base max height for teeth far from influence
const MIN_TOOTH_HEIGHT: f32 = 0.06; // Minimum possible tooth height (must be > padding)
const INFLUENCE_RADIUS: f32 = 0.3; // Radius of influence in normalized space
const MAX_HEIGHT_BOOST: f32 = 0.42; // Maximum additional height from influence


#[derive(Debug)]
struct Tooth {
    start: Vec2,
    mid: Vec2,
    end: Vec2,
}

#[derive(Debug)]
struct CellData {
    teeth: Vec<Tooth>,   // Teeth stored in normalized coordinates (0.0 to 1.0)
    throat_center: Vec2, // Normalized center position
    throat_radius: Vec2, // Normalized radii (x, y)
}

/// Calculate the number of influence points based on total cell count.
fn calculate_influence_point_count(total_cells: usize) -> usize {
    ((total_cells as f32 / CELLS_PER_INFLUENCE_POINT).ceil() as usize).max(1)
}

/// Generate random influence points in normalized canvas space (0.0-1.0).
fn generate_influence_points(count: usize, rng: &mut Random) -> Vec<Vec2> {
    (0..count)
        .map(|_| vec2(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)))
        .collect()
}

/// Calculate distance from a point to the nearest influence point.
fn distance_to_nearest_influence(cell_center_norm: Vec2, influence_points: &[Vec2]) -> f32 {
    influence_points
        .iter()
        .map(|&point| {
            let dx = cell_center_norm.x - point.x;
            let dy = cell_center_norm.y - point.y;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(f32::INFINITY, f32::min)
}

/// Calculate max tooth height based on distance to nearest influence point.
/// Closer to influence = taller teeth (for horizontal teeth).
fn calculate_max_height_from_influence(distance: f32) -> f32 {
    if distance >= INFLUENCE_RADIUS {
        BASE_MAX_HEIGHT
    } else {
        let influence_factor = 1.0 - (distance / INFLUENCE_RADIUS);
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}

/// Calculate INVERSE max tooth height - farther from influence = taller teeth (for vertical teeth).
fn calculate_inverse_max_height_from_influence(distance: f32) -> f32 {
    if distance >= INFLUENCE_RADIUS {
        // Far away: maximum height
        BASE_MAX_HEIGHT + MAX_HEIGHT_BOOST
    } else {
        // Close by: shorter teeth
        let influence_factor = distance / INFLUENCE_RADIUS; // Inverted: distance/radius instead of 1.0 - distance/radius
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}

/// Calculate canvas-wide normalized center position for a cell during grid construction.
/// This replicates what CellContext::center_norm_abs() will provide after construction.
fn cell_center_canvas_norm(row: u32, col: u32, total_rows: u32, total_cols: u32) -> Vec2 {
    vec2(
        (col as f32 + 0.5) / total_cols as f32,
        (row as f32 + 0.5) / total_rows as f32,
    )
}

/// Grid example with cell-specific data (teeth) stored for performance.
/// This approach is better when you have complex calculations that would be
/// expensive to recompute on every draw.
#[derive(AppState)]
struct State {
    rng: Random,
    current_seed: u64,
    work_size: Vec2,
    grid: Grid<CellData>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    capture_next_draw: bool,
    draw: Draw,
    influence_points: Vec<Vec2>, // Stored in normalized canvas coords (0-1)
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    // // Choose a color palette
    let palette: PalettesSelection = rng.random();
    log::info!("Palette: {:?}", palette);

    let rows = rng.random_range(1..MAX_ROWS);
    let cols = rng.random_range(1..MAX_COLS);

    // Generate influence points
    let total_cells = (rows * cols) as usize;
    let influence_count = calculate_influence_point_count(total_cells);
    let influence_points = generate_influence_points(influence_count, &mut rng);

    log::info!(
        "Created {} influence points for {}x{} grid",
        influence_count,
        rows,
        cols
    );

    // Grid with cell data containing teeth influenced by proximity to influence points
    let grid = Grid::builder(rows, cols, work_size)
        .with_cell_data(|row, col, bounds, rng| {
            let cell_center = cell_center_canvas_norm(row, col, rows, cols);
            generate_cell_data_influenced(row, col, bounds, cell_center, &influence_points, rng)
        })
        .build(&mut rng);

    log::info!("Created {}x{} grid", rows, cols);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");

    let draw = get_draw_setup(gfx, work_size, false, BG_COLOR);

    State {
        rng,
        current_seed: seed,
        work_size,
        grid,
        palette,
        show_grid: false,
        needs_redraw: true,
        capture_next_draw: false,
        draw,
        influence_points,
    }
}

fn update(app: &mut App, state: &mut State) {
    // R key - redraw
    if app.keyboard.was_pressed(KeyCode::KeyR) {
        let new_seed = state.rng.random();
        state.rng.reseed(new_seed);
        state.current_seed = new_seed;
        log::info!("New seed: {}", new_seed);

        // Choose new palette
        state.palette = state.rng.random();
        log::info!("Palette: {:?}", state.palette);

        // Create a new grid with different size
        let rows = state.rng.random_range(1..MAX_ROWS);
        let cols = state.rng.random_range(1..MAX_COLS);

        // Generate new influence points
        let total_cells = (rows * cols) as usize;
        let influence_count = calculate_influence_point_count(total_cells);
        state.influence_points = generate_influence_points(influence_count, &mut state.rng);

        log::info!(
            "Created {} influence points for {}x{} grid",
            influence_count,
            rows,
            cols
        );

        // Create grid with influence-based teeth
        state.grid = Grid::builder(rows, cols, state.work_size)
            .with_cell_data(|row, col, bounds, rng| {
                let cell_center = cell_center_canvas_norm(row, col, rows, cols);
                generate_cell_data_influenced(
                    row,
                    col,
                    bounds,
                    cell_center,
                    &state.influence_points,
                    rng,
                )
            })
            .build(&mut state.rng);

        log::info!("Created {}x{} grid", rows, cols);


        state.needs_redraw = true;
    }

    // C key - queue capture next draw
    if app.keyboard.was_pressed(KeyCode::KeyC) {
        state.capture_next_draw = true;
    }

    // G key - toggle grid overlay
    if app.keyboard.was_pressed(KeyCode::KeyG) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid overlay: {}", state.show_grid);
    }
}


fn generate_cell_data_influenced(
    _row: u32,
    _col: u32,
    _bounds: Rect,
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
    rng: &mut Random,
) -> CellData {
    // Calculate max height based on distance to nearest influence point
    let distance = distance_to_nearest_influence(cell_center_norm, influence_points);

    // Randomly choose whether horizontal or vertical teeth react to influence
    let horizontal_influenced = rng.random_bool(0.5);

    // Calculate max heights for influenced and non-influenced teeth
    let max_height_influenced = calculate_max_height_from_influence(distance);
    let max_height_base = BASE_MAX_HEIGHT;

    let mut teeth: Vec<Tooth> = vec![];

    let tooth_width = 0.1;
    let padding = 0.05;

    for i in 2..10 {
        // Bottom teeth (horizontal)
        let max_height = if horizontal_influenced {
            max_height_influenced
        } else {
            max_height_base
        };
        let min_height = MIN_TOOTH_HEIGHT.min(max_height * 0.5);
        let tooth_height = rng.random_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, 1.0 - tooth_height);
        let start = vec2(boundary - tooth_width, 1.0 - padding);
        let end = vec2(boundary, 1.0 - padding);
        teeth.push(Tooth { start, mid, end });

        // Top teeth (horizontal)
        let tooth_height = rng.random_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, tooth_height);
        let start = vec2(boundary - tooth_width, padding);
        let end = vec2(boundary, padding);
        teeth.push(Tooth { start, mid, end });

        // Left teeth (vertical)
        let max_height = if horizontal_influenced {
            max_height_base
        } else {
            max_height_influenced
        };
        let min_height = MIN_TOOTH_HEIGHT.min(max_height * 0.5);
        let tooth_height = rng.random_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(tooth_height, boundary - 0.05);
        let start = vec2(padding, boundary - tooth_width);
        let end = vec2(padding, boundary);
        teeth.push(Tooth { start, mid, end });

        // Right teeth (vertical)
        let tooth_height = rng.random_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(1.0 - tooth_height, boundary - 0.05);
        let start = vec2(1.0 - padding, boundary - tooth_width);
        let end = vec2(1.0 - padding, boundary);
        teeth.push(Tooth { start, mid, end });
    }

    // Throat details (normalized coordinates)
    let throat_center = vec2(0.5, 0.5);
    let throat_radius = vec2(0.25, 0.25);

    CellData {
        teeth,
        throat_center,
        throat_radius,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.needs_redraw {
        state.draw = get_draw_setup(gfx, state.work_size, false, BG_COLOR);
        for cell in state.grid.cells() {
            // Draw "mouth" - outer layer with rounded corners
            let corner_radius = cell.bounds.width.min(cell.bounds.height) * 0.1;
            state
                .draw
                .rect(
                    (cell.offset.x, cell.offset.y),
                    (cell.bounds.width, cell.bounds.height),
                )
                .corner_radius(corner_radius)
                .color(MOUTH_COLOR);

            // Draw "gums" - inner layer with rounded corners
            let padding = cell.norm_size(vec2(0.05, 0.05));
            let inner_corner_radius =
                (cell.bounds.width - padding.x).min(cell.bounds.height - padding.y) * 0.12;
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
                .corner_radius(inner_corner_radius)
                .color(GUMS_COLOR);

            // Draw "throat" - dark ellipse at center (behind teeth)
            let center = cell.to_px(cell.data.throat_center);
            let throat_radius_x = cell.bounds.width * cell.data.throat_radius.x;
            let throat_radius_y = cell.bounds.height * cell.data.throat_radius.y;
            state
                .draw
                .ellipse((center.x, center.y), (throat_radius_x, throat_radius_y))
                .color(THROAT_COLOR);

            // Draw teeth from cell data (pre-generated in normalized coords, converted to pixels here)
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
                    // .stroke_color(Color::BLACK)
                    .stroke_color(TOOTH_STROKE)
                    .fill_color(TOOTH_COLOR)
                    .fill();
            }
        }
        state.needs_redraw = false;
    }

    if state.capture_next_draw {
        // Use 2x supersampling for better antialiasing in captures
        // On native: automatically downsamples to work_size for smaller files
        // On WASM: saves full supersampled image (browser download)
        let supersample_factor = 2.0;
        let mut capture = CapturingTexture::new_with_supersample(
            gfx,
            &state.work_size,
            BG_COLOR,
            format!("renders/bobas-nightmare/{}", state.current_seed),
            0.0,
            supersample_factor,
        );
        // Render the existing draw to the supersampled texture
        gfx.render_to(&capture.render_texture, &state.draw);
        capture.capture(app, gfx);
        log::info!(
            "Capture completed with {}x supersampling",
            supersample_factor
        );
        state.capture_next_draw = false;
    }


    if state.show_grid {
        // Draw influence points and their radii for debugging
        for &point in &state.influence_points {
            let abs_pos = state.grid.norm_to_pixels(point);

            // Draw influence radius circle
            let radius_pixels = INFLUENCE_RADIUS * state.work_size.x.min(state.work_size.y);
            state
                .draw
                .circle(radius_pixels)
                .position(abs_pos.x, abs_pos.y)
                .fill_color(Color::from_rgba(0.0, 0.0, 1.0, 0.12))
                .fill()
                .stroke(2.0)
                .stroke_color(Color::from_rgba(0.0, 0.0, 1.0, 0.4));

            // Draw influence point marker
            state
                .draw
                .circle(8.0)
                .position(abs_pos.x, abs_pos.y)
                .color(Color::BLUE)
                .fill();
        }

        state
            .grid
            .draw_overlay(&mut state.draw, Color::GREEN, GRID_STROKE);

        // When grid is enabled, we always redraw to ensure reactivity to grid controls
        state.needs_redraw = true;
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
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_multisampling(8);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}
