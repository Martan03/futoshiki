use std::collections::HashSet;

use super::DomainTrait;

#[derive(Debug, Clone)]
pub struct HashDomain(pub HashSet<usize>);

impl DomainTrait for HashDomain {
    fn remove(&mut self, value: usize) -> bool {
        self.0.remove(&value)
    }

    fn remove_greater(&mut self, value: usize) -> bool {
        let len = self.0.len();
        self.0.retain(|&v| v < value);
        (!self.0.is_empty())
            .then_some(self.0.len() != len)
            .unwrap_or(false)
    }

    fn remove_lower(&mut self, value: usize) -> bool {
        let len = self.0.len();
        self.0.retain(|&v| v > value);
        (!self.0.is_empty())
            .then_some(self.0.len() != len)
            .unwrap_or(false)
    }

    fn min(&self) -> usize {
        self.0.iter().min().copied().unwrap_or(0)
    }

    fn max(&self) -> usize {
        self.0.iter().max().copied().unwrap_or(0)
    }

    fn values(&self) -> Vec<usize> {
        self.0.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::solver::domain::{hash_domain::HashDomain, DomainTrait};

    #[test]
    fn hash_domain_remove() {
        let mut domain = HashDomain(HashSet::from_iter([1, 2, 3, 4]));

        assert!(domain.remove(3));
        assert_eq!(domain.0, HashSet::from_iter([1, 2, 4]));

        assert!(!domain.remove(3));
        assert_eq!(domain.0, HashSet::from_iter([1, 2, 4]));
    }

    #[test]
    fn hash_domain_remove_greater() {
        let mut domain = HashDomain(HashSet::from_iter([1, 2, 3, 4, 6]));

        assert!(domain.remove_greater(4));
        assert_eq!(domain.0, HashSet::from_iter([1, 2, 3]));

        assert!(!domain.remove_greater(4));
        assert_eq!(domain.0, HashSet::from_iter([1, 2, 3]));
    }

    #[test]
    fn hash_domain_remove_lower() {
        let mut domain = HashDomain(HashSet::from_iter([1, 3, 4, 5, 6]));

        assert!(domain.remove_lower(4));
        assert_eq!(domain.0, HashSet::from_iter([5, 6]));

        assert!(!domain.remove_lower(4));
        assert_eq!(domain.0, HashSet::from_iter([5, 6]));
    }

    #[test]
    fn hash_domain_min() {
        let domain = HashDomain(HashSet::from_iter([3, 4, 6]));
        assert_eq!(domain.min(), 3);
    }

    #[test]
    fn hash_domain_max() {
        let domain = HashDomain(HashSet::from_iter([3, 4, 6]));
        assert_eq!(domain.max(), 6);
    }

    #[test]
    fn hash_domain_values() {
        let domain = HashDomain(HashSet::from_iter([3, 4, 6]));
        let mut values = domain.values();
        values.sort();
        assert_eq!(values, vec![3, 4, 6]);
    }
}
