#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            // Избегаем деления на нуль. Возвращать ошибку в этом месте будет дорого
            // для остальных вычислений.
            self
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        let expected = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_length() {
        let v = Vector3::new(2.0, 3.0, 6.0);
        let mag = v.length(); // 4 + 9 + 36 = 49 = 7^2
        assert_approx_eq!(mag, 7.0, f32::EPSILON);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3::new(3.0, 0.0, 0.0);
        let normalized = v.normalize();
        let expected = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(normalized, expected);
    }

    #[test]
    fn test_vector_normalize_zero() {
        let v = Vector3::new(0.0, 0.0, 0.0);
        let expected = v.clone();
        let normalized = v.normalize();

        // Нормализация нулевого вектора должна вернуть нулевой вектор
        assert_eq!(expected, normalized);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        let expected = Vector3::new(5.0, 7.0, 9.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector3::new(5.0, 5.0, 5.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        let expected = Vector3::new(4.0, 3.0, 2.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        let expected = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(result, expected);
    }
}
