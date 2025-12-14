use crate::math::vectors::{Direction3, Normal3};

pub struct FragmentShader {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: i32,
}

impl FragmentShader {
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: i32) -> FragmentShader {
        FragmentShader {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn process(&self, normal: Normal3, light: Direction3) -> f32 {
        let diffuse = normal.dot(light).max(0.0) * self.diffuse;
        let reflect_dir = reflect(-light, normal);
        let spec = reflect_dir
            // (0, 0. 1) - направление взгляда камеры в view space.
            .dot(Direction3::new_unchecked(0.0, 0.0, 1.0))
            .max(0.0);
        let specular = spec.powi(self.shininess) * self.specular;
        (self.ambient + diffuse + specular).clamp(0.0, 1.0)
    }
}

fn reflect(incident: Direction3, normal: Normal3) -> Direction3 {
    (*incident - 2.0 * incident.dot(normal) * (*normal))
        .normalize()
        .unwrap()
}
