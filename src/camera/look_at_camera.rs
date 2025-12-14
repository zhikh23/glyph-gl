use crate::camera::base::Camera;
use crate::math::matrices::Matrix4;
use crate::math::vectors::{UnitVector3, Vector3};

pub struct LookAtCamera {
    pub eye: Vector3,
    pub target: Vector3,
    pub size: (f32, f32),
    pub near: f32,
    pub far: f32,
}

impl LookAtCamera {
    /// Создаёт камеру, смотрящую на цель
    pub fn new(eye: Vector3, target: Vector3, ortho_size: (f32, f32)) -> Self {
        Self {
            eye,
            target,
            size: ortho_size,
            near: 0.1,
            far: 300.0,
        }
    }

    pub fn look_at(&mut self, new_target: Vector3) {
        self.target = new_target;
    }

    pub fn orbit_around_target(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let distance = (self.eye - self.target).length();

        // Вычисляем новые углы
        let direction = (self.eye - self.target)
            .normalize()
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0));
        let current_yaw = direction.x.atan2(direction.z); // atan2(z, x) для Y вверх
        let current_pitch = direction.y.asin();

        let new_yaw = current_yaw + delta_yaw.to_radians();
        let new_pitch = (current_pitch + delta_pitch.to_radians())
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        // Новая позиция камеры
        self.eye = Vector3::new(
            self.target.x + distance * new_pitch.cos() * new_yaw.sin(),
            self.target.y + distance * new_pitch.sin(),
            self.target.z + distance * new_pitch.cos() * new_yaw.cos(),
        );
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.target - self.eye)
            .normalize()
            .unwrap_or(UnitVector3::new_unchecked(0.0, 0.0, 1.0));
        let current_distance = (self.target - self.eye).length();
        let new_distance = (current_distance + delta).max(0.1);

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
        /*
        Matrix4::orthographic(
            -self.size.0 / 2.0,
            self.size.0 / 2.0,
            -self.size.1 / 2.0,
            self.size.1 / 2.0,
            self.near,
            self.far,
        )
         */
        Matrix4::perspective(
            50.0_f32.to_radians(),
            self.size.0 / self.size.1,
            self.near,
            self.far,
        )
    }
}
