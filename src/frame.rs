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
use crate::settings::Settings;

pub struct Tab {
    pub inner: Split,

    pub windows: usize,
    pub selected_idx: usize,
}

impl Tab {
    pub fn resize(&mut self, dimensions: Rect, gap_size: f32) {
        self.inner.resize(dimensions, gap_size);
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

    settings: Rc<Settings>,
    size: (f32, f32),
}

impl Frame {
    pub fn new(settings: Rc<Settings>) -> Self {
        let mut ret = Self {
            // test
            tabs: vec![Tab {
                windows: 1,
                selected_idx: 0,
                inner: Split::Window(Window::new(Rect::default(), 0)),
            }],
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
        if let Split::Window(win) = self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx]
        {
            let mut dimensions = win.dimensions;
            dimensions.x -= half;
            dimensions.y -= half;
            dimensions.w += self.settings.lines.window_gaps;
            dimensions.h += self.settings.lines.window_gaps;

            draw_rect_outlines(dimensions, half, self.settings.colors.selected_window);
        } else {
            unreachable!();
        }
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

    fn split(&mut self, direction: SplitDirection) {
        match self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx] {
            Split::Split(..) => unreachable!(),
            Split::Window(win) => {
                let win = win.clone();
                let idx = self.tabs[self.curr_tab].selected_idx;
                self.tabs[self.curr_tab][idx] = Split::Split(
                    Box::new(Split::Window(win)),
                    Box::new(Split::Window(win)),
                    0.5,
                    direction,
                );
                self.tabs[self.curr_tab].windows += 1;
            }
        }
        self.resize();
    }

    fn kill_pane(&mut self) {

    }

    fn move_up(&mut self) {
        let win = &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + 2.,
                win.dimensions.y - self.settings.lines.window_gaps * 2. - 4.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.tabs[self.curr_tab].inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.tabs[self.curr_tab].selected_idx = i;
                break;
            }
        }
    }
    fn move_down(&mut self) {
        let win = &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + 2.,
                win.dimensions.y + win.dimensions.h + self.settings.lines.window_gaps * 2. + 4.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.tabs[self.curr_tab].inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.tabs[self.curr_tab].selected_idx = i;
                break;
            }
        }
    }
    fn move_right(&mut self) {
        let win = &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x + win.dimensions.w + self.settings.lines.window_gaps * 2. + 4.,
                win.dimensions.y + 2.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.tabs[self.curr_tab].inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.tabs[self.curr_tab].selected_idx = i;
                break;
            }
        }
    }
    fn move_left(&mut self) {
        let win = &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx];
        let point;
        if let Split::Window(win) = win {
            point = vec2(
                win.dimensions.x - self.settings.lines.window_gaps * 2. - 4.,
                win.dimensions.y + 2.,
            );
        } else {
            unreachable!()
        }

        for (i, x) in self.tabs[self.curr_tab].inner.iter().enumerate() {
            if x.dimensions.contains(point) {
                self.tabs[self.curr_tab].selected_idx = i;
                break;
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::LeftControl) {
            if let Some(ch) = get_char_pressed() {
                match ch {
                    '-' => self.split(SplitDirection::Horizontal),
                    '\\' => self.split(SplitDirection::Vertical),
                    'h' => self.move_left(),
                    'l' => self.move_right(),
                    'k' => self.move_up(),
                    'j' => self.move_down(),
                    _ => (),
                }
            }
        } else {
            if let Split::Window(selected_win) =
                &self.tabs[self.curr_tab][self.tabs[self.curr_tab].selected_idx]
            {
                selected_win.update(&mut self.buffers[selected_win.buffer_index]);
            }
        }
    }

    fn resize(&mut self) {
        let gaps = self.settings.opts.outer_gaps;
        let dimensions = Rect {
            x: gaps,
            y: gaps,
            w: self.size.0 - gaps * 2.,
            h: self.size.1 - gaps * 2.,
        };

        self.tabs[self.curr_tab].resize(dimensions, self.settings.lines.window_gaps / 2.);
    }
}
