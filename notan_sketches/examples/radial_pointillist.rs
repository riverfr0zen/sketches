use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::colors;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, get_rng, get_work_size_for_screen, set_html_bgcolor,
    CapturingTexture, CommonHelpModal, EventsFocus, ScreenDimensions,
};
use notan_touchy::{TouchGesture, TouchState};
use std::mem::size_of_val;
use std::ops::RangeInclusive;
use uuid::Uuid;

const CLEAR_COLOR: Color = Color::GRAY;
const UPDATE_STEP: f32 = 0.0;
// const UPDATE_STEP: f32 = 0.001;
// const UPDATE_STEP: f32 = 0.5;
// const UPDATE_STEP: f32 = 1.0;
// Capture interval
// const CAPTURE_INTERVAL: f32 = 10.0;
// const CAPTURE_INTERVAL: f32 = 40.0;
const CAPTURE_INTERVAL: f32 = 60.0;
// const CAPTURE_INTERVAL: f32 = 60.0 * 15.0;
// const CAPTURE_INTERVAL: f32 = 60.0 * 5.0;
const MAX_CAPTURES: u32 = 3;

// const RADIAL_CHANGE_INTERVAL: RangeInclusive<f32> = 5.0..=CAPTURE_INTERVAL * MAX_CAPTURES as f32;
const RADIAL_CHANGE_INTERVAL: RangeInclusive<f32> = 5.0..=CAPTURE_INTERVAL;
// const RADIAL_CHANGE_INTERVAL: RangeInclusive<f32> = 5.0..=30.0;
const PARENT_RADIUS: RangeInclusive<f32> = 0.01..=0.1;
const SPAWN_RADIUS: RangeInclusive<f32> = 0.01..=0.075;
const SPAWN2_RADIUS: RangeInclusive<f32> = 0.005..=0.05;
const PARENT_RADIUS_LARGE: RangeInclusive<f32> = 0.01..=0.2;
const SPAWN_RADIUS_LARGE: RangeInclusive<f32> = 0.01..=0.2;
const SPAWN2_RADIUS_LARGE: RangeInclusive<f32> = 0.001..=0.2;
const PARENT_RADIUS_SMALL: RangeInclusive<f32> = 0.001..=0.02;
const SPAWN_RADIUS_SMALL: RangeInclusive<f32> = 0.001..=0.01;
const SPAWN2_RADIUS_SMALL: RangeInclusive<f32> = 0.001..=0.005;

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
const MAX_NODES_BYTES: u32 = 10485760;
// const CIRCLE_TEXTURE_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);
// const CIRCLE_TEXTURE_COLOR: Color = Color::from_rgb(0.7, 0.7, 0.7);
const CIRCLE_TEXTURE_COLOR: Color = Color::WHITE;
const DEFAULT_ALPHA: f32 = 0.5;
// const ALPHA_FREQ: RangeInclusive<f32> = 0.001..=5.0;
const ALPHA_FREQ: RangeInclusive<f32> = 0.001..=1.0;
const PALETTE: [Color; 21] = [
    colors::PEACOCK,
    colors::AEGEAN,
    colors::AZURE,
    colors::CERULEAN,
    colors::STONE,
    colors::OCHRE,
    colors::OLIVE,
    colors::SAFFRON,
    colors::BANANA,
    colors::LAGUNA,
    colors::SACRAMENTO,
    colors::SEAWEED,
    colors::PICKLE,
    colors::LIME,
    colors::EMERALD,
    colors::PICKLE,
    colors::GRAYPURP,
    colors::MAHOGANY,
    colors::CARMINE,
    colors::SCARLET,
    colors::SALMON,
];
const COLOR_CHANGE_CHANCE_RANGE: RangeInclusive<f32> = 0.0..=0.5;
const BRUSH_BASIC: usize = 0;
const BRUSH_CIRCLE: usize = 1;
const BRUSH_EMBOSSED: usize = 2;
const BRUSH_SPLAT: usize = 3;
const BRUSH_SCRATCH: usize = 4;
const BRUSH_THREEPOINT: usize = 5;
const BRUSH_SCRAPE: usize = 6;
const BRUSH_RECTS: usize = 7;
const IS_WASM: bool = cfg!(target_arch = "wasm32");
// SEED can optionally be specified here. If specified, `reinitialize_drawing` won't be called even if MAX_CAPTURES is exceeded.
const SEED: Option<u64> = None;
// const SEED: Option<u64> = Some(13236161089428852814);

