use std::collections::HashSet;

use arc::Arc;
use queue::Queue;
use termint::geometry::Vec2;

use crate::board::board_struct::Board;

use super::domain::Domain;

pub mod arc;
pub mod queue;

pub struct AC3<'a> {
    board: &'a mut Board,
    values: Vec<Box<dyn Domain>>,
    queue: Queue,
}

impl<'a> AC3<'a> {
    pub fn generate(
        board: &'a mut Board,
        values: Vec<Box<dyn Domain>>,
    ) -> Vec<HashSet<usize>> {
        let mut ac = Self {
            board,
            values,
            queue: Queue::new(),
        };
        todo!()
    }
}

struct ArcPush<'a> {
    pub queue: &'a mut Queue,
    pub size: usize,
    pub pos: Vec2,
    pub second: usize,
}

impl<'a> AC3<'a> {
    /// Resolves given arc
    fn resolve(&mut self, arc: Arc) {
        match arc {
            Arc::Unique(f, s) => self.resolve_unique(f, s),
            Arc::Inequality(l, g) => self.resolve_inequality(l, g),
        }
    }

    /// Resolves unique arc, pushes related arcs when domain changes
    fn resolve_unique(&mut self, first: usize, second: usize) {
        if self.values[second].remove(self.board[first].value()) {
            self.push_arcs(second, first);
        }
    }

    /// Resolves the inequality arc, pushes related arcs when domain changes
    fn resolve_inequality(&mut self, lower: usize, greater: usize) {
        let min = self.values[lower].min();
        let max = self.values[greater].max();

        if self.values[lower].remove_greater(max) {
            self.push_arcs(lower, greater);
        }
        if self.values[greater].remove_greater(min) {
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

        let first = det.pos.y + det.pos.y * det.size;
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
}
