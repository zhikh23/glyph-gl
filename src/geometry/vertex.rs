use crate::math::vectors::Vector3;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Vertex(Vector3);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }
}

impl Deref for Vertex {
    type Target = Vector3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
