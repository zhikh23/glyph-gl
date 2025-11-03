use crate::math::vectors::Vector3;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
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

    pub fn apply(&self, v: &Vector3) -> Vector3 {
        let x =
            self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3];
        let y =
            self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3];
        let z =
            self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3];
        Vector3::new(x, y, z)
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

        // Проверяем структуру матрицы трансляции
        assert_eq!(translation.data[0][0], 1.0);
        assert_eq!(translation.data[0][3], 2.0); // x translation

        assert_eq!(translation.data[1][1], 1.0);
        assert_eq!(translation.data[1][3], 3.0); // y translation

        assert_eq!(translation.data[2][2], 1.0);
        assert_eq!(translation.data[2][3], 4.0); // z translation

        assert_eq!(translation.data[3][3], 1.0);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix4 {
            data: [
                [1.0, 2.0, 0.0, 0.0],
                [3.0, 4.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let b = Matrix4 {
            data: [
                [5.0, 6.0, 0.0, 0.0],
                [7.0, 8.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let result = a.multiply(&b);
        let expected = Matrix4 {
            data: [
                [19.0, 22.0, 0.0, 0.0],
                [43.0, 50.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_matrix_vector_transformation() {
        let v = Vector3::new(4.0, 5.0, 6.0);
        let translation = Matrix4::translation(1.0, 2.0, 3.0);
        let result = translation.apply(&v);
        let expected = Vector3::new(5.0, 7.0, 9.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_identity_apply() {
        let identity = Matrix4::identity();
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = identity.apply(&v);
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
