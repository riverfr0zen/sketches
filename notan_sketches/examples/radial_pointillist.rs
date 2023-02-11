use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, CapturingTexture, ScreenDimensions,
};
use std::mem::size_of_val;
use std::ops::RangeInclusive;
use uuid::Uuid;

// const DEFAULT_WORK_SIZE: Vec2 = ScreenDimensions::DEFAULT;
// const DEFAULT_WORK_SIZE: Vec2 = ScreenDimensions::RES_1080P;
const DEFAULT_WORK_SIZE: Vec2 = ScreenDimensions::RES_5K;
const UPDATE_STEP: f32 = 0.0;
// const UPDATE_STEP: f32 = 0.001;
// const UPDATE_STEP: f32 = 0.5;
// const UPDATE_STEP: f32 = 1.0;
const SPAWN_ANGLE_STEP: RangeInclusive<f32> = 1.0..=45.0;
const SPAWN2_ANGLE_STEP: RangeInclusive<f32> = 1.0..=45.0;
// The frequency of the wave that determines the distance of the Spawn2's position
// from its parent
const SPAWN2_WAVE_FREQ: RangeInclusive<f32> = 3.0..=30.0;
// How many nodes are cleared during node size management
const NODES_ROTATED: usize = 1024;
// Max memory used for nodes
// 10 KB: 10240
// 100 KB: 102400
// 500KB: 512000
// 1 MB: 1048576
// 10 MB: 10485760
const MAX_NODES_BYTES: u32 = 102400;
// const CIRCLE_TEXTURE_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);
// const CIRCLE_TEXTURE_COLOR: Color = Color::from_rgb(0.7, 0.7, 0.7);
const CIRCLE_TEXTURE_COLOR: Color = Color::WHITE;
const DEFAULT_ALPHA: f32 = 0.5;
const ALPHA_FREQ: RangeInclusive<f32> = 0.001..=5.0;
// Capture interval
// const CAPTURE_INTERVAL: f32 = 10.0;
const CAPTURE_INTERVAL: f32 = 60.0 * 5.0;
const MAX_CAPTURES: u32 = 3;


#[derive(Debug, PartialEq)]
enum SpawnStrategy {
    Random,
    RandomAnyChild,
    RandomChildOfNode,
}

impl SpawnStrategy {
    fn random(rng: &mut Random) -> Self {
        match rng.gen_range(0..10) {
            8 => Self::Random,
            9 => Self::RandomAnyChild,
            _ => Self::RandomChildOfNode,
        }
    }
}


#[derive(Debug)]
pub struct Settings {
    spawn_strategy: SpawnStrategy,
    vary_spawn_distance: bool,
    spawn_angle_step: f32,
    spawn2_angle_step: f32,
    spawn2_wave_freq: f32,
    alpha_freq: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            spawn_strategy: SpawnStrategy::RandomChildOfNode,
            vary_spawn_distance: true,
            spawn_angle_step: 10.0,
            spawn2_angle_step: 1.0,
            spawn2_wave_freq: 20.0,
            alpha_freq: 0.5,
        }
    }
}

impl Settings {
    fn randomize(rng: &mut Random) -> Self {
        let mut vary_spawn_distance = true;
        if rng.gen_range(0..10) > 7 {
            vary_spawn_distance = false;
        }
        Self {
            spawn_strategy: SpawnStrategy::random(rng),
            vary_spawn_distance: vary_spawn_distance,
            spawn_angle_step: rng.gen_range(SPAWN_ANGLE_STEP),
            spawn2_angle_step: rng.gen_range(SPAWN2_ANGLE_STEP),
            spawn2_wave_freq: rng.gen_range(SPAWN2_WAVE_FREQ),
            alpha_freq: rng.gen_range(ALPHA_FREQ),
        }
    }
}


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
    pub spawn_last_angle: f32,
    pub spawn2_last_angle: f32,
    pub alpha: f32,
    pub active: bool,
    pub rendered: bool,
}


impl Node {
    fn is_within_view(&self, work_size: &Vec2) -> bool {
        self.pos.x > 0.0 && self.pos.x < work_size.x && self.pos.y > 0.0 && self.pos.y < work_size.y
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
            spawn_last_angle: 0.0,
            spawn2_last_angle: 0.0,
            alpha: DEFAULT_ALPHA,
            active: false,
            rendered: false,
        }
    }
}


