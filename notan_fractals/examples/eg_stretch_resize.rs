/**
    The example at https://github.com/Nazariglez/notan/blob/develop/examples/draw_projection.rs
    shows how to set a projection that scales the content and maintains aspect ratio when the
    window size is changed.

    I've hacked it here to scale the content, but NOT maintain aspect ration (i.e. content
    will stretch to match window).
*/
use notan::draw::*;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_fractals::utils::get_scaling_projection;

const WORK_SIZE: Vec2 = vec2(800.0, 600.0);


#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::default().resizable(true);

    notan::init()
        .add_config(win_config)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .build()
}


fn draw(gfx: &mut Graphics) {
    let (width, height) = gfx.size();
    let win_size = vec2(width as f32, height as f32);

    // get the projection that will fit and center our content in the screen
    let projection = get_scaling_projection(win_size, WORK_SIZE);

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // We set our projection here
    // Anything draw bellow will keep the aspect ratio
    draw.set_projection(Some(projection));

    // Our resolution bounds
    draw.rect((0.0, 0.0), WORK_SIZE.into())
        .color(Color::ORANGE)
        .stroke(10.0);

    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));

    // draw to screen
    gfx.render(&draw);
}
