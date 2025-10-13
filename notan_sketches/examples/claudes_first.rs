use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);

#[derive(AppState)]
struct State {
    rng: Random,
    circle_pos: Vec2,
}

fn init(_gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    // Generate random position within the work size
    let circle_pos = vec2(
        rng.gen_range(0.0..WORK_SIZE.x),
        rng.gen_range(0.0..WORK_SIZE.y),
    );

    State { rng, circle_pos }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::R) {
        // Reseed the RNG with a new random seed
        let new_seed = state.rng.gen();
        state.rng.reseed(new_seed);
        log::info!("New seed: {}", new_seed);

        // Generate new random position with reseeded RNG
        state.circle_pos = vec2(
            state.rng.gen_range(0.0..WORK_SIZE.x),
            state.rng.gen_range(0.0..WORK_SIZE.y),
        );
        log::debug!("New position: {:?}", state.circle_pos);
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

    // Draw blue circle at random position
    draw.circle(100.0)
        .position(state.circle_pos.x, state.circle_pos.y)
        .color(Color::BLUE);

    // Render to screen
    gfx.render(&draw);
}
