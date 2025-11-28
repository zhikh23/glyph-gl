use crate::rendering::frame_buffer::FrameBuffer;

pub trait OutputFormatter {
    fn frame_to_string(&self, buffer: &FrameBuffer) -> String;
}
