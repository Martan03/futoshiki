use rand::{Rng, RngExt};

use crate::solver::{bt_solver::BtSolver, Solver};

use super::board_struct::Board;

/// Struct used for generating a board
pub struct BoardGen<R> {
    rng: R,
    board: Board,
}

impl<R: Rng> BoardGen<R> {
    /// Generates new board using backtracking.
    pub fn generate(size: usize, rng: R) -> Board {
        let mut bgen = Self {
            rng,
            board: Board::new(size),
        };

        BtSolver::generator(&mut bgen.board, &mut bgen.rng).solve();

        bgen.rem_vals();
        bgen.board
    }
}

impl<R: Rng> BoardGen<R> {
    /// Removes n values from the board based on the size of the board.
    fn rem_vals(&mut self) {
        let gen_num =
            ((self.board.size * self.board.size) as f64 * 0.8) as usize;
        let gen_conds = (gen_num as f64 * 0.4) as usize;
        for _ in 0..gen_conds {
            self.add_cond();
        }
        for _ in 0..gen_num {
            self.rem_val();
        }
    }

    /// Adds condition on random place.
    /// (randomly selects vertical or horizontal)
    fn add_cond(&mut self) {
        match self.rng.random_range(0..2) {
            0 => self.add_hor_cond(),
            _ => self.add_ver_cond(),
        }
    }

    /// Adds horizontal condition to random place in the board.
    fn add_hor_cond(&mut self) {
        let mut pos = self.rng.random_range(0..self.board.hor_conds.len());
        while self.board.hor_conds[pos].is_some() {
            pos = self.rng.random_range(0..self.board.hor_conds.len());
        }

        let lsize = self.board.size.saturating_sub(1);
        let cell_pos = (pos % lsize) + (pos / lsize) * self.board.size;
        self.board.hor_conds[pos] = Some(
            self.board[cell_pos].value() > self.board[cell_pos + 1].value(),
        );
    }

    /// Adds vertical condition to random place in the board.
    fn add_ver_cond(&mut self) {
        let mut pos = self.rng.random_range(0..self.board.ver_conds.len());
        while self.board.ver_conds[pos].is_some() {
            pos = self.rng.random_range(0..self.board.ver_conds.len());
        }

        let cell_pos = (pos % self.board.size)
            + (pos / self.board.size) * self.board.size;
        self.board.ver_conds[pos] = Some(
            self.board[cell_pos].value()
                > self.board[cell_pos + self.board.size()].value(),
        );
    }

    /// Removes value from random cell in the board.
    fn rem_val(&mut self) {
        let mut pos = self.rng.random_range(0..self.board.cells.len());
        while self.board[pos].value() == 0 {
            pos = self.rng.random_range(0..self.board.cells.len());
        }

        self.board[pos].set(0);
    }
}