#[derive(Debug, PartialEq)]
enum SpawnStrategy {
    Random,
    RandomAnyChild,
    RandomChildOfNode,
}

impl SpawnStrategy {
    fn random(rng: &mut Random) -> Self {
        match rng.gen_range(0..5) {
            3 => Self::Random,
            4 => Self::RandomAnyChild,
            _ => Self::RandomChildOfNode,
        }
    }
}

#[derive(Debug, PartialEq)]
enum RadialRangeStyle {
    Small,
    Medium,
    Large,
    SmallToLarge,
    LargeToSmall,
    SmallLargeSmall,
    LargeSmallLarge,
    MediumLargeMedium,
    LargeMediumLarge,
    SmallMediumSmall,
    MediumSmallMedium,
    MediumSmallSmall,
    SwapParentSpawn,
    SwapParentSpawn2,
    SwapSpawnSpawn2,
    None,
}

impl RadialRangeStyle {
    fn random(rng: &mut Random) -> Self {
        match rng.gen_range(0..=13) {
            13 => Self::SwapSpawnSpawn2,
            12 => Self::SwapParentSpawn2,
            11 => Self::SwapParentSpawn,
            10 => Self::MediumLargeMedium,
            9 => Self::LargeMediumLarge,
            8 => Self::SmallMediumSmall,
            7 => Self::MediumSmallMedium,
            6 => Self::LargeSmallLarge,
            5 => Self::SmallLargeSmall,
            4 => Self::LargeToSmall,
            3 => Self::SmallToLarge,
            2 => Self::Small,
            1 => Self::Large,
            _ => Self::Medium,
        }
    }

    fn random_large(rng: &mut Random) -> Self {
        match rng.gen_range(0..=8) {
            8 => Self::SwapParentSpawn2,
            7 => Self::SwapParentSpawn,
            6 => Self::MediumLargeMedium,
            5 => Self::LargeMediumLarge,
            4 => Self::LargeSmallLarge,
            3 => Self::SmallLargeSmall,
            2 => Self::LargeToSmall,
            1 => Self::SmallToLarge,
            _ => Self::Large,
        }
    }

    fn random_medium(rng: &mut Random) -> Self {
        match rng.gen_range(0..=2) {
            2 => Self::MediumLargeMedium,
            1 => Self::MediumSmallMedium,
            _ => Self::Medium,
        }
    }

