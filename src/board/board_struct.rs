use std::ops::{Index, IndexMut};

/// Represents the game board of Futoshiki
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    cells: Vec<usize>,
    size: usize,
}

impl Board {
    /// Creates a square [`Board`] with given size
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![0; size * size],
            size,
        }
    }
}

impl Index<usize> for Board {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl Default for Board {
    /// Creates a square [`Board`] with the size of 4
    fn default() -> Self {
        Self {
            cells: vec![0; 16],
            size: 4,
        }
    }
}
