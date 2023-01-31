use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors::{AEGEAN, SAFFRON};
use notan_sketches::utils::{get_common_win_config, get_draw_setup, get_rng, ScreenDimensions};
use std::mem::size_of_val;
use uuid::Uuid;

// const WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
const WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
// const UPDATE_STEP: f32 = 0.0;
const UPDATE_STEP: f32 = 0.001;
// const UPDATE_STEP: f32 = 0.5;
// const UPDATE_STEP: f32 = 1.0;
const SPAWN_ANGLE_STEP: f32 = 30.0;
// const SPAWN_STRATEGY: &str = "random";
const SPAWN_STRATEGY: &str = "random any child";
// const SPAWN_STRATEGY: &str = "random child of node";
// const RANDOMIZE_SPAWN_DISTANCE: bool = false;
const RANDOMIZE_SPAWN_DISTANCE: bool = true;
// How many nodes are cleared during node size management
const NODES_ROTATED: usize = 100;
// Max memory used for nodes
// 10 KB: 10240
// 1 MB: 1048576
// 10 MB: 10485760
const MAX_NODES_BYTES: u32 = 10485760;
const DEFAULT_ALPHA: f32 = 0.5;


#[derive(Clone, PartialEq)]
pub enum NodeClass {
    PARENT,
    SPAWN,
    SPAWN2,
}


#[derive(Clone)]
pub struct Node {
    pub id: Uuid,
    pub class: NodeClass,
    pub parent_id: Uuid,
    pub pos: Vec2,
    pub last_angle: f32,
    pub active: bool,
    pub alpha: f32,
}


impl Node {
    fn is_within_view(&self) -> bool {
        self.pos.x > 0.0 && self.pos.x < WORK_SIZE.x && self.pos.y > 0.0 && self.pos.y < WORK_SIZE.y
    }

    fn is_parent(&self) -> bool {
        return self.class == NodeClass::PARENT;
    }
}


impl Default for Node {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            class: NodeClass::SPAWN,
            parent_id: Uuid::nil(),
            pos: vec2(0.0, 0.0),
            last_angle: 0.0,
            active: false,
            alpha: DEFAULT_ALPHA,
        }
    }
}


#[derive(AppState)]
pub struct State {
    pub rng: Random,
    pub last_update: f32,
    pub circle_texture: Texture,
    pub nodes: Vec<Node>,
    pub parent_radius: f32,
    pub spawn_radius: f32,
    pub spawn_max_distance: f32,
}

impl State {
    fn get_active_node(&mut self) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.active == true)
    }

    fn manage_node_size(&mut self) {
        // How to get size in bytes of a slice: https://stackoverflow.com/a/62614320
        let nodes_size = size_of_val(&*self.nodes);
        if nodes_size > MAX_NODES_BYTES as usize {
            log::debug!(
                "nodes limit reached at {}: {} bytes",
                self.last_update,
                nodes_size
            );
            // Using `saturating_sub` to handle the case where NODES_ROTATED
            // is greater than the number of elements in the vector
            // See https://stackoverflow.com/a/28952552
            let rotate_length = self.nodes.len().saturating_sub(NODES_ROTATED);
            self.nodes.rotate_left(rotate_length);
            self.nodes.truncate(rotate_length);
        }
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

    // The texture radius is large because we want large textures that look nice when app is maximized
    // let circle_texture =
    //     create_circle_texture(gfx, WORK_SIZE.x * 0.5, Color::from_rgb(0.5, 0.5, 0.5));
    let circle_texture = create_circle_texture(gfx, WORK_SIZE.x * 0.5, Color::WHITE);

    State {
        rng: rng,
        last_update: 0.0,
        circle_texture: circle_texture,
        nodes: vec![],
        parent_radius: WORK_SIZE.x * 0.02,
        spawn_radius: WORK_SIZE.x * 0.01,
        spawn_max_distance: WORK_SIZE.x * 0.1,
    }
}

