use macroquad::prelude::*;
use std::ops::{Index, IndexMut};

use crate::frame::split::{Split, SplitDirection};

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

    pub fn split(&mut self, direction: SplitDirection) {
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

    pub fn move_up(&mut self, window_gaps: f32) {
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

    pub fn move_down(&mut self, window_gaps: f32) {
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

    pub fn move_right(&mut self, window_gaps: f32) {
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

    pub fn move_left(&mut self, window_gaps: f32) {
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

    pub fn switch_buffer(&mut self, n: i32, buf_len: usize) {
        let idx = self.selected;
        if let Split::Window(win) = &mut self[idx] {
            win.buffer_index = ((win.buffer_index as i32 + n).rem_euclid(buf_len as i32)) as usize;
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
