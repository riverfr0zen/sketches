use notan::draw::*;
use notan::prelude::*;


pub struct ShaderRenderTexture {
    pub rt: RenderTexture,
}

impl ShaderRenderTexture {
    pub fn new(gfx: &mut Graphics, width: f32, height: f32) -> Self {
        let rt = gfx
            .create_render_texture(width as _, height as _)
            .build()
            .unwrap();
        Self { rt }
    }

    pub fn draw<F>(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Pipeline,
        // Up to 5 uniforms are allowed for now (see hack below)
        uniforms: Vec<&Buffer>,
        draw_fn: F,
    ) where
        F: Fn(&mut Draw),
    {
        let rt_draw = &mut self.rt.create_draw();

        // HACKY WAY OF BUILDING CUSTOM PIPELINE for n uniforms. Revisit later.
        match uniforms.len() {
            5 => {
                rt_draw
                    .shape_pipeline()
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[0])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[1])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[2])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[3])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[4]);
            }
            4 => {
                rt_draw
                    .shape_pipeline()
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[0])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[1])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[2])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[3]);
            }
            3 => {
                rt_draw
                    .shape_pipeline()
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[0])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[1])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[2]);
            }
            2 => {
                rt_draw
                    .shape_pipeline()
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[0])
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[1]);
            }
            1 => {
                rt_draw
                    .shape_pipeline()
                    .pipeline(&pipeline)
                    .uniform_buffer(&uniforms[0]);
            }
            _ => panic!(concat!(
                "Max number of uniforms is 5 due to Irfan's hacky implementation! ",
                "You can increase this by updating notan_sketches::shaders::ShaderRenderTexture::draw()"
            )),
        }
        draw_fn(rt_draw);
        rt_draw.shape_pipeline().remove();
        gfx.render_to(&self.rt, rt_draw);
    }

    /// Common draw that sizes the shader to the whole texture
    pub fn draw_filled(&mut self, gfx: &mut Graphics, pipeline: &Pipeline, uniforms: Vec<&Buffer>) {
        self.draw(gfx, &pipeline, uniforms, |srtdraw| {
            srtdraw
                .rect((0.0, 0.0), (srtdraw.width(), srtdraw.height()))
                .fill_color(Color::GRAY)
                .fill();
        });
    }
}
