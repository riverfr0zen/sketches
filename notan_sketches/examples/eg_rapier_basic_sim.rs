//!
//! This is an adaptation of the Basic Simulation Example from Rapier documentation at:
//! https://rapier.rs/docs/user_guides/rust/getting_started#basic-simulation-example
//!
//! Notan FPS limiting from:
//! https://github.com/Nazariglez/ngt/blob/main/src/game.rs
//!
use notan::draw::*;
use notan::extra::FpsLimit;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};
use rapier2d::prelude::*;

const MAX_FPS: u8 = 60;
const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
// const GROUND_SIZE: Vec2 = vec2(WORK_SIZE.x, WORK_SIZE.y * 0.1);
const GROUND_SIZE: Vec2 = vec2(WORK_SIZE.x * 0.5, WORK_SIZE.y * 0.5);
const GROUND_POS: Vec2 = vec2(WORK_SIZE.x * 0.2, WORK_SIZE.y - GROUND_SIZE.y);
// const BALL_RADIUS: f32 = WORK_SIZE.y * 0.025;
const BALL_RADIUS: f32 = WORK_SIZE.y * 0.05;
// const BALL_START_POS: Vec2 = vec2(WORK_SIZE.x * 0.2, WORK_SIZE.y * 0.1);
const BALL_START_POS: Vec2 = vec2(WORK_SIZE.x * 0.162, WORK_SIZE.y * 0.1);
const BALL2_START_POS: Vec2 = vec2(WORK_SIZE.x * 0.666, WORK_SIZE.y * 0.1);
const GRAVITY: f32 = -9.81;
// NOTE: The physics scaling below became more meaningful when framerate was controlled,
// for e.g. by enabling vsync or by introducing an explicit frame limit. (In this example
// I am doing both).
//
// 1 meter = 50 work size units
const PHYS_SCALE: f32 = 50.0;
// Notice how the effect of gravity reduces when the physics scale is brought down.
// This is because the gravity value remains constant (is not scaled) but the
// objects and distances between them are now greater in terms of meters.
// 1 meter = 1 work size units
// const PHYS_SCALE: f32 = 1.0;
// const PHYS_SCALE: f32 = 0.1;

/// Converts game (WORK_SIZE) units to physics scale (meters)
fn to_phys_scale(gfx_length: f32) -> Real {
    gfx_length / PHYS_SCALE
}


/// For our purposes here this is just an alias for to_phys_scale
fn to_phys_x(gfx_pos: f32) -> Real {
    return to_phys_scale(gfx_pos);
}


/// Compensates for coordinate system differences (from top-bottom to bottom-top)
fn to_phys_y(gfx_pos: f32) -> Real {
    return to_phys_scale(WORK_SIZE.y - gfx_pos);
}


fn to_gfx_scale(physics_length: Real) -> f32 {
    return physics_length * PHYS_SCALE;
}


fn to_gfx_x(physics_pos: Real) -> f32 {
    return to_gfx_scale(physics_pos);
}


/// Compensates for coordinate system differences (from bottom-top to top-bottom)
fn to_gfx_y(physics_pos: Real) -> f32 {
    return WORK_SIZE.y - to_gfx_scale(physics_pos);
}


#[derive(Debug, Clone, Copy)]
enum SimulationMode {
    PAUSED,
    RUNNING,
    STEP,
}


