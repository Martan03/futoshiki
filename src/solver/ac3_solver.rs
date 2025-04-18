use termint::geometry::Vec2;

use crate::{
    board::board_struct::Board,
    solver::{
        ac3::AC3,
        domain::{
            bit_domain::BitDomain, hash_domain::HashDomain, DomainTrait,
        },
        Solver,
    },
};

pub struct AC3Solver<'a, D>
where
    D: DomainTrait + Clone,
{
    board: &'a mut Board,
    values: Vec<D>,
}

impl<'a> AC3Solver<'a, BitDomain> {
    /// Creates new AC3 solver with given board and bitmap domain
    pub fn bit(board: &'a mut Board) -> Self {
        let value = BitDomain::default(board.size());
        Self::new(board, value)
    }
}

impl<'a> AC3Solver<'a, HashDomain> {
    /// Creates new AC3 solver with given board and hashset domain
    pub fn hash(board: &'a mut Board) -> Self {
        let value = HashDomain::default(board.size());
        Self::new(board, value)
    }
}

impl<'a, D> Solver<'a> for AC3Solver<'a, D>
where
    D: DomainTrait + Clone,
{
    fn solve(&mut self) -> bool {
        let Some(pos) = self.find_cell() else {
            return true;
        };

        let id = pos.x + pos.y * self.board.size();
        for val in self.values[id].values() {
            let vals = self.values.clone();

            if !self.assign(val, pos) {
                self.board[id].set(0);
                self.values = vals;
                continue;
            }
            if self.solve() {
                return true;
            }
            self.board[id].set(0);
            self.values = vals;
        }
        false
    }
}

impl<'a, D> AC3Solver<'a, D>
where
    D: DomainTrait + Clone,
{
    fn new(board: &'a mut Board, value: D) -> Self {
        let mut values = vec![value; board.size() * board.size()];
        AC3::generate(board, &mut values);
        Self { board, values }
    }

    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    fn assign(&mut self, val: usize, pos: Vec2) -> bool {
        let id = pos.x + pos.y * self.board.size();
        self.board[id].set(val);
        AC3::eliminate(self.board, &mut self.values, pos)
    }

    /// Finds unassigned cell with the smallest domain (least possible values)
    fn find_cell(&self) -> Option<Vec2> {
        let mut min_val = usize::MAX;
        let mut min = None;

        for pos in self.board.rect() {
            let id = pos.x + pos.y * self.board.size();
            if self.board[id].value() > 0 {
                continue;
            }

            let values = self.values[id].values().len();
            if values < min_val {
                min_val = values;
                min = Some(pos);
            }
        }
        min
    }
}
