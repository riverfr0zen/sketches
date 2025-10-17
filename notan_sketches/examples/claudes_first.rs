use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::colors::PalettesSelection;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, ScreenDimensions,
};
use palette::{FromColor, Hsv, Shade, Srgb};
use std::f32::consts::PI;

const ROWS: u32 = 15;
const COLS: u32 = 15;
const MAX_CHILD_CIRCLES: u32 = 3;
const DARKEN_MAX: f32 = 0.2;
const LIGHTEN_MAX: f32 = 0.3;
const CHILD_RADIUS_MOD_MAX: f32 = 0.5;
const CHILD_RADIUS_MOD_MIN: f32 = 0.125;
const GRID_STROKE: f32 = 5.0;


#[derive(Clone)]
struct ChildCircle {
    angle: f32,
    radius: f32,
    color: Color,
}

fn vary_color(color: Color, rng: &mut Random) -> Color {
    let srgb = Srgb::new(color.r, color.g, color.b);
    let mut hsv = Hsv::from_color(srgb);

    // Randomly darken or lighten
    if rng.gen_bool(0.5) {
        hsv = hsv.darken(rng.gen_range(0.0..DARKEN_MAX));
    } else {
        hsv = hsv.lighten(rng.gen_range(0.0..LIGHTEN_MAX));
    }

    let result = Srgb::from_color(hsv);
    Color::new(result.red, result.green, result.blue, color.a)
}

