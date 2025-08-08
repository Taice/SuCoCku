use macroquad::prelude::*;

pub struct Colors {
    pub square_color: Color,
    pub bg_color: Color,
    pub outer_color: Color,
    pub box_color: Color,
    pub normal_color: Color,
    pub window_gaps: Color,
    pub selected_window: Color,

    pub normal_font: Color,
    pub note_font: Color,
    pub cmd_font: Color,
    pub status_font: Color,

    pub cmd_bg: Color,
    pub status_bg: Color,

    pub highlight_main: Color,
    pub highlight_sub: Color,
    pub visual_highlight_color: Color,
}
impl Default for Colors {
    fn default() -> Self {
        Colors {
            square_color: WHITE,
            bg_color: WHITE,
            outer_color: BLACK,
            box_color: BLACK,
            window_gaps: BLACK,
            selected_window: GRAY,

            normal_color: DARKGRAY,
            normal_font: Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            cmd_font: BLACK,
            status_font: WHITE,

            cmd_bg: WHITE,
            status_bg: DARKGRAY,

            note_font: DARKGRAY,

            highlight_sub: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.2,
            },
            highlight_main: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.4,
            },
            visual_highlight_color: Color {
                r: 0.4,
                g: 0.4,
                b: 0.7,
                a: 1.0,
            },
        }
    }
}
