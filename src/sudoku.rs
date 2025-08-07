mod mode;

use crate::{
    settings::{FONT_SCALE, Settings},
    sudoku::mode::Mode,
};

use macroquad::prelude::*;
use std::{
    f32,
    ops::{Deref, DerefMut, Index, IndexMut},
};

const NOTE_FLAG: u16 = 15;

struct SudokuBoard([[u16; 9]; 9]);

impl Deref for SudokuBoard {
    type Target = [[u16; 9]; 9];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SudokuBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Into<usize>> Index<(T, T)> for SudokuBoard {
    type Output = u16;
    fn index(&self, index: (T, T)) -> &Self::Output {
        &self.0[index.0.into()][index.1.into()]
    }
}
impl<T: Into<usize>> IndexMut<(T, T)> for SudokuBoard {
    fn index_mut(&mut self, index: (T, T)) -> &mut Self::Output {
        &mut self.0[index.0.into()][index.1.into()]
    }
}

pub struct Sudoku {
    board: SudokuBoard,
    settings: Settings,
    mode: Mode,

    cmd: String,

    curr_keybind: String,
    repeat: u8,

    col: u8,
    row: u8,
}

impl Sudoku {
    pub fn new(settings: Settings) -> Self {
        Self {
            board: SudokuBoard([[0; 9]; 9]),
            settings,
            mode: Mode::Normal,

            cmd: String::default(),

            curr_keybind: String::default(),
            repeat: 0,

            col: 4,
            row: 4,
        }
    }

    pub fn draw(&self) {
        clear_background(self.settings.colors.bg_color);
        let cmd_size = self.settings.get_cmd_size();
        let min_len = f32::min(screen_width(), screen_height() - cmd_size);
        let (side, box_size) = self.settings.get_lengths(min_len);

        draw_rectangle(0., 0., side, side, WHITE);
        draw_inlines(&self.settings, side, box_size);
        draw_box_lines(&self.settings, side, box_size);
        draw_outlines(&self.settings, side);

        self.draw_grid(box_size);
        self.draw_cmd_line(cmd_size, side);
        self.draw_statusbar(cmd_size / 2., side);
    }

    fn draw_grid(&self, box_size: f32) {
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.get_num_font_size(box_size),
            font_scale: FONT_SCALE,
            color: self.settings.colors.normal_font,
            ..Default::default()
        };
        let mut y = self.settings.lines.outer_width;

        let mut x = self.settings.lines.outer_width;
        for (i, row) in self.board.iter().enumerate() {
            let num_y = y + self.settings.get_y_num_offset(box_size);
            let note_y = y + self.settings.get_y_note_offset(box_size);
            for (j, n) in row.iter().enumerate() {
                self.draw_highlight(i, j, y, x, box_size);
                if *n != 0 {
                    // note
                    if n & (1 << NOTE_FLAG) != 0 {
                        let x = x + self.settings.get_x_note_offset(box_size);
                        draw_notes(&self.settings, box_size, x, note_y, *n, &self.settings.font);
                    // num
                    } else {
                        let x = x + self.settings.get_x_num_offset(box_size);
                        draw_text_ex(&n.to_string(), x, num_y, text_params.clone());
                    }
                }

                if j % 3 == 2 {
                    x += self.settings.lines.box_width;
                } else {
                    x += self.settings.lines.normal_width;
                }
                x += box_size;
            }
            x = self.settings.lines.outer_width;

            if i % 3 == 2 {
                y += self.settings.lines.box_width;
            } else {
                y += self.settings.lines.normal_width;
            }
            y += box_size;
        }
    }
    fn draw_statusbar(&self, bar_size: f32, side: f32) {
        draw_rectangle(0.0, side, side, bar_size, self.settings.colors.status_bg);
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.opts.command_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.colors.status_font,
            ..Default::default()
        };
        let y_offset = -4.0;
        let y = side + bar_size + y_offset;

        draw_text_ex(
            &format!("-- {} --", self.mode.to_string()),
            0.0,
            y,
            text_params.clone(),
        );

