mod split;
mod tab;
mod window;

use macroquad::prelude::*;
use split::Split;
use std::cell::RefCell;
use std::rc::Rc;

use window::Window;

use window::buffer::Buffer;

use crate::draw_rect_outlines;
use crate::frame::split::SplitDirection;
use crate::frame::tab::Tab;
use crate::settings::{FONT_SCALE, Settings};
use crate::sudoku::Sudoku;

enum Mode {
    Buffer,
    Normal,
}

pub struct Frame {
    tabs: Vec<Tab>,
    curr_tab: usize,

    buffers: Vec<Buffer>,

    mode: Mode,

    tabn: usize,
    settings: Rc<RefCell<Settings>>,
    size: (f32, f32),
}

impl Frame {
    pub fn new(settings: Settings) -> Self {
        let settings = Rc::new(RefCell::new(settings));
        let mut ret = Self {
            mode: Mode::Normal,
            // test
            tabs: vec![Tab {
                name: "Tab #1".to_string(),
                windows: 1,
                selected: 0,
                inner: Split::Window(Window::new(Rect::default(), 0)),
            }],
            tabn: 1,
            curr_tab: 0,
            buffers: vec![Buffer::new(Rc::clone(&settings))],
            settings: settings,
            size: (0.0, 0.0),
        };
        ret.update();
        ret
    }
    pub fn draw(&mut self) {
        clear_background(self.settings.borrow().colors.bg_color);

        let half = self.settings.borrow().lines.window_gaps / 2.;
        for window in self.tabs[self.curr_tab].inner.iter() {
            window.render(&self.buffers[window.buffer_index]);
            let dimensions = window.dimensions;
            draw_rect_outlines(
                Rect::new(
                    dimensions.x - half,
                    dimensions.y - half,
                    dimensions.w + self.settings.borrow().lines.window_gaps,
                    dimensions.h + self.settings.borrow().lines.window_gaps,
                ),
                half,
                self.settings.borrow().colors.window_gaps,
            );
        }
        if let Split::Window(win) = self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected] {
            // draw selcete window outline
            let mut dimensions = win.dimensions;
            dimensions.x -= half;
            dimensions.y -= half;
            dimensions.w += self.settings.borrow().lines.window_gaps;
            dimensions.h += self.settings.borrow().lines.window_gaps;
            draw_rect_outlines(
                dimensions,
                half,
                self.settings.borrow().colors.selected_window,
            );

            // draw current tab's statusbar/command-line
            let cmd_size = self.settings.borrow().get_cmd_size();
            let status_rect = Rect::new(0.0, self.size.1 - cmd_size, self.size.0, cmd_size / 2.);
            self.buffers[win.buffer_index]
                .data
                .draw_statusbar(&status_rect);
            let cmd_rect = Rect::new(
                status_rect.x,
                status_rect.y + cmd_size / 2.,
                status_rect.w,
                status_rect.h,
            );
            self.buffers[win.buffer_index].data.draw_cmd_line(&cmd_rect);
            // draw_rectangle(status_rect.x, status_rect.y, status_rect.w, status_rect.h, GREEN);
            // draw_rectangle(cmd_rect.x, cmd_rect.y, cmd_rect.w, cmd_rect.h, BLUE);
        } else {
            unreachable!();
        }
        self.draw_tabline();
    }

    pub fn update(&mut self) {
        let width = screen_width();
        let height = screen_height();
        if self.size != (width, height) {
            self.size = (width, height);
            self.resize();
        }
        self.handle_input();
    }

    fn new_tab(&mut self) {
        self.tabn += 1;
        self.tabs.push(Tab {
            name: format!("Tab #{}", self.tabn),
            inner: Split::Window(Window::new(Rect::default(), self.buffers.len())),
            windows: 1,
            selected: 0,
        });
        self.curr_tab = self.tabs.len() - 1;
        self.buffers.push(Buffer {
            data: Sudoku::new(Rc::clone(&self.settings)),
        });
        self.resize();
    }
    fn close_tab(&mut self) {
        let mut len = self.tabs.len();
        if len > 1 {
            self.tabs.remove(self.curr_tab);
            len -= 1;
            if self.curr_tab >= len {
                self.curr_tab = len - 1;
            }
        }
        self.resize();
    }

    fn switch_tab(&mut self, n: i32) {
        self.curr_tab = ((self.curr_tab as i32 + n).rem_euclid(self.tabs.len() as i32)) as usize;
        self.resize();
    }

    fn new_buffer(&mut self) {
        let idx = self.tabs[self.curr_tab].selected;
        if let Split::Window(win) = &mut self.tabs[self.curr_tab][idx] {
            win.buffer_index = self.buffers.len();
            self.buffers.push(Buffer {
                data: Sudoku::new(Rc::clone(&self.settings)),
            });
        }
    }

    fn split(&mut self, direction: SplitDirection) {
        self.tabs[self.curr_tab].split(direction);
        self.resize();
    }

    fn kill_pane(&mut self) {
        self.tabs[self.curr_tab].kill_pane();
        self.resize();
    }

    fn buffer_mode(&mut self) {
        self.mode = Mode::Buffer;
    }

    fn switch_buffer(&mut self, n: i32) {
        self.tabs[self.curr_tab].switch_buffer(n, self.buffers.len());
    }

    fn kill_buffer(&mut self) {
        let mut len = self.buffers.len();
        if len > 1 {
            let win_idx = self.tabs[self.curr_tab].selected;
            if let Split::Window(win) = self.tabs[self.curr_tab][win_idx] {
                self.buffers.remove(win.buffer_index);
            }
            len -= 1;
            for win in self.tabs[self.curr_tab].inner.iter_mut() {
                if win.buffer_index >= len {
                    win.buffer_index = len - 1;
                }
            }
        }
    }

    fn handle_input(&mut self) {
        match self.mode {
            Mode::Buffer => self.handle_input_buffer(),
            Mode::Normal => self.handle_input_normal(),
        }
    }

    fn handle_input_buffer(&mut self) {
        if let Some(k) = get_last_key_pressed() {
            match k {
                KeyCode::Tab => {
                    self.switch_buffer(1);
                    return;
                }
                KeyCode::Escape => {
                    self.mode = Mode::Normal;
                }
                _ => (),
            }
        }
        if let Some(c) = get_char_pressed() {
            self.mode = Mode::Normal;
            match c {
                'l' => self.switch_buffer(1),
                'h' => self.switch_buffer(-1),
                'n' => self.new_buffer(),
                'k' => self.kill_buffer(),
                _ => self.mode = Mode::Buffer,
            }
        }
    }

    fn handle_input_normal(&mut self) {
        if is_key_down(KeyCode::LeftControl) {
            self.handle_frame_input();
        } else {
            if let Split::Window(selected_win) =
                &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected]
            {
                selected_win.update(&mut self.buffers[selected_win.buffer_index]);
            }
        }
    }

    fn handle_frame_input(&mut self) {
        if let Some(kc) = get_last_key_pressed() {
            match kc {
                KeyCode::Tab => {
                    self.switch_tab(if is_key_down(KeyCode::LeftShift) {
                        -1
                    } else {
                        1
                    });
                }
                _ => (),
            }
        }
        if let Some(ch) = get_char_pressed() {
            match ch {
                '\\' => self.split(SplitDirection::Vertical),
                '-' => self.split(SplitDirection::Horizontal),
                'h' => {
                    self.tabs[self.curr_tab].move_left(self.settings.borrow().lines.window_gaps);
                }
                'l' => {
                    self.tabs[self.curr_tab].move_right(self.settings.borrow().lines.window_gaps);
                }
                'k' => {
                    self.tabs[self.curr_tab].move_up(self.settings.borrow().lines.window_gaps);
                }
                'j' => {
                    self.tabs[self.curr_tab].move_down(self.settings.borrow().lines.window_gaps);
                }
                'x' => self.kill_pane(),
                't' => self.new_tab(),
                'w' => self.close_tab(),
                'b' => self.buffer_mode(),
                _ => (),
            }
        }
    }

    fn draw_tabline(&self) {
        let mut max = 0.0;
        for x in &self.tabs {
            let len = measure_text(
                &x.name,
                Some(&self.settings.borrow().font),
                self.settings.borrow().opts.tabline_font_size,
                FONT_SCALE,
            )
            .width;
            if len > max {
                max = len;
            }
        }
        max += self.settings.borrow().opts.tabline_gap * 3.;

        let tab_size = self.settings.borrow().get_tabline_size()
            + self.settings.borrow().opts.tabline_gap * 2.;
        let mut rect = Rect::new(
            self.settings.borrow().opts.outer_gaps,
            self.settings.borrow().opts.outer_gaps,
            max,
            tab_size,
        );
        let text_params: TextParams = TextParams {
            font: Some(&self.settings.borrow().font),
            font_size: self.settings.borrow().opts.tabline_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.borrow().colors.inactive_tab_font,
            ..Default::default()
        };

        for (i, tab) in self.tabs.iter().enumerate() {
            if i == self.curr_tab {
                let mut text_params = text_params.clone();
                text_params.color = self.settings.borrow().colors.selected_tab_font;
                draw_rect_outlines(
                    rect,
                    self.settings.borrow().opts.tabline_gap,
                    self.settings.borrow().colors.window_gaps,
                );
                let text_pos = center_text(
                    &tab.name,
                    &self.settings.borrow().font,
                    self.settings.borrow().opts.tabline_font_size,
                    rect,
                );
                draw_text_ex(&tab.name, text_pos.x, text_pos.y, text_params);
            } else {
                let text_params = text_params.clone();
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    self.settings.borrow().colors.inactive_tab_color,
                );
                let text_pos = center_text(
                    &tab.name,
                    &self.settings.borrow().font,
                    self.settings.borrow().opts.tabline_font_size,
                    rect,
                );
                draw_text_ex(&tab.name, text_pos.x, text_pos.y, text_params);
            }
            rect.x += max + self.settings.borrow().opts.tabline_gap * 2.;
        }
    }

    fn resize(&mut self) {
        let gaps = self.settings.borrow().opts.outer_gaps;
        let tabline_size = self.settings.borrow().get_tabline_size()
            + self.settings.borrow().opts.tabline_gap * 3.;
        let cmd_size = self.settings.borrow().get_cmd_size();
        let dimensions = Rect {
            x: gaps,
            y: gaps + tabline_size,
            w: self.size.0 - gaps * 2.,
            h: self.size.1 - gaps * 2. - tabline_size - cmd_size,
        };

        self.tabs[self.curr_tab].resize(dimensions, self.settings.borrow().lines.window_gaps);
    }
}

pub fn center_text(text: &str, font: &Font, font_size: u16, rect: Rect) -> Vec2 {
    let dim = measure_text(text, Some(font), font_size, FONT_SCALE);
    let x = (rect.w - dim.width) / 2. + rect.x;
    let y = (rect.h - dim.height) / 2. + rect.y + dim.height;

    (x, y).into()
}
