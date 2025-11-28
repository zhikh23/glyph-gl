use crate::output::formatter::OutputFormatter;
use crate::rendering::frame_buffer::FrameBuffer;

pub struct ASCIIOutputFormatter {
    gradient: Vec<char>,
}

impl ASCIIOutputFormatter {
    pub fn new(gradient: Vec<char>) -> Self {
        Self { gradient }
    }

    pub fn from_ascii_string(ascii_string: &str) -> Self {
        let grad: Vec<char> = ascii_string.as_bytes().iter().map(|u| *u as char).collect();
        Self::new(grad)
    }

    pub fn with_default_ascii_gradient() -> Self {
        Self::from_ascii_string(" .:-=+*#%@")
    }
}

impl OutputFormatter for ASCIIOutputFormatter {
    fn frame_to_string(&self, buffer: &FrameBuffer) -> String {
        let mut s = String::new();
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let intensity = buffer.get(x, y);
                let idx = ((self.gradient.len() - 1) as f32 * intensity) as usize;
                s.push(self.gradient[idx]);
            }
            s.push('\n');
        }
        s
    }
}
