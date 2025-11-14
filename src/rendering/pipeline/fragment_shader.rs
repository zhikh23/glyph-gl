use crate::math::vectors::UnitVector3;

pub struct FragmentShader {
    light_dir: UnitVector3,
}

impl FragmentShader {
    pub fn new(light_dir: UnitVector3) -> FragmentShader {
        FragmentShader { light_dir }
    }

    pub fn process(&self, normal: UnitVector3) -> f32 {
        self.light_dir.dot(normal).clamp(0.05, 1.0)
    }
}
