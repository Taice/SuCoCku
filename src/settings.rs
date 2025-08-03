use macroquad::prelude::*;

pub const BASE_BOX_SIZE: f32 = (500 - 8 - 8 - 12) as f32 / 9.;

pub struct Settings {
    pub outer_line: f32,
    pub box_line: f32,
    pub normal_line: f32,

    pub outer_color: Color,
    pub box_color: Color,
    pub normal_color: Color,

    pub num_font_scale: f32,
    pub note_font_scale: f32,
    num_font_size: u16,
    note_font_size: u16,
    pub font_color: Color,
    pub note_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            outer_line: 5.0,
            box_line: 4.0,
            normal_line: 2.0,
            outer_color: BLACK,
            box_color: BLACK,
            normal_color: DARKGRAY,
            font_color: Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },

            num_font_size: 120,
            num_font_scale: 0.5,

            note_font_size: 33,
            note_font_scale: 0.5,

            note_color: DARKGRAY,
        }
    }
}

impl Settings {
    pub fn get_lengths(&self, min_size: f32) -> (f32, f32) {
        let offset = self.outer_line * 2.0 + self.box_line * 2.0 + self.normal_line * 6.0;
        let s = min_size - offset;
        let rem = s % 9.;
        let a = min_size - rem;
        (a, ( s - rem ) / 9.)
    }

    pub fn get_num_font_size(&self, box_size: f32) -> u16 {
        (self.num_font_size as f32 * (box_size / BASE_BOX_SIZE)) as u16
    }
    pub fn get_note_font_size(&self, box_size: f32) -> u16 {
        (self.note_font_size as f32 * (box_size / BASE_BOX_SIZE)) as u16
    }

    pub fn get_x_num_offset(&self, box_size: f32) -> f32 {
        10.0 * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_x_note_offset(&self, box_size: f32) -> f32 {
        4.0 * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_y_num_offset(&self, box_size: f32) -> f32 {
        (BASE_BOX_SIZE - 5.) * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_y_note_offset(&self, box_size: f32) -> f32 {
        (BASE_BOX_SIZE - 38.) * (box_size / BASE_BOX_SIZE)
    }
}
