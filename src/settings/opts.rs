use crate::settings::BASE_COMMAND_FONT_SIZE;

pub struct Opts {
    pub command_font_size: u16,
    pub visual_highlight_size: f32,

    pub auto_candidate_elimination: bool,
    pub auto_fill_candidates: bool,

    pub check_input: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            command_font_size: BASE_COMMAND_FONT_SIZE,
            visual_highlight_size: 3.0,

            auto_candidate_elimination: false,
            auto_fill_candidates: false,

            check_input: true,
        }
    }
}