    fn random_small(rng: &mut Random) -> Self {
        match rng.gen_range(0..=2) {
            2 => Self::MediumSmallSmall,
            1 => Self::SmallMediumSmall,
            _ => Self::Small,
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    spawn_strategy: SpawnStrategy,
    vary_spawn_distance: bool,
    radial_change_step: f32,
    radial_range_style: RadialRangeStyle,
    parent_radius: f32,
    spawn_radius: f32,
    spawn2_radius: f32,
    spawn_angle_step: f32,
    spawn2_angle_step: f32,
    spawn2_wave_freq: f32,
    parent_alpha_freq: f32,
    spawn_alpha_freq: f32,
    spawn2_alpha_freq: f32,
    parent_color: Color,
    spawn_color: Color,
    spawn2_color: Color,
    color_change_chance: f32,
    parent_brush: Texture,
    spawn_brush: Texture,
    spawn2_brush: Texture,
    use_assigned_brushes: bool,
}

impl Settings {
    // Note this is not a Default impl
    pub fn default(work_size: &Vec2, brushes: Vec<&Texture>) -> Self {
        let parent_brush = brushes[0].clone();
        let spawn_brush = brushes[0].clone();
        let spawn2_brush = brushes[0].clone();

        Self {
            spawn_strategy: SpawnStrategy::RandomChildOfNode,
            vary_spawn_distance: true,
            radial_change_step: CAPTURE_INTERVAL * 0.5,
            radial_range_style: RadialRangeStyle::None,
            parent_radius: work_size.x * 0.02,
            spawn_radius: work_size.x * 0.015,
            spawn2_radius: work_size.x * 0.005,
            spawn_angle_step: 10.0,
            spawn2_angle_step: 5.0,
            spawn2_wave_freq: 20.0,
            parent_alpha_freq: 0.5,
            spawn_alpha_freq: 0.5,
            spawn2_alpha_freq: 0.5,
            parent_color: colors::AEGEAN,
            spawn_color: colors::SEAWEED,
            spawn2_color: colors::SALMON,
            color_change_chance: 0.0,
            parent_brush,
            spawn_brush,
            spawn2_brush,
            use_assigned_brushes: true,
        }
    }

    fn gen_radial_ranges(
        rng: &mut Random,
        work_size: &Vec2,
        for_time: Option<f32>,
    ) -> (RadialRangeStyle, f32, f32, f32) {
        let radial_range_style = match for_time {
            Some(the_time) => match the_time {
                t if t >= 0.75 => RadialRangeStyle::random_small(rng),
                t if t >= 0.5 && t < 0.75 => RadialRangeStyle::random_medium(rng),
                t if t >= 0.25 && t < 0.5 => RadialRangeStyle::random(rng),
                t if t < 0.25 => RadialRangeStyle::random_large(rng),
                _ => RadialRangeStyle::random(rng),
            },
            None => RadialRangeStyle::random(rng),
        };

        let (parent_radius, spawn_radius, spawn2_radius) = match &radial_range_style {
            RadialRangeStyle::Small => (
                work_size.x * rng.gen_range(PARENT_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_SMALL),
            ),
            RadialRangeStyle::Large => (
                work_size.x * rng.gen_range(PARENT_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_LARGE),
            ),
            RadialRangeStyle::SmallToLarge => (
                work_size.x * rng.gen_range(PARENT_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_LARGE),
            ),
            RadialRangeStyle::LargeToSmall => (
                work_size.x * rng.gen_range(PARENT_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_SMALL),
            ),
            RadialRangeStyle::LargeSmallLarge => (
                work_size.x * rng.gen_range(PARENT_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_LARGE),
            ),
            RadialRangeStyle::SmallLargeSmall => (
                work_size.x * rng.gen_range(PARENT_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_SMALL),
            ),
            RadialRangeStyle::MediumLargeMedium => (
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
            ),
            RadialRangeStyle::LargeMediumLarge => (
                work_size.x * rng.gen_range(PARENT_RADIUS_LARGE),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_LARGE),
            ),
            RadialRangeStyle::SmallMediumSmall => (
                work_size.x * rng.gen_range(PARENT_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_SMALL),
            ),
            RadialRangeStyle::MediumSmallMedium => (
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
            ),
            RadialRangeStyle::MediumSmallSmall => (
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS_SMALL),
                work_size.x * rng.gen_range(SPAWN2_RADIUS_SMALL),
            ),
            RadialRangeStyle::SwapParentSpawn => (
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
            ),
            RadialRangeStyle::SwapParentSpawn2 => (
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(PARENT_RADIUS),
            ),
            RadialRangeStyle::SwapSpawnSpawn2 => (
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
            ),
            _ => (
                work_size.x * rng.gen_range(PARENT_RADIUS),
                work_size.x * rng.gen_range(SPAWN_RADIUS),
                work_size.x * rng.gen_range(SPAWN2_RADIUS),
            ),
        };
        (
            radial_range_style,
            parent_radius,
            spawn_radius,
            spawn2_radius,
        )
    }

