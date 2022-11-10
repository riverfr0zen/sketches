///
/// Based on Rapier documentation at:
/// https://rapier.rs/docs/user_guides/rust/getting_started
///
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup};
use rapier2d::prelude::*;

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);
const GROUND_SIZE: Vec2 = vec2(WORK_SIZE.x, WORK_SIZE.y * 0.1);

#[derive(AppState)]
struct State {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    ball_body_handle: RigidBodyHandle,
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
        let collider = ColliderBuilder::cuboid(GROUND_SIZE.x * 0.5, GROUND_SIZE.y * 0.5).build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);


        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        Self {
            rigid_body_set: rigid_body_set,
            collider_set: collider_set,
            ball_body_handle: ball_body_handle,
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


fn init(gfx: &mut Graphics) -> State {
    let state = State::default();
    state
}


fn update(state: &mut State) {
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

    let ball_body = &state.rigid_body_set[state.ball_body_handle];
    println!("Ball altitude: {}", ball_body.translation().y);
}


fn draw(
    // app: &mut App,
    gfx: &mut Graphics,
    state: &mut State,
) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, false, Color::OLIVE);


    draw.rect(
        (0.0, WORK_SIZE.y - GROUND_SIZE.y),
        (GROUND_SIZE.x, GROUND_SIZE.y),
    )
    .color(Color::BLUE)
    // .stroke(1.0);
    .fill();

    // draw to screen
    gfx.render(&draw);

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = get_common_win_config();

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .update(update)
        .build()
}


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
