use notan::draw::*;
// use notan::log;
use notan::math::Vec2;
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
        // log::info!("ShaderRenderTexture created: {width} x {height}");
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
#[cfg(debug_assertions)]
fn vertex_info() -> VertexInfo {
    VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x4)
}

// Hot-reloading shape pipeline for debug mode
#[cfg(debug_assertions)]
pub fn create_hot_shape_pipeline(
    device: &mut Device,
    fragment_path: &str,
) -> Result<Pipeline, String> {
    let vert = std::fs::read("examples/assets/shaders/shapes.vert.glsl").unwrap();
    let frag = std::fs::read(fragment_path).unwrap();

    device
        .create_pipeline()
        .from_raw(&vert, &frag)
        .with_vertex_info(&vertex_info())
        .with_color_blend(BlendMode::NORMAL)
        .build()
}

// Regular shape pipeline for release mode
// Re-export from notan::draw
#[cfg(not(debug_assertions))]
pub use notan::draw::create_shape_pipeline;

pub struct ShaderReloadManager {
    needs_reload: bool,
    pub frame: usize,
}

impl Default for ShaderReloadManager {
    fn default() -> Self {
        Self {
            needs_reload: false,
            frame: 0,
        }
    }
}

impl ShaderReloadManager {
    pub fn needs_reload(&mut self) -> bool {
        if self.needs_reload {
            self.needs_reload = false;
            return true;
        }
        false
    }

    pub fn update(&mut self) {
        if self.frame % 60 == 0 {
            self.needs_reload = true;
        }
        self.frame += 1;
    }
}

#[uniform]
#[derive(Copy, Clone)]
pub struct CommonData {
    pub u_time: f32,
    pub u_resolution: Vec2,
}

impl CommonData {
    pub fn new(u_time: f32, u_resolution: Vec2) -> Self {
        Self {
            u_time,
            u_resolution,
        }
    }
}