    fn randomize(rng: &mut Random, work_size: &Vec2, brushes: &Vec<Texture>) -> Self {
        // return Settings::default(work_size, brushes);

        let mut vary_spawn_distance = true;
        if rng.gen_range(0..10) > 7 {
            vary_spawn_distance = false;
        }

        let parent_brush = brushes[rng.gen_range(0..brushes.len())].clone();
        let spawn_brush = brushes[rng.gen_range(0..brushes.len())].clone();
        let spawn2_brush = brushes[rng.gen_range(0..brushes.len())].clone();
        let use_assigned_brushes: bool = rng.gen();

        // let mut palette = PALETTE.to_vec();
        // let parent_color = palette.remove(rng.gen_range(0..palette.len()));
        // let spawn_color = palette.remove(rng.gen_range(0..palette.len()));
        // let spawn2_color = palette.remove(rng.gen_range(0..palette.len()));

        let (parent_color, spawn_color, spawn2_color) = Settings::choose_colors(rng);
        let color_change_chance = rng.gen_range(COLOR_CHANGE_CHANCE_RANGE);

        let (radial_range_style, parent_radius, spawn_radius, spawn2_radius) =
            Self::gen_radial_ranges(rng, work_size, Some(0.0));

        Self {
            spawn_strategy: SpawnStrategy::random(rng),
            vary_spawn_distance,
            radial_change_step: rng.gen_range(RADIAL_CHANGE_INTERVAL),
            radial_range_style,
            parent_radius,
            spawn_radius,
            spawn2_radius,
            spawn_angle_step: rng.gen_range(SPAWN_ANGLE_STEP),
            spawn2_angle_step: rng.gen_range(SPAWN2_ANGLE_STEP),
            spawn2_wave_freq: rng.gen_range(SPAWN2_WAVE_FREQ),
            parent_alpha_freq: rng.gen_range(ALPHA_FREQ),
            spawn_alpha_freq: rng.gen_range(ALPHA_FREQ),
            spawn2_alpha_freq: rng.gen_range(ALPHA_FREQ),
            parent_color,
            spawn_color,
            spawn2_color,
            color_change_chance,
            parent_brush,
            spawn_brush,
            spawn2_brush,
            use_assigned_brushes,
        }
    }

    fn choose_colors(rng: &mut Random) -> (Color, Color, Color) {
        let mut palette = PALETTE.to_vec();
        (
            palette.remove(rng.gen_range(0..palette.len())),
            palette.remove(rng.gen_range(0..palette.len())),
            palette.remove(rng.gen_range(0..palette.len())),
        )
    }

    fn change_colors(&mut self, rng: &mut Random) {
        let roll = rng.gen_range(0.0..1.0);
        if roll < self.color_change_chance {
            (self.parent_color, self.spawn_color, self.spawn2_color) = Settings::choose_colors(rng);
            log::debug!(
                "Changed colors: Chance {}, rolled {}\n: parent: {}, spawn: {}, spawn2: {}",
                self.color_change_chance,
                roll,
                self.parent_color,
                self.spawn_color,
                self.spawn2_color
            );
        } else {
            log::debug!(
                "No color change. Chance {}, rolled {} ",
                self.color_change_chance,
                roll
            );
        }
    }


    fn change_radial_ranges(&mut self, rng: &mut Random, work_size: &Vec2, for_time: Option<f32>) {
        (
            self.radial_range_style,
            self.parent_radius,
            self.spawn_radius,
            self.spawn2_radius,
        ) = Self::gen_radial_ranges(rng, work_size, for_time);
        log::debug!(
            "Changed radial ranges:\nradial_range_style: {:?}\nparent_radius: {}\nspawn_radius: {}\nspawn2_radius: {}",
            self.radial_range_style, self.parent_radius, self.spawn_radius, self.spawn2_radius
        );
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
    pub last_initialized: f32,
    pub last_update: f32,
    pub update_count: f32,
    pub last_radial_change: f32,
    pub capture: CapturingTexture,
    pub brushes: Vec<Texture>,
    pub draw_parent_alpha: f32,
    pub nodes: Vec<Node>,
    pub spawn_max_distance_mod: f32,
    pub settings: Settings,
    pub reinit_next_draw: bool,
    pub capture_next_draw: bool,
    pub touch: TouchState,
    events_focus: EventsFocus,
    help_modal: CommonHelpModal,
}

impl State {
    fn get_active_node(&self) -> Option<usize> {
        self.nodes.iter().position(|node| node.active == true)
    }

    fn increment_update_count(&mut self) {
        if self.update_count >= f32::MAX {
            self.update_count = 0.0;
        } else {
            self.update_count += 0.01;
        }
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

    fn reinitialize_drawing(&mut self, gfx: &mut Graphics, curr_time: f32) {
        log::debug!("Maximum captures reached. Creating new seed and re-randomizing settings...");
        self.last_initialized = curr_time;
        self.last_radial_change = curr_time;
        let (mut rng, capture) = init_rng_and_capture(gfx, &self.work_size);
        self.settings = Settings::randomize(&mut rng, &self.work_size, &self.brushes);
        log::debug!("With settings: {:#?}", self.settings);
        self.rng = rng;
        self.capture = capture;
        // Manually lock the newly reset capture so that it does not immediately
        // start capturing again.
        self.capture.capture_lock = true;
    }
}

fn create_circle_texture(gfx: &mut Graphics, radius: f32, color: Color) -> Texture {
    let rt = gfx
        .create_render_texture((radius * 2.0) as u32, (radius * 2.0) as u32)
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

fn create_basic_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/basic.png"))
        .build()
        .unwrap()
}

fn create_splat_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/splat.png"))
        .build()
        .unwrap()
}

fn create_scratch_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/scratch.png"))
        .build()
        .unwrap()
}

