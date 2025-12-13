use crate::camera::base::Camera;
use crate::math::matrices::Matrix4;
use crate::math::vectors::{UnitVector3, Vector3};

pub struct FPVCamera {
    position: Vector3,
    yaw: f32,   // Поворот вокруг Oy: влево/вправо
    pitch: f32, // Поворот вокруг оси Ox: верх/вниз
    size: (f32, f32),
}

impl FPVCamera {
    pub fn new(position: Vector3, yaw: f32, pitch: f32, size: (f32, f32)) -> Self {
        Self {
            position,
            yaw,
            pitch,
            size,
        }
    }

    pub fn translate(&mut self, movement: Vector3) {
        self.position += movement;
    }

    pub fn rotate_yaw(&mut self, degrees: f32) {
        self.yaw += degrees;
    }

    pub fn rotate_pitch(&mut self, degrees: f32) {
        self.pitch += degrees;
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn yaw(&self) -> &f32 {
        &self.yaw
    }

    pub fn pitch(&self) -> &f32 {
        &self.pitch
    }
}

impl Camera for FPVCamera {
    fn view(&self) -> Matrix4 {
        let forward = UnitVector3::new_unchecked(
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
        );
        let world_up = UnitVector3::new_unchecked(0.0, 1.0, 0.0);
        let right = world_up.cross(*forward).normalize().unwrap();
        let up = forward.cross(*right).normalize().unwrap();
        Matrix4::view_matrix(forward, up, right, self.position)
    }

    fn proj(&self) -> Matrix4 {
        Matrix4::orthographic(
            -self.size.0 / 2.0,
            self.size.0 / 2.0,
            -self.size.1 / 2.0,
            self.size.1 / 2.0,
            0.1,
            100.0,
        )
    }
}
