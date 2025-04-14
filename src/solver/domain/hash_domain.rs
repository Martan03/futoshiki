use std::collections::HashSet;

use super::Domain;

pub struct HashDomain(pub HashSet<usize>);

impl Domain for HashDomain {
    fn remove(&mut self, value: usize) -> bool {
        self.0.remove(&value)
    }

    fn remove_greater(&mut self, value: usize) -> bool {
        let len = self.0.len();
        self.0.retain(|&v| v > value);
        (!self.0.is_empty())
            .then_some(self.0.len() != len)
            .unwrap_or(false)
    }

    fn remove_lower(&mut self, value: usize) -> bool {
        let len = self.0.len();
        self.0.retain(|&v| v < value);
        (!self.0.is_empty())
            .then_some(self.0.len() != len)
            .unwrap_or(false)
    }

    fn min(&mut self) -> usize {
        self.0.iter().min().copied().unwrap_or(0)
    }

    fn max(&mut self) -> usize {
        self.0.iter().max().copied().unwrap_or(0)
    }
}
