use criterion::{Criterion, criterion_group, criterion_main};
use glyph_gl::camera::look_at_camera::LookAtCamera;
use glyph_gl::config::{Config, ShadingMode};
use glyph_gl::geometry::mesh::Mesh;
use glyph_gl::io::obj_loader::ObjLoader;
use glyph_gl::rendering::renderer::Renderer;

fn render_teapot_benchmark(c: &mut Criterion) {
    let config = Config::default().with_resolution(1024, 1024);

    let raw_mesh = ObjLoader::load_from_file("examples/teapot.obj")
        .unwrap_or_else(|e| panic!("failed to load model: {:?}", e));
    let mut mesh = match config.shading_mode {
        ShadingMode::Flat => Mesh::with_flat_normals(raw_mesh)
            .unwrap_or_else(|e| panic!("failed to create mesh: {:?}", e)),
        ShadingMode::Smooth => Mesh::with_smooth_normals(raw_mesh)
            .unwrap_or_else(|e| panic!("failed to create mesh: {:?}", e)),
    };
    mesh.fit(2.0);
    mesh.centering();

    let mut renderer = Renderer::new(&config);

    let aspect = config.frame_width as f32 / config.frame_height as f32;
    let camera = LookAtCamera::new(
        config.camera_pos,
        config.camera_target,
        (aspect, 1.0),
        config.fov.to_radians(),
        config.near,
        config.far,
    );

    c.bench_function("render teapot", |b| {
        b.iter(|| renderer.render(&mesh, &camera));
    });
}

criterion_group!(benches, render_teapot_benchmark);
criterion_main!(benches);
