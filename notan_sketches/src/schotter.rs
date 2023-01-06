use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;


// Visualization modifier
pub enum VizMod {
    BASIC,
    SOLID,
}


pub fn _create_box_texture(
    gfx: &mut Graphics,
    tile_size: f32,
    stroke_width: f32,
    vizmod: VizMod,
) -> Texture {
    let rt = gfx
        .create_render_texture(tile_size as i32, tile_size as i32)
        .build()
        .unwrap();

    let tile_size = tile_size as f32;
    let mut draw = gfx.create_draw();
    draw.set_size(tile_size, tile_size);
    match vizmod {
        VizMod::SOLID => {
            draw.rect((0.0, 0.0), (tile_size, tile_size))
                .fill_color(Color::WHITE)
                .fill()
                // .stroke_color(Color::BLACK)
                // .stroke_color(Color::new(0.5, 0.5, 0.5, 1.0))
                .stroke_color(Color::new(0.8, 0.8, 0.8, 1.0))
                .stroke(stroke_width);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
        _ => {
            draw.clear(Color::TRANSPARENT);
            draw.rect((0.0, 0.0), (tile_size, tile_size))
                .color(Color::BLACK)
                .stroke(stroke_width);

            gfx.render_to(&rt, &draw);
            rt.take_inner()
        }
    }
}


pub fn create_basic_box_texture(gfx: &mut Graphics, tile_size: f32, stroke_width: f32) -> Texture {
    _create_box_texture(gfx, tile_size, stroke_width, VizMod::BASIC)
}


pub fn create_solid_box_texture(gfx: &mut Graphics, tile_size: f32, stroke_width: f32) -> Texture {
    _create_box_texture(gfx, tile_size, stroke_width, VizMod::SOLID)
}
