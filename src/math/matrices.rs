use std::ops::Index;

use crate::math::vectors::{Normal3, Vector3};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix3 {
    data: [[f32; 3]; 3],
}

impl Matrix3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }

    fn inv(&self) -> Option<Matrix3> {
        let det = self.det();
        if det.abs() < f32::EPSILON {
            return None; // Вырожденная матрица
        }

        let inv_det = 1.0 / det;

        // Миноры (через детерминанты 2x2)
        let m00 = self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1];
        let m01 = self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0];
        let m02 = self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0];

        let m10 = self.data[0][1] * self.data[2][2] - self.data[0][2] * self.data[2][1];
        let m11 = self.data[0][0] * self.data[2][2] - self.data[0][2] * self.data[2][0];
        let m12 = self.data[0][0] * self.data[2][1] - self.data[0][1] * self.data[2][0];

        let m20 = self.data[0][1] * self.data[1][2] - self.data[0][2] * self.data[1][1];
        let m21 = self.data[0][0] * self.data[1][2] - self.data[0][2] * self.data[1][0];
        let m22 = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];

        // Матрица алгебраических дополнений с транспонированием
        Some(Matrix3::new([
            [m00 * inv_det, -m10 * inv_det, m20 * inv_det],
            [-m01 * inv_det, m11 * inv_det, -m21 * inv_det],
            [m02 * inv_det, -m12 * inv_det, m22 * inv_det],
        ]))
    }

    fn det(&self) -> f32 {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            - self.data[0][1]
                * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
            + self.data[0][2]
                * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }

    pub fn transpose(&self) -> Matrix3 {
        Matrix3 {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2]],
            ],
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Matrix4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(dx: f32, dy: f32, dz: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, dx],
                [0.0, 1.0, 0.0, dy],
                [0.0, 0.0, 1.0, dz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut res = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        res
    }

    fn upper_3x3(&self) -> Matrix3 {
        Matrix3::new([
            [self.data[0][0], self.data[0][1], self.data[0][2]],
            [self.data[1][0], self.data[1][1], self.data[1][2]],
            [self.data[2][0], self.data[2][1], self.data[2][2]],
        ])
    }
}

impl Index<usize> for Matrix3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub trait Transformer<T> {
    fn transform(&self, t: T) -> T;
}

impl Transformer<Vector3> for Matrix4 {
    fn transform(&self, v: Vector3) -> Vector3 {
        let x =
            self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3];
        let y =
            self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3];
        let z =
            self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3];
        Vector3::new(x, y, z)
    }
}

impl Transformer<Normal3> for Matrix4 {
    fn transform(&self, n: Normal3) -> Normal3 {
        let m3 = self.upper_3x3();
        let inv = m3.inv().unwrap().transpose();
        let x = inv[0][0] * n.x + inv[0][1] * n.y + inv[0][2] * n.z;
        let y = inv[1][0] * n.x + inv[1][1] * n.y + inv[1][2] * n.z;
        let z = inv[2][0] * n.x + inv[2][1] * n.y + inv[2][2] * n.z;
        Vector3::new(x, y, z).normalize().unwrap() // Афинные преобразования сохраняет вектор
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vectors::Vector3;

    #[test]
    fn test_matrix_identity() {
        let identity = Matrix4::identity();

        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert_eq!(identity.data[i][j], 1.0);
                } else {
                    assert_eq!(identity.data[i][j], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_matrix_translation() {
        let translation = Matrix4::translation(2.0, 3.0, 4.0);
        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 2.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(expected, translation);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix4::new([
            [1.0, 2.0, 0.0, 0.0],
            [3.0, 4.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Matrix4::new([
            [5.0, 6.0, 0.0, 0.0],
            [7.0, 8.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result = a.multiply(&b);
        let expected = Matrix4::new([
            [19.0, 22.0, 0.0, 0.0],
            [43.0, 50.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_matrix_vector_transformation() {
        let v = Vector3::new(4.0, 5.0, 6.0);
        let translation = Matrix4::translation(1.0, 2.0, 3.0);
        let result = translation.transform(v);
        let expected = Vector3::new(5.0, 7.0, 9.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_identity_apply() {
        let identity = Matrix4::identity();
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = identity.transform(v);
        assert_eq!(v, result);
    }

    #[test]
    fn test_matrix_multiplication_identity() {
        let identity = Matrix4::identity();
        let matrix = Matrix4::translation(1.0, 2.0, 3.0);

        let result1 = identity.multiply(&matrix);
        let result2 = matrix.multiply(&identity);

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(result1.data[i][j], matrix.data[i][j]);
                assert_eq!(result2.data[i][j], matrix.data[i][j]);
            }
        }
    }
}
