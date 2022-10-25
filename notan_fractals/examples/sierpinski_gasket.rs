use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::{get_common_win_config, get_draw_setup};

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const WORK_SIZE: Vec2 = vec2(1920.0, 1080.0);


#[derive(AppState)]
struct State {
    max_depth: usize,
}


impl Default for State {
    fn default() -> Self {
        State { max_depth: 0 }
    }
}


fn update(app: &mut App, state: &mut State) {
    // if app.keyboard.is_down(KeyCode::W) {
    //     state.y -= MOVE_SPEED * app.timer.delta_f32();
    // }
    if app.keyboard.was_pressed(KeyCode::Up) {
        state.max_depth += 1;
        log::debug!("state.max_depth increased: {}", state.max_depth);
    }

    if app.keyboard.was_pressed(KeyCode::Down) && state.max_depth > 0 {
        state.max_depth -= 1;
        log::debug!("state.max_depth decreased: {}", state.max_depth);
    }

    if app.keyboard.was_pressed(KeyCode::R) {
        state.max_depth = State::default().max_depth;
        log::debug!("state.max_depth reset: {}", state.max_depth);
    }
}


fn get_vec2_midpoint(vec_a: Vec2, vec_b: Vec2) -> Vec2 {
    vec2((vec_a.x + vec_b.x) / 2.0, (vec_a.y + vec_b.y) / 2.0)
}


fn draw_gasket(draw: &mut Draw, state: &mut State, a: Vec2, b: Vec2, c: Vec2, curr_depth: usize) {
    if state.max_depth == 0 {
        draw.triangle((a.x, a.y), (b.x, b.y), (c.x, c.y))
            .color(Color::PURPLE)
            .fill();
    } else {
        let mid_ab: Vec2 = get_vec2_midpoint(a, b);
        let mid_ac: Vec2 = get_vec2_midpoint(a, c);

        let a1 = vec2(a.x, a.y);
        let b1 = vec2(mid_ab.x, mid_ab.y);
        let c1 = vec2(mid_ac.x, mid_ac.y);

        let a2 = vec2(mid_ab.x, mid_ab.y);
        let b2 = vec2(b.x, b.y);
        let c2 = vec2(a.x, b.y);

        let a3 = vec2(mid_ac.x, mid_ac.y);
        let b3 = vec2(c.x, c.y);
        let c3 = vec2(a.x, c.y);
        if curr_depth + 1 == state.max_depth {
            draw.triangle((a1.x, a1.y), (b1.x, b1.y), (c1.x, c1.y))
                .color(Color::PURPLE)
                .fill();

            draw.triangle((a2.x, a2.y), (b2.x, b2.y), (c2.x, c2.y))
                .color(Color::RED)
                .fill();

            draw.triangle((a3.x, a3.y), (b3.x, b3.y), (c3.x, c3.y))
                .color(Color::ORANGE)
                .fill();
        } else {
            draw_gasket(draw, state, a1, b1, c1, curr_depth + 1);
            draw_gasket(draw, state, a2, b2, c2, curr_depth + 1);
            draw_gasket(draw, state, a3, b3, c3, curr_depth + 1);
        }
    }
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    let a = vec2(WORK_SIZE.x / 2.0, 0.0);
    let b = vec2(WORK_SIZE.x, WORK_SIZE.y);
    let c = vec2(0.0, WORK_SIZE.y);
    draw_gasket(&mut draw, state, a, b, c, 0);

    // draw to screen
    gfx.render(&draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    // notan::init()
    // notan::init_with(init)
    notan::init_with(State::default)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}
