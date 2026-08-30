use std::ops::{Index, IndexMut};

use termint::geometry::{Rect, Vec2};

use super::cell::Cell;

/// Represents the game board of Futoshiki
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub hor_conds: Vec<Option<bool>>,
    pub ver_conds: Vec<Option<bool>>,
    pub selected: Vec2,
    pub size: usize,
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

        while val > self.size && val >= 10 {
            val %= 10_usize.pow((val as f64).log10().floor() as u32)
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

    /// Enables all board values
    pub fn enable_vals(&mut self) {
        for pos in Rect::new(0, 0, self.size, self.size) {
            self[pos].enable();
        }
    }

    /// Disables all non-zero values
    pub fn disable_vals(&mut self) {
        for pos in Rect::new(0, 0, self.size, self.size) {
            if self[pos].value() != 0 {
                self[pos].disable();
            }
        }
    }

    /// Clones current cells vector
    pub fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    /// Sets cells to given value
    pub fn cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
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
        let mut board = Board {
            cells: vec![Cell::empty(); 16],
            hor_conds: vec![None; 12],
            ver_conds: vec![None; 12],
            selected: Vec2::new(0, 0),
            size: 4,
        };
        board.hor_conds[1] = Some(true);
        board.hor_conds[2] = Some(true);
        board.hor_conds[11] = Some(true);
        board.ver_conds[0] = Some(false);
        board.ver_conds[4] = Some(true);
        board.ver_conds[5] = Some(false);
        board.ver_conds[6] = Some(false);
        board
    }
}

/// Some board presets used for testing purposes
impl Board {
    pub fn trivial() -> Self {
        let cells = vec![2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 3, 1, 0, 2];
        Self {
            cells: cells.into_iter().map(Cell::from).collect(),
            hor_conds: vec![None; 12],
            ver_conds: vec![None; 12],
            selected: Vec2::new(0, 0),
            size: 4,
        }
    }

    pub fn easy() -> Self {
        let cells = vec![0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut board = Self {
            cells: cells.into_iter().map(Cell::from).collect(),
            hor_conds: vec![None; 12],
            ver_conds: vec![None; 12],
            selected: Vec2::new(0, 0),
            size: 4,
        };
        board.ver_conds[0] = Some(true);
        board.ver_conds[8] = Some(true);
        board.ver_conds[10] = Some(true);
        board
    }

    pub fn tricky() -> Self {
        let cells = vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut board = Self {
            cells: cells.into_iter().map(Cell::from).collect(),
            hor_conds: vec![None; 12],
            ver_conds: vec![None; 12],
            selected: Vec2::new(0, 0),
            size: 4,
        };
        board.hor_conds[2] = Some(true);
        board.hor_conds[4] = Some(true);
        board.hor_conds[8] = Some(true);
        board.hor_conds[9] = Some(true);
        board.ver_conds[11] = Some(true);
        board
    }

    pub fn extreme() -> Self {
        let mut board = Self {
            cells: vec![Cell::empty(); 16],
            hor_conds: vec![None; 12],
            ver_conds: vec![None; 12],
            selected: Vec2::new(0, 0),
            size: 4,
        };
        board.hor_conds[1] = Some(true);
        board.hor_conds[2] = Some(true);
        board.hor_conds[11] = Some(true);
        board.ver_conds[0] = Some(false);
        board.ver_conds[4] = Some(true);
        board.ver_conds[5] = Some(false);
        board.ver_conds[6] = Some(false);
        board
    }
}

#[cfg(test)]
mod tests {
    use termint::geometry::Vec2;

    use super::Board;

    #[test]
    fn board_push() {
        let mut board = Board::new(12);
        board.set_selected(Vec2::new(3, 2));

        board.push(1);
        assert_eq!(board.cells[27].value(), 1);
        board.push(1);
        assert_eq!(board.cells[27].value(), 11);

        board.push(2);
        assert_eq!(board.cells[27].value(), 12);

        board.push(5);
        assert_eq!(board.cells[27].value(), 5);
    }

    #[test]
    fn board_pop() {
        let mut board = Board::new(12);
        board.set_selected(Vec2::new(0, 0));
        board.cells[0].set(12);

        board.pop();
        assert_eq!(board.cells[0].value(), 1);
        board.pop();
        assert_eq!(board.cells[0].value(), 0);
    }

    #[test]
    fn board_clear() {
        let mut board = Board::new(12);
        board.set_selected(Vec2::new(0, 0));
        board.cells[0].set(12);

        board.clear();
        assert_eq!(board.cells[0].value(), 0);
    }
}
