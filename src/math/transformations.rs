// transformations.rs содержит матрицы преобразований, специфичные для задач компьютерной графики

use crate::math::matrices::Matrix4;
use crate::math::vectors::{UnitVector3, Vector3};

impl Matrix4 {
    pub fn view_matrix(
        forward: UnitVector3,
        up: UnitVector3,
        right: UnitVector3,
        position: Vector3,
    ) -> Self {
        Self::new([
            [right.x, right.y, right.z, -position.dot(*right)],
            [up.x, up.y, up.z, -position.dot(*up)],
            [-forward.x, -forward.y, -forward.z, position.dot(*forward)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;

        Self::new([
            [2.0 / width, 0.0, 0.0, -(right + left) / width],
            [0.0, 2.0 / height, 0.0, -(top + bottom) / height],
            [0.0, 0.0, -2.0 / depth, -(far + near) / depth],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vectors::{UnitVector3, Vector3};
    use crate::{assert_approx_eq, assert_matrix4_approx_eq};
    use std::f32::consts::SQRT_2;

    #[test]
    fn test_view_matrix_identity_position() {
        let forward = UnitVector3::new_unchecked(0.0, 0.0, 1.0); // Смотрит вдоль +Z
        let up = UnitVector3::new_unchecked(0.0, 1.0, 0.0); // Вверх по Y
        let right = UnitVector3::new_unchecked(1.0, 0.0, 0.0); // Вправо по X
        let position = Vector3::new(0.0, 0.0, 0.0); // Начало координат

        let view = Matrix4::view_matrix(forward, up, right, position);

        // Камера в (0, 0, 0) смотрит вдоль оси Z, верх направлен вдоль оси OY
        // Но так как камера направлена в +Z, а не -Z, то видовая матрица с точки зрения линейного
        // оператора представляет собой инвертирование компоненты Z.
        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0], // Z инвертирована
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_matrix4_approx_eq!(view, expected, f32::EPSILON);
    }

    #[test]
    fn test_view_matrix_translated() {
        let forward = UnitVector3::new_unchecked(0.0, 0.0, 1.0);
        let up = UnitVector3::new_unchecked(0.0, 1.0, 0.0);
        let right = UnitVector3::new_unchecked(1.0, 0.0, 0.0);
        let position = Vector3::new(2.0, 3.0, 5.0); // Камера смещена

        let view = Matrix4::view_matrix(forward, up, right, position.clone());

        // Позиция камеры априори должна быть транслирована в (0, 0, 0).
        let camera_pos = view.transform(position);
        assert_approx_eq!(camera_pos.x, 0.0, f32::EPSILON);
        assert_approx_eq!(camera_pos.y, 0.0, f32::EPSILON);
        assert_approx_eq!(camera_pos.z, 0.0, f32::EPSILON);

        // Точка перед камерой должна иметь отрицательную Z в пространстве камеры
        let point_in_front = position + Vector3::new(0.0, 0.0, 1.0);
        let transformed = view.transform(point_in_front);
        assert_approx_eq!(transformed.z, -1.0, f32::EPSILON);
    }

    #[test]
    fn test_view_matrix_rotated_45_degrees() {
        // Камера смотрит под углом 45 между +X и +Z
        let angle = 45.0_f32.to_radians();
        let forward = UnitVector3::new_unchecked(angle.sin(), 0.0, angle.cos());
        let up = UnitVector3::new_unchecked(0.0, 1.0, 0.0);
        let right = UnitVector3::new_unchecked(angle.cos(), 0.0, -angle.sin());
        let position = Vector3::new(0.0, 0.0, 0.0);

        let view = Matrix4::view_matrix(forward, up, right, position);

        // Проверяем, что ось forward камеры действительно смотрит в правильном направлении
        // В мировых координатах forward = (sin45, 0, cos45)
        // В пространстве камеры это должно стать (0, 0, -1) - вперед по -Z

        let world_forward = Vector3::new(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0); // Примерно forward
        let camera_forward = view.transform(world_forward);

        // В пространстве камеры forward должен быть направлен вдоль -Z
        assert_approx_eq!(camera_forward.x, 0.0, f32::EPSILON);
        assert_approx_eq!(camera_forward.y, 0.0, f32::EPSILON);
        assert_approx_eq!(camera_forward.z, -1.0, f32::EPSILON);

        // Точка справа от камеры
        let world_right = Vector3::new(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0);
        let camera_right = view.transform(world_right);
        assert_approx_eq!(camera_right.x, 1.0, f32::EPSILON); // В пространстве камеры +X
        assert_approx_eq!(camera_right.y, 0.0, f32::EPSILON);
        assert_approx_eq!(camera_right.z, 0.0, f32::EPSILON);
    }

    #[test]
    fn test_orthographic_coordinate_mapping() {
        let ortho = Matrix4::orthographic(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0);

        // Левый нижний угол -> (-1, -1) в NDC
        let left_bottom = Vector3::new(-10.0, -10.0, -50.0);
        let result = ortho.transform(left_bottom);
        assert_approx_eq!(result.x, -1.0, f32::EPSILON);
        assert_approx_eq!(result.y, -1.0, f32::EPSILON);

        // Правый верхний угол -> (1, 1) в NDC
        let right_top = Vector3::new(10.0, 10.0, -50.0);
        let result = ortho.transform(right_top);
        assert_approx_eq!(result.x, 1.0, f32::EPSILON);
        assert_approx_eq!(result.y, 1.0, f32::EPSILON);

        // Центр -> (0, 0) в NDC
        let center = Vector3::new(0.0, 0.0, -50.0);
        let result = ortho.transform(center);
        assert_approx_eq!(result.x, 0.0, f32::EPSILON);
        assert_approx_eq!(result.y, 0.0, f32::EPSILON);
    }

    #[test]
    fn test_orthographic_z_mapping() {
        let ortho = Matrix4::orthographic(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);

        // Near plane -> -1 в NDC
        let near_point = Vector3::new(0.0, 0.0, -1.0);
        let result = ortho.transform(near_point);
        assert_approx_eq!(result.z, -1.0, f32::EPSILON);

        // Far plane -> 1 в NDC
        let far_point = Vector3::new(0.0, 0.0, -10.0);
        let result = ortho.transform(far_point);
        assert_approx_eq!(result.z, 1.0, f32::EPSILON * 10.0);

        // Середина -> 0 в NDC
        let mid_point = Vector3::new(0.0, 0.0, -(1.0 + 10.0) / 2.0);
        let result = ortho.transform(mid_point);
        assert_approx_eq!(result.z, 0.0, f32::EPSILON);
    }
}
