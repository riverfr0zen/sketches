use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng, ScreenDimensions};


// const WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;


pub struct Node {
    pub pos: Vec2,
    pub last_angle: f32,
    pub active: bool,
    pub is_central: bool,
}


#[derive(AppState)]
pub struct State {
    pub rng: Random,
    pub last_update: f32,
    pub nodes: Vec<Node>,
    pub parent_radius: f32,
    pub spawn_radius: f32,
    pub spawn_max_distance: f32,
}

impl State {
    fn get_active_node(&mut self) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.active == true)
    }
}


fn init() -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);
    State {
        rng: rng,
        last_update: 0.0,
        nodes: vec![],
        parent_radius: WORK_SIZE.x * 0.02,
        spawn_radius: WORK_SIZE.y * 0.01,
        spawn_max_distance: WORK_SIZE.x * 0.1,
    }
}


fn update(app: &mut App, state: &mut State) {
    let curr_time = app.timer.time_since_init();
    // if curr_time - state.last_update > 0.05 {
    if curr_time - state.last_update > 1.0 {
        log::debug!("=o=");

        let min_distance = state.parent_radius * 1.5;
        let distance = state.rng.gen_range(min_distance..state.spawn_max_distance);
        if let Some(node) = state.get_active_node() {
            if node.last_angle < 360.0 {
                node.last_angle += 30.0;
                log::debug!("angle: {}", node.last_angle);

                let spawn_x = node.pos.x + node.last_angle.to_radians().cos() * distance;
                let spawn_y = node.pos.y + node.last_angle.to_radians().sin() * distance;
                state.nodes.push(Node {
                    pos: vec2(spawn_x, spawn_y),
                    last_angle: 0.0,
                    active: false,
                    is_central: false,
                });
            } else {
                node.active = false;
            }
        } else {
            state.nodes.push(Node {
                pos: vec2(
                    state.rng.gen_range(0.0..WORK_SIZE.x),
                    state.rng.gen_range(0.0..WORK_SIZE.y),
                ),
                last_angle: 0.0,
                active: true,
                is_central: true,
            });
        }


        state.last_update = curr_time;
    }
}


fn draw(
    gfx: &mut Graphics,
    state: &mut State,
    // app: &mut App,
) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    for node in state.nodes.iter() {
        let radius: f32;
        let fill_color: Color;
        if node.is_central {
            radius = state.parent_radius;
            fill_color = Color::GREEN;
        } else {
            radius = state.spawn_radius;
            fill_color = Color::ORANGE;
        }
        draw.circle(radius)
            .position(node.pos.x, node.pos.y)
            .fill_color(fill_color)
            .fill();
    }


    gfx.render(draw);
    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
        // let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_HDPLUS.x as i32,
        // ScreenDimensions::RES_HDPLUS.y as i32,
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);


    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}
