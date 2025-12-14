use crate::camera::base::Camera;
use crate::math::matrices::Matrix4;
use crate::math::vectors::{UnitVector3, Vector3};

pub struct LookAtCamera {
    pub eye: Vector3,
    pub target: Vector3,
    pub frustum_size: (f32, f32),
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl LookAtCamera {
    pub fn new(
        eye: Vector3,
        target: Vector3,
        frustum_size: (f32, f32),
        fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            eye,
            target,
            frustum_size,
            fov,
            near,
            far,
        }
    }

    pub fn orbit_around_target(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let distance = (self.eye - self.target).length();

        let direction = (self.eye - self.target)
            .normalize()
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0));
        let current_yaw = direction.x.atan2(direction.z); // atan2(z, x) для Y вверх
        let current_pitch = direction.y.asin();

        let new_yaw = current_yaw + delta_yaw.to_radians();
        let new_pitch = (current_pitch + delta_pitch.to_radians())
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        self.eye = Vector3::new(
            self.target.x + distance * new_pitch.cos() * new_yaw.sin(),
            self.target.y + distance * new_pitch.sin(),
            self.target.z + distance * new_pitch.cos() * new_yaw.cos(),
        );
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.target - self.eye)
            .normalize()
            // Здесь fallback оправдан, так как камера теоретически может оказаться вплотную к
            // объекту - в таком случае любое направление можно принять за правильное
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0));
        let current_distance = (self.target - self.eye).length();

        // Не позволяем приблизиться к цели ближе, чем near
        let new_distance = (current_distance + delta).max(self.near);
        self.eye = self.target - (*direction) * new_distance;
    }

    pub fn eye(&self) -> &Vector3 {
        &self.eye
    }

    pub fn target(&self) -> &Vector3 {
        &self.target
    }
}

impl Camera for LookAtCamera {
    fn view(&self) -> Matrix4 {
        let forward = (self.target - self.eye)
            .normalize()
            // Здесь fallback оправдан, так как камера теоретически может оказаться вплотную к
            // объекту - в таком случае любое направление можно принять за правильное
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0));
        let world_up = Vector3::new(0.0, 1.0, 0.0);
        let right = world_up.cross(*forward).normalize().unwrap();
        let up = forward.cross(*right).normalize().unwrap();
        Matrix4::view_matrix(forward, up, right, self.eye)
    }

    /// Возвращает перспективную матрицу проекции
    fn proj(&self) -> Matrix4 {
        Matrix4::perspective(
            self.fov,
            self.frustum_size.0 / self.frustum_size.1,
            self.near,
            self.far,
        )
    }
}
