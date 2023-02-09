use notan::draw::*;
use notan::log;
use notan::math::{vec2, vec3, Mat4, Vec2};
use notan::prelude::*;


/// This returns a projection that keeps the aspect ratio while scaling
/// and fitting the content in our window
/// It also returns the ratio in case we need it to calculate positions
/// or manually scale something
///
/// Taken from the following example:
/// https://github.com/Nazariglez/notan/blob/develop/examples/draw_projection.rs
pub fn get_aspect_fit_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, f32) {
    let ratio = (win_size.x / work_size.x).min(win_size.y / work_size.y);

    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let scale = Mat4::from_scale(vec3(ratio, ratio, 1.0));
    let position = vec3(
        (win_size.x - work_size.x * ratio) * 0.5,
        (win_size.y - work_size.y * ratio) * 0.5,
        1.0,
    );
    let translation = Mat4::from_translation(position);
    (projection * translation * scale, ratio)
}


/// Returns a projection for scaling content to the window size WITHOUT maintaining aspect ratio
/// (i.e. content will be stretched to fit window)
///
/// Based on the following example:
/// https://github.com/Nazariglez/notan/blob/develop/examples/draw_projection.rs
pub fn get_scaling_projection(win_size: Vec2, work_size: Vec2) -> Mat4 {
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let scale = Mat4::from_scale(vec3(
        win_size.x / work_size.x,
        win_size.y / work_size.y,
        1.0,
    ));
    projection * scale
}


/// Set up a Draw with some common basics
pub fn get_draw_setup(
    gfx: &mut Graphics,
    work_size: Vec2,
    aspect_fit: bool,
    clear_color: Color,
) -> Draw {
    let (width, height) = gfx.size();
    let win_size = vec2(width as f32, height as f32);

    let mut draw = gfx.create_draw();
    draw.clear(clear_color);

    if aspect_fit {
        let (projection, _) = get_aspect_fit_projection(win_size, work_size);
        draw.set_projection(Some(projection));
    } else {
        let projection = get_scaling_projection(win_size, work_size);
        draw.set_projection(Some(projection));
    }
    return draw;
}


pub fn get_common_win_config() -> WindowConfig {
    #[cfg(not(target_arch = "wasm32"))]
    return WindowConfig::default().resizable(true);

    #[cfg(target_arch = "wasm32")]
    return WindowConfig::default().resizable(true).maximized(true);
}


#[non_exhaustive]
pub struct ScreenDimensions;

impl ScreenDimensions {
    // Many based on https://en.wikipedia.org/wiki/Graphics_display_resolution
    pub const MINIMUM: Vec2 = vec2(500.0, 500.0);
    pub const DEFAULT: Vec2 = vec2(800.0, 600.0);
    pub const RES_QHD: Vec2 = vec2(960.0, 540.0);
    pub const RES_720P: Vec2 = vec2(1280.0, 720.0);
    pub const RES_HDPLUS: Vec2 = vec2(1600.0, 900.0);
    pub const RES_1080P: Vec2 = vec2(1920.0, 1080.0);
    pub const RES_1440P: Vec2 = vec2(2560.0, 1440.0);
    pub const RES_4K: Vec2 = vec2(3840.0, 2160.0);
    pub const RES_4KISH: Vec2 = vec2(3500.0, 1800.0);
    pub const RES_5K: Vec2 = vec2(5120.0, 2880.0);
    pub const RES_8K: Vec2 = vec2(7680.0, 4320.0);
}


pub fn get_rng(seed: Option<u64>) -> (Random, u64) {
    let mut rng = Random::default();
    let _seed: u64;
    if let Some(seed) = seed {
        _seed = seed;
    } else {
        _seed = rng.gen();
    }
    // log::debug!("seed: {}", _seed);
    rng.reseed(_seed);
    (rng, _seed)
}


pub struct CapturingTexture {
    pub render_texture: RenderTexture,
    pub capture_to: String,
    /// Capture interval in seconds. 0.0 for no capture.
    pub capture_interval: f32,
    pub last_capture: f32,
    pub capture_lock: bool,
}

impl CapturingTexture {
    fn create_render_texture(
        gfx: &mut Graphics,
        work_size: &Vec2,
        bgcolor: Color,
    ) -> RenderTexture {
        let render_texture = gfx
            .create_render_texture(work_size.x as _, work_size.y as _)
            // .create_render_texture(width, height)
            // .with_filter(TextureFilter::Linear, TextureFilter::Linear)
            // .with_depth()
            .build()
            .unwrap();
        let mut draw = render_texture.create_draw();
        draw.clear(bgcolor);
        gfx.render_to(&render_texture, &draw);
        render_texture
    }

    pub fn new(
        gfx: &mut Graphics,
        work_size: &Vec2,
        bgcolor: Color,
        capture_to: String,
        capture_interval: f32,
    ) -> Self {
        Self {
            render_texture: Self::create_render_texture(gfx, work_size, bgcolor),
            capture_to: capture_to,
            capture_interval: capture_interval,
            last_capture: 0.0,
            capture_lock: false,
        }
    }

    pub fn capture(&mut self, app: &mut App, gfx: &mut Graphics) {
        if self.capture_lock {
            self.last_capture = app.timer.time_since_init();
            log::debug!("Last capture completed at {} seconds", self.last_capture);
            self.capture_lock = false;
        } else {
            if self.capture_interval > 0.0
                && ((app.timer.time_since_init() - self.last_capture) > self.capture_interval)
            {
                log::debug!("Beginning capture at {}", app.timer.time_since_init());
                let filepath = format!("{}_{}.png", self.capture_to, app.timer.time_since_init());
                self.render_texture.to_file(gfx, &filepath).unwrap();
                self.capture_lock = true;
            }
        }
    }
}