#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    tile_width: f32,
    tile_height: f32,
    circle_radius: f32,
    circle_positions: Vec<Vec2>,
    circle_colors: Vec<Color>,
    child_circles: Vec<Vec<ChildCircle>>,
    tile_bg_colors: Vec<Color>,
    palette: PalettesSelection,
    bg_palette: PalettesSelection,
    show_grid: bool,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);

    // Calculate tile dimensions
    let tile_width = work_size.x / COLS as f32;
    let tile_height = work_size.y / ROWS as f32;

    // Calculate circle radius - use 80% of the smallest tile dimension to allow movement
    let circle_radius = (tile_width.min(tile_height) / 2.0) * 0.8;
    log::info!("Circle radius: {}", circle_radius);

    // Choose a random palette for circles
    let palette: PalettesSelection = rng.gen();
    log::info!("Circle Palette: {:?}", palette);

    // Choose a different palette for backgrounds
    let mut bg_palette: PalettesSelection = rng.gen();
    while format!("{:?}", bg_palette) == format!("{:?}", palette) {
        bg_palette = rng.gen();
    }
    log::info!("Background Palette: {:?}", bg_palette);

    // Generate random positions and colors for each tile
    // Constrain positions so circles stay within tile boundaries
    let mut circle_positions = Vec::new();
    let mut circle_colors = Vec::new();
    let mut child_circles = Vec::new();
    let mut tile_bg_colors = Vec::new();

    for _ in 0..(ROWS * COLS) {
        circle_positions.push(vec2(
            rng.gen_range(circle_radius..(tile_width - circle_radius)),
            rng.gen_range(circle_radius..(tile_height - circle_radius)),
        ));
        let parent_color = colors::Palettes::choose_color(&palette);
        circle_colors.push(parent_color);

        // Generate background color for this tile
        let bg_color = colors::Palettes::choose_color(&bg_palette);
        tile_bg_colors.push(bg_color);

        // Generate child circles for this parent
        let num_children = rng.gen_range(0..=MAX_CHILD_CIRCLES);
        let mut children = Vec::new();
        for _ in 0..num_children {
            let angle = rng.gen_range(0.0..(2.0 * PI));
            let child_radius = rng.gen_range(
                (circle_radius * CHILD_RADIUS_MOD_MIN)..(circle_radius * CHILD_RADIUS_MOD_MAX),
            );
            let child_color = vary_color(parent_color, &mut rng);
            children.push(ChildCircle {
                angle,
                radius: child_radius,
                color: child_color,
            });
        }
        child_circles.push(children);
    }

    State {
        rng,
        work_size,
        tile_width,
        tile_height,
        circle_radius,
        circle_positions,
        circle_colors,
        child_circles,
        tile_bg_colors,
        palette,
        bg_palette,
        show_grid: false,
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        // Reseed the RNG with a new random seed
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Choose a new random palette for circles
        state.palette = state.rng.gen();
        log::info!("Circle Palette: {:?}", state.palette);

        // Choose a different palette for backgrounds
        state.bg_palette = state.rng.gen();
        while format!("{:?}", state.bg_palette) == format!("{:?}", state.palette) {
            state.bg_palette = state.rng.gen();
        }
        log::info!("Background Palette: {:?}", state.bg_palette);

        // Generate new random positions and colors for each tile
        // Constrain positions so circles stay within tile boundaries
        state.circle_positions.clear();
        state.circle_colors.clear();
        state.child_circles.clear();
        state.tile_bg_colors.clear();
        for _ in 0..(ROWS * COLS) {
            state.circle_positions.push(vec2(
                state
                    .rng
                    .gen_range(state.circle_radius..(state.tile_width - state.circle_radius)),
                state
                    .rng
                    .gen_range(state.circle_radius..(state.tile_height - state.circle_radius)),
            ));
            let parent_color = colors::Palettes::choose_color(&state.palette);
            state.circle_colors.push(parent_color);

            // Generate background color for this tile
            let bg_color = colors::Palettes::choose_color(&state.bg_palette);
            state.tile_bg_colors.push(bg_color);

            // Generate child circles for this parent
            let num_children = state.rng.gen_range(0..=MAX_CHILD_CIRCLES);
            let mut children = Vec::new();
            for _ in 0..num_children {
                let angle = state.rng.gen_range(0.0..(2.0 * PI));
                let child_radius = state
                    .rng
                    .gen_range((state.circle_radius / 8.0)..(state.circle_radius / 3.0));
                let child_color = vary_color(parent_color, &mut state.rng);
                children.push(ChildCircle {
                    angle,
                    radius: child_radius,
                    color: child_color,
                });
            }
            state.child_circles.push(children);
        }
    }

    if app.keyboard.was_pressed(KeyCode::G) {
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
            // let win_config = get_common_win_config().high_dpi(true).size(
            // ScreenDimensions::RES_4KISH.x as u32,
            // ScreenDimensions::RES_4KISH.y as u32,
            // ScreenDimensions::RES_HDPLUS.x as i32,
            // ScreenDimensions::RES_HDPLUS.y as i32,
            ScreenDimensions::RES_1080P.x as u32,
            ScreenDimensions::RES_1080P.y as u32,
            // ScreenDimensions::DEFAULT.x as i32,
            // ScreenDimensions::DEFAULT.y as i32,
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

fn draw(gfx: &mut Graphics, state: &mut State) {
    // Set up draw with scaling projection (aspect_fit = false)
    let mut draw = get_draw_setup(gfx, state.work_size, false, Color::WHITE);

    // Draw grid if enabled
    if state.show_grid {
        // Draw vertical lines
        for col in 0..=COLS {
            let x = col as f32 * state.tile_width;
            draw.path()
                .move_to(x, 0.0)
                .line_to(x, state.work_size.y)
                .stroke_color(Color::GREEN)
                .stroke(GRID_STROKE);
        }

        // Draw horizontal lines
        for row in 0..=ROWS {
            let y = row as f32 * state.tile_height;
            draw.path()
                .move_to(0.0, y)
                .line_to(state.work_size.x, y)
                .stroke_color(Color::GREEN)
                .stroke(GRID_STROKE);
        }
    }


    // Draw tiled circles
    let mut tile_index = 0;
    for row in 0..ROWS {
        for col in 0..COLS {
            // Calculate the tile's top-left corner
            let tile_x = col as f32 * state.tile_width;
            let tile_y = row as f32 * state.tile_height;

            // Draw tile background
            let bg_color = state.tile_bg_colors[tile_index];
            draw.rect((tile_x, tile_y), (state.tile_width, state.tile_height))
                .color(bg_color);

            // Get the position and color for this specific tile
            let circle_pos = state.circle_positions[tile_index];
            let circle_color = state.circle_colors[tile_index];
            let children = &state.child_circles[tile_index];

            // Calculate absolute position
            let abs_x = tile_x + circle_pos.x;
            let abs_y = tile_y + circle_pos.y;

            // Draw the parent circle
            draw.circle(state.circle_radius)
                .position(abs_x, abs_y)
                .color(circle_color);

            // Draw child circles on the parent's circumference
            for child in children {
                let child_x = abs_x + state.circle_radius * child.angle.cos();
                let child_y = abs_y + state.circle_radius * child.angle.sin();
                draw.circle(child.radius)
                    .position(child_x, child_y)
                    .color(child.color);
            }

            tile_index += 1;
        }
    }


    // Render to screen
    gfx.render(&draw);
}
