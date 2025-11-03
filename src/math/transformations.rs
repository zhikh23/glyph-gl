// transformations.rs содержит матрицы преобразований, специфичные для задач компьютерной графики

use crate::math::matrices::Matrix4;
use crate::math::vectors::{UnitVector3, Vector3};

impl Matrix4 {
    pub fn view_matrix(
        forward: UnitVector3,
        up: UnitVector3,
        right: UnitVector3,
        position: Vector3,
    ) -> Matrix4 {
        Self::new([
            [right.x, right.y, right.z, -position.dot(&right)],
            [up.x, up.y, up.z, -position.dot(&up)],
            [-forward.x, -forward.y, -forward.z, position.dot(&forward)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_matrix4_approx_eq;
    use crate::math::vectors::{UnitVector3, Vector3};

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
            [0.0, 0.0, -1.0, 0.0], // Z инвертирована!
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_matrix4_approx_eq!(view, expected, f32::EPSILON);
    }
}
