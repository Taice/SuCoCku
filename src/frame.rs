mod split;
pub mod window;

use macroquad::prelude::*;
use split::Split;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use window::Window;

use window::buffer::Buffer;

use crate::draw_rect_outlines;
use crate::frame::split::SplitDirection;
use crate::settings::{FONT_SCALE, Settings};
use crate::sudoku::Sudoku;

pub struct Tab {
    pub name: String,
    pub inner: Split,

    pub windows: usize,
    pub selected: usize,
}

impl Tab {
    pub fn resize(&mut self, dimensions: Rect, gap_size: f32) {
        self.inner.resize(dimensions, gap_size);
    }
    pub fn kill_pane(&mut self) {
        if self.windows > 1 {
            self.inner.kill_pane(self.selected, &mut 0);
            self.windows -= 1;
            if self.selected >= self.windows {
                self.selected = self.windows - 1;
            }
        }
    }
    fn split(&mut self, direction: SplitDirection) {
        match self[self.selected] {
            Split::Split(..) => unreachable!(),
            Split::Window(win) => {
                let win = win.clone();
                let idx = self.selected;
                self[idx] = Split::Split(
                    Box::new(Split::Window(win)),
                    Box::new(Split::Window(win)),
                    0.5,
                    direction,
                );
                self.windows += 1;
            }
        }
    }
    fn move_up(&mut self, window_gaps: f32) {
        let win = &self[self.selected];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + 2.,
                win.dimensions.y - window_gaps * 2. - 4.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.selected = i;
                break;
            }
        }
    }
    fn move_down(&mut self, window_gaps: f32) {
        let win = &self[self.selected];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + 2.,
                win.dimensions.y + win.dimensions.h + window_gaps * 2. + 4.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.selected = i;
                break;
            }
        }
    }
    fn move_right(&mut self, window_gaps: f32) {
        let win = &self[self.selected];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + win.dimensions.w + window_gaps * 2. + 4.,
                win.dimensions.y + 2.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.selected = i;
                break;
            }
        }
    }
    fn move_left(&mut self, window_gaps: f32) {
        let win = &self[self.selected];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x - window_gaps * 2. - 4.,
                win.dimensions.y + 2.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.selected = i;
                break;
            }
        }
    }
}

impl<Idx: Into<usize>> Index<Idx> for Tab {
    type Output = Split;
    fn index(&self, index: Idx) -> &Self::Output {
        self.inner.idx(index.into(), &mut 0).unwrap()
    }
}

impl<Idx: Into<usize>> IndexMut<Idx> for Tab {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.inner.idx_mut(index.into(), &mut 0).unwrap()
    }
}

pub struct Frame {
    tabs: Vec<Tab>,
    curr_tab: usize,

    buffers: Vec<Buffer>,

    tabn: usize,
    settings: Rc<Settings>,
    size: (f32, f32),
}

impl Frame {
    pub fn new(settings: Rc<Settings>) -> Self {
        let mut ret = Self {
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
        clear_background(self.settings.colors.bg_color);

        let half = self.settings.lines.window_gaps / 2.;
        for window in self.tabs[self.curr_tab].inner.iter() {
            window.render(&self.buffers[window.buffer_index]);
            let dimensions = window.dimensions;
            draw_rect_outlines(
                Rect::new(
                    dimensions.x - half,
                    dimensions.y - half,
                    dimensions.w + self.settings.lines.window_gaps,
                    dimensions.h + self.settings.lines.window_gaps,
                ),
                half,
                self.settings.colors.window_gaps,
            );
        }
        if let Split::Window(win) = self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected] {
            let mut dimensions = win.dimensions;
            dimensions.x -= half;
            dimensions.y -= half;
            dimensions.w += self.settings.lines.window_gaps;
            dimensions.h += self.settings.lines.window_gaps;

            draw_rect_outlines(dimensions, half, self.settings.colors.selected_window);
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

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::LeftControl) {
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
                    'h' => self.tabs[self.curr_tab].move_left(self.settings.lines.window_gaps),
                    'l' => self.tabs[self.curr_tab].move_right(self.settings.lines.window_gaps),
                    'k' => self.tabs[self.curr_tab].move_up(self.settings.lines.window_gaps),
                    'j' => self.tabs[self.curr_tab].move_down(self.settings.lines.window_gaps),
                    'x' => self.kill_pane(),
                    't' => self.new_tab(),
                    'w' => self.close_tab(),
                    'n' => self.new_buffer(),
                    _ => (),
                }
            }
        } else {
            if let Split::Window(selected_win) =
                &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected]
            {
                selected_win.update(&mut self.buffers[selected_win.buffer_index]);
            }
        }
    }

    fn draw_tabline(&self) {
        let mut max = 0.0;
        for x in &self.tabs {
            let len = measure_text(
                &x.name,
                Some(&self.settings.font),
                self.settings.opts.tabline_font_size,
                FONT_SCALE,
            )
            .width;
            if len > max {
                max = len;
            }
        }
        max += self.settings.opts.tabline_gap * 3.;

        let tab_size = self.settings.get_tabline_size() + self.settings.opts.tabline_gap * 2.;
        let mut rect = Rect::new(
            self.settings.opts.outer_gaps,
            self.settings.opts.outer_gaps,
            max,
            tab_size,
        );
        let text_params: TextParams = TextParams {
            font: Some(&self.settings.font),
            font_size: self.settings.opts.tabline_font_size,
            font_scale: FONT_SCALE,
            color: self.settings.colors.inactive_tab_font,
            ..Default::default()
        };

        for (i, tab) in self.tabs.iter().enumerate() {
            if i == self.curr_tab {
                let mut text_params = text_params.clone();
                text_params.color = self.settings.colors.selected_tab_font;
                draw_rect_outlines(
                    rect,
                    self.settings.opts.tabline_gap,
                    self.settings.colors.window_gaps,
                );
                let text_pos = center_text(
                    &tab.name,
                    &self.settings.font,
                    self.settings.opts.tabline_font_size,
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
                    self.settings.colors.inactive_tab_color,
                );
                let text_pos = center_text(
                    &tab.name,
                    &self.settings.font,
                    self.settings.opts.tabline_font_size,
                    rect,
                );
                draw_text_ex(&tab.name, text_pos.x, text_pos.y, text_params);
            }
            rect.x += max + self.settings.opts.tabline_gap * 2.;
        }
    }

    fn resize(&mut self) {
        let gaps = self.settings.opts.outer_gaps;
        let tabline_size = self.settings.get_tabline_size() + self.settings.opts.tabline_gap * 3.;
        let dimensions = Rect {
            x: gaps,
            y: gaps + tabline_size,
            w: self.size.0 - gaps * 2.,
            h: self.size.1 - gaps * 2. - tabline_size,
        };

        self.tabs[self.curr_tab].resize(dimensions, self.settings.lines.window_gaps);
    }
}

fn center_text(text: &str, font: &Font, font_size: u16, rect: Rect) -> Vec2 {
    let dim = measure_text(text, Some(font), font_size, FONT_SCALE);
    let x = (rect.w - dim.width) / 2. + rect.x;
    let y = (rect.h - dim.height) / 2. + rect.y + dim.height;

    (x, y).into()
}
