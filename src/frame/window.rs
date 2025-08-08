pub mod buffer;

use macroquad::prelude::*;

use buffer::Buffer;

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub dimensions: Rect,
    pub buffer_index: usize,
}

impl Window {
    pub fn new(dimensions: Rect, buffer_index: usize) -> Self {
        Self {
            dimensions,
            buffer_index,
        }
    }

    pub fn render(&self, buffer: &Buffer) {
        buffer.draw(&self.dimensions);
    }

    pub fn update(&self, buffer: &mut Buffer) {
        buffer.update();
    }
}
