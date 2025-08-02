mod settings;
use settings::Settings;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sudoku".to_string(),
        window_width: 500,
        window_height: 500,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut sudoku = Sudoku::default();
    let mut settings = Settings::default();
    loop {
        clear_background(GRAY);
        sudoku.draw(&settings);
        next_frame().await
    }
}

#[derive(Default)]
pub struct Sudoku {
    nums: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn draw(&self, s: &Settings) {
        let min_size = f32::min(screen_width(), screen_height());
        let box_size = s.get_box_size(min_size);

        draw_rectangle(0., 0., min_size, min_size, WHITE);

        let mut half = s.outer_line / 2.;
        // Draw Sudoku lines
        draw_line(0.0, half, min_size, half, s.outer_line, s.line_color); // TOP_LEFT_RIGHT
        draw_line(half, 0.0, half, min_size, s.outer_line, s.line_color); // TOP_LEFT_BOT
        draw_line(
            min_size,
            0.0,
            min_size,
            min_size,
            s.outer_line,
            s.line_color,
        ); // TOP_RIGHT_BOT    
        draw_line(
            half,
            min_size - half,
            min_size,
            min_size - half,
            s.outer_line,
            s.line_color,
        ); // BOT_LEFT_RIGHT

        for n in 1..=8 {
            let point;
            let width;
            let i = n as f32;
            if n % 3 == 0 {
                half = s.box_line / 2.;
                point = s.outer_line
                    + half
                    + i * box_size
                    + i / 3. * s.box_line
                    + (i - i / 3.) * s.normal_line;
                width = s.box_line;
            } else {
                half = s.normal_line / 2.;
                point = s.outer_line
                    + half
                    + i * box_size
                    + (n / 3) as f32 * s.box_line
                    + (n - (n / 3)) as f32 * s.normal_line;
                width = s.normal_line;
            }
            draw_line(0.0, point, min_size, point, width, s.line_color);
            draw_line(point, 0.0, point, min_size, width, s.line_color);
        }

        let mut x = s.outer_line;
        let mut y = s.outer_line;

        for (i, row) in self.nums.iter().enumerate() {
            y = s.outer_line
                + (i / 3) as f32 * s.box_line
                + (i - (i / 3)) as f32 * s.normal_line
                + i as f32 * box_size;
            if i != 0 {
                if i % 3 == 0 {
                    y += s.box_line;
                } else {
                    y += s.normal_line;
                }
            }
            for (j, n) in row.iter().enumerate() {
                draw_text_ex(
                    &self.nums[i][j].to_string(),
                    x,
                    y,
                    TextParams {
                        font_size: s.font_size,
                        font_scale: s.get_font_scale(box_size),
                        color: s.font_color,
                        ..Default::default()
                    },
                );
                draw_rectangle(x, y, 4., 4., GREEN);
                x += box_size;
                if j % 3 == 0 {
                    x += s.box_line;
                } else {
                    x += s.normal_line;
                }
            }
            x = s.outer_line;

            y += box_size;
        }
        println!();
    }
}
