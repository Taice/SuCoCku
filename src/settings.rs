pub mod config;

use config::Config;

use macroquad::prelude::*;

macro_rules! assign_if_some {
    ($target:expr, $field:ident, $opt:expr) => {
        if let Some(val) = $opt {
            $target.$field = val;
        }
    };
}
macro_rules! assign_if_some_map {
    ($target:expr, $field:ident, $opt:expr, $map:expr) => {
        if let Some(val) = $opt {
            $target.$field = $map(val);
        }
    };
}

pub const NUM_FONT_SIZE: u16 = 120;
pub const NUM_FONT_SCALE: f32 = 0.5;
pub const NOTE_FONT_SIZE: u16 = 33;
pub const NOTE_FONT_SCALE: f32 = 0.5;

pub const BASE_BOX_SIZE: f32 = (500 - 8 - 8 - 12) as f32 / 9.;

pub struct Settings {
    pub outer_line: f32,
    pub box_line: f32,
    pub normal_line: f32,

    pub outer_color: Color,
    pub box_color: Color,
    pub normal_color: Color,

    pub normal_font_color: Color,
    pub note_font_color: Color,

    pub font: Font,
}

impl Settings {
    pub fn from_config(config: &Option<Config>) -> Self {
        let font = load_ttf_font_from_bytes(include_bytes!("../assets/Roboto-Regular.ttf"))
            .expect("WTF you do bro.");
            let mut default = Self::new(font);
        if let Some(config) = config {

            if let Some(lines) = &config.lines {
                assign_if_some!(default, outer_line, lines.outer_line_width);
                assign_if_some!(default, box_line, lines.box_line_width);
                assign_if_some!(default, normal_line, lines.normal_line_width);
            }

            if let Some(colors) = &config.colors {
                let into = |c: [f32; 4]| Color {
                    r: c[0],
                    g: c[1],
                    b: c[2],
                    a: c[3],
                };
                assign_if_some_map!(default, outer_color, colors.outer_line, into);
                assign_if_some_map!(default, box_color, colors.box_line, into);
                assign_if_some_map!(default, normal_color, colors.normal_line, into);
                assign_if_some_map!(default, normal_font_color, colors.note_font_color, into);
                assign_if_some_map!(default, note_font_color, colors.normal_font_color, into);
            }
        }     
        default
    }
    pub fn new(font: Font) -> Self {
        Self {
            outer_line: 5.0,
            box_line: 4.0,
            normal_line: 2.0,
            outer_color: BLACK,
            box_color: BLACK,
            normal_color: DARKGRAY,
            normal_font_color: Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },

            note_font_color: DARKGRAY,

            font
        }
    }
    pub fn get_lengths(&self, min_size: f32) -> (f32, f32) {
        let offset = self.outer_line * 2.0 + self.box_line * 2.0 + self.normal_line * 6.0;
        let s = min_size - offset;
        let rem = s % 9.;
        let a = min_size - rem;
        (a, (s - rem) / 9.)
    }

    pub fn get_num_font_size(&self, box_size: f32) -> u16 {
        (NUM_FONT_SIZE as f32 * (box_size / BASE_BOX_SIZE)) as u16
    }
    pub fn get_note_font_size(&self, box_size: f32) -> u16 {
        (NOTE_FONT_SIZE as f32 * (box_size / BASE_BOX_SIZE)) as u16
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
