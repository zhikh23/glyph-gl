use crate::geometry::face::Face;
use crate::geometry::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, faces: Vec<Face>) -> Mesh {
        Mesh { vertices, faces }
    }
}
