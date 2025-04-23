use crate::board::board_struct::Board;

use super::{
    ac3::AC3,
    domain::{bit_domain::BitDomain, hash_domain::HashDomain, DomainTrait},
    values::{ConstRngValues, ConstValues, DomainValues, Values},
    Solver,
};

/// Implements backtracking solver, which supports both domains and static
/// values. Domains are used in combination with AC-3, where AC-3 generates
/// the domains and backtracking then solves it, without any further pruning.
pub struct BtSolver<'a, V>
where
    V: Values,
{
    board: &'a mut Board,
    values: V,
}

impl<'a> BtSolver<'a, ConstValues> {
    /// Creates a new instance of the backtracking solver with given board and
    /// with constant values source
    pub fn new(board: &'a mut Board) -> Self {
        let size = board.size();
        Self {
            board,
            values: ConstValues::new(size),
        }
    }
}

impl<'a> BtSolver<'a, ConstRngValues> {
    /// Creates new instance of the backtracking solver used for generating
    /// new board
    pub fn generator(board: &'a mut Board) -> Self {
        let size = board.size();
        Self {
            board,
            values: ConstRngValues::new(size),
        }
    }
}

impl<'a> BtSolver<'a, DomainValues<BitDomain>> {
    /// Creates new backtracking solver with given board and bitmap domain
    pub fn bit(board: &'a mut Board) -> Self {
        let value = BitDomain::default(board.size());
        Self::domain(board, value)
    }
}

impl<'a> BtSolver<'a, DomainValues<HashDomain>> {
    /// Creates new backtracking solver with given board and hashset domain
    pub fn hash(board: &'a mut Board) -> Self {
        let value = HashDomain::default(board.size());
        Self::domain(board, value)
    }
}

impl<'a, D> BtSolver<'a, DomainValues<D>>
where
    D: DomainTrait + Clone,
{
    fn domain(board: &'a mut Board, value: D) -> Self {
        let mut values = vec![value; board.size() * board.size()];
        AC3::generate(board, &mut values);
        Self {
            board,
            values: DomainValues::new(values),
        }
    }
}

impl<'a, V> Solver<'a> for BtSolver<'a, V>
where
    V: Values,
{
    fn solve(&mut self) -> bool {
        self.solve_inner(0, 0)
    }
}

impl<V> BtSolver<'_, V>
where
    V: Values,
{
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

        for num in self.values.get(id) {
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

    fn is_valid(&self, val: usize, x: usize, y: usize) -> bool {
        // Checks row and column uniqueness
        for pos in 0..self.board.size() {
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