#[derive(AppState)]
struct State {
    sim_mode: SimulationMode,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    ball_body_handle: RigidBodyHandle,
    ball2_body_handle: RigidBodyHandle,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl Default for State {
    fn default() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(
            to_phys_scale(GROUND_SIZE.x) * 0.5,
            to_phys_scale(GROUND_SIZE.y) * 0.5,
        )
        // Have to translate the object like this because of differences in origin
        // between render objects and physics objects.
        //
        // @TODO: Move these compensations to the phys_x/y fns. Will need to pass in
        // object's size (half extents). Also note that right now we aren't doing the
        // complementary adjustments on the gfx side (in to_gfx_x/y). It seems to look okay,
        // but that's probably because the off values are negligible.
        .translation(vector![
            to_phys_x(GROUND_POS.x + (GROUND_SIZE.x * 0.5)),
            to_phys_y(GROUND_POS.y + GROUND_SIZE.y * 0.5)
        ])
        .build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![
                to_phys_x(BALL_START_POS.x + BALL_RADIUS),
                to_phys_y(BALL_START_POS.y - BALL_RADIUS)
            ])
            .build();
        let collider = ColliderBuilder::ball(to_phys_scale(BALL_RADIUS))
            .restitution(0.7)
            .build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);


        /* Create second bouncing ball. */
        let rigid_body2 = RigidBodyBuilder::dynamic()
            .translation(vector![
                to_phys_x(BALL2_START_POS.x + BALL_RADIUS),
                to_phys_y(BALL2_START_POS.y - BALL_RADIUS)
            ])
            .build();
        let collider2 = ColliderBuilder::ball(to_phys_scale(BALL_RADIUS))
            .restitution(0.7)
            .build();
        let ball2_body_handle = rigid_body_set.insert(rigid_body2);
        collider_set.insert_with_parent(collider2, ball2_body_handle, &mut rigid_body_set);


        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, GRAVITY];
        let integration_parameters = IntegrationParameters::default();
        // NOTE: Normally many of the items below would be declared as mutable (see original example)
        // but here they don't have to be because this is a simple example.
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        Self {
            sim_mode: SimulationMode::RUNNING,
            rigid_body_set: rigid_body_set,
            collider_set: collider_set,
            ball_body_handle: ball_body_handle,
            ball2_body_handle: ball2_body_handle,
            gravity: gravity,
            integration_parameters: integration_parameters,
            physics_pipeline: physics_pipeline,
            island_manager: island_manager,
            broad_phase: broad_phase,
            narrow_phase: narrow_phase,
            impulse_joint_set: impulse_joint_set,
            multibody_joint_set: multibody_joint_set,
            ccd_solver: ccd_solver,
            physics_hooks: physics_hooks,
            event_handler: event_handler,
        }
    }
}


// fn init(gfx: &mut Graphics) -> State {
fn init() -> State {
    log::info!("Press \'x\' to reset");
    log::info!("Press \'p\' to pause");
    log::info!("Press \'s\' to step");
    log::info!("Press \'r\' to run (default)");

    let state = State::default();
    state
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::X) {
        log::debug!("State reset");
        // Reset everything except sim_mode
        let prev_mode = state.sim_mode;
        *state = State::default();
        state.sim_mode = prev_mode;
    }

    if app.keyboard.was_pressed(KeyCode::P) {
        log::debug!("Simulation paused");
        state.sim_mode = SimulationMode::PAUSED;
    }

    if app.keyboard.was_pressed(KeyCode::R) {
        log::debug!("Simulation running");
        state.sim_mode = SimulationMode::RUNNING;
    }

    if app.keyboard.was_pressed(KeyCode::S) {
        log::debug!("Simulation stepping. Press 's' for next step.");
        state.sim_mode = SimulationMode::STEP;
    }

    match state.sim_mode {
        SimulationMode::RUNNING | SimulationMode::STEP => {
            state.physics_pipeline.step(
                &state.gravity,
                &state.integration_parameters,
                &mut state.island_manager,
                &mut state.broad_phase,
                &mut state.narrow_phase,
                &mut state.rigid_body_set,
                &mut state.collider_set,
                &mut state.impulse_joint_set,
                &mut state.multibody_joint_set,
                &mut state.ccd_solver,
                &state.physics_hooks,
                &state.event_handler,
            );
            if let SimulationMode::STEP = state.sim_mode {
                state.sim_mode = SimulationMode::PAUSED;
            }
        }
        SimulationMode::PAUSED => (),
    }
}


