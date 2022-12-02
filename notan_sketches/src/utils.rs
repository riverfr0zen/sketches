use notan::draw::*;
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
