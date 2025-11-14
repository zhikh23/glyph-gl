use crate::math::vectors::UnitVector3;

pub struct Face {
    pub indices: [usize; 3],
    pub normal: UnitVector3,
}

impl Face {
    pub fn new(a: usize, b: usize, c: usize, normal: UnitVector3) -> Self {
        Self {
            indices: [a, b, c],
            normal,
        }
    }
}
