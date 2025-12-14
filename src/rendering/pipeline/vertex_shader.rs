use crate::geometry::mesh::Vertex;
use crate::math::matrices::{Matrix4, Transformer};
use crate::math::vectors::{Normal3, Vector3};

#[derive(Clone)]
pub struct ProcessedVertex {
    pub ndc_pos: Vector3,
    pub view_pos: Vector3,
    pub view_nor: Normal3,
    pub inv_w: f32,
}

pub struct VertexShader;

impl VertexShader {
    pub fn new() -> VertexShader {
        Self
    }

    pub fn process(&self, vertex: &Vertex, view: &Matrix4, proj: &Matrix4) -> ProcessedVertex {
        let view_pos = view.transform(vertex.pos);
        let view_nor = view.transform(vertex.nor);
        let clip_pos = proj.transform(view_pos.extend(1.0));
        let inv_w = 1.0 / clip_pos.w;
        let ndc_pos = clip_pos.truncate() / clip_pos.w;
        ProcessedVertex {
            ndc_pos,
            view_pos,
            view_nor,
            inv_w,
        }
    }
}
