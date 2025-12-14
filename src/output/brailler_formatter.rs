use crate::output::formatter::OutputFormatter;
use crate::rendering::frame_buffer::FrameBuffer;

#[derive(Default)]
pub struct BrailleColorFormatter;

impl OutputFormatter for BrailleColorFormatter {
    fn frame_to_string(&self, buffer: &FrameBuffer) -> String {
        let mut result = String::new();

        let braille_width = buffer.width().div_ceil(2);
        let braille_height = buffer.height().div_ceil(4);

        for by in 0..braille_height {
            for bx in 0..braille_width {
                let (braille_char, avg_intensity) =
                    self.compute_braille_with_intensity(buffer, bx, by);

                if avg_intensity > 0.0 {
                    let color_code = self.intensity_to_color(avg_intensity);
                    result.push_str(&color_code);
                    result.push(braille_char);
                    result.push_str("\x1b[0m");
                } else {
                    result.push(' ');
                }
            }
            result.push_str("\r\n");
        }

        result
    }
}

impl BrailleColorFormatter {
    fn compute_braille_with_intensity(
        &self,
        buffer: &FrameBuffer,
        bx: usize,
        by: usize,
    ) -> (char, f32) {
        let mut braille_bits = 0u8;
        let mut total_intensity = 0.0;
        let mut dot_count = 0;

        for dot in 0..8 {
            let (dx, dy) = Self::braille_dot_position(dot);
            let x = bx * 2 + dx;
            let y = by * 4 + dy;

            if x < buffer.width() && y < buffer.height() {
                let intensity = buffer.get(x, y);
                total_intensity += intensity;

                if intensity > 0.0 {
                    braille_bits |= 1 << dot;
                    dot_count += 1;
                }
            }
        }

        let avg_intensity = if dot_count > 0 {
            total_intensity / dot_count as f32
        } else {
            0.0
        };

        let braille_char = Self::braille_bits_to_char(braille_bits);
        (braille_char, avg_intensity)
    }

    fn intensity_to_color(&self, intensity: f32) -> String {
        let t = intensity.clamp(0.0, 1.0);
        /*
        let (r, g, b) = if t < 0.0722 {
            let b = (t / 0.0722 * 255.0) as u8;
            (0, 0, b)
        } else if t < 0.2848 {
            let r = ((t - 0.0722) / 0.2126 * 255.0) as u8;
            (r, 0, 255)
        } else {
            let g = ((t - 0.2848) / 0.7152 * 255.0) as u8;
            (255, g, 255)
        };
        */
        let (r, g, b) = ((t * 255.0) as u8, (t * 255.0) as u8, (t * 255.0) as u8);
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    fn braille_dot_position(dot_index: u8) -> (usize, usize) {
        match dot_index {
            // Стандартный порядок брайля:
            0 => (0, 0), // точка 1
            1 => (0, 1), // точка 2
            2 => (0, 2), // точка 3
            3 => (1, 0), // точка 4
            4 => (1, 1), // точка 5
            5 => (1, 2), // точка 6
            6 => (0, 3), // точка 7
            7 => (1, 3), // точка 8
            _ => (0, 0),
        }
    }

    fn braille_bits_to_char(bits: u8) -> char {
        let braille_base: u32 = 0x2800;
        if bits > 0 {
            char::from_u32(braille_base + bits as u32).unwrap_or('⠀')
        } else {
            ' '
        }
    }
}
