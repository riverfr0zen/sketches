use notan::draw::*;
use notan::egui::{self, *};
use notan::log;
use notan::math::{vec2, Rect, Vec2};
use notan::prelude::*;
use notan_sketches::colors::{Palettes, PalettesSelection};
use notan_sketches::gridutils::Grid;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, CapturingTexture,
    ScreenDimensions,
};

const MAX_DIMENSION: u32 = 10;
const GRID_STROKE: f32 = 5.0;

#[derive(Debug, Clone)]
struct Eye {
    center: Vec2, // Normalized position
    radius: Vec2, // Normalized radii (x, y)
}

#[derive(Debug, Clone)]
struct Mouth {
    center: Vec2, // Normalized position
    radius: Vec2, // Normalized radii (x, y)
}

#[derive(Debug)]
struct SmileyData {
    face_center: Vec2, // Normalized center position
    face_radius: f32,  // Normalized radius
    left_eye: Eye,
    right_eye: Eye,
    mouth: Mouth,
    // Colors
    bg_color: Color,
    face_color: Color,
    eye_color: Color,
    mouth_color: Color,
}

/// Check if an ellipse's extreme points are all within a circle
fn ellipse_fits_in_circle(
    ellipse_center: Vec2,
    ellipse_radius: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    // Check the 4 extreme points of the axis-aligned ellipse
    let points = [
        vec2(ellipse_center.x + ellipse_radius.x, ellipse_center.y), // right
        vec2(ellipse_center.x - ellipse_radius.x, ellipse_center.y), // left
        vec2(ellipse_center.x, ellipse_center.y + ellipse_radius.y), // bottom
        vec2(ellipse_center.x, ellipse_center.y - ellipse_radius.y), // top
    ];

    for point in &points {
        let dx = point.x - circle_center.x;
        let dy = point.y - circle_center.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq > circle_radius * circle_radius {
            return false;
        }
    }
    true
}

fn generate_round_eyes(rng: &mut Random) -> (Eye, Eye) {
    // Generate left eye
    let eye_radius_x = rng.gen_range(0.01..0.09);
    let eye_radius_y = rng.gen_range(0.01..0.09);

    // Left eye: position on left side of face (x < face_center.x)
    // Keep within a safer zone: face is 0.1-0.9, center at 0.5
    let eye_x = rng.gen_range(0.25..0.5);
    let eye_y = rng.gen_range(0.25..0.45);

    let left_eye = Eye {
        center: vec2(eye_x, eye_y),
        radius: vec2(eye_radius_x, eye_radius_y),
    };


    // Generate right eye
    let eye_radius_x = rng.gen_range(0.01..0.09);
    let eye_radius_y = rng.gen_range(0.01..0.09);

    // Right eye: position on right side of face (x > face_center.x)
    let eye_x = rng.gen_range(0.5..0.75);
    let eye_y = rng.gen_range(0.25..0.45);

    let right_eye = Eye {
        center: vec2(eye_x, eye_y),
        radius: vec2(eye_radius_x, eye_radius_y),
    };

    (left_eye, right_eye)
}

/// Generate random smiley data for a cell
fn generate_smiley_data(
    _row: u32,
    _col: u32,
    _bounds: Rect,
    palette: &PalettesSelection,
    rng: &mut Random,
) -> SmileyData {
    // Face: consistent across cells
    let face_center = vec2(0.5, 0.5);
    let face_radius = 0.48;

    let (left_eye, right_eye) = generate_round_eyes(rng);

    // Generate mouth with validation
    let mut attempts = 0;
    let mouth = loop {
        attempts += 1;
        if attempts > 100 {
            // Fallback to safe default
            break Mouth {
                center: vec2(0.5, 0.6),
                radius: vec2(0.08, 0.03),
            };
        }

        let mouth_radius_x = rng.gen_range(0.01..0.15);
        let mouth_radius_y: f32 = rng.gen_range(0.01..0.15);
        let mouth_y_offset = rng.gen_range(0.05..0.4);
        let mouth_y = face_center.y + mouth_y_offset;

        let mouth = Mouth {
            center: vec2(face_center.x, mouth_y),
            radius: vec2(mouth_radius_x, mouth_radius_y),
        };

        if ellipse_fits_in_circle(mouth.center, mouth.radius, face_center, face_radius) {
            break mouth;
        }
    };

    // Generate colors for this cell
    let bg_color = Palettes::choose_color(palette);

    // Ensure face color is different from background
    let mut face_color = Palettes::choose_color(palette);
    while face_color == bg_color {
        face_color = Palettes::choose_color(palette);
    }

    // Ensure eye color is different from face
    let mut eye_color = Palettes::choose_color(palette);
    while eye_color == face_color {
        eye_color = Palettes::choose_color(palette);
    }

    // Ensure mouth color is different from face
    let mut mouth_color = Palettes::choose_color(palette);
    while mouth_color == face_color {
        mouth_color = Palettes::choose_color(palette);
    }

    SmileyData {
        face_center,
        face_radius,
        left_eye,
        right_eye,
        mouth,
        bg_color,
        face_color,
        eye_color,
        mouth_color,
    }
}

#[derive(AppState)]
struct State {
    rng: Random,
    current_seed: u64,
    work_size: Vec2,
    grid: Grid<SmileyData>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    capture_next_draw: bool,
    draw: Draw,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    // Choose a color palette
    let palette: PalettesSelection = rng.gen();
    log::info!("Palette: {:?}", palette);

