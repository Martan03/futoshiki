use crate::{board::board_struct::Board, solver::Solver};

/// Backtracking solver
#[derive(Debug, PartialEq)]
pub struct BtSolver<'a> {
    board: &'a mut Board,
}

impl<'a> Solver<'a> for BtSolver<'a> {
    fn solve(board: &'a mut Board) -> bool {
        let mut solver = Self { board };
        solver.solve_inner(0, 0)
    }
}

impl<'a> BtSolver<'a> {
    /// Recursive solver, which tries every value, until it finds solution
    fn solve_inner(&mut self, mut x: usize, mut y: usize) -> bool {
        if x == self.board.size() {
            if y + 1 == self.board.size() {
                return true;
            }
            y += 1;
            x = 0;
        }

        let id = x + y * self.board.size();
        if self.board[id].value() > 0 {
            return self.solve_inner(x + 1, y);
        }

        for num in 1..=self.board.size() {
            if !self.is_valid(num, x, y) {
                continue;
            }
            self.board[id].set(num);
            if self.solve_inner(x + 1, y) {
                return true;
            }
            self.board[id].set(0);
        }
        false
    }

    /// Checks if the given value can be inserted on the given coordinates
    fn is_valid(&self, val: usize, x: usize, y: usize) -> bool {
        // Checks row and column uniqueness
        for (pos, _) in (0..self.board.size()).enumerate() {
            if self.board[x + pos * self.board.size()].value() == val
                || self.board[pos + y * self.board.size()].value() == val
            {
                return false;
            }
        }

        // Checks conditions
        let lsize = self.board.size().saturating_sub(1);
        self.check_cond(val, x.checked_sub(1).map(|xs| (xs, y)), |x, y| {
            (self.board.hor_conds[x + y * lsize], true)
        }) && self.check_cond(
            val,
            y.checked_sub(1).map(|ys| (x, ys)),
            |x, y| (self.board.ver_conds[x + y * self.board.size()], true),
        ) && self.check_cond(
            val,
            Self::check_cond_pos(x, y, lsize, self.board.size()),
            |x, y| (self.board.hor_conds[x + y * lsize], false),
        ) && self.check_cond(
            val,
            Self::check_cond_pos(x, y, self.board.size(), lsize),
            |x, y| (self.board.ver_conds[x + y * self.board.size()], false),
        )
    }

    /// Checks if the condition is valid
    fn check_cond<F>(
        &self,
        val: usize,
        pos: Option<(usize, usize)>,
        cond: F,
    ) -> bool
    where
        F: Fn(usize, usize) -> (Option<bool>, bool),
    {
        let Some((x, y)) = pos else {
            return true;
        };

        let id = x + y * self.board.size();
        if self.board[id].value() == 0 {
            return true;
        }

        match cond(x, y) {
            (Some(true), true) => self.board[id].value() > val,
            (Some(true), false) => val > self.board[id].value(),
            (Some(false), true) => self.board[id].value() < val,
            (Some(false), false) => val < self.board[id].value(),
            _ => true,
        }
    }

    /// Checks if the given position is valid (less then the maximum value)
    fn check_cond_pos(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        (x < width && y < height).then_some((x, y))
    }
}
