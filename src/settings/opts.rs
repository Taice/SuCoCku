use crate::settings::{BASE_COMMAND_FONT_SIZE, BASE_TABLINE_FONT_SIZE};

pub struct Opts {
    pub outer_gaps: f32,
    pub command_font_size: u16,
    pub tabline_font_size: u16,
    pub tabline_gap: f32,
    pub visual_highlight_size: f32,

    pub auto_candidate_elimination: bool,
    pub auto_fill_candidates: bool,

    pub check_input: bool,

    pub highlight_square_instead_of_note: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            outer_gaps: 4.0,
            command_font_size: BASE_COMMAND_FONT_SIZE,
            tabline_font_size: BASE_TABLINE_FONT_SIZE,
            tabline_gap: 4.0,
            visual_highlight_size: 3.0,

            auto_candidate_elimination: false,
            auto_fill_candidates: false,

            check_input: true,

            highlight_square_instead_of_note: false,
        }
    }
}
