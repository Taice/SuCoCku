pub mod colors;
pub mod config;
pub mod lines;
pub mod opts;

use crate::unwrap_or_else;

use std::collections::HashMap;

use colors::Colors;
use config::Config;
use lines::Lines;
use opts::Opts;

use macroquad::prelude::*;

macro_rules! assign_if_some {
    ($target:expr, $opt:expr) => {
        if let Some(val) = $opt {
            $target = val;
        }
    };
}
macro_rules! assign_if_some_map {
    ($target:expr, $opt:expr, $map:expr) => {
        if let Some(val) = $opt {
            $target = $map(val);
        }
    };
}

pub const FONT_SCALE: f32 = 0.5;
const BASE_NUM_FONT_SIZE: u16 = 122;
const BASE_NOTE_FONT_SIZE: u16 = 35;
const BASE_COMMAND_FONT_SIZE: u16 = 40;
const BASE_TABLINE_FONT_SIZE: u16 = 40;

pub const BASE_BOX_SIZE: f32 = 468.0 / 9.;

pub struct Settings {
    pub lines: Lines,
    pub colors: Colors,
    pub opts: Opts,
    pub font: Font,
    /// <(mode, keybind), action>
    pub keymaps: HashMap<(String, String), String>,
}

impl Settings {
    pub fn from_config(config: &Option<Config>) -> Self {
        let font = load_ttf_font_from_bytes(include_bytes!("../assets/Roboto-Regular.ttf"))
            .expect("WTF you do bro.");
        let mut default = Self::new(font);
        if let Some(config) = config {
            if let Some(lines) = &config.lines {
                assign_if_some!(default.lines.outer_width, lines.outer_line_width);
                assign_if_some!(default.lines.box_width, lines.box_line_width);
                assign_if_some!(default.lines.normal_width, lines.normal_line_width);
                assign_if_some!(default.lines.window_gaps, lines.window_gaps);
            }

            if let Some(colors) = &config.colors {
                let into = |c: [f32; 4]| Color {
                    r: c[0],
                    g: c[1],
                    b: c[2],
                    a: c[3],
                };
                assign_if_some_map!(default.colors.square_color, colors.square_color, into);
                assign_if_some_map!(default.colors.bg_color, colors.bg_color, into);
                assign_if_some_map!(default.colors.outer_color, colors.outer_line, into);
                assign_if_some_map!(default.colors.box_color, colors.box_line, into);
                assign_if_some_map!(default.colors.normal_color, colors.normal_line, into);
                assign_if_some_map!(default.colors.window_gaps, colors.window_gaps, into);
                assign_if_some_map!(default.colors.selected_window, colors.selected_window, into);
                assign_if_some_map!(default.colors.selected_tab, colors.selected_tab, into);
                assign_if_some_map!(default.colors.selected_tab_font, colors.selected_tab_font, into);
                assign_if_some_map!(default.colors.inactive_tab_font, colors.inactive_tab_font, into);
                assign_if_some_map!(default.colors.inactive_tab_color, colors.inactive_tab_color, into);

                assign_if_some_map!(default.colors.normal_font, colors.normal_font_color, into);
                assign_if_some_map!(default.colors.note_font, colors.note_font_color, into);
                assign_if_some_map!(default.colors.cmd_font, colors.cmd_font_color, into);
                assign_if_some_map!(default.colors.status_font, colors.status_font_color, into);

                assign_if_some_map!(default.colors.cmd_bg, colors.cmd_bg_color, into);
                assign_if_some_map!(default.colors.status_bg, colors.status_bg_color, into);

                assign_if_some_map!(default.colors.highlight_main, colors.highlight_main, into);
                assign_if_some_map!(default.colors.highlight_sub, colors.highlight_sub, into);
                assign_if_some_map!(
                    default.colors.visual_highlight_color,
                    colors.visual_highlight_color,
                    into
                );
            }

            if let Some(o) = &config.opts {
                assign_if_some!(default.opts.outer_gaps, o.outer_gaps);
                assign_if_some!(default.opts.command_font_size, o.command_font_size);
                assign_if_some!(default.opts.tabline_font_size, o.tabline_font_size);
                assign_if_some!(default.opts.tabline_gap, o.tabline_gap);
                assign_if_some!(default.opts.visual_highlight_size, o.visual_highlight_size);
                assign_if_some!(
                    default.opts.auto_candidate_elimination,
                    o.auto_candidate_elimination
                );
                assign_if_some!(default.opts.auto_fill_candidates, o.auto_fill_candidates);
                assign_if_some!(default.opts.check_input, o.check_input);
            }
            if let Some(keymaps) = &config.keymaps {
                default.keymaps = match parse_config_keymaps(&keymaps) {
                    Ok(x) => x,
                    Err(err_msg) => {
                        eprintln!("{err_msg}");
                        std::process::exit(1);
                    }
                };
            } else {
                default.keymaps = default_keymaps();
            }
        }
        default
    }
    pub fn new(font: Font) -> Self {
        let lines = Lines::default();
        let colors = Colors::default();
        let opts = Opts::default();
        Self {
            colors,
            lines,
            opts,
            font,
            keymaps: HashMap::new(),
        }
    }
    pub fn get_highlight_size(&self, box_size: f32) -> f32 {
        self.opts.visual_highlight_size * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_lengths(&self, min_size: f32) -> (f32, f32) {
        let offset = self.lines.outer_width * 2.0
            + self.lines.box_width * 2.0
            + self.lines.normal_width * 6.0;
        let s = min_size - offset;
        let rem = s % 9.;
        let a = min_size - rem;
        (a, (s - rem) / 9.)
    }

    pub fn get_num_font_size(&self, box_size: f32) -> u16 {
        (BASE_NUM_FONT_SIZE as f32 * (box_size / BASE_BOX_SIZE)) as u16
    }
    pub fn get_note_font_size(&self, box_size: f32) -> u16 {
        (BASE_NOTE_FONT_SIZE as f32 * (box_size / BASE_BOX_SIZE)) as u16
    }

    pub fn get_cmd_size(&self) -> f32 {
        ((self.opts.command_font_size as f32 / BASE_COMMAND_FONT_SIZE as f32) * 22. * 2.).ceil()
    }
    pub fn get_tabline_size(&self) -> f32 {
        ((self.opts.tabline_font_size as f32 / BASE_TABLINE_FONT_SIZE as f32) * 22.).ceil()
    }

    pub fn get_x_num_offset(&self, box_size: f32) -> f32 {
        9.0 * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_x_note_offset(&self, box_size: f32) -> f32 {
        4.0 * (box_size / BASE_BOX_SIZE)
    }

    pub fn get_y_num_offset(&self, box_size: f32) -> f32 {
        (BASE_BOX_SIZE - 4.) * (box_size / BASE_BOX_SIZE)
    }
    pub fn get_y_note_offset(&self, box_size: f32) -> f32 {
        (BASE_BOX_SIZE - 37.) * (box_size / BASE_BOX_SIZE)
    }

    pub fn get_y_tab_offset(&self) -> f32 {
        -8. * (self.opts.tabline_font_size as f32 / BASE_TABLINE_FONT_SIZE as f32)
    }
}

macro_rules! new_keymap {
    ($map:expr, $( $mode:expr ),* ; $key:expr => $action:expr ) => {
        $(
            $map.insert(($mode.to_string(), $key.to_string()), $action.to_string());
        )*
    };
}

fn default_keymaps() -> HashMap<(String, String), String> {
    let mut hmap = HashMap::new();

    let nm = "normal";
    let n = "note";
    let i = "insert";
    let g = "go";

    new_keymap!(hmap, nm, n, i; "h" => "move left");
    new_keymap!(hmap, nm, n, i; "j" => "move down");
    new_keymap!(hmap, nm, n, i; "k" => "move up");
    new_keymap!(hmap, nm, n, i; "l" => "move right");
    new_keymap!(hmap, nm, n, i, g; " " => "mark");

    new_keymap!(hmap, nm; "g" => "go");
    new_keymap!(hmap, nm; "i" => "insert");
    new_keymap!(hmap, nm, "visual"; "n" => "note");

    new_keymap!(hmap, nm, n; "v" => "mode visual");
    new_keymap!(hmap, "visual"; "h" => "mark; move left");
    new_keymap!(hmap, "visual"; "j" => "mark; move down");
    new_keymap!(hmap, "visual"; "k" => "mark; move up");
    new_keymap!(hmap, "visual"; "l" => "mark; move right");

    hmap
}

fn parse_config_keymaps(
    keymaps: &HashMap<String, String>,
) -> Result<HashMap<(String, String), String>, &'static str> {
    let mut res = HashMap::new();
    for (k, action) in keymaps {
        let mut split = k.split(";");
        let modes = unwrap_or_else!(split.next(), {
            return Err("Invalid key");
        });
        let bind = unwrap_or_else!(split.next(), {
            return Err("No mode/keybind specified");
        });
        for mode in modes.split(",") {
            res.insert((mode.to_string(), bind.to_string()), action.clone());
        }
    }
    Ok(res)
}

#[test]
fn parse_config_keymaps_works() {
    let mut keymaps = HashMap::new();
    keymaps.insert("banana,insert,philza;gr,ah".to_string(), "k".to_string());
    keymaps.insert("pizda;pilgrim".to_string(), "e".to_string());
    let new_keymaps = parse_config_keymaps(&keymaps).unwrap();
    let mut expected = HashMap::new();
    new_keymap!(expected, "banana", "insert", "philza"; "gr,ah" => "k");
    new_keymap!(expected, "pizda"; "pilgrim" => "e");
    assert_eq!(new_keymaps, expected);
}
