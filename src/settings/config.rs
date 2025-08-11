use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Option<Colors>,
    pub lines: Option<Lines>,
    pub opts: Option<Opts>,
    pub keymaps: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub square_color: Option<[f32; 4]>,
    pub bg_color: Option<[f32; 4]>,
    pub outer_line: Option<[f32; 4]>,
    pub box_line: Option<[f32; 4]>,
    pub normal_line: Option<[f32; 4]>,
    pub window_gaps: Option<[f32; 4]>,
    pub selected_window: Option<[f32; 4]>,
    pub selected_tab: Option<[f32; 4]>,
    pub selected_tab_font: Option<[f32; 4]>,
    pub inactive_tab_font: Option<[f32; 4]>,
    pub inactive_tab_color: Option<[f32; 4]>,

    pub normal_font_color: Option<[f32; 4]>,
    pub note_font_color: Option<[f32; 4]>,
    pub cmd_font_color: Option<[f32; 4]>,
    pub status_font_color: Option<[f32; 4]>,

    pub cmd_bg_color: Option<[f32; 4]>,
    pub status_bg_color: Option<[f32; 4]>,

    pub highlight_color: Option<[f32; 4]>,
    pub visual_highlight_color: Option<[f32; 4]>,

    pub invalid_color: Option<[f32; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct Lines {
    pub outer_line_width: Option<f32>,
    pub box_line_width: Option<f32>,
    pub normal_line_width: Option<f32>,
    pub window_gaps: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub outer_gaps: Option<f32>,
    pub command_font_size: Option<u16>,
    pub tabline_font_size: Option<u16>,
    pub tabline_gap: Option<f32>,
    pub visual_highlight_size: Option<f32>,

    pub auto_candidate_elimination: Option<bool>,
    pub auto_fill_candidates: Option<bool>,

    pub check_input: Option<bool>,

    pub highlight_square_instead_of_note: Option<bool>,

    pub remove_invalid: Option<bool>,
}