fn spawn_random(state: &mut State) {
    state.nodes.push(Node {
        class: NodeClass::PARENT,
        pos: vec2(
            state.rng.gen_range(0.0..WORK_SIZE.x),
            state.rng.gen_range(0.0..WORK_SIZE.y),
        ),
        // last_angle: 0.0,
        active: true,
        ..Default::default()
    });
}


/// Converts any available child node to an active parent
fn spawn_random_any_child(state: &mut State) {
    // get spawn candidates (angle == 0)
    let mut candidates: Vec<&mut Node> = state
        .nodes
        .iter_mut()
        .filter(|node| node.last_angle == 0.0 && node.is_within_view())
        .collect();
    if candidates.len() > 0 {
        // select random candidate
        let candidate_index = state.rng.gen_range(0..candidates.len());
        let candidate = &mut candidates[candidate_index];
        // set candidate to parent
        candidate.class = NodeClass::PARENT;
        // set candidate to active
        candidate.active = true;
    }
}


/// Converts only available child nodes of the provided parent to an active parent
fn spawn_random_node_child(state: &mut State, parent: Node) {
    let mut candidates: Vec<&mut Node> = state
        .nodes
        .iter_mut()
        .filter(|node| {
            parent.id == node.parent_id && node.last_angle == 0.0 && node.is_within_view()
        })
        .collect();
    if candidates.len() > 0 {
        // select random candidate
        let candidate_index = state.rng.gen_range(0..candidates.len());
        let candidate = &mut candidates[candidate_index];
        // set candidate to parent
        candidate.class = NodeClass::PARENT;
        // set candidate to active
        candidate.active = true;
    }
}


fn update(app: &mut App, state: &mut State) {
    let curr_time = app.timer.time_since_init();
    if curr_time - state.last_update > UPDATE_STEP {
        let min_distance = state.parent_radius * 1.5;
        let distance: f32;
        if RANDOMIZE_SPAWN_DISTANCE {
            distance = state.rng.gen_range(min_distance..state.spawn_max_distance);
        } else {
            distance = state.spawn_max_distance;
        }
        // Need this offset so that spawn are positioned from the center of
        // parent node (because of how texture image positioning works)
        let spawn_offset = state.parent_radius * 0.5;

        if let Some(node) = state.get_active_node() {
            if node.last_angle < 360.0 {
                node.last_angle += SPAWN_ANGLE_STEP;
                // log::debug!("angle: {}", node.last_angle);
                let spawn_x =
                    node.pos.x + spawn_offset + node.last_angle.to_radians().cos() * distance;
                let spawn_y =
                    node.pos.y + spawn_offset + node.last_angle.to_radians().sin() * distance;
                let parent_id = node.id.clone();
                state.nodes.push(Node {
                    parent_id: parent_id,
                    pos: vec2(spawn_x, spawn_y),
                    // last_angle: 0.0,
                    // active: false,
                    // is_parent: false,
                    ..Default::default()
                });
            } else {
                node.active = false;
                if SPAWN_STRATEGY == "random any child" {
                    spawn_random_any_child(state);
                } else if SPAWN_STRATEGY == "random child of node" {
                    let node_clone = node.clone();
                    spawn_random_node_child(state, node_clone);
                }
            }
        } else {
            spawn_random(state);
        }
        state.last_update = curr_time;
        state.manage_node_size();
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut get_draw_setup(gfx, WORK_SIZE, false, Color::WHITE);

    for node in state.nodes.iter() {
        let texture: &Texture;
        let size: f32;
        let color: Color;
        if node.is_parent() {
            texture = &state.circle_texture;
            size = state.parent_radius * 2.0;
            color = AEGEAN;
        } else {
            texture = &state.circle_texture;
            size = state.spawn_radius * 2.0;
            color = SAFFRON;
        }
        draw.image(&texture)
            // .alpha_mode(BlendMode::OVER)
            .alpha(node.alpha)
            // .alpha(alpha_mod)
            .color(color)
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
