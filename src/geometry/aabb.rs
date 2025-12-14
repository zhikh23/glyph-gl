use crate::math::vectors::Vector3;

#[derive(Debug)]
pub struct Aabb {
    min: Vector3,
    max: Vector3,
}

impl Aabb {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Self {
        Self {
            min: Vector3::new(left, bottom, far),
            max: Vector3::new(right, top, near),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn depth(&self) -> f32 {
        self.max.z - self.min.z
    }

    pub fn size(&self) -> Vector3 {
        Vector3::new(self.width(), self.height(), self.depth())
    }

    pub fn max_extent(&self) -> f32 {
        let size = self.size();
        size.x.max(size.y).max(size.z)
    }
}
