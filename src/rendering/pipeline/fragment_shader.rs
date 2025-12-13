use crate::math::vectors::{Direction3, Normal3, UnitVector3, Vector3};

pub struct FragmentShader {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl FragmentShader {
    pub fn new(ambient: f32, diffuse: f32) -> FragmentShader {
        FragmentShader {
            ambient,
            diffuse,
            specular: 0.3,
            shininess: 8.0,
        }
    }

    pub fn process(&self, normal: Normal3, light: Direction3) -> f32 {
        /*
        let diffuse = normal.dot(self.light_dir).max(0.0) * self.diffuse;
        (self.ambient + diffuse).clamp(0.0, 1.0)
         */
        let diffuse = normal.dot(light).max(0.0) * self.diffuse;
        let specular = if diffuse > 0.0 {
            let reflect_dir = reflect(-light, normal);
            let spec = reflect_dir
                .dot(*Direction3::new_unchecked(0.0, 0.0, 1.0))
                .max(0.0);
            spec.powf(self.shininess) * self.specular
        } else {
            0.0
        };
        (self.ambient + diffuse + specular).clamp(0.0, 1.0)
    }
}

fn reflect(incident: UnitVector3, normal: UnitVector3) -> Vector3 {
    *incident - 2.0 * incident.dot(normal) * (*normal)
}
