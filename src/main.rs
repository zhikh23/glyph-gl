pub mod camera;
pub mod geometry;
pub mod io;
pub mod math;
pub mod rendering;

use crate::camera::fpv_camera::FPVCamera;
use crate::geometry::face::Face;
use crate::geometry::mesh::Mesh;
use crate::geometry::vertex::Vertex;
use crate::math::vectors::{UnitVector3, Vector3};
use crate::rendering::renderer::Renderer;
use std::f32::consts::SQRT_2;

fn main() {
    let camera = FPVCamera::new(
        Vector3::new(0.0, 0.0, -10.0), // должно быть 10
        0.0,
        0.0,
        (6.0, 3.0),
    );
    let light = UnitVector3::new_unchecked(SQRT_2 / 2.0 - 0.5, 0.0, SQRT_2 / 2.0 + 0.3);
    let mut renderer = Renderer::new(160, 40, &camera, light);
    //let mesh = mock_cube_mesh();
    let mesh = Mesh::new(
        vec![
            Vertex::new(-1.0, -1.0, 0.0),
            Vertex::new(-1.0, 1.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0),
        ],
        vec![Face::new(
            0,
            1,
            2,
            UnitVector3::new_unchecked(0.0, 0.0, 1.0),
        )],
    );
    renderer.render(&mesh);
    let frame = renderer.frame();
    println!("{}", frame);
}
