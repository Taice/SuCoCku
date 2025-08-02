use macroquad::prelude::*;

const BASE_BOX_SIZE: f32 = (500 - 8 - 8 - 12) as f32 / 9.;

pub struct Settings {
    pub outer_line: f32,
    pub box_line: f32,
    pub normal_line: f32,
    pub line_color: Color,
    pub font_color: Color,
    pub font_size: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            outer_line: 4.0,
            box_line: 4.0,
            normal_line: 2.0,
            line_color: BLACK,
            font_color: DARKGRAY,
            font_size: 20,
        }
    }
}

impl Settings {
    pub fn get_box_size(&self, min_size: f32) -> f32 {
        (min_size - self.outer_line * 2. - self.box_line * 2. - self.normal_line * 6.) / 9.
    }

    pub fn get_font_scale(&self, box_size: f32) -> f32 {
        self.font_size as f32 / (box_size / BASE_BOX_SIZE)
    }
}
