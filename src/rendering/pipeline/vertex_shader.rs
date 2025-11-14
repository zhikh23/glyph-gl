use std::ops::Deref;

use crate::geometry::vertex::Vertex;
use crate::math::matrices::Matrix4;

pub struct ProcessedVertex(Vertex);

impl Deref for ProcessedVertex {
    type Target = Vertex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct VertexShader {
    proj: Matrix4,
    width: usize,
    height: usize,
}

impl VertexShader {
    pub fn new(proj: Matrix4, width: usize, height: usize) -> VertexShader {
        Self {
            proj,
            width,
            height,
        }
    }

    pub fn process(&self, vertex: Vertex) -> ProcessedVertex {
        let clip_pos = self.proj.transform(*vertex);
        let screen_x = (clip_pos.x + 1.0) * 0.5 * self.width as f32;
        let screen_y = (1.0 - clip_pos.y) * 0.5 * self.height as f32;
        ProcessedVertex(Vertex::new(screen_x, screen_y, clip_pos.z))
    }
}
