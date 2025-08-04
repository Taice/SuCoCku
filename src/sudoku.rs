use crate::settings::{NOTE_FONT_SCALE, NUM_FONT_SCALE, Settings};

use macroquad::prelude::*;
use std::f32;

pub struct Sudoku {
    nums: [[u16; 9]; 9],
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            nums: [[0b0; 9]; 9],
        }
    }
}

impl Sudoku {
    pub fn draw(&self, s: &Settings) {
        let min_len = f32::min(screen_width(), screen_height());
        let (side, box_size) = s.get_lengths(min_len);

        draw_rectangle(0., 0., side, side, WHITE);
        draw_inlines(s, side, box_size);
        draw_box_lines(s, side, box_size);
        draw_outlines(s, side);

        self.draw_grid(s, box_size);
    }

    fn draw_grid(&self, s: &Settings, box_size: f32) {
        let text_params = TextParams {
            font: Some(&s.font),
            font_size: s.get_num_font_size(box_size),
            font_scale: NUM_FONT_SCALE,
            color: s.normal_font_color,
            ..Default::default()
        };
        let mut y = s.outer_line;
        let mut x = s.outer_line;

        for (i, row) in self.nums.iter().enumerate() {
            let num_y = y + s.get_y_num_offset(box_size);
            let note_y = y + s.get_y_note_offset(box_size);
            for (j, n) in row.iter().enumerate() {
                if *n != 0 {
                    // note
                    if n & (1 << 15) != 0 {
                        let x = x + s.get_x_note_offset(box_size);
                        draw_notes(s, box_size, x, note_y, *n, &s.font);
                    // num
                    } else {
                        let x = x + s.get_x_num_offset(box_size);
                        draw_text_ex(&n.to_string(), x, num_y, text_params.clone());
                    }
                }

                if j % 3 == 2 {
                    x += s.box_line;
                } else {
                    x += s.normal_line;
                }
                x += box_size;
            }
            x = s.outer_line;

            if i % 3 == 2 {
                y += s.box_line;
            } else {
                y += s.normal_line;
            }
            y += box_size;
        }
    }
}

pub fn draw_box_lines(s: &Settings, side: f32, box_size: f32) {
    let mut point = box_size + s.outer_line;
    for n in 1..9 {
        if n % 3 == 0 {
            draw_rectangle(point, 0.0, s.box_line, side, s.box_color);
            draw_rectangle(0.0, point, side, s.box_line, s.box_color);
            point += s.box_line;
        } else {
            point += s.normal_line;
        }

        point += box_size;
    }
}

pub fn draw_inlines(s: &Settings, side: f32, box_size: f32) {
    let mut point = box_size + s.outer_line;
    for n in 1..9 {
        if n % 3 != 0 {
            draw_rectangle(point, 0.0, s.normal_line, side, s.normal_color);
            draw_rectangle(0.0, point, side, s.normal_line, s.normal_color);
            point += s.normal_line;
        } else {
            point += s.box_line;
        }

        point += box_size;
    }
}

pub fn draw_outlines(s: &Settings, side: f32) {
    let half = s.outer_line / 2.;
    //Draw Sudoku lines
    draw_line(0.0, half, side, half, s.outer_line, s.outer_color); // TOP_LEFT_RIGHT
    draw_line(half, 0.0, half, side, s.outer_line, s.outer_color); // TOP_LEFT_BOT
    draw_line(
        side - half,
        0.0,
        side - half,
        side,
        s.outer_line,
        s.outer_color,
    ); // TOP_RIGHT_BOT    
    draw_line(
        half,
        side - half,
        side,
        side - half,
        s.outer_line,
        s.outer_color,
    ); // BOT_LEFT_RIGHT
}

pub fn draw_notes(s: &Settings, box_size: f32, x: f32, y: f32, num: u16, font: &Font) {
    let text_params = TextParams {
        font: Some(font),
        font_size: s.get_note_font_size(box_size),
        font_scale: NOTE_FONT_SCALE,
        color: s.note_font_color,
        ..Default::default()
    };

    let note_size = box_size / 3.;
    let mut coords = (x, y);
    for i in 0..3 {
        for j in 0..3 {
            let n = i * 3 + j;
            if num & (1 << n) > 0 {
                draw_text_ex(
                    &(n + 1).to_string(),
                    coords.0,
                    coords.1,
                    text_params.clone(),
                );
            }
            coords.0 += note_size;
        }
        coords.0 = x;
        coords.1 += note_size;
    }
}
