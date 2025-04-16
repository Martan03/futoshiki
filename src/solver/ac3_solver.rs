use termint::geometry::Vec2;

use crate::{
    board::board_struct::Board,
    solver::{
        ac3::AC3,
        domain::{
            bit_domain::BitDomain, hash_domain::HashDomain, DomainTrait,
            Domains,
        },
        Solver,
    },
};

pub struct AC3Solver<'a> {
    board: &'a mut Board,
    values: Domains,
}

impl<'a> AC3Solver<'a> {
    /// Creates new AC3 solver with given board and bitmap domain
    pub fn bit(board: &'a mut Board) -> Self {
        let value = Box::new(BitDomain::default(board.size()));
        Self::new(board, value)
    }

    /// Creates new AC3 solver with given board and hashset domain
    pub fn hash(board: &'a mut Board) -> Self {
        let value = Box::new(HashDomain::default(board.size()));
        Self::new(board, value)
    }

    fn new(board: &'a mut Board, value: Box<dyn DomainTrait>) -> Self {
        let mut values: Domains = vec![value; board.size() * board.size()];
        AC3::generate(board, &mut values);
        Self { board, values }
    }
}

impl<'a> Solver<'a> for AC3Solver<'a> {
    fn solve(&mut self) -> bool {
        let Some(pos) = self.find_cell() else {
            return true;
        };

        let id = pos.x + pos.y * self.board.size();
        let values: Vec<usize> = self.values[id].values();
        for val in values {
            let vals = self.values.clone();

            self.assign(val, pos);
            if self.solve() {
                return true;
            }
            self.board[id].set(0);
            self.values = vals;
        }
        false
    }
}

impl AC3Solver<'_> {
    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    fn assign(&mut self, val: usize, pos: Vec2) {
        let id = pos.x + pos.y * self.board.size();
        self.board[id].set(val);
        AC3::eliminate(self.board, &mut self.values, pos);
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
