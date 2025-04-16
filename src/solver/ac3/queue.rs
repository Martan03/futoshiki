use std::collections::{HashSet, VecDeque};

use super::arc::Arc;

#[derive(Debug)]
pub struct Queue {
    queue: VecDeque<Arc>,
    set: HashSet<Arc>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    pub fn push(&mut self, item: Arc) {
        if self.set.insert(item.clone()) {
            self.queue.push_back(item);
        }
    }

    pub fn pop(&mut self) -> Option<Arc> {
        if let Some(item) = self.queue.pop_front() {
            self.set.remove(&item);
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::ac3::arc::Arc;

    use super::Queue;

    #[test]
    fn test_push() {
        let mut queue = Queue::new();

        queue.push(Arc::Unique(1, 2));
        assert_eq!(queue.queue.len(), 1);
        queue.push(Arc::Unique(1, 2));
        assert_eq!(queue.queue.len(), 1);

        queue.push(Arc::Inequality(1, 2));
        assert_eq!(queue.queue.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut queue = Queue::new();

        queue.push(Arc::Unique(1, 2));
        queue.push(Arc::Inequality(1, 2));

        assert_eq!(queue.queue.len(), 2);
        assert_eq!(queue.pop(), Some(Arc::Unique(1, 2)));
        assert_eq!(queue.queue.len(), 1);
        assert_eq!(queue.pop(), Some(Arc::Inequality(1, 2)));
        assert_eq!(queue.queue.len(), 0);
        assert_eq!(queue.pop(), None);
    }
}
