use crate::board::board_struct::Board;

use super::Solver;

pub struct FcSolver<'a> {
    board: &'a mut Board,
}

impl<'a> Solver<'a> for FcSolver<'a> {
    fn solve(board: &'a mut Board) -> bool {
        let mut solver = Self { board };
        solver.solve_inner()
    }
}

impl<'a> FcSolver<'a> {
    fn gen_values(&mut self) {
        let mut rows = vec![];
        let mut cols = vec![];

        for y in 0..self.board.size() {
            let mut row: Vec<usize> = (0..self.board.size()).collect();
            let mut col: Vec<usize> = (0..self.board.size()).collect();

            for x in 0..self.board.size() {
                self.rem_val(&mut row, x + y * self.board.size());
                self.rem_val(&mut col, y + x * self.board.size());
            }

            rows.push(row);
            cols.push(col);
        }
    }

    fn solve_inner(&mut self) -> bool {
        todo!()
    }

    fn rem_val(&self, arr: &mut Vec<usize>, id: usize) {
        let val = self.board[id].value();
        if val == 0 {
            return;
        }

        if let Some(vid) = arr.iter().position(|v| *v == val) {
            arr.remove(vid);
        }
    }
}