fn create_embossed_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/embossed.png"))
        .build()
        .unwrap()
}

fn create_threepoint_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/three-point.png"))
        .build()
        .unwrap()
}

fn create_scrape_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/scrape.png"))
        .build()
        .unwrap()
}

fn create_rects_brush_texture(gfx: &mut Graphics) -> Texture {
    gfx.create_texture()
        .from_image(include_bytes!("assets/brushes/rects.png"))
        .build()
        .unwrap()
}

fn init_rng_and_capture(gfx: &mut Graphics, work_size: &Vec2) -> (Random, CapturingTexture) {
    let (rng, seed) = get_rng(SEED);
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
    let work_size = get_work_size_for_screen(app, gfx);

    let (mut rng, capture) = init_rng_and_capture(gfx, &work_size);

    // The texture radius is large because we want large textures that look nice when app is maximized
    let circle_brush = create_circle_texture(gfx, work_size.x * 0.5, CIRCLE_TEXTURE_COLOR);
    let basic_brush = create_basic_brush_texture(gfx);
    let embossed_brush = create_embossed_brush_texture(gfx);
    let splat_brush = create_splat_brush_texture(gfx);
    let scratch_brush = create_scratch_brush_texture(gfx);
    let threepoint_brush = create_threepoint_brush_texture(gfx);
    let scrape_brush = create_scrape_brush_texture(gfx);
    let rects_brush = create_rects_brush_texture(gfx);

    let brushes = vec![
        basic_brush,
        circle_brush,
        embossed_brush,
        splat_brush,
        scratch_brush,
        threepoint_brush,
        scrape_brush,
        rects_brush,
    ];

    let settings = Settings::randomize(&mut rng, &work_size, &brushes);
    log::debug!("With settings: {:#?}", settings);

    let help_text = concat!(
        "Controls:\n\n",
        "Press 'R' to start a new painting\n\n",
        "Press 'C' to capture image\n\n",
        "Press 'S' to view source code\n\n",
        "Click mouse to close help\n",
    );

    let touch_help_text = concat!(
        "Controls:\n\n",
        // "Swipe left to start a\nnew painting\n\n",
        "Swipe left to start a new painting\n\n",
        "Swipe down to save image\n\n",
        "Swipe up to view source code\n\n",
        "Tap to close help\n",
    );

    let info_text = concat!(
        "\"Radial Pointillist\"\n",
        "Copyright 2023 Irfan Baig\n",
        "License: MIT"
    );

    State {
        work_size,
        rng,
        last_initialized: 0.0,
        last_update: 0.0,
        update_count: 0.0,
        last_radial_change: 0.0,
        capture,
        brushes,
        draw_parent_alpha: 0.0,
        nodes: vec![],
        spawn_max_distance_mod: 2.0,
        settings,
        reinit_next_draw: false,
        capture_next_draw: false,
        touch: TouchState::default(),
        events_focus: EventsFocus(false),
        help_modal: CommonHelpModal::new(
            gfx,
            help_text.to_string(),
            touch_help_text.to_string(),
            Some(info_text.to_string()),
        ),
    }
}

