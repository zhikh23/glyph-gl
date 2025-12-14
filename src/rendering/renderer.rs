use crate::camera::base::Camera;
use crate::config::Config;
use crate::geometry::mesh::Mesh;
use crate::math::vectors::Direction3;
use crate::output::formatter::OutputFormatter;
use crate::rendering::frame_buffer::FrameBuffer;
use crate::rendering::pipeline::fragment_shader::FragmentShader;
use crate::rendering::pipeline::vertex_shader::VertexShader;
use crate::rendering::triangle_rasterizer::TriangleRasterizer;
use crate::rendering::z_buffer::ZBuffer;

pub struct Renderer {
    frame_buffer: FrameBuffer,
    z_buffer: ZBuffer,
    rasterizer: TriangleRasterizer,
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
}

impl Renderer {
    pub fn new(config: &Config) -> Self {
        Self {
            frame_buffer: FrameBuffer::new(config.frame_width, config.frame_height),
            z_buffer: ZBuffer::new(config.frame_width, config.frame_height),
            rasterizer: TriangleRasterizer::new(config.frame_width, config.frame_height),
            vertex_shader: VertexShader::new(),
            fragment_shader: FragmentShader::new(
                config.light_ambient,
                config.light_diffuse,
                config.light_specular,
                config.light_specular,
            ),
        }
    }

    pub fn render(&mut self, mesh: &Mesh, camera: &impl Camera) {
        self.frame_buffer.clear();
        self.z_buffer.clear();

        let view = camera.view();
        let proj = camera.proj();

        for tr in mesh.iter() {
            let (v0, v1, v2) = (
                self.vertex_shader.process(&tr.vertices()[0], &view, &proj),
                self.vertex_shader.process(&tr.vertices()[1], &view, &proj),
                self.vertex_shader.process(&tr.vertices()[2], &view, &proj),
            );
            self.rasterizer.rasterize_triangle(
                [v0, v1, v2],
                // Свет направлен по направлению взгляда камеры.
                // Так как на данном этапе всё находится в view space,
                // то направление взгляда известно.
                Direction3::new_unchecked(0.0, 0.0, 1.0),
                &mut self.z_buffer,
                &mut self.frame_buffer,
                &self.fragment_shader,
            )
        }
    }

    pub fn frame(&self, output: &impl OutputFormatter) -> String {
        output.frame_to_string(&self.frame_buffer)
    }
}
