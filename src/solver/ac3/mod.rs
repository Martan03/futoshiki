use arc::Arc;
use queue::Queue;
use termint::geometry::{Rect, Vec2};

use crate::board::board_struct::Board;

use super::domain::Domains;

pub mod arc;
pub mod queue;

pub struct AC3<'a> {
    board: &'a mut Board,
    values: &'a mut Domains,
    queue: Queue,
}

impl<'a> AC3<'a> {
    /// Prunes the given domain using the AC3 algorithm, starts with every
    /// relation in the queue
    pub fn generate(board: &'a mut Board, values: &'a mut Domains) -> bool {
        let mut ac = Self {
            board,
            values,
            queue: Queue::new(),
        };

        ac.gen_unique();
        ac.gen_inequality();

        ac.process().is_some()
    }

    /// Prunes the given domain using the AC3 algorithm, but starts with only
    /// directly related cells in the queue
    pub fn eliminate(
        board: &'a mut Board,
        values: &'a mut Domains,
        pos: Vec2,
    ) -> bool {
        let mut ac = Self {
            board,
            values,
            queue: Queue::new(),
        };

        let id = pos.x + pos.y * ac.board.size();
        let y = pos.y * ac.board.size();
        for x in 0..ac.board.size() {
            // Row unique arcs
            if x != pos.x {
                ac.queue.push(Arc::Unique(id, x + y));
            }

            // Column unique arcs
            if x != pos.y {
                ac.queue.push(Arc::Unique(id, pos.x + x * ac.board.size()));
            }
        }
        ac.push_arcs(id, usize::MAX);

        ac.process().is_some()
    }
}

impl AC3<'_> {
    /// Processes the arcs in the queue
    fn process(&mut self) -> Option<()> {
        while let Some(arc) = self.queue.pop() {
            match arc {
                Arc::Unique(f, s) => self.resolve_unique(f, s)?,
                Arc::Inequality(l, g) => self.resolve_inequality(l, g)?,
            }
        }
        Some(())
    }

    /// Adds all the unique arcs to the queue
    fn gen_unique(&mut self) {
        for pos in self.board.rect() {
            for i in (pos.x + 1)..self.board.size() {
                // Row unique arcs
                let y = pos.y * self.board.size();
                self.queue.push(Arc::Unique(pos.x + y, i + y));

                // Column unique arcs
                self.queue.push(Arc::Unique(
                    pos.y + pos.x * self.board.size(),
                    pos.y + i * self.board.size(),
                ));
            }
        }
    }

    /// Adds all inequality arcs to the queue
    fn gen_inequality(&mut self) {
        let lsize = self.board.size().saturating_sub(1);
        for pos in Rect::new(0, 0, lsize, self.board.size()) {
            let spos = Vec2::new(pos.x + 1, pos.y);

            let cond = self.board.hor_conds[pos.x + pos.y * lsize];
            self.gen_cond_arc(pos, spos, cond);

            let cond = self.board.ver_conds[pos.y + pos.x * self.board.size()];
            self.gen_cond_arc(pos.inverse(), spos.inverse(), cond);
        }
    }

    /// Resolves unique arc, pushes related arcs when domain changes
    fn resolve_unique(&mut self, f: usize, s: usize) -> Option<()> {
        let (f, s) = match (self.board[f].value(), self.board[s].value()) {
            (0, 0) => return Some(()),
            (0, _) => (s, f),
            (_, 0) => (f, s),
            _ => return Some(()),
        };

        if self.values[s].remove(self.board[f].value()) {
            if self.values[s].is_empty() {
                return None;
            }
            self.push_arcs(s, f);
        }
        Some(())
    }

    /// Resolves the inequality arc, pushes related arcs when domain changes
    fn resolve_inequality(
        &mut self,
        lower: usize,
        greater: usize,
    ) -> Option<()> {
        let min = self.values[lower].min();
        let max = self.values[greater].max();

        if self.values[lower].remove_greater(max) {
            if self.values[lower].is_empty() {
                return None;
            }
            self.push_arcs(lower, greater);
        }
        if self.values[greater].remove_lower(min) {
            if self.values[greater].is_empty() {
                return None;
            }
            self.push_arcs(greater, lower);
        }
        Some(())
    }

    /// Pushes all the arcs related to the first cell except relation with
    /// second cell
    fn push_arcs(&mut self, first: usize, second: usize) {
        let pos =
            Vec2::new(first % self.board.size(), first / self.board.size());
        let lsize = self.board.size().saturating_sub(1);

        let id = pos.x + pos.y * self.board.size();
        if let Some(xs) = pos.x.checked_sub(1) {
            let cond = self.board.hor_conds[xs + pos.y * lsize];
            let sid = xs + pos.y * self.board.size();
            self.push_arc(cond, id, sid, second);
        }
        if let Some(ys) = pos.y.checked_sub(1) {
            let sid = pos.x + ys * self.board.size();
            let cond = self.board.ver_conds[sid];
            self.push_arc(cond, id, sid, second);
        }

        if pos.x < lsize {
            let cond = self.board.hor_conds[pos.x + pos.y * lsize];
            let sid = pos.x + 1 + pos.y * self.board.size();
            self.push_arc(cond.map(|v| !v), id, sid, second);
        }
        if pos.y < lsize {
            let cond = self.board.ver_conds[pos.x + pos.y * self.board.size()];
            let sid = pos.x + (pos.y + 1) * self.board.size();
            self.push_arc(cond.map(|v| !v), id, sid, second);
        }
    }

    /// Pushes given arc to the queue
    fn push_arc(
        &mut self,
        cond: Option<bool>,
        fid: usize,
        sid: usize,
        second: usize,
    ) {
        if sid == second {
            return;
        }
        match cond {
            Some(true) => self.queue.push(Arc::Inequality(fid, sid)),
            Some(false) => self.queue.push(Arc::Inequality(sid, fid)),
            None => {}
        }
    }

    fn gen_cond_arc(&mut self, fpos: Vec2, spos: Vec2, cond: Option<bool>) {
        let f = fpos.x + fpos.y * self.board.size();
        let s = spos.x + spos.y * self.board.size();
        match cond {
            Some(true) => self.queue.push(Arc::Inequality(s, f)),
            Some(false) => self.queue.push(Arc::Inequality(f, s)),
            None => {}
        }
    }
}
