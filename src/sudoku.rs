mod message;
mod mode;
mod sudoku_board;

use crate::{
    settings::{FONT_SCALE, Settings},
    sudoku::{
        mode::Mode,
        sudoku_board::{BacktrackResult, SudokuBoard},
    },
    unwrap_or_else,
};

use arboard::Clipboard;
use macroquad::prelude::*;
use std::{f32, rc::Rc};

const NOTE_FLAG: u16 = 15;
const ALL_NOTES: u16 = 0b1000000111111111;

#[derive(Default)]
struct Selection([u16; 9]);

impl Selection {
    fn toggle(&mut self, y: impl Into<usize>, x: impl Into<usize>) {
        self.0[y.into()] ^= 1 << x.into();
    }
    fn get(&self, y: impl Into<usize>, x: impl Into<usize>) -> bool {
        self.0[y.into()] & (1 << x.into()) > 0
    }
    fn clear(&mut self) {
        for x in &mut self.0 {
            *x = 0;
        }
    }
}

pub struct Sudoku {
    board: SudokuBoard,
    only_solution: Option<SudokuBoard>,
    settings: Rc<Settings>,
    mode: Mode,

    cmd: String,

    selected: Selection,

    curr_keybind: String,
    repeat: u8,

    col: u8,
    row: u8,
}

impl Sudoku {
    pub fn new(settings: Rc<Settings>) -> Self {
        Self {
            only_solution: None,
            board: SudokuBoard(
                [[if settings.opts.auto_fill_candidates {
                    ALL_NOTES
                } else {
                    0
                }; 9]; 9],
            ),
            settings: Rc::clone(&settings),
            mode: Mode::Normal,

            selected: Selection::default(),

            cmd: String::default(),

            curr_keybind: String::default(),
            repeat: 0,

            col: 4,
            row: 4,
        }
    }

    pub fn draw(&self, mut dimensions: Rect) {
        let cmd_size = self.settings.get_cmd_size();
        let min_len = f32::min(dimensions.w, dimensions.h - cmd_size);
        let (side, box_size) = self.settings.get_lengths(min_len);

        let height_offset = (dimensions.h - side - cmd_size) / 2.;
        let width_offset = (dimensions.w - side) / 2.;

        dimensions.x += width_offset;
        dimensions.y += height_offset;

        draw_rectangle(
            dimensions.x,
            dimensions.y,
            side,
            side,
            self.settings.colors.square_color,
        );
        draw_inlines(&self.settings, &dimensions, side, box_size);
        draw_box_lines(&self.settings, &dimensions, side, box_size);
        draw_outlines(&self.settings, &dimensions, side);

        self.draw_grid(&dimensions, box_size);
        self.draw_cmd_line(&dimensions, cmd_size, side);
        self.draw_statusbar(&dimensions, cmd_size / 2., side);
    }

    fn draw_grid(&self, dimensions: &Rect, square_size: f32) {
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.get_num_font_size(square_size),
            font_scale: FONT_SCALE,
            color: self.settings.colors.normal_font,
            ..Default::default()
        };

        let x_note_offset = self.settings.get_x_note_offset(square_size);
        let y_note_offset = self.settings.get_y_note_offset(square_size);
        let x_num_offset = self.settings.get_x_num_offset(square_size);
        let y_num_offset = self.settings.get_y_num_offset(square_size);

        let highlight_size = self.settings.get_highlight_size(square_size);

