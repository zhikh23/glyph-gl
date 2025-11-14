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

    pub fn view_matrix(&self) -> Matrix4 {
        let forward = UnitVector3::new_unchecked(
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
        );
        let world_up = UnitVector3::new_unchecked(0.0, 1.0, 0.0);
        let right = world_up.cross(*forward).normalize().unwrap();
        let up = forward.cross(*right).normalize().unwrap();

        let proj_matrix = Matrix4::orthographic(
            -self.size.0 / 2.0,
            self.size.0 / 2.0,
            -self.size.1 / 2.0,
            self.size.1 / 2.0,
            0.1,
            100.0,
        );

        let view_matrix = Matrix4::view_matrix(forward, up, right, self.position);
        proj_matrix.multiply(&view_matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_camera_look_forward() {
        let camera = FPVCamera::new(Vector3::new(0.0, 0.0, 5.0), 0.0, 0.0, (4.0, 3.0));

        let view_matrix = camera.view_matrix();

        let point_in_front = Vector3::new(0.0, 0.0, 0.0);
        let transformed = view_matrix.transform(point_in_front);

        assert_approx_eq!(transformed.x, 0.0, f32::EPSILON);
        assert_approx_eq!(transformed.y, 0.0, f32::EPSILON);
        assert!(transformed.z < 0.0);
    }

    #[test]
    fn test_camera_yaw_rotation() {
        let camera = FPVCamera::new(
            Vector3::new(0.0, 0.0, 0.0),
            90.0, // Камера смотрит вправо
            0.0,
            (4.0, 3.0),
        );

        let view_matrix = camera.view_matrix();

        let point_right = Vector3::new(1.0, 0.0, 0.0);
        let transformed = view_matrix.transform(point_right);

        assert!(transformed.z < 0.0);
        assert_approx_eq!(transformed.x, 0.0, f32::EPSILON);
    }

    #[test]
    fn test_coordinate_system_orientation() {
        let camera = FPVCamera::new(Vector3::new(0.0, 0.0, 0.0), 0.0, 0.0, (4.0, 3.0));
        let view_matrix = camera.view_matrix();

        let test_right = Vector3::new(1.0, 0.0, 0.0);
        let test_up = Vector3::new(0.0, 1.0, 0.0);
        let test_forward = Vector3::new(0.0, 0.0, -1.0);

        let transformed_right = view_matrix.transform(test_right);
        let transformed_up = view_matrix.transform(test_up);
        let transformed_forward = view_matrix.transform(test_forward);

        assert!(transformed_right.x > 0.0);
        assert!(transformed_up.y > 0.0);
        assert!(transformed_forward.z < 0.0);
    }

    #[test]
    fn test_camera_movement_consistency() {
        let camera1 = FPVCamera::new(Vector3::new(0.0, 0.0, 5.0), 0.0, 0.0, (4.0, 3.0));
        let camera2 = FPVCamera::new(Vector3::new(2.0, 0.0, 5.0), 0.0, 0.0, (4.0, 3.0));

        let matrix1 = camera1.view_matrix();
        let matrix2 = camera2.view_matrix();

        let test_point = Vector3::new(0.0, 0.0, 0.0);
        let result1 = matrix1.transform(test_point);
        let result2 = matrix2.transform(test_point);

        assert!(result1.x > result2.x, "{} > {}", result1.x, result2.x);
    }
}
