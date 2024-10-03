use std::{
    cmp::min,
    ops::{Index, IndexMut},
};

use termint::geometry::Coords;

use super::cell::Cell;

/// Represents the game board of Futoshiki
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    cells: Vec<Cell>,
    pub hor_conds: Vec<Option<bool>>,
    pub ver_conds: Vec<Option<bool>>,
    pub selected: Coords,
    size: usize,
}

impl Board {
    /// Creates a square [`Board`] with given size
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![Cell::empty(); size * size],
            hor_conds: vec![None; size * size.saturating_sub(1)],
            ver_conds: vec![None; size * size.saturating_sub(1)],
            selected: Coords::new(0, 0),
            size,
        }
    }

    /// Gets the size of the [`Board`]
    pub fn size(&self) -> usize {
        self.size
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

    /// Clears selected cell
    pub fn clear(&mut self) {
        let sel = self.selected.x + self.selected.y * self.size;
        _ = self[sel].set(0);
    }

    /// Moves selected up
    pub fn up(&mut self) {
        self.selected.y = self.selected.y.saturating_sub(1);
    }

    /// Moves selected up
    pub fn down(&mut self) {
        self.selected.y = min(self.selected.y + 1, self.size - 1);
    }

    /// Moves selected up
    pub fn left(&mut self) {
        self.selected.x = self.selected.x.saturating_sub(1);
    }

    /// Moves selected up
    pub fn right(&mut self) {
        self.selected.x = min(self.selected.x + 1, self.size - 1);
    }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl Index<Coords> for Board {
    type Output = Cell;

    fn index(&self, index: Coords) -> &Self::Output {
        &self.cells[index.x + index.y * self.size]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl IndexMut<Coords> for Board {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
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
        ver_conds[6] = Some(true);
        ver_conds[9] = Some(false);

        Self {
            cells,
            hor_conds,
            ver_conds,
            selected: Coords::new(0, 0),
            size: 4,
        }
    }
}
