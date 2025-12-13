use crate::geometry::mesh::Vertex;
use crate::math::matrices::{Matrix4, Transformer};
use crate::math::vectors::{Normal3, Vector3};

#[derive(Clone)]
pub struct ProcessedVertex {
    pub ndc_pos: Vector3,
    pub view_pos: Vector3,
    pub view_nor: Normal3,
}

pub struct VertexShader;

impl VertexShader {
    pub fn new() -> VertexShader {
        Self
    }

    pub fn process(&self, vertex: &Vertex, view: &Matrix4, proj: &Matrix4) -> ProcessedVertex {
        let view_pos = view.transform(vertex.pos);
        let view_nor = view.transform(vertex.nor);
        let ndc_pos = proj.transform(view_pos);
        ProcessedVertex {
            ndc_pos,
            view_pos,
            view_nor,
        }
    }
}
