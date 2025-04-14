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

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
