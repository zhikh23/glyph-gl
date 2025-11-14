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
            data: vec![0.0; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(0.0);
    }

    pub fn test_and_set(&mut self, x: usize, y: usize, z: f32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let value = self.data.get_mut(y * self.width + x).unwrap_or_else(|| {
            panic!(
                "out of bounds ({}, {}): {}, {}",
                self.width, self.height, x, y
            )
        });
        if *value > z {
            *value = z;
            true
        } else {
            false
        }
    }
}
