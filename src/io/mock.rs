use crate::geometry::face::Face;
use crate::geometry::mesh::Mesh;
use crate::geometry::vertex::Vertex;
use crate::math::vectors::UnitVector3;

pub fn mock_cube_mesh() -> Mesh {
    //     v7-----v6
    //    /|      /|
    //   v3-----v2 |
    //   | |     | |       y
    //   | v4----|-v5      | z
    //   |/      |/        |/
    //   v0-----v1         0---- x
    let vertices = vec![
        Vertex::new(-1.0, -1.0, -1.0), // v0
        Vertex::new(1.0, -1.0, -1.0),  // v1
        Vertex::new(1.0, 1.0, -1.0),   // v2
        Vertex::new(-1.0, 1.0, -1.0),  // v3
        Vertex::new(-1.0, -1.0, 1.0),  // v4
        Vertex::new(1.0, -1.0, 1.0),   // v5
        Vertex::new(1.0, 1.0, 1.0),    // v6
        Vertex::new(-1.0, 1.0, 1.0),   // v7
    ];
    let faces = vec![
        // Нижняя грань
        Face::new(0, 1, 2, UnitVector3::new_unchecked(0.0, 1.0, 0.0)),
        Face::new(2, 3, 0, UnitVector3::new_unchecked(0.0, 1.0, 0.0)),
        // Верхняя грань
        Face::new(4, 6, 5, UnitVector3::new_unchecked(0.0, -1.0, 0.0)),
        Face::new(6, 4, 7, UnitVector3::new_unchecked(0.0, -1.0, 0.0)),
        // Передняя грань (Z = 1)
        Face::new(3, 2, 6, UnitVector3::new_unchecked(0.0, 0.0, -1.0)),
        Face::new(6, 7, 3, UnitVector3::new_unchecked(0.0, 0.0, -1.0)),
        // Задняя грань (Z = -1)
        Face::new(1, 0, 4, UnitVector3::new_unchecked(0.0, 0.0, 1.0)),
        Face::new(4, 5, 1, UnitVector3::new_unchecked(0.0, 0.0, 1.0)),
        // Левая грань (X = -1)
        Face::new(0, 3, 7, UnitVector3::new_unchecked(1.0, 0.0, 0.0)),
        Face::new(7, 4, 0, UnitVector3::new_unchecked(1.0, 0.0, 0.0)),
        // Правая грань (X = 1)
        Face::new(2, 1, 5, UnitVector3::new_unchecked(-1.0, 0.0, 0.0)),
        Face::new(5, 6, 2, UnitVector3::new_unchecked(-1.0, 0.0, 0.0)),
    ];
    Mesh::new(vertices, faces)
}
