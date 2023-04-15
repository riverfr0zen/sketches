use crate::shaderutils::ShaderRenderTexture;
// use notan::draw::*;
// use notan::log;
use notan::math::{Vec2, Vec3};
use notan::prelude::*;

// NOTE: You will need to `touch` this file to re-compile if the shader below is changed
pub const FRAG: ShaderSource =
    notan::include_fragment_shader!("examples/assets/shaders/emo_tile.frag.glsl");

pub struct TileShaderBundle {
    pub srt: ShaderRenderTexture,
    pub common_ubo: Buffer,
    pub tile_colors_ubo: Buffer,
}

impl TileShaderBundle {
    pub fn new(gfx: &mut Graphics, work_size: &Vec2, tile_color: &Color, bg_color: &Color) -> Self {
        Self {
            srt: ShaderRenderTexture::new(gfx, work_size.x, work_size.y),
            common_ubo: gfx
                .create_uniform_buffer(1, "Common")
                .with_data(&[0.0, work_size.x, work_size.y])
                .build()
                .unwrap(),
            tile_colors_ubo: gfx
                .create_uniform_buffer(2, "TileColors")
                .with_data(&[
                    // Vec3::new(tile_color.r, tile_color.g, tile_color.b),
                    // Vec3::new(BG_COLOR.r, BG_COLOR.g, BG_COLOR.b),
                    tile_color.r,
                    tile_color.g,
                    tile_color.b,
                    bg_color.r,
                    bg_color.g,
                    bg_color.b,
                ])
                .build()
                .unwrap(),
        }
    }

    pub fn draw_filled(&mut self, gfx: &mut Graphics, pipeline: &Pipeline) {
        self.srt
            .draw_filled(gfx, pipeline, vec![&self.common_ubo, &self.tile_colors_ubo]);
    }
}
