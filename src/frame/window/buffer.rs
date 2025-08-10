use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{settings::Settings, sudoku::Sudoku};

pub struct Buffer {
    pub data: Sudoku,
}

impl Buffer {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            data: Sudoku::new(settings),
        }
    }
    pub fn draw(&self, dimensions: &Rect) {
        self.data.draw(*dimensions);
    }
    pub fn update(&mut self) {
        self.data.update();
    }
}
