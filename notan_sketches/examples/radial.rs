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
    pub is_parent: bool,
}


#[derive(AppState)]
pub struct State {
    pub rng: Random,
    pub last_update: f32,
    pub parent_texture: Texture,
    pub spawn_texture: Texture,
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


fn create_circle_texture(gfx: &mut Graphics, radius: f32, color: Color) -> Texture {
    let rt = gfx
        .create_render_texture((radius * 2.0) as i32, (radius * 2.0) as i32)
        .build()
        .unwrap();
    let mut draw = gfx.create_draw();
    draw.set_size(radius * 2.0, radius * 2.0);
    draw.circle(radius)
        .position(radius, radius)
        .fill_color(color)
        .fill();
    gfx.render_to(&rt, &draw);
    rt.take_inner()
}

fn init(gfx: &mut Graphics) -> State {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    // The texture radiuses are large because we want large textures that look nice when app is maximized
    let parent_texture = create_circle_texture(gfx, WORK_SIZE.x * 0.5, Color::NAVY);
    let spawn_texture = create_circle_texture(gfx, WORK_SIZE.x * 0.5, Color::ORANGE);

    State {
        rng: rng,
        last_update: 0.0,
        parent_texture: parent_texture,
        spawn_texture: spawn_texture,
        nodes: vec![],
        parent_radius: WORK_SIZE.x * 0.02,
        spawn_radius: WORK_SIZE.x * 0.01,
        spawn_max_distance: WORK_SIZE.x * 0.1,
    }
}


fn update(app: &mut App, state: &mut State) {
    let curr_time = app.timer.time_since_init();
    // if curr_time - state.last_update > 0.001 {
    if curr_time - state.last_update > 1.0 {
        log::debug!("=o=");

        let min_distance = state.parent_radius * 1.5;
        // let distance = state.spawn_max_distance;
        let distance = state.rng.gen_range(min_distance..state.spawn_max_distance);
        // Need this offset so that spawn are positioned from the center of
        // parent node (because of how texture image positioning works)
        let spawn_offset = state.parent_radius * 0.5;

        if let Some(node) = state.get_active_node() {
            if node.last_angle < 360.0 {
                node.last_angle += 30.0;
                log::debug!("angle: {}", node.last_angle);

                let spawn_x =
                    node.pos.x + spawn_offset + node.last_angle.to_radians().cos() * distance;
                let spawn_y =
                    node.pos.y + spawn_offset + node.last_angle.to_radians().sin() * distance;
                state.nodes.push(Node {
                    pos: vec2(spawn_x, spawn_y),
                    last_angle: 0.0,
                    active: false,
                    is_parent: false,
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
                is_parent: true,
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
        let texture: &Texture;
        let size: f32;
        if node.is_parent {
            texture = &state.parent_texture;
            size = state.parent_radius * 2.0;
        } else {
            texture = &state.spawn_texture;
            size = state.spawn_radius * 2.0;
        }
        draw.image(&texture)
            .position(node.pos.x, node.pos.y)
            .size(size, size);
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
