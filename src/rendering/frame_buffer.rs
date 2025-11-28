pub struct FrameBuffer {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            panic!(
                "out of bounds ({}, {}): {}, {}",
                self.width, self.height, x, y
            );
        }
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, intensity: f32) {
        if x >= self.width || y >= self.height {
            panic!(
                "out of bounds ({}, {}): {}, {}",
                self.width, self.height, x, y
            );
        }
        self.data[y * self.width + x] = intensity;
    }

    pub fn clear(&mut self) {
        self.data.fill(0.0)
    }
}
