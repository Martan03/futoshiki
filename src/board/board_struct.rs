use std::ops::{Index, IndexMut};

use termint::geometry::{Rect, Vec2};

use super::cell::Cell;

/// Represents the game board of Futoshiki
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    cells: Vec<Cell>,
    pub hor_conds: Vec<Option<bool>>,
    pub ver_conds: Vec<Option<bool>>,
    pub selected: Vec2,
    size: usize,
}

impl Board {
    /// Creates a square [`Board`] with given size
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![Cell::empty(); size * size],
            hor_conds: vec![None; size * size.saturating_sub(1)],
            ver_conds: vec![None; size * size.saturating_sub(1)],
            selected: Vec2::new(0, 0),
            size,
        }
    }

    /// Resets the [`Board`] to be the same size but empty
    pub fn reset(&mut self) {
        self.hor_conds = vec![None; self.size * self.size.saturating_sub(1)];
        self.ver_conds = vec![None; self.size * self.size.saturating_sub(1)];
        self.cells = vec![Cell::empty(); self.size * self.size];
    }

    /// Gets the size of the [`Board`]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets rectangle of the [`Board`]
    pub fn rect(&self) -> Rect {
        Rect::new(0, 0, self.size, self.size)
    }

    /// Pushes the given digit to the selected cell
    pub fn push(&mut self, value: usize) {
        if value > self.size {
            return;
        }

        let sel = self.selected.x + self.selected.y * self.size;
        let mut val = self[sel].value() * 10 + value;
        if val > self.size {
            val = value;
        }

        _ = self[sel].set(val);
    }

    /// Pops the last digit from current value
    pub fn pop(&mut self) {
        let sel = self.selected.x + self.selected.y * self.size;
        let val = self[sel].value() / 10;
        self[sel].set(val);
    }

    /// Clears selected cell
    pub fn clear(&mut self) {
        let sel = self.selected.x + self.selected.y * self.size;
        _ = self[sel].set(0);
    }

    /// Sets selected cell to given position
    pub fn set_selected(&mut self, pos: Vec2) {
        self.selected = pos;
    }

    /// Disables all non-zero values
    pub fn disable_vals(&mut self) {
        for pos in Rect::new(0, 0, self.size, self.size) {
            if self[pos].value() != 0 {
                self[pos].disable();
            }
        }
    }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl Index<Vec2> for Board {
    type Output = Cell;

    fn index(&self, index: Vec2) -> &Self::Output {
        &self.cells[index.x + index.y * self.size]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl IndexMut<Vec2> for Board {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        &mut self.cells[index.x + index.y * self.size]
    }
}

impl Default for Board {
    /// Creates a square [`Board`] with the size of 4
    fn default() -> Self {
        let mut cells = vec![Cell::empty(); 16];
        cells[5] = Cell::new(3);

        let mut hor_conds = vec![None; 12];
        hor_conds[6] = Some(true);
        hor_conds[5] = Some(false);

        let mut ver_conds = vec![None; 12];
        ver_conds[6] = Some(false);
        ver_conds[9] = Some(true);

        Self {
            cells,
            hor_conds,
            ver_conds,
            selected: Vec2::new(0, 0),
            size: 4,
        }
    }
}
