use crate::camera::fpv_camera::FPVCamera;
use crate::geometry::mesh::Mesh;
use crate::math::vectors::UnitVector3;
use crate::rendering::frame_buffer::FrameBuffer;
use crate::rendering::pipeline::fragment_shader::FragmentShader;
use crate::rendering::pipeline::vertex_shader::VertexShader;
use crate::rendering::rasterizer::triangle_rasterizer::TriangleRasterizer;
use crate::rendering::z_buffer::ZBuffer;

pub struct Renderer {
    frame_buffer: FrameBuffer,
    z_buffer: ZBuffer,
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
}

impl Renderer {
    pub fn new(width: usize, height: usize, camera: &FPVCamera, light: UnitVector3) -> Self {
        Self {
            frame_buffer: FrameBuffer::new(width, height),
            z_buffer: ZBuffer::new(width, height),
            vertex_shader: VertexShader::new(camera.view_matrix(), width, height),
            fragment_shader: FragmentShader::new(light),
        }
    }

    pub fn render(&mut self, mesh: &Mesh) {
        self.frame_buffer.clear();
        self.z_buffer.clear();

        for face in &mesh.faces {
            let v0 = &mesh.vertices[face.indices[0]];
            let v1 = &mesh.vertices[face.indices[1]];
            let v2 = &mesh.vertices[face.indices[2]];

            let processed_v0 = self.vertex_shader.process(*v0);
            let processed_v1 = self.vertex_shader.process(*v1);
            let processed_v2 = self.vertex_shader.process(*v2);

            TriangleRasterizer::rasterize_triangle(
                processed_v0,
                processed_v1,
                processed_v2,
                face.normal,
                &mut self.z_buffer,
                &mut self.frame_buffer,
                &self.fragment_shader,
            )
        }
    }

    pub fn frame(&self) -> String {
        self.frame_buffer.to_string()
    }
}