    let dimensional_count = rng.gen_range(1..MAX_DIMENSION);
    let rows = dimensional_count;
    let cols = dimensional_count;

    // Grid with cell data containing smiley faces (including colors)
    let grid = Grid::builder(rows, cols, work_size)
        .with_cell_data(|row, col, bounds, rng| {
            generate_smiley_data(row, col, bounds, &palette, rng)
        })
        .build(&mut rng);

    log::info!("Created {}x{} grid", rows, cols);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");
    log::info!("Press C to capture");

    // Use background color from first cell (they can vary per cell now)
    let bg_color = grid.cells().next().map(|c| c.data.bg_color).unwrap_or(Color::BLACK);
    let draw = get_draw_setup(gfx, work_size, false, bg_color);

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
    }
}

fn update(app: &mut App, state: &mut State) {
    // R key - redraw
    if app.keyboard.was_pressed(KeyCode::R) {
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        state.current_seed = new_seed;
        log::info!("New seed: {}", new_seed);

        // Choose new palette
        state.palette = state.rng.gen();
        log::info!("Palette: {:?}", state.palette);

        // Create a new grid with different size
        let dimensional_count = state.rng.gen_range(1..MAX_DIMENSION);
        let rows = dimensional_count;
        let cols = dimensional_count;

        // Create grid with smiley data (including colors)
        state.grid = Grid::builder(rows, cols, state.work_size)
            .with_cell_data(|row, col, bounds, rng| {
                generate_smiley_data(row, col, bounds, &state.palette, rng)
            })
            .build(&mut state.rng);

        log::info!("Created {}x{} grid", rows, cols);

        state.needs_redraw = true;
    }

    // C key - queue capture next draw
    if app.keyboard.was_pressed(KeyCode::C) {
        state.capture_next_draw = true;
    }

    // G key - toggle grid overlay
    if app.keyboard.was_pressed(KeyCode::G) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid overlay: {}", state.show_grid);
        // state.needs_redraw = true;
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    if state.needs_redraw {
        // Use first cell's bg color for overall background
        let bg_color = state.grid.cells().next().map(|c| c.data.bg_color).unwrap_or(Color::BLACK);
        state.draw = get_draw_setup(gfx, state.work_size, false, bg_color);

        for cell in state.grid.cells() {
            let smiley = &cell.data;

            // Draw cell background (per-cell background color)
            state
                .draw
                .rect((cell.offset.x, cell.offset.y), (cell.bounds.width, cell.bounds.height))
                .color(smiley.bg_color)
                .fill();

            // Draw face circle
            let face_center_px = cell.to_px(smiley.face_center);
            let face_radius_px = cell.bounds.width.min(cell.bounds.height) * smiley.face_radius;

            state
                .draw
                .circle(face_radius_px)
                .position(face_center_px.x, face_center_px.y)
                .color(smiley.face_color)
                .fill();

            // Draw left eye
            let left_eye_center_px = cell.to_px(smiley.left_eye.center);
            let left_eye_radius_px = vec2(
                cell.bounds.width * smiley.left_eye.radius.x,
                cell.bounds.height * smiley.left_eye.radius.y,
            );

            state
                .draw
                .ellipse(
                    (left_eye_center_px.x, left_eye_center_px.y),
                    (left_eye_radius_px.x, left_eye_radius_px.y),
                )
                .color(smiley.eye_color)
                .fill();

            // Draw right eye
            let right_eye_center_px = cell.to_px(smiley.right_eye.center);
            let right_eye_radius_px = vec2(
                cell.bounds.width * smiley.right_eye.radius.x,
                cell.bounds.height * smiley.right_eye.radius.y,
            );

            state
                .draw
                .ellipse(
                    (right_eye_center_px.x, right_eye_center_px.y),
                    (right_eye_radius_px.x, right_eye_radius_px.y),
                )
                .color(smiley.eye_color)
                .fill();

            // Draw mouth
            let mouth_center_px = cell.to_px(smiley.mouth.center);
            let mouth_radius_px = vec2(
                cell.bounds.width * smiley.mouth.radius.x,
                cell.bounds.height * smiley.mouth.radius.y,
            );

            state
                .draw
                .ellipse(
                    (mouth_center_px.x, mouth_center_px.y),
                    (mouth_radius_px.x, mouth_radius_px.y),
                )
                .color(smiley.mouth_color)
                .fill();
        }

        state.needs_redraw = false;
    }

    if state.capture_next_draw {
        // Use 2x supersampling for better antialiasing in captures
        let supersample_factor = 2.0;
        let bg_color = state.grid.cells().next().map(|c| c.data.bg_color).unwrap_or(Color::BLACK);
        let mut capture = CapturingTexture::new_with_supersample(
            gfx,
            &state.work_size,
            bg_color,
            format!("renders/smiley_gen/{}", state.current_seed),
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
        state
            .grid
            .draw_overlay(&mut state.draw, Color::GREEN, GRID_STROKE);

        // When grid is enabled, we always redraw to ensure reactivity to grid controls
        state.needs_redraw = true;
    }

    gfx.render(&state.draw);

    // Add egui UI on top
    let output = plugins.egui(|ctx| {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Seed:");
                ui.label(state.current_seed.to_string());
            });
        });
    });

    gfx.render(&output);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_vsync(true)
        .set_multisampling(8)
        .set_size(
            ScreenDimensions::RES_1080P.x as u32,
            ScreenDimensions::RES_1080P.y as u32,
        );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_multisampling(8);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}
