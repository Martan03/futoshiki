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
    pub fn generate(board: &'a mut Board, values: &'a mut Domains) {
        let mut ac = Self {
            board,
            values,
            queue: Queue::new(),
        };

        ac.gen_unique();
        ac.gen_inequality();

        ac.process();
    }

    /// Prunes the given domain using the AC3 algorithm, but starts with only
    /// directly related cells in the queue
    pub fn eliminate(
        board: &'a mut Board,
        values: &'a mut Domains,
        pos: Vec2,
    ) {
        let mut ac = Self {
            board,
            values,
            queue: Queue::new(),
        };

        let id = pos.x + pos.y * ac.board.size();
        for x in 0..ac.board.size() {
            // Row unique arcs
            if x != pos.x {
                let y = pos.y * ac.board.size();
                ac.queue.push(Arc::Unique(id, x + y));
            }

            // Column unique arcs
            if x != pos.y {
                ac.queue.push(Arc::Unique(id, pos.x + x * ac.board.size()));
            }
        }
        ac.push_arcs(id, usize::MAX);

        ac.process();
    }
}

struct ArcPush<'a> {
    pub queue: &'a mut Queue,
    pub size: usize,
    pub pos: Vec2,
    pub second: usize,
}

impl<'a> AC3<'a> {
    /// Processes the arcs in the queue
    fn process(&mut self) {
        while let Some(arc) = self.queue.pop() {
            match arc {
                Arc::Unique(f, s) => self.resolve_unique(f, s),
                Arc::Inequality(l, g) => self.resolve_inequality(l, g),
            }
        }
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
    fn resolve_unique(&mut self, f: usize, s: usize) {
        let (f, s) = match (self.board[f].value(), self.board[s].value()) {
            (0, 0) => return,
            (0, _) => (s, f),
            (_, 0) => (f, s),
            _ => return,
        };

        if self.values[s].remove(self.board[f].value()) {
            self.push_arcs(s, f);
        }
    }

    /// Resolves the inequality arc, pushes related arcs when domain changes
    fn resolve_inequality(&mut self, lower: usize, greater: usize) {
        let min = self.values[lower].min();
        let max = self.values[greater].max();

        if self.values[lower].remove_greater(max) {
            self.push_arcs(lower, greater);
        }
        if self.values[greater].remove_lower(min) {
            self.push_arcs(greater, lower);
        }
    }

    /// Pushes all the arcs related to the first cell except relation with
    /// second cell
    fn push_arcs(&mut self, first: usize, second: usize) {
        let pos =
            Vec2::new(first % self.board.size(), first / self.board.size());
        let size = self.board.size();
        let lsize = self.board.size().saturating_sub(1);

        let mut details = ArcPush {
            queue: &mut self.queue,
            size,
            pos,
            second,
        };

        Self::push_arc(
            &mut details,
            &self.board.hor_conds,
            (-1, 0),
            false,
            |x, y| Some(x + y * lsize),
        );
        Self::push_arc(
            &mut details,
            &self.board.ver_conds,
            (0, -1),
            false,
            |x, y| Some(x + y * size),
        );
        _ = Self::push_arc(
            &mut details,
            &self.board.hor_conds,
            (1, 0),
            true,
            |_, _| (pos.x < lsize).then_some(pos.x + pos.y * lsize),
        );
        _ = Self::push_arc(
            &mut details,
            &self.board.ver_conds,
            (0, 1),
            true,
            |_, _| (pos.y < lsize).then_some(pos.x + pos.y * size),
        );
    }

    fn push_arc<F>(
        det: &mut ArcPush,
        conds: &[Option<bool>],
        (ox, oy): (isize, isize),
        positive: bool,
        get_id: F,
    ) -> Option<()>
    where
        F: Fn(usize, usize) -> Option<usize>,
    {
        let x = det.pos.x.checked_add_signed(ox)?;
        let y = det.pos.y.checked_add_signed(oy)?;

        let cond = conds.get(get_id(x, y)?).copied().flatten()? ^ positive;

        let first = det.pos.x + det.pos.y * det.size;
        let second = x + y * det.size;
        if second == det.second {
            return Some(());
        }

        let arc = match cond {
            true => Arc::Inequality(first, second),
            false => Arc::Inequality(second, first),
        };
        det.queue.push(arc);
        Some(())
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
