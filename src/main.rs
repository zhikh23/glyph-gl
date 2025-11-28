pub mod camera;
pub mod geometry;
pub mod io;
pub mod math;
pub mod output;
pub mod rendering;

use crate::camera::fpv_camera::FPVCamera;
use crate::io::obj_loader::ObjLoader;
use crate::math::vectors::Vector3;
use crate::output::brailler_formatter::BrailleColorFormatter;
use crate::rendering::renderer::Renderer;

fn main() {
    let camera = FPVCamera::new(Vector3::new(0.0, 1.5, -10.0), 0.0, 0.0, (6.0, 4.0));
    let light = Vector3::new(1.0, 0.0, -1.0).normalize().unwrap();

    let output = Box::new(BrailleColorFormatter);
    let mut renderer = Renderer::new(160 * 2, 40 * 4, &camera, light, output);
    let mesh = ObjLoader::load_from_file("./models/teapot.obj")
        .unwrap_or_else(|e| panic!("failed to load model: {:?}", e));
    renderer.render(&mesh);
    let frame = renderer.frame();
    println!("{}", frame);
}
