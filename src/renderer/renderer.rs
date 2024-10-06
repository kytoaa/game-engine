use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct RenderVert {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}
impl RenderVert {
    pub fn new(position: [f32; 2]) -> RenderVert {
        RenderVert { position }
    }
}

pub struct Renderer {}
