use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn dot(&self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Option<UnitVector3> {
        let len = self.length();
        if len > 0.0 {
            Some(UnitVector3(self * (1.0 / len)))
        } else {
            // Невозможно нормализовать нулевой вектор
            None
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitVector3(Vector3);

impl UnitVector3 {
    pub fn new_unchecked(x: f32, y: f32, z: f32) -> UnitVector3 {
        UnitVector3(Vector3::new(x, y, z))
    }

    pub fn dot(&self, rhs: UnitVector3) -> f32 {
        self.0.dot(rhs.0)
    }
}

impl Deref for UnitVector3 {
    type Target = Vector3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn dot(&self, rhs: Vector2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
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
        let result = v1.dot(v2);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let result = v1.cross(v2);
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
        let normalized = v.normalize().unwrap();
        let expected = UnitVector3::new_unchecked(1.0, 0.0, 0.0);
        assert_eq!(normalized, expected);
    }

    #[test]
    fn test_vector_normalize_zero() {
        let v = Vector3::new(0.0, 0.0, 0.0);
        let res = v.normalize();
        assert!(res.is_none());
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
