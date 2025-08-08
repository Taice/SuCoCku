use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::sudoku::{ALL_NOTES, is_note, n_bit_off};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SudokuBoard(pub [[u16; 9]; 9]);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BacktrackResult {
    OneSolution(SudokuBoard),
    MoreSolutions,
    NoSolution,
}

impl SudokuBoard {
    pub fn from_str(&mut self, str: &str) -> Option<SudokuBoard> {
        let mut new = SudokuBoard([[0; 9]; 9]);
        for (i, x) in str.chars().enumerate() {
            if !x.is_ascii_digit() {
                return None;
            }
            new[(i / 9, i % 9)] = (x as u8 - b'0') as u16;
        }
        Some(new)
    }

    pub fn fill_cell_candidates(&mut self) {
        for row in &mut self.0 {
            for col in row {
                if *col == 0 || is_note(*col) {
                    *col = ALL_NOTES
                }
            }
        }
        for y in 0u8..9 {
            for x in 0..9 {
                self.fix_notes_around(y, x);
            }
        }
    }

    pub fn fix_notes_around(&mut self, y: u8, x: u8) {
        let mut num = self[(y, x)];
        if num == 0 || is_note(num) {
            return;
        }
        num -= 1;
        // check in boxes
        for y in ((y / 3) * 3)..((y / 3) * 3 + 3) {
            for x in ((x / 3) * 3)..((x / 3) * 3 + 3) {
                if is_note(self[(y, x)]) {
                    // turn n bit off
                    self[(y, x)] &= !(1 << num);
                }
            }
        }
        for n in 0u8..9 {
            // check in row
            if is_note(self[(y, n)]) {
                n_bit_off(&mut self[(y, n)], num);
            }
            // check in col
            if is_note(self[(n, x)]) {
                n_bit_off(&mut self[(n, x)], num);
            }
        }
    }

    pub fn solve(&mut self) -> BacktrackResult {
        return self.backtrack(&mut BacktrackResult::NoSolution);
    }
    fn backtrack(&mut self, solve_state: &mut BacktrackResult) -> BacktrackResult {
        if !self.is_valid() {
            return *solve_state;
        }
        if let Some((y, x)) = self.find_empty_space() {
            let before = self[(y, x)];
            for num in 1..=9 {
                self[(y, x)] = num;

                match self.backtrack(solve_state) {
                    BacktrackResult::NoSolution => (),
                    BacktrackResult::MoreSolutions => return BacktrackResult::MoreSolutions,
                    solved => *solve_state = solved,
                }
            }
            self[(y, x)] = before;
            return *solve_state;
        } else {
            return if matches!(*solve_state, BacktrackResult::OneSolution(_)) {
                BacktrackResult::MoreSolutions
            } else {
                BacktrackResult::OneSolution(self.clone())
            };
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for (i, row) in self.0.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if *n == 0 || is_note(*n) {
                    continue;
                }
                let num = *n as usize - 1;
                let box_index = get_box_index(i, j);

                if rows[i][num] || cols[j][num] || boxes[box_index][num] {
                    return false;
                }

                rows[i][num] = true;
                cols[j][num] = true;
                boxes[box_index][num] = true;
            }
        }
        true
    }

    fn find_empty_space(&self) -> Option<(usize, usize)> {
        for (i, row) in self.0.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 0 || is_note(*cell) {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

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

fn get_box_index(y: impl Into<usize>, x: impl Into<usize>) -> usize {
    let y = y.into();
    let x = x.into();
    (y / 3) * 3 + (x / 3)
}
