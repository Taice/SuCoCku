use crate::frame::window::Window;
use macroquad::prelude::*;

pub enum SplitDirection {
    Vertical,
    Horizontal,
}

pub enum Split {
    Window(Window),
    Split(Box<Split>, Box<Split>, f32, SplitDirection),
}

impl Split {
    pub fn idx(&self, wanted: usize, idx: &mut usize) -> Option<&Window> {
        match self {
            Self::Split(lhs, rhs, _, _) => rhs.idx(wanted, idx).or_else(|| lhs.idx(wanted, idx)),
            Split::Window(window) => {
                if *idx == wanted {
                    return Some(window);
                }
                *idx += 1;
                None
            }
        }
    }
    pub fn idx_mut(&mut self, wanted: usize, idx: &mut usize) -> Option<&mut Window> {
        match self {
            Self::Split(lhs, rhs, _, _) => lhs
                .idx_mut(wanted, idx)
                .or_else(|| rhs.idx_mut(wanted, idx)),
            Split::Window(window) => {
                if *idx == wanted {
                    return Some(window);
                }
                *idx += 1;
                None
            }
        }
    }

    pub fn resize(&mut self, mut dimensions: Rect, gap_size: f32) {
        match self {
            Split::Window(window) => {
                dimensions.y += gap_size / 2.;
                dimensions.x += gap_size / 2.;
                dimensions.w += gap_size / 2.;
                dimensions.h += gap_size / 2.;

                window.dimensions = dimensions;
            }
            Split::Split(up, down, percent, SplitDirection::Vertical) => {
                let height = dimensions.h * *percent;
                let up_dimensions = Rect {
                    x: dimensions.x,
                    y: dimensions.y,
                    h: height,
                    w: dimensions.w,
                };
                let down_dimensions = Rect {
                    x: dimensions.x,
                    y: dimensions.y + height,
                    h: dimensions.h - height,
                    w: dimensions.w,
                };
                up.resize(up_dimensions, gap_size);
                down.resize(down_dimensions, gap_size);
            }
            Split::Split(left, right, percent, SplitDirection::Horizontal) => {
                let width = dimensions.w * *percent;
                let left_dimensions = Rect {
                    x: dimensions.x,
                    y: dimensions.y,
                    h: dimensions.h,
                    w: width,
                };
                let right_dimensions = Rect {
                    x: dimensions.x + width,
                    y: dimensions.y,
                    h: dimensions.h,
                    w: dimensions.w - width,
                };
                left.resize(left_dimensions, gap_size);
                right.resize(right_dimensions, gap_size);
            }
        }
    }

    pub fn iter(&self) -> SplitIterator {
        SplitIterator::new(self)
    }
    pub fn iter_mut(&mut self) -> SplitIteratorMut {
        SplitIteratorMut::new(self)
    }
}

pub struct SplitIterator<'a> {
    stack: Vec<&'a Split>,
}

impl<'a> SplitIterator<'a> {
    pub fn new(root: &'a Split) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for SplitIterator<'a> {
    type Item = &'a Window;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(split) = self.stack.pop() {
            match split {
                Split::Window(win) => return Some(win),
                Split::Split(lhs, rhs, _, _) => {
                    self.stack.push(lhs);
                    self.stack.push(rhs);
                }
            }
        }
        None
    }
}

pub struct SplitIteratorMut<'a> {
    stack: Vec<&'a mut Split>,
}

impl<'a> SplitIteratorMut<'a> {
    pub fn new(root: &'a mut Split) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for SplitIteratorMut<'a> {
    type Item = &'a mut Window;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Split::Window(window) => return Some(window),
                Split::Split(lhs, rhs, _, _) => {
                    let (lhs_ref, rhs_ref) = (&mut **lhs, &mut **rhs);
                    self.stack.push(rhs_ref);
                    self.stack.push(lhs_ref);
                }
            }
        }
        None
    }
}
