use crate::math::vectors::{Direction3, Normal3, UnitVector3, Vector2, Vector3};
use crate::rendering::frame_buffer::FrameBuffer;
use crate::rendering::pipeline::fragment_shader::FragmentShader;
use crate::rendering::pipeline::vertex_shader::ProcessedVertex;
use crate::rendering::z_buffer::ZBuffer;

struct ScreenBounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl ScreenBounds {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> ScreenBounds {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            min_x: self.min_x.max(other.min_x),
            max_x: self.max_x.min(other.max_x),
            min_y: self.min_y.max(other.min_y),
            max_y: self.max_y.min(other.max_y),
        }
    }
}

pub struct TriangleRasterizer {
    width: usize,
    height: usize,
}

impl TriangleRasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn rasterize_triangle(
        &self,
        processed: [ProcessedVertex; 3],
        light: Direction3,
        z_buffer: &mut ZBuffer,
        frame_buffer: &mut FrameBuffer,
        fragment_shader: &FragmentShader,
    ) {
        if Self::is_triangle_in_frustum(&processed) || Self::is_backface(&processed) {
            return;
        }
        let screen_triangle = processed.clone().map(|pv| self.ndc_to_screen(pv.ndc_pos));
        let bounds = Self::triangle_bounds(&screen_triangle).intersect(&self.screen_bounds());
        for y in bounds.min_y..=bounds.max_y {
            for x in bounds.min_x..=bounds.max_x {
                let point = Vector2::new(x as f32, y as f32);
                if let Some(barycentric) = Self::barycentric(point, screen_triangle) {
                    let depth = Self::interpolate_depth(barycentric, &processed);
                    if z_buffer.test_and_set(x, y, depth) {
                        let normal = Self::interpolate_normal(barycentric, &processed);
                        let intensity = fragment_shader.process(normal, light);
                        frame_buffer.set(x, y, intensity);
                    }
                }
            }
        }
    }

    fn ndc_to_screen(&self, ndc: Vector3) -> Vector2 {
        let x = (ndc.x + 1.0) * 0.5 * (self.width as f32 - 1.0);
        let y = (1.0 - ndc.y) * 0.5 * (self.height as f32 - 1.0);
        Vector2::new(x, y)
    }

    fn screen_bounds(&self) -> ScreenBounds {
        ScreenBounds::new(0, self.width, 0, self.height)
    }

    fn triangle_bounds(triangle: &[Vector2; 3]) -> ScreenBounds {
        let mut bounds = ScreenBounds {
            min_x: triangle[0].x as usize,
            max_x: triangle[0].x as usize,
            min_y: triangle[0].y as usize,
            max_y: triangle[0].y as usize,
        };
        for coord in triangle[1..].iter() {
            let x = coord.x as usize;
            if x < bounds.min_x {
                bounds.min_x = x;
            } else if x > bounds.max_x {
                bounds.max_x = x;
            }

            let y = coord.y as usize;
            if y < bounds.min_y {
                bounds.min_y = y;
            } else if y > bounds.max_y {
                bounds.max_y = y;
            }
        }
        bounds
    }

    fn barycentric(point: Vector2, screen_triangle: [Vector2; 3]) -> Option<Vector3> {
        // Вычисление барицентрических координат для вектора point с заданным базисом (a, b, c)
        // сводится к решению СЛАУ. Естественно использование метода Крамера.
        let (a, b, c) = (screen_triangle[0], screen_triangle[1], screen_triangle[2]);

        let v0 = b - a;
        let v1 = c - a;
        let v2 = point - a;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        // Проверка на вырожденность треугольника
        let denom = d00 * d11 - d01 * d01;
        if denom.abs() < f32::EPSILON {
            return None;
        }

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        // Проверка, что точка оказалась внутри треугольника
        if u >= 0.0 && v >= 0.0 && w >= 0.0 {
            Some(Vector3::new(u, v, w))
        } else {
            None
        }
    }

    fn interpolate_depth(barycentric: Vector3, vertices: &[ProcessedVertex; 3]) -> f32 {
        let (v0, v1, v2) = (&vertices[0], &vertices[1], &vertices[2]);
        barycentric.x * v0.view_pos.z
            + barycentric.y * v1.view_pos.z
            + barycentric.z * v2.view_pos.z
    }

    fn interpolate_normal(barycentric: Vector3, vertices: &[ProcessedVertex; 3]) -> Normal3 {
        let (v0, v1, v2) = (&vertices[0], &vertices[1], &vertices[2]);
        (barycentric.x * (*v0.view_nor) * v0.inv_w
            + barycentric.y * (*v1.view_nor) * v1.inv_w
            + barycentric.z * (*v2.view_nor) * v2.inv_w)
            .normalize()
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0))
    }

    fn is_triangle_in_frustum(vertices: &[ProcessedVertex; 3]) -> bool {
        let min_x = vertices
            .iter()
            .fold(f32::INFINITY, |acc, v| acc.min(v.ndc_pos.x));
        let max_x = vertices
            .iter()
            .fold(f32::NEG_INFINITY, |acc, v| acc.max(v.ndc_pos.x));
        let min_y = vertices
            .iter()
            .fold(f32::INFINITY, |acc, v| acc.min(v.ndc_pos.y));
        let max_y = vertices
            .iter()
            .fold(f32::NEG_INFINITY, |acc, v| acc.max(v.ndc_pos.y));

        min_x > 1.0 || max_x < -1.0 || min_y > 1.0 || max_y < -1.0
    }

    fn is_backface(vertices: &[ProcessedVertex; 3]) -> bool {
        let (v0, v1, v2) = (&vertices[0], &vertices[1], &vertices[2]);

        let edge1 = v1.ndc_pos - v0.ndc_pos;
        let edge2 = v2.ndc_pos - v0.ndc_pos;

        let cross = edge1.y * edge2.x - edge1.x * edge2.y;

        cross <= 0.0
    }
}
