use std::fmt::Display;

pub const ASCII_GRADIENT: &str =
    " .\'`^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
//" .:-=+*#%@";

pub struct FrameBuffer {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![ASCII_GRADIENT.as_bytes()[0] as char; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, intensity: f32) {
        if x >= self.width || y >= self.height {
            panic!(
                "out of bounds ({}, {}): {}, {}",
                self.width, self.height, x, y
            );
        }
        let idx = ((ASCII_GRADIENT.len() - 1) as f32 * intensity) as usize;
        let ch = ASCII_GRADIENT.as_bytes()[idx] as char;
        self.data[y * self.width + x] = ch;
    }

    pub fn clear(&mut self) {
        self.data.fill(ASCII_GRADIENT.as_bytes()[0] as char);
    }
}

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let total = self.data.len();
        for (i, &ch) in self.data.iter().enumerate() {
            write!(f, "{}", ch)?;
            // Последний '\n' не печатается
            if (i + 1) % self.width == 0 && i < total - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
