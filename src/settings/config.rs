use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Option<Colors>,
    pub lines: Option<Lines>,
    pub opts: Option<Opts>,
    pub keymaps: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub square_color: Option<[f32; 4]>,
    pub bg_color: Option<[f32; 4]>,
    pub outer_line: Option<[f32; 4]>,
    pub box_line: Option<[f32; 4]>,
    pub normal_line: Option<[f32; 4]>,

    pub normal_font_color: Option<[f32; 4]>,
    pub note_font_color: Option<[f32; 4]>,
    pub cmd_font_color: Option<[f32; 4]>,
    pub cmd_bg_color: Option<[f32; 4]>,

    pub highlight_main: Option<[f32; 4]>,
    pub highlight_sub: Option<[f32; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct Lines {
    pub outer_line_width: Option<f32>,
    pub box_line_width: Option<f32>,
    pub normal_line_width: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub highlight_box: Option<bool>,
    pub highlight_in_line: Option<bool>,
    pub highlight_cell: Option<bool>,
    pub command_font_size: Option<u16>,
}
