use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::tui::theme::Theme;

use super::board_struct::Board;

pub struct BoardGen {
    rng: ThreadRng,
    board: Board,
}

impl BoardGen {
    /// Generates new board
    pub fn generate(size: usize) -> Board {
        let mut bgen = Self {
            rng: rand::thread_rng(),
            board: Board::new(size, Theme::dark()),
        };

        bgen.generate_board(0, 0);
        bgen.rem_vals();
        bgen.board
    }
}

impl BoardGen {
    /// Generates random solved board
    fn generate_board(&mut self, mut x: usize, mut y: usize) -> bool {
        if x == self.board.size() {
            if y + 1 == self.board.size() {
                return true;
            }
            y += 1;
            x = 0;
        }

        let id = x + y * self.board.size();
        if self.board[id].value() > 0 {
            return self.generate_board(x + 1, y);
        }

        let mut domain: Vec<usize> = (1..=self.board.size()).collect();
        domain.shuffle(&mut self.rng);
        for num in domain {
            if !self.is_valid(num, x, y) {
                continue;
            }
            self.board[id].set(num);
            if self.generate_board(x + 1, y) {
                return true;
            }
            self.board[id].set(0);
        }
        false
    }

    /// Removes n values from the board based on the size of the board
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

    /// Adds condition on random place
    /// (randomly selects vertical or horizontal)
    fn add_cond(&mut self) {
        match self.rng.gen_range(0..2) {
            0 => self.add_hor_cond(),
            _ => self.add_ver_cond(),
        }
    }

    /// Adds horizontal condition to random place in the board
    fn add_hor_cond(&mut self) {
        let mut pos = self.rng.gen_range(0..self.board.hor_conds.len());
        while self.board.hor_conds[pos].is_some() {
            pos = self.rng.gen_range(0..self.board.hor_conds.len());
        }

        let lsize = self.board.size.saturating_sub(1);
        let cell_pos = (pos % lsize) + (pos / lsize) * self.board.size;
        self.board.hor_conds[pos] = Some(
            self.board[cell_pos].value() > self.board[cell_pos + 1].value(),
        );
    }

    /// Adds vertical condition to random place in the board
    fn add_ver_cond(&mut self) {
        let mut pos = self.rng.gen_range(0..self.board.ver_conds.len());
        while self.board.ver_conds[pos].is_some() {
            pos = self.rng.gen_range(0..self.board.ver_conds.len());
        }

        let cell_pos = (pos % self.board.size)
            + (pos / self.board.size) * self.board.size;
        self.board.ver_conds[pos] = Some(
            self.board[cell_pos].value()
                > self.board[cell_pos + self.board.size()].value(),
        );
    }

    /// Removes value from random cell in the boards
    fn rem_val(&mut self) {
        let mut pos = self.rng.gen_range(0..self.board.cells.len());
        while self.board[pos].value() == 0 {
            pos = self.rng.gen_range(0..self.board.cells.len());
        }

        self.board[pos].set(0);
    }

    /// Checks if value is unique in its row and column
    fn is_valid(&self, val: usize, x: usize, y: usize) -> bool {
        for pos in 0..self.board.size() {
            if self.board[x + pos * self.board.size()].value() == val
                || self.board[pos + y * self.board.size()].value() == val
            {
                return false;
            }
        }
        true
    }
}