        let text;
        if self.repeat > 0 {
            text = format!("{}{}", self.repeat, self.curr_keybind)
        } else {
            text = self.curr_keybind.clone();
        }
        let width = measure_text(
            &text,
            Some(&self.settings.font),
            self.settings.opts.command_font_size,
            FONT_SCALE,
        )
        .width;

        let x = side - width;
        draw_text_ex(&text, x, y, text_params);
    }
    fn draw_cmd_line(&self, cmd_size: f32, side: f32) {
        draw_rectangle(0.0, side, side, cmd_size, self.settings.colors.cmd_bg);
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.opts.command_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.colors.cmd_font,
            ..Default::default()
        };
        let y_offset = -4.0;
        let y = side + cmd_size + y_offset;
        draw_text_ex(&self.cmd, 0.0, y, text_params);
    }

    fn draw_highlight(&self, i: usize, j: usize, y: f32, x: f32, box_size: f32) {
        let mut color = Color::from_rgba(0, 0, 0, 0);

        if self.settings.opts.highlight_in_line {
            if i == self.row as usize || j == self.col as usize {
                color = self.settings.colors.highlight_sub;
            }
        }
        if self.settings.opts.highlight_box {
            if get_box_index(i, j) == get_box_index(self.row, self.col) {
                color = self.settings.colors.highlight_sub;
            }
        }
        if self.settings.opts.highlight_cell && self.row as usize == i && self.col as usize == j {
            color = self.settings.colors.highlight_main
        }

        draw_rectangle(x, y, box_size, box_size, color);
    }

    pub fn update(&mut self) {
        self.handle_input();
    }

    pub fn try_keybind(&mut self) -> bool {
        let mode = self.mode.to_string();
        let action = if let Some(action) = self
            .settings
            .keymaps
            .get(&(mode, self.curr_keybind.clone()))
        {
            action.clone()
        } else {
            return false;
        };
        if let Some(_) = action.find(";")
            && self.repeat > 0
        {
            let commands = action.split(";").collect::<Vec<_>>();
            for _ in 0..self.repeat {
                commands.iter().for_each(|x| self.process_cmd(x));
            }
        } else {
            let cmd = format!("{}{action}", self.repeat);
            self.process_cmd(&cmd);
        }
        self.flush();
        true
    }

    fn handle_input(&mut self) {
        // global base case
        if is_key_down(KeyCode::Escape) {
            self.mode = Mode::Normal;
            self.flush();
            return;
        }

        match &mut self.mode {
            Mode::Normal => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        ':' => {
                            self.mode = Mode::Command;
                            self.cmd.clear();
                            self.flush();
                        }
                        '0'..='9' => {
                            self.repeat = self
                                .repeat
                                .saturating_mul(10)
                                .saturating_add(c as u8 - b'0')
                        }
                        ch if ch.is_ascii_alphabetic() => {
                            self.update_keybind(ch);
                        }
                        _ => (),
                    }
                }
            }
            Mode::Command => {
                if let Some(keycode) = get_last_key_pressed() {
                    match keycode {
                        KeyCode::Backspace => {
                            if is_key_down(KeyCode::LeftControl) {
                                while let Some(val) = self.cmd.pop() {
                                    if let ' ' | '_' | '\'' = val {
                                        break;
                                    }
                                }
                            } else {
                                self.cmd.pop();
                            }
                        }
                        KeyCode::Enter => {
                            self.process_cmd(&self.cmd.clone());
                        }
                        _ => {
                            if let Some(ch) = get_char_pressed() {
                                if ch.is_alphanumeric() || ch == ' ' {
                                    self.cmd.push(ch);
                                }
                            }
                        }
                    }
                }
            }
            Mode::Note => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        '0'..='9' => {
                            self.process_cmd(&format!("{c}note"));
                        }
                        ch if ch.is_alphabetic() => {
                            self.update_keybind(ch);
                        }
                        _ => (),
                    }
                }
            }
            Mode::Insert => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        '0'..='9' => {
                            self.process_cmd(&format!("{c}insert"));
                        }
                        ch if ch.is_alphabetic() => {
                            self.update_keybind(ch);
                        }
                        _ => (),
                    }
                }
            }
            Mode::Go(row) => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        '1'..='9' => {
                            if *row == 0 {
                                *row = c as u8 - b'0';
                            } else {
                                self.process_cmd(&format!("{}{c}go", self.row.clone()));
                            }
                        }
                        '0' => (),
                        ch if ch.is_alphabetic() => {
                            self.update_keybind(ch);
                        }
                        _ => (),
                    }
                }
            }
            Mode::Custom(_) => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        '0'..='9' => {
                            self.repeat = self
                                .repeat
                                .saturating_mul(10)
                                .saturating_add(c as u8 - b'0');
                        }
                        ch if ch.is_alphabetic() => {
                            self.update_keybind(ch);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn update_keybind(&mut self, c: char) {
        self.curr_keybind += &c.to_string();
        if !self.try_keybind() {
            if !self.matching_keymap_exists() {
                self.flush();
            }
        }
    }

    fn matching_keymap_exists(&self) -> bool {
        let idx = self.curr_keybind.len();
        let curr_mode = self.mode.to_string();
        for ((mode, keybind), _) in &self.settings.keymaps {
            if *mode != curr_mode {
                continue;
            }
            if keybind.len() < idx {
                continue;
            }
            if keybind[..idx] == self.curr_keybind {
                return true;
            }
        }
        false
    }

    fn flush(&mut self) {
        self.curr_keybind.clear();
        self.repeat = 0;
    }

    #[rustfmt::skip]
    fn process_cmd(&mut self, cmd: &str) {
        let mut trim = cmd.trim();

        let mut repeat: u8 = 0;
        let mut repeat_end = 0;
        for (i, c) in trim.chars().enumerate() {
            if !c.is_ascii_digit() {
                repeat_end = i;
                break;
            }
            if let Some(first) = repeat.checked_mul(10) {
                if let Some(val) = first.checked_add(c as u8 - b'0') {
                    repeat = val;
                }
            }
        }
        trim = &trim[repeat_end..];

        let mut args = &trim[0..0];
        let str = if let Some(space) = trim.find(' ') {
            args = &trim[space..].trim();
            &trim[..space]
        } else {
            trim
        };
        self.mode = Mode::Normal;
        let repeat = if repeat == 0 {None} else {Some(repeat)};
        match str {
            "insert" | "i"   => self.insert(repeat),
            "note"   | "n"   => self.note(repeat),
            "go"     | "g"   => self.go(repeat),

            "move"   | "mov" => self.mov(args, repeat),
            _ => {
                self.cmd_log(format!("Invalid command: {str}"));
            }
        }
    }

    fn cmd_log(&mut self, err_msg: String) {
        self.cmd = err_msg;
    }

    // COMMANDS
    fn mov(&mut self, args: &str, repeat: Option<u8>) {
        if let Some(_) = args.find(' ') {
            self.cmd_log("Invalid usage: mov u/d/l/r".to_string())
        }

        let repeat = repeat.unwrap_or(1);
        match args {
            "u" | "up" => {
                for _ in 0..repeat {
                    if self.row > 0 {
                        self.row -= 1;
                    } else {
                        break;
                    }
                }
            }
            "d" | "down" => {
                for _ in 0..repeat {
                    if self.row < 8 {
                        self.row += 1;
                    } else {
                        break;
                    }
                }
            }
            "l" | "left" => {
                for _ in 0..repeat {
                    if self.col > 0 {
                        self.col -= 1;
                    } else {
                        break;
                    }
                }
            }
            "r" | "right" => {
                for _ in 0..repeat {
                    if self.col < 8 {
                        self.col += 1;
                    } else {
                        break;
                    }
                }
            }
            "mode" => {
                self.mode(args);
            }
            _ => self.cmd_log("Invalid usage: mov u/d/l/r".to_string()),
        }
    }

    fn insert(&mut self, repeat: Option<u8>) {
        if let Some(num) = repeat {
            if !(1..=9).contains(&num) {
                self.cmd = "Invalid usage: <num>insert".to_string();
                return;
            }

            self.board[(self.row, self.col)] = num as u16;
        } else {
            self.mode = Mode::Insert;
        }
    }

    fn note(&mut self, repeat: Option<u8>) {
        if let Some(note) = repeat {
            if !(1..=9).contains(&note) {
                self.cmd = "Invalid usage: <note>note".to_string();
                return;
            }

            let cell = &mut self.board[(self.row, self.col)];
            if *cell == 0 || *cell & (1 << NOTE_FLAG) != 0 {
                *cell |= 1 << NOTE_FLAG;
                *cell ^= 1 << note - 1;
            } else {
                self.cmd_log("Err: Cell is already filled with a number".to_string());
            }
        } else {
            self.mode = Mode::Note;
        }
    }

    fn go(&mut self, repeat: Option<u8>) {
        if let Some(goto) = repeat {
            let y = goto / 10;
            let x = goto % 10;

            if y <= 0 || y > 9 || x <= 0 {
                self.cmd_log("Invalid usage: <y><x>go".to_string());
                return;
            }

            self.row = y - 1;
            self.col = x - 1;
        } else {
            self.mode = Mode::Go(0);
        }
    }

    fn mode(&mut self, mode: &str) {
        self.mode = Mode::Custom(mode.to_string());
    }
}

