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
        // Texture should be cleared so that shader alpha changes register.
        // @TODO: This could be made optional to support situations where draw retention is
        // actually desired.
        rt_draw.clear(Color::TRANSPARENT);

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


// Taken from notan_draw::shapes::painter
fn vertex_info() -> VertexInfo {
    VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x4)
}

// Adapted from notan_draw::shapes::painter::create_shape_pipeline
pub fn create_hot_shape_pipeline(
    device: &mut Device,
    fragment_path: &str,
    // fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    // let fragment = fragment.unwrap_or(&SHAPES_FRAGMENT);
    let vert = std::fs::read("examples/assets/shaders/shapes.vert.glsl").unwrap();
    let frag = std::fs::read(fragment_path).unwrap();

    device
        .create_pipeline()
        // .from(&SHAPES_VERTEX, fragment)
        .from_raw(&vert, &frag)
        .with_vertex_info(&vertex_info())
        .with_color_blend(BlendMode::NORMAL)
        .build()
}


// Provides functionality to manage hot reloading of shaders
struct HotPipelinesManager {
    pipelines: Vec<Pipeline>,
    // TODO
    //pub fn add_shape_pipeline()
}
