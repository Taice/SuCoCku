use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Option<Colors>,
    pub lines: Option<Lines>,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub outer_line: Option<[f32; 4]>,
    pub box_line: Option<[f32; 4]>,
    pub normal_line: Option<[f32; 4]>,

    pub normal_font_color: Option<[f32; 4]>,
    pub note_font_color: Option<[f32; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct Lines {
    pub outer_line_width: Option<f32>,
    pub box_line_width: Option<f32>,
    pub normal_line_width: Option<f32>,
}
