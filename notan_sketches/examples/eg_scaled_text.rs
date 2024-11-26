use notan::draw::*;
use notan::log;
use notan::math::{vec2, vec3, Mat4, Vec2};
use notan::prelude::*;
use notan::text::*;

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);

#[derive(AppState)]
struct State {
    font: Font,
}

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

pub fn get_draw_setup(gfx: &mut Graphics, work_size: Vec2, clear_color: Color) -> Draw {
    let (width, height) = gfx.size();
    let win_size = vec2(width as f32, height as f32);

    let mut draw = gfx.create_draw();
    draw.clear(clear_color);

    let (projection, _) = get_aspect_fit_projection(win_size, work_size);
    draw.set_projection(Some(projection));
    return draw;
}

fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("./assets/fonts/ubuntu/Ubuntu-B.ttf"))
        .unwrap();

    let state = State { font: font };
    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = get_draw_setup(gfx, WORK_SIZE, Color::GRAY);

    let mut text = gfx.create_text();
    text.add("This is some sample text, This is some sample text")
        .font(&state.font)
        .color(Color::PURPLE)
        .size(60.0)
        .max_width(WORK_SIZE.x * 0.5)
        .position(WORK_SIZE.x * 0.25, 200.0)
        .h_align_left()
        .v_align_middle();

    draw.text(
        &state.font,
        "This is some sample text, This is some sample text",
    )
    .color(Color::PURPLE)
    .size(60.0)
    .max_width(WORK_SIZE.x * 0.5)
    .position(WORK_SIZE.x * 0.25, 400.0)
    .h_align_left()
    .v_align_middle();

    gfx.render(&draw);
    gfx.render(&text);

    // log::debug!("fps: {}", app.timer.fps().round());
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::default().set_resizable(true);

    // notan::init_with(State::default)
    // notan::init()
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        // .update(update)
        .build()
}