#[derive(AppState)]
pub struct State {
    /// The work_size attr is meant to be set at init() and not changed thereafter.
    pub work_size: Vec2,
    pub rng: Random,
    pub last_update: f32,
    pub capture: CapturingTexture,
    pub circle_texture: Texture,
    pub draw_alpha: f32,
    pub nodes: Vec<Node>,
    pub parent_radius: f32,
    pub spawn_radius: f32,
    pub spawn_max_distance: f32,
    pub settings: Settings,
}


impl State {
    fn get_active_node(&self) -> Option<usize> {
        self.nodes.iter().position(|node| node.active == true)
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


fn init_rng_and_capture(gfx: &mut Graphics, work_size: &Vec2) -> (Random, CapturingTexture) {
    let (rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let capture = CapturingTexture::new(
        gfx,
        &work_size,
        Color::WHITE,
        format!("renders/radial_pointillist/{}", seed),
        CAPTURE_INTERVAL,
    );
    (rng, capture)
}


fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let work_size = DEFAULT_WORK_SIZE;

    let (mut rng, capture) = init_rng_and_capture(gfx, &work_size);

    // The texture radius is large because we want large textures that look nice when app is maximized
    let circle_texture = create_circle_texture(gfx, work_size.x * 0.5, CIRCLE_TEXTURE_COLOR);

    // let settings = Settings::default();
    let settings = Settings::randomize(&mut rng);
    log::debug!("With settings: {:#?}", settings);
    State {
        work_size: work_size,
        rng: rng,
        last_update: 0.0,
        capture,
        circle_texture: circle_texture,
        draw_alpha: 0.0,
        nodes: vec![],
        parent_radius: work_size.x * 0.02,
        spawn_radius: work_size.x * 0.01,
        spawn_max_distance: work_size.x * 0.1,
        settings: settings,
    }
}

fn spawn_random(state: &mut State) {
    state.nodes.push(Node {
        class: NodeClass::PARENT,
        alpha: state.draw_alpha,
        pos: vec2(
            state.rng.gen_range(0.0..state.work_size.x),
            state.rng.gen_range(0.0..state.work_size.y),
        ),
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
        .filter(|node| node.spawn_last_angle == 0.0 && node.is_within_view(&state.work_size))
        .collect();
    if candidates.len() > 0 {
        // select random candidate
        let candidate_index = state.rng.gen_range(0..candidates.len());
        let candidate = &mut candidates[candidate_index];
        // set candidate to parent
        candidate.class = NodeClass::PARENT;
        // set candidate to active
        candidate.active = true;
        candidate.rendered = false;
    }
}


/// Converts only available child nodes of the provided parent to an active parent
fn spawn_random_node_child(state: &mut State, parent: Node) {
    let mut candidates: Vec<&mut Node> = state
        .nodes
        .iter_mut()
        .filter(|node| {
            parent.id == node.parent_id
                && node.spawn_last_angle == 0.0
                && node.is_within_view(&state.work_size)
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
        candidate.rendered = false;
    }
}


fn update(app: &mut App, state: &mut State) {
    let curr_time = app.timer.time_since_init();

    state.draw_alpha = (curr_time * state.settings.alpha_freq).sin().abs();

    if curr_time - state.last_update > UPDATE_STEP {
        if let Some(active_node) = state.get_active_node() {
            let nodes = &mut state.nodes;

            let min_distance = state.parent_radius * 1.5;
            let distance: f32;
            if state.settings.vary_spawn_distance {
                distance = state.rng.gen_range(min_distance..state.spawn_max_distance);
            } else {
                distance = state.spawn_max_distance;
            }
            // Need this offset so that spawn are positioned from the center of
            // parent node (because of how texture image positioning works)
            let spawn_offset = state.parent_radius * 0.5;
            let parent_id = nodes[active_node].id.clone();

            if nodes[active_node].spawn_last_angle < 360.0
                || nodes[active_node].spawn2_last_angle < 360.0
            {
                if nodes[active_node].spawn_last_angle < 360.0 {
                    nodes[active_node].spawn_last_angle += state.settings.spawn_angle_step;
                    // log::debug!("angle: {}", node.last_angle);
                    let spawn_x = nodes[active_node].pos.x
                        + spawn_offset
                        + nodes[active_node].spawn_last_angle.to_radians().cos() * distance;
                    let spawn_y = nodes[active_node].pos.y
                        + spawn_offset
                        + nodes[active_node].spawn_last_angle.to_radians().sin() * distance;
                    nodes.push(Node {
                        parent_id: parent_id,
                        pos: vec2(spawn_x, spawn_y),
                        alpha: state.draw_alpha,
                        ..Default::default()
                    });
                }

                if nodes[active_node].spawn2_last_angle < 360.0 {
                    nodes[active_node].spawn2_last_angle += state.settings.spawn2_angle_step;
                    // Spawn2 distance changes as a wave
                    let spawn2_distance = min_distance
                        + ((curr_time * state.settings.spawn2_wave_freq).sin().abs()
                            * state.spawn_max_distance);
                    let spawn2_x = nodes[active_node].pos.x
                        + spawn_offset
                        + nodes[active_node].spawn2_last_angle.to_radians().cos() * spawn2_distance;
                    let spawn2_y = nodes[active_node].pos.y
                        + spawn_offset
                        + nodes[active_node].spawn2_last_angle.to_radians().sin() * spawn2_distance;

                    nodes.push(Node {
                        class: NodeClass::SPAWN2,
                        parent_id: parent_id,
                        pos: vec2(spawn2_x, spawn2_y),
                        alpha: state.draw_alpha,
                        ..Default::default()
                    });
                }
            } else {
                nodes[active_node].active = false;
                if state.settings.spawn_strategy == SpawnStrategy::RandomAnyChild {
                    spawn_random_any_child(state);
                } else if state.settings.spawn_strategy == SpawnStrategy::RandomChildOfNode {
                    let node_clone = nodes[active_node].clone();
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


fn draw_nodes(draw: &mut Draw, state: &mut State) {
    for node in state.nodes.iter_mut().filter(|node| !node.rendered) {
        let texture: &Texture;
        let size: f32;
        let color: Color;
        match node.class {
            NodeClass::PARENT => {
                texture = &state.circle_texture;
                size = state.parent_radius * 2.0;
                color = colors::AEGEAN;
                // color = colors::CERULEAN;
            }
            NodeClass::SPAWN => {
                texture = &state.circle_texture;
                size = state.spawn_radius * 2.0;
                // color = colors::SAFFRON;
                // color = colors::SALMON;
                color = colors::SEAWEED;
            }
            NodeClass::SPAWN2 => {
                texture = &state.circle_texture;
                size = state.spawn_radius * 0.75;
                // color = colors::SEAWEED;
                // color = colors::PICKLE;
                color = colors::SALMON;
                // color = colors::AEGEAN;
            }
        }
        draw.image(&texture)
            .alpha_mode(BlendMode::OVER)
            .alpha(node.alpha)
            // .alpha(alpha_mod)
            .color(color)
            .position(node.pos.x, node.pos.y)
            .size(size, size);
        node.rendered = true;
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let draw = &mut state.capture.render_texture.create_draw();
    draw_nodes(draw, state);
    gfx.render_to(&state.capture.render_texture, draw);
    state.capture.capture(app, gfx);
    if state.capture.num_captures >= MAX_CAPTURES {
        log::debug!("Maximum captures reached. Creating new seed and re-randomizing settings...");
        let (mut rng, capture) = init_rng_and_capture(gfx, &state.work_size);
        state.settings = Settings::randomize(&mut rng);
        log::debug!("With settings: {:#?}", state.settings);
        state.rng = rng;
        state.capture = capture;
        // Manually lock the newly reset capture so that it does not immediately
        // start capturing again.
        state.capture.capture_lock = true;
    }


    let rdraw = &mut get_draw_setup(gfx, state.work_size, false, Color::WHITE);
    rdraw.image(&state.capture.render_texture);

    gfx.render(rdraw);
    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
        // let win_config = get_common_win_config().high_dpi(true).size(
        // ScreenDimensions::RES_4KISH.x as i32,
        // ScreenDimensions::RES_4KISH.y as i32,
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
