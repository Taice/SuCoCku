use std::fmt::{self, Display};

use crate::frame::window::Window;
use macroquad::prelude::*;

pub enum SplitDirection {
    Vertical,
    Horizontal,
}

impl Display for SplitDirection {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Vertical => "V",
            Self::Horizontal => "H",
        })
    } 
}

pub enum Split {
    Window(Window),
    Split(Box<Split>, Box<Split>, f32, SplitDirection),
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_tree(node: &Split, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            let pad = " ".repeat(indent);
            match node {
                Split::Window(..) => {
                    writeln!(f, "{}Window", pad)
                }
                Split::Split(left, right, _, dir) => {
                    writeln!(f, "{}Split {}", pad, dir)?;
                    fmt_tree(left, f, indent + 2)?;
                    fmt_tree(right, f, indent + 2)
                }
            }
        }
        fmt_tree(self, f, 0)
    }
}

impl Split {
    pub fn idx(&self, wanted: usize, idx: &mut usize) -> Option<&Split> {
        match self {
            Self::Split(lhs, rhs, _, _) => rhs.idx(wanted, idx).or_else(|| lhs.idx(wanted, idx)),
            Split::Window(_) => {
                if *idx == wanted {
                    return Some(self);
                }
                *idx += 1;
                None
            }
        }
    }
    pub fn idx_mut(&mut self, wanted: usize, idx: &mut usize) -> Option<&mut Split> {
        match self {
            Self::Split(lhs, rhs, _, _) => rhs
                .idx_mut(wanted, idx)
                .or_else(|| lhs.idx_mut(wanted, idx)),
            Split::Window(_) => {
                if *idx == wanted {
                    return Some(self);
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
                dimensions.w -= gap_size;
                dimensions.h -= gap_size;

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

    pub fn kill_pane(&mut self, wanted: usize, idx: &mut usize) -> bool {
        match self {
            Self::Split(lhs, rhs, _, _) => {
                if rhs.kill_pane(wanted, idx) {
                    let new_self = std::mem::replace(&mut **lhs, Self::Window(Window::default()));
                    *self = new_self;
                    return false;
                }
                if lhs.kill_pane(wanted, idx) {
                    let new_self = std::mem::replace(&mut **rhs, Self::Window(Window::default()));
                    *self = new_self;
                    return false;
                }
                false
            }
            Self::Window(_) => {
                if *idx == wanted {
                    *idx += 1;
                    true
                } else {
                    *idx += 1;
                    false
                }
            }
        }
    }

    pub fn iter(&self) -> SplitIterator {
        SplitIterator::new(self)
    }
    #[allow(dead_code)]
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