fn draw(
    //app: &mut App,
    gfx: &mut Graphics,
    state: &mut State,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::OLIVE);


    draw.rect((GROUND_POS.x, GROUND_POS.y), (GROUND_SIZE.x, GROUND_SIZE.y))
        .color(Color::BLUE)
        // .stroke(1.0);
        .fill();


    let ball_body = &state.rigid_body_set[state.ball_body_handle];
    let ball2_body = &state.rigid_body_set[state.ball2_body_handle];
    // log::debug!(
    //     "Ball altitude: {}, to_gfx_y: {}",
    //     ball_body.translation().y,
    //     to_gfx_y(ball_body.translation().y)
    // );
    // log::debug!(
    //     "Ball2 altitude: {}, to_gfx_y: {}",
    //     ball2_body.translation().y,
    //     to_gfx_y(ball2_body.translation().y)
    // );

    draw.circle(BALL_RADIUS)
        // .position(100.0, ball_body.translation().y)
        .position(
            to_gfx_x(ball_body.translation().x),
            to_gfx_y(ball_body.translation().y),
        )
        .color(Color::ORANGE)
        .fill();


    draw.circle(BALL_RADIUS)
        // .position(100.0, ball_body.translation().y)
        .position(
            to_gfx_x(ball2_body.translation().x),
            to_gfx_y(ball2_body.translation().y),
        )
        .color(Color::ORANGE)
        .fill();

    // draw to screen
    gfx.render(&draw);
}


#[notan_main]
fn main() -> Result<(), String> {
    // Trying to call .vsync(true) in Wayland results in a crash
    // probably because Wayland already has vsync on.
    //
    // Looks like the issue happens here:
    // https://github.com/rust-windowing/glutin/issues/1444
    //
    // I've created an issue for Notan at:
    // https://github.com/Nazariglez/notan/issues/187
    //
    // let win_config = get_common_win_config().vsync(true).high_dpi(true);
    let win_config = get_common_win_config().high_dpi(true);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .add_plugin(FpsLimit::new(MAX_FPS))
        .draw(draw)
        .update(update)
        .build()
}


// Original example code
//
// fn main() {
//     let mut rigid_body_set = RigidBodySet::new();
//     let mut collider_set = ColliderSet::new();

//     /* Create the ground. */
//     let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
//     collider_set.insert(collider);

//     /* Create the bouncing ball. */
//     let rigid_body = RigidBodyBuilder::dynamic()
//         .translation(vector![0.0, 10.0])
//         .build();
//     let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
//     let ball_body_handle = rigid_body_set.insert(rigid_body);
//     collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

//     /* Create other structures necessary for the simulation. */
//     let gravity = vector![0.0, -9.81];
//     let integration_parameters = IntegrationParameters::default();
//     let mut physics_pipeline = PhysicsPipeline::new();
//     let mut island_manager = IslandManager::new();
//     let mut broad_phase = BroadPhase::new();
//     let mut narrow_phase = NarrowPhase::new();
//     let mut impulse_joint_set = ImpulseJointSet::new();
//     let mut multibody_joint_set = MultibodyJointSet::new();
//     let mut ccd_solver = CCDSolver::new();
//     let physics_hooks = ();
//     let event_handler = ();

//     /* Run the game loop, stepping the simulation once per frame. */
//     for _ in 0..2000 {
//         physics_pipeline.step(
//             &gravity,
//             &integration_parameters,
//             &mut island_manager,
//             &mut broad_phase,
//             &mut narrow_phase,
//             &mut rigid_body_set,
//             &mut collider_set,
//             &mut impulse_joint_set,
//             &mut multibody_joint_set,
//             &mut ccd_solver,
//             &physics_hooks,
//             &event_handler,
//         );

//         let ball_body = &rigid_body_set[ball_body_handle];
//         println!("Ball altitude: {}", ball_body.translation().y);
//     }
// }