        let mut y = self.settings.lines.outer_width + dimensions.y;
        let mut x = self.settings.lines.outer_width + dimensions.x;
        for (i, row) in self.board.iter().enumerate() {
            let num_y = y + y_num_offset;
            let note_y = y + y_note_offset;
            for (j, n) in row.iter().enumerate() {
                // draw selected
                if self.selected.get(i, j) || (self.row as usize, self.col as usize) == (i, j) {
                    let mut neighbors = [false; 8];
                    let mut counter = 0;
                    for yy in [-1, 0, 1] {
                        for xx in [-1, 0, 1] {
                            if yy == 0 && xx == 0 {
                                continue;
                            }
                            let y = i as i8 + yy;
                            let x = j as i8 + xx;
                            if (0..9).contains(&y) && (0..9).contains(&x) {
                                neighbors[counter] = self.selected.get(y as usize, x as usize)
                                    || (y as u8 == self.row && x as u8 == self.col);
                            }
                            counter += 1;
                        }
                    }

                    let mut check_corners = [true; 4];

                    // SIDES
                    if !neighbors[1] {
                        draw_rectangle(
                            x,
                            y,
                            square_size,
                            highlight_size,
                            self.settings.colors.visual_highlight_color,
                        );
                        check_corners[0] = false;
                        check_corners[1] = false;
                    }
                    if !neighbors[3] {
                        draw_rectangle(
                            x,
                            y,
                            highlight_size,
                            square_size,
                            self.settings.colors.visual_highlight_color,
                        );
                        check_corners[0] = false;
                        check_corners[3] = false;
                    }
                    if !neighbors[4] {
                        draw_rectangle(
                            x + square_size - highlight_size,
                            y,
                            highlight_size,
                            square_size,
                            self.settings.colors.visual_highlight_color,
                        );
                        check_corners[1] = false;
                        check_corners[2] = false;
                    }
                    if !neighbors[6] {
                        draw_rectangle(
                            x,
                            y + square_size - highlight_size,
                            square_size,
                            highlight_size,
                            self.settings.colors.visual_highlight_color,
                        );
                        check_corners[2] = false;
                        check_corners[3] = false;
                    }

                    if check_corners[0] {
                        if !neighbors[0] {
                            draw_rectangle(
                                x,
                                y,
                                highlight_size,
                                highlight_size,
                                self.settings.colors.visual_highlight_color,
                            );
                        }
                    }
                    if check_corners[1] {
                        if !neighbors[2] {
                            draw_rectangle(
                                x + square_size - highlight_size,
                                y,
                                highlight_size,
                                highlight_size,
                                self.settings.colors.visual_highlight_color,
                            );
                        }
                    }
                    if check_corners[2] {
                        if !neighbors[7] {
                            draw_rectangle(
                                x + square_size - highlight_size,
                                y + square_size - highlight_size,
                                highlight_size,
                                highlight_size,
                                self.settings.colors.visual_highlight_color,
                            );
                        }
                    }
                    if check_corners[3] {
                        if !neighbors[5] {
                            draw_rectangle(
                                x,
                                y + square_size - highlight_size,
                                highlight_size,
                                highlight_size,
                                self.settings.colors.visual_highlight_color,
                            );
                        }
                    }
                }
                if *n != 0 {
                    // note
                    if is_note(*n) {
                        let x = x + x_note_offset;
                        draw_notes(
                            &self.settings,
                            square_size,
                            x,
                            note_y,
                            *n,
                            &self.settings.font,
                        );
                    // num
                    } else {
                        let x = x + x_num_offset;
                        draw_text_ex(&n.to_string(), x, num_y, text_params.clone());
                    }
                }

                if j % 3 == 2 {
                    x += self.settings.lines.box_width;
                } else {
                    x += self.settings.lines.normal_width;
                }
                x += square_size;
            }
            x = self.settings.lines.outer_width + dimensions.x;

            if i % 3 == 2 {
                y += self.settings.lines.box_width;
            } else {
                y += self.settings.lines.normal_width;
            }
            y += square_size;
        }
    }
    fn draw_statusbar(&self, dimensions: &Rect, bar_size: f32, side: f32) {
        draw_rectangle(
            dimensions.x,
            side + dimensions.y,
            side,
            bar_size,
            self.settings.colors.status_bg,
        );
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.opts.command_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.colors.status_font,
            ..Default::default()
        };
        let y_offset = -4.0;
        let y = dimensions.y + side + bar_size + y_offset;

        draw_text_ex(
            &format!("-- {} --", self.mode.to_string().to_uppercase()),
            dimensions.x,
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

        let x = dimensions.x + side - width;
        draw_text_ex(&text, x, y, text_params);
    }
    fn draw_cmd_line(&self, dimensions: &Rect, cmd_size: f32, side: f32) {
        draw_rectangle(
            dimensions.x,
            dimensions.y + side,
            side,
            cmd_size,
            self.settings.colors.cmd_bg,
        );
        let text_params = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.opts.command_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.colors.cmd_font,
            ..Default::default()
        };
        let y_offset = -4.0;
        let y = dimensions.y + side + cmd_size + y_offset;
        draw_text_ex(&self.cmd, dimensions.x, y, text_params);
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
        if action.find(";").is_some() {
            let commands = action.split(";").collect::<Vec<_>>();
            if self.repeat > 0 {
                for _ in 0..self.repeat {
                    commands.iter().for_each(|x| self.process_cmd(x));
                }
            } else {
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
            self.selected.clear();
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
                        _ => self.update_keybind(c),
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
                        _ => self.update_keybind(c),
                    }
                }
            }
            Mode::Insert => {
                if let Some(c) = get_char_pressed() {
                    match c {
                        '0'..='9' => {
                            self.process_cmd(&format!("{c}insert"));
                        }
                        _ => self.update_keybind(c),
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
                        _ => self.update_keybind(c),
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
                        _ => self.update_keybind(c),
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
        let repeat = if repeat == 0 {None} else {Some(repeat)};
        match str {
            "insert" | "i"   => self.insert(repeat),
            "note"   | "n"   => self.note(repeat),
            "go"     | "g"   => self.go(repeat),

            "move"   | "mov" => self.mov(args, repeat),
            "mode"           => self.mode(args),
            "mark"           => self.mark(),
            "fill"           => self.board.fill_cell_candidates(),
            "import"         => self.import_clipboard(),
            _ => {
                self.cmd_log(format!("Invalid command: {str}"));
            }
        }
        if self.mode == Mode::Command {
            self.mode = Mode::Normal;
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
            _ => self.cmd_log("Invalid usage: mov u/d/l/r".to_string()),
        }
    }

    fn insert(&mut self, repeat: Option<u8>) {
        if let Some(num) = repeat {
            if !(1..=9).contains(&num) {
                self.cmd = "Invalid usage: <num>insert".to_string();
                return;
            }

            let before = self.board[(self.row, self.col)];
            self.board[(self.row, self.col)] = num as u16;
            if self.settings.opts.check_input {
                if let Some(solution) = &self.only_solution {
                    if solution[(self.row, self.col)] != num as u16 {
                        self.board[(self.row, self.col)] = before;
                        return;
                    }
                } else {
                    let mut clone = self.board.clone();
                    match clone.solve() {
                        BacktrackResult::NoSolution => {
                            self.board[(self.row, self.col)] = before;
                            return;
                        }
                        BacktrackResult::OneSolution(solution) => {
                            self.only_solution = Some(solution);
                        }
                        BacktrackResult::MoreSolutions => (),
                    }
                }
            }
            if self.settings.opts.auto_candidate_elimination {
                self.board.fix_notes_around(self.row, self.col);
            }
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

            if !self.selected.get(self.row, self.col) {
                let cell = &mut self.board[(self.row, self.col)];
                if *cell == 0 || is_note(*cell) {
                    n_bit_on(cell, NOTE_FLAG);
                    toggle_bit(cell, note - 1);
                } else {
                    self.cmd_log("Err: Cell is already filled with a number".to_string());
                }
            }
            for (i, row) in self.selected.0.iter().enumerate() {
                if *row == 0 {
                    continue;
                }
                for col in 0..9 {
                    if *row & (1 << col) > 0 {
                        let cell = &mut self.board[(i, col)];
                        if *cell == 0 || is_note(*cell) {
                            n_bit_on(cell, NOTE_FLAG);
                            toggle_bit(cell, note - 1);
                        }
                    }
                }
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

    fn mark(&mut self) {
        self.selected.toggle(self.row, self.col);
    }

    fn import_clipboard(&mut self) {
        let mut clipboard = if let Ok(x) = Clipboard::new() {
            x
        } else {
            self.cmd_log("Couldn't open clipboard.".to_string());
            return;
        };
        match clipboard.get_text() {
            Ok(text) => {
                let new = unwrap_or_else!(self.board.from_str(&text), {
                    self.cmd_log("Invalid sudoku".to_string());
                    return;
                });
                if self.settings.opts.auto_fill_candidates {
                    self.board.fill_cell_candidates();
                }

                let mut new_new = new.clone();

                match new_new.solve() {
                    BacktrackResult::NoSolution => {
                        self.cmd_log("Board has no solution.".to_string());
                        return;
                    }
                    BacktrackResult::OneSolution(solution) => {
                        self.only_solution = Some(solution);
                    }
                    BacktrackResult::MoreSolutions => {
                        self.cmd_log("Note: Sudoku has multiple solutions.".to_string());
                    }
                }

                self.board = new;
                if self.settings.opts.auto_fill_candidates {
                    self.board.fill_cell_candidates();
                }
            }
            Err(e) => {
                self.cmd_log(e.to_string());
            }
        }
    }
}

pub fn draw_box_lines(s: &Settings, dimensions: &Rect, side: f32, box_size: f32) {
    let mut point = box_size + s.lines.outer_width;
    for n in 1..9 {
        if n % 3 == 0 {
            draw_rectangle(
                dimensions.x + point,
                dimensions.y,
                s.lines.box_width,
                side,
                s.colors.box_color,
            );
            draw_rectangle(
                dimensions.x,
                dimensions.y + point,
                side,
                s.lines.box_width,
                s.colors.box_color,
            );
            point += s.lines.box_width;
        } else {
            point += s.lines.normal_width;
        }

        point += box_size;
    }
}

pub fn draw_inlines(s: &Settings, dimensions: &Rect, side: f32, box_size: f32) {
    let mut point = box_size + s.lines.outer_width;
    for n in 1..9 {
        if n % 3 != 0 {
            draw_rectangle(
                dimensions.x + point,
                dimensions.y,
                s.lines.normal_width,
                side,
                s.colors.normal_color,
            );
            draw_rectangle(
                dimensions.x,
                dimensions.y + point,
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

pub fn draw_outlines(s: &Settings, dimensions: &Rect, side: f32) {
    // Draw Sudoku lines
    draw_rectangle(
        dimensions.x,
        dimensions.y,
        side,
        s.lines.outer_width,
        s.colors.outer_color,
    );
    draw_rectangle(
        dimensions.x,
        dimensions.y,
        s.lines.outer_width,
        side,
        s.colors.outer_color,
    );
    draw_rectangle(
        dimensions.x + side - s.lines.outer_width,
        dimensions.y,
        s.lines.outer_width,
        side,
        s.colors.outer_color,
    );
    draw_rectangle(
        dimensions.x,
        dimensions.y + side - s.lines.outer_width,
        side,
        s.lines.outer_width,
        s.colors.outer_color,
    );
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

fn is_note(num: u16) -> bool {
    num & (1 << NOTE_FLAG) != 0
}

fn toggle_bit(num: &mut u16, bit: impl Into<u16>) {
    *num ^= 1 << bit.into();
}

fn n_bit_off(num: &mut u16, bit: impl Into<u16>) {
    *num &= !(1 << bit.into());
}
fn n_bit_on(num: &mut u16, bit: impl Into<u16>) {
    *num |= 1 << bit.into();
}