fn spawn_random(state: &mut State) {
    state.nodes.push(Node {
        class: NodeClass::PARENT,
        alpha: state.draw_parent_alpha,
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

fn open_source_code(app: &mut App) {
    {
        // log::debug!("opening source code...");
        let src_url = "https://github.com/riverfr0zen/sketches/blob/main/notan_sketches/examples/radial_pointillist.rs";
        app.backend.open_link(src_url, true);
    }
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    state.events_focus.detect(&evt);

    let gesture = state.touch.get_gesture(&app.timer.elapsed_f32(), &evt);
    // log::debug!("gesture found: {:?}", gesture);

    match evt {
        Event::MouseUp { .. } => {
            if state.events_focus.has_focus() {
                state.help_modal.handle_mouse_up()
            }
        }
        _ => {}
    }

    if gesture.is_some() {
        if !state.help_modal.handle_first_touch_with_help() {
            match gesture {
                Some(TouchGesture::SwipeLeft) => state.reinit_next_draw = true,
                Some(TouchGesture::SwipeDown) => state.capture_next_draw = true,
                Some(TouchGesture::SwipeUp) => open_source_code(app),
                Some(TouchGesture::Tap) => state.help_modal.toggle_touch_help(),
                _ => {}
            }
        }
    }
}

fn update(app: &mut App, state: &mut State) {
    if state.events_focus.has_focus() {
        if app.keyboard.was_pressed(KeyCode::R) {
            log::debug!("R");
            state.reinit_next_draw = true;
        }

        if app.keyboard.was_pressed(KeyCode::C) {
            log::debug!("C");
            state.capture_next_draw = true;
        }

        if app.keyboard.was_pressed(KeyCode::S) {
            log::debug!("S");
            open_source_code(app);
        }
    }

    let upcount = state.update_count;
    state.draw_parent_alpha = (upcount * state.settings.parent_alpha_freq).sin().abs();
    let spawn_alpha: f32 = (upcount * state.settings.spawn_alpha_freq).sin().abs();
    let spawn2_alpha: f32 = (upcount * state.settings.spawn2_alpha_freq).sin().abs();

    let curr_time = app.timer.elapsed_f32();
    let curr_painting_time = curr_time - state.last_initialized;

    if curr_time - state.last_radial_change > state.settings.radial_change_step {
        log::debug!("Current time in painting: {curr_painting_time}");
        let for_time = curr_painting_time / RADIAL_CHANGE_INTERVAL.end();
        state
            .settings
            .change_radial_ranges(&mut state.rng, &state.work_size, Some(for_time));
        state.settings.change_colors(&mut state.rng);
        state.last_radial_change = curr_time;
    }

    if curr_time - state.last_update > UPDATE_STEP {
        if let Some(active_node) = state.get_active_node() {
            let nodes = &mut state.nodes;

            let min_distance = state.settings.parent_radius * 1.5;
            let max_distance = min_distance * state.spawn_max_distance_mod;
            let distance: f32;
            if state.settings.vary_spawn_distance {
                distance = state.rng.gen_range(min_distance..max_distance);
            } else {
                distance = max_distance;
            }
            let parent_id = nodes[active_node].id.clone();
            // Need this offset so that spawn are positioned from the center of
            // parent node (because of how texture image positioning works)
            let spawn_offset = state.settings.parent_radius - state.settings.spawn_radius;

            if nodes[active_node].spawn_last_angle < 360.0
                || nodes[active_node].spawn2_last_angle < 360.0
            {
                if nodes[active_node].spawn_last_angle < 360.0 {
                    nodes[active_node].spawn_last_angle += state.settings.spawn_angle_step;
                    let spawn_x = nodes[active_node].pos.x
                        + spawn_offset
                        + nodes[active_node].spawn_last_angle.to_radians().cos() * distance;
                    let spawn_y = nodes[active_node].pos.y
                        + spawn_offset
                        + nodes[active_node].spawn_last_angle.to_radians().sin() * distance;
                    nodes.push(Node {
                        parent_id: parent_id,
                        pos: vec2(spawn_x, spawn_y),
                        alpha: spawn_alpha,
                        ..Default::default()
                    });
                }

                let spawn2_offset = state.settings.parent_radius - state.settings.spawn2_radius;
                if nodes[active_node].spawn2_last_angle < 360.0 {
                    nodes[active_node].spawn2_last_angle += state.settings.spawn2_angle_step;
                    // Spawn2 distance changes as a wave
                    let spawn2_max_distance = max_distance * 0.75;
                    let spawn2_distance = min_distance
                        + ((upcount * state.settings.spawn2_wave_freq).sin().abs()
                            * spawn2_max_distance);
                    let spawn2_x = nodes[active_node].pos.x
                        + spawn2_offset
                        + nodes[active_node].spawn2_last_angle.to_radians().cos() * spawn2_distance;
                    let spawn2_y = nodes[active_node].pos.y
                        + spawn2_offset
                        + nodes[active_node].spawn2_last_angle.to_radians().sin() * spawn2_distance;

                    nodes.push(Node {
                        class: NodeClass::SPAWN2,
                        parent_id: parent_id,
                        pos: vec2(spawn2_x, spawn2_y),
                        alpha: spawn2_alpha,
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
        state.increment_update_count();
    }
}

fn draw_nodes(draw: &mut Draw, state: &mut State) {
    for node in state.nodes.iter_mut().filter(|node| !node.rendered) {
        let size: f32;
        let color: Color;
        let brush_chance = state.rng.gen_range(0..15);
        let mut texture = match brush_chance {
            11 => &state.brushes[BRUSH_RECTS],
            10 => &state.brushes[BRUSH_SCRAPE],
            9 => &state.brushes[BRUSH_THREEPOINT],
            8 => &state.brushes[BRUSH_SCRATCH],
            7 => &state.brushes[BRUSH_EMBOSSED],
            6 => &state.brushes[BRUSH_SPLAT],
            3..=5 => &state.brushes[BRUSH_CIRCLE],
            _ => &state.brushes[BRUSH_BASIC],
        };
        let texture_angle: f32 = state.rng.gen_range(0.0..=360.0);
        match node.class {
            NodeClass::PARENT => {
                if state.settings.use_assigned_brushes {
                    texture = &state.settings.parent_brush;
                }
                size = state.settings.parent_radius * 2.0;
                color = state.settings.parent_color;
            }
            NodeClass::SPAWN => {
                if state.settings.use_assigned_brushes {
                    texture = &state.settings.spawn_brush;
                }
                size = state.settings.spawn_radius * 2.0;
                color = state.settings.spawn_color;
            }
            NodeClass::SPAWN2 => {
                if state.settings.use_assigned_brushes {
                    texture = &state.settings.spawn2_brush;
                }
                size = state.settings.spawn2_radius * 2.0;
                color = state.settings.spawn2_color;
            }
        }
        draw.image(&texture)
            .alpha_mode(BlendMode::OVER)
            .alpha(node.alpha)
            .color(color)
            .position(node.pos.x, node.pos.y)
            .rotate_degrees_from(
                (node.pos.x + size * 0.5, node.pos.y + size * 0.5),
                texture_angle,
            )
            .size(size, size);
        node.rendered = true;
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let curr_time = app.timer.elapsed_f32();
    if state.reinit_next_draw {
        state.reinitialize_drawing(gfx, curr_time);
        state.reinit_next_draw = false;
    }

    let draw = &mut state.capture.render_texture.create_draw();
    draw_nodes(draw, state);
    gfx.render_to(&state.capture.render_texture, draw);

    if state.capture_next_draw {
        state.capture.capture(app, gfx);
        state.capture_next_draw = false;
    } else if !IS_WASM {
        state.capture.periodic_capture(app, gfx);
        if SEED.is_none() && state.capture.num_captures >= MAX_CAPTURES {
            state.reinitialize_drawing(gfx, curr_time);
        }
    }

    let rdraw = &mut get_draw_setup(gfx, state.work_size, true, CLEAR_COLOR);
    rdraw.image(&state.capture.render_texture);

    state.help_modal.draw(rdraw, state.work_size);

    gfx.render(rdraw);
}

#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    let win_config = get_common_win_config()
        .set_high_dpi(true)
        .set_multisampling(8)
        .set_vsync(true)
        .set_size(
            // let win_config = get_common_win_config().high_dpi(true).size(
            ScreenDimensions::RES_4KISH.x as u32,
            ScreenDimensions::RES_4KISH.y as u32,
            // ScreenDimensions::RES_HDPLUS.x as i32,
            // ScreenDimensions::RES_HDPLUS.y as i32,
            // ScreenDimensions::RES_1080P.x as u32,
            // ScreenDimensions::RES_1080P.y as u32,
            // ScreenDimensions::DEFAULT.x as i32,
            // ScreenDimensions::DEFAULT.y as i32,
        );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().set_high_dpi(true);

    set_html_bgcolor(CLEAR_COLOR);

    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .touch_as_mouse(false)
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}
