use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const ROWS: u32 = 5;
const COLS: u32 = 5;

#[derive(AppState)]
struct State {
    rng: Random,
    tile_width: f32,
    tile_height: f32,
    circle_positions: Vec<Vec2>,
    show_grid: bool,
}

fn init(_gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    // Calculate tile dimensions
    let tile_width = WORK_SIZE.x / COLS as f32;
    let tile_height = WORK_SIZE.y / ROWS as f32;

    // Generate random positions for each tile
    let mut circle_positions = Vec::new();
    for _ in 0..(ROWS * COLS) {
        circle_positions.push(vec2(
            rng.gen_range(0.0..tile_width),
            rng.gen_range(0.0..tile_height),
        ));
    }

    State {
        rng,
        tile_width,
        tile_height,
        circle_positions,
        show_grid: false,
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        // Reseed the RNG with a new random seed
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Generate new random positions for each tile
        state.circle_positions.clear();
        for _ in 0..(ROWS * COLS) {
            state.circle_positions.push(vec2(
                state.rng.gen_range(0.0..state.tile_width),
                state.rng.gen_range(0.0..state.tile_height),
            ));
        }
    }

    if app.keyboard.was_pressed(KeyCode::G) {
        state.show_grid = !state.show_grid;
        log::debug!("Grid toggled: {}", state.show_grid);
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    // Set up draw with scaling projection (aspect_fit = false)
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    // Draw tiled circles
    let mut tile_index = 0;
    for row in 0..ROWS {
        for col in 0..COLS {
            // Calculate the tile's top-left corner
            let tile_x = col as f32 * state.tile_width;
            let tile_y = row as f32 * state.tile_height;

            // Get the position for this specific tile
            let circle_pos = state.circle_positions[tile_index];

            // Draw the circle at the unique position within this tile
            draw.circle(50.0)
                .position(tile_x + circle_pos.x, tile_y + circle_pos.y)
                .color(Color::BLUE);

            tile_index += 1;
        }
    }

    // Draw grid if enabled
    if state.show_grid {
        // Draw vertical lines
        for col in 0..=COLS {
            let x = col as f32 * state.tile_width;
            draw.path()
                .move_to(x, 0.0)
                .line_to(x, WORK_SIZE.y)
                .stroke_color(Color::GREEN)
                .stroke(1.0);
        }

        // Draw horizontal lines
        for row in 0..=ROWS {
            let y = row as f32 * state.tile_height;
            draw.path()
                .move_to(0.0, y)
                .line_to(WORK_SIZE.x, y)
                .stroke_color(Color::GREEN)
                .stroke(1.0);
        }
    }

    // Render to screen
    gfx.render(&draw);
}
