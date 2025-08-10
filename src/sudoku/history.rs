#[derive(Clone, Copy)]
pub struct Change {
    pub pos: (u8, u8),
    pub before: u16,
    pub after: u16,
}

#[derive(Default)]
pub struct History {
    undo_stack: Vec<Vec<Change>>,
    redo_stack: Vec<Vec<Change>>,
}

impl History {
    pub fn record_change(&mut self, changes: Vec<Change>) {
        self.undo_stack.push(changes);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<Vec<Change>> {
        if let Some(changes) = self.undo_stack.pop() {
            self.redo_stack.push(changes.clone());
            Some(changes)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<Vec<Change>> {
        if let Some(changes) = self.redo_stack.pop() {
            self.undo_stack.push(changes.clone());
            Some(changes)
        } else {
            None
        }
    }
}
