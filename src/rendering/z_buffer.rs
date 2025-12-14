#[derive(Debug)]
pub struct ZBuffer {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![f32::NEG_INFINITY; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(f32::NEG_INFINITY);
    }

    pub fn test_and_set(&mut self, x: usize, y: usize, z: f32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let value = self.data.get_mut(y * self.width + x).unwrap();
        if z > *value {
            *value = z;
            true
        } else {
            false
        }
    }
}