pub fn draw_box_lines(s: &Settings, side: f32, box_size: f32) {
    let mut point = box_size + s.lines.outer_width;
    for n in 1..9 {
        if n % 3 == 0 {
            draw_rectangle(point, 0.0, s.lines.box_width, side, s.colors.box_color);
            draw_rectangle(0.0, point, side, s.lines.box_width, s.colors.box_color);
            point += s.lines.box_width;
        } else {
            point += s.lines.normal_width;
        }

        point += box_size;
    }
}

pub fn draw_inlines(s: &Settings, side: f32, box_size: f32) {
    let mut point = box_size + s.lines.outer_width;
    for n in 1..9 {
        if n % 3 != 0 {
            draw_rectangle(
                point,
                0.0,
                s.lines.normal_width,
                side,
                s.colors.normal_color,
            );
            draw_rectangle(
                0.0,
                point,
                side,
                s.lines.normal_width,
                s.colors.normal_color,
            );
            point += s.lines.normal_width;
        } else {
            point += s.lines.box_width;
        }

        point += box_size;
    }
}

pub fn draw_outlines(s: &Settings, side: f32) {
    let half = s.lines.outer_width / 2.;
    //Draw Sudoku lines
    draw_line(
        0.0,
        half,
        side,
        half,
        s.lines.outer_width,
        s.colors.outer_color,
    ); // TOP_LEFT_RIGHT
    draw_line(
        half,
        0.0,
        half,
        side,
        s.lines.outer_width,
        s.colors.outer_color,
    ); // TOP_LEFT_BOT
    draw_line(
        side - half,
        0.0,
        side - half,
        side,
        s.lines.outer_width,
        s.colors.outer_color,
    ); // TOP_RIGHT_BOT    
    draw_line(
        half,
        side - half,
        side,
        side - half,
        s.lines.outer_width,
        s.colors.outer_color,
    ); // BOT_LEFT_RIGHT
}

pub fn draw_notes(s: &Settings, box_size: f32, x: f32, y: f32, num: u16, font: &Font) {
    let text_params = TextParams {
        font: Some(font),
        font_size: s.get_note_font_size(box_size),
        font_scale: FONT_SCALE,
        color: s.colors.note_font,
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

fn get_box_index(row: impl Into<usize>, col: impl Into<usize>) -> usize {
    (row.into() / 3) * 3 + (col.into() / 3)
}
