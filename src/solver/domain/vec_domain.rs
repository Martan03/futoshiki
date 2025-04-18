use super::DomainTrait;

#[derive(Debug, Clone)]
pub struct VecDomain(pub Vec<usize>);

impl VecDomain {
    pub fn default(max: usize) -> Self {
        Self((1..=max).collect())
    }
}

impl DomainTrait for VecDomain {
    fn remove(&mut self, value: usize) -> Option<bool> {
        let len = self.0.len();
        self.0.retain(|&v| v != value);

        let new_len = self.0.len();
        (new_len != 0).then_some(len != new_len)
    }

    fn remove_greater(&mut self, value: usize) -> Option<bool> {
        let len = self.0.len();
        self.0.retain(|&v| v < value);

        let new_len = self.0.len();
        (new_len != 0).then_some(new_len != len)
    }

    fn remove_lower(&mut self, value: usize) -> Option<bool> {
        let len = self.0.len();
        self.0.retain(|&v| v > value);

        let new_len = self.0.len();
        (new_len != 0).then_some(new_len != len)
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
    use crate::solver::domain::{vec_domain::VecDomain, DomainTrait};

    #[test]
    fn vec_domain_default() {
        let domain = VecDomain::default(5);
        assert_eq!(domain.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn hash_domain_remove() {
        let mut domain = VecDomain(vec![2, 3]);

        assert_eq!(domain.remove(3), Some(true));
        assert_eq!(domain.0, vec![2]);

        assert_eq!(domain.remove(3), Some(false));
        assert_eq!(domain.0, vec![2]);

        assert_eq!(domain.remove(2), None);
        assert_eq!(domain.0, Vec::<usize>::new());
        assert_eq!(domain.remove(1), None);
    }

    #[test]
    fn hash_domain_remove_greater() {
        let mut domain = VecDomain(vec![1, 2, 3, 4, 6]);

        assert_eq!(domain.remove_greater(4), Some(true));
        assert_eq!(domain.0, vec![1, 2, 3]);

        assert_eq!(domain.remove_greater(4), Some(false));
        assert_eq!(domain.0, vec![1, 2, 3]);

        assert_eq!(domain.remove_greater(1), None);
        assert_eq!(domain.0, Vec::<usize>::new());
        assert_eq!(domain.remove_greater(4), None);
    }

    #[test]
    fn hash_domain_remove_lower() {
        let mut domain = VecDomain(vec![1, 3, 4, 5, 6]);

        assert_eq!(domain.remove_lower(4), Some(true));
        assert_eq!(domain.0, vec![5, 6]);

        assert_eq!(domain.remove_lower(4), Some(false));
        assert_eq!(domain.0, vec![5, 6]);

        assert_eq!(domain.remove_lower(6), None);
        assert_eq!(domain.0, Vec::<usize>::new());
        assert_eq!(domain.remove_lower(3), None);
    }

    #[test]
    fn hash_domain_min() {
        let domain = VecDomain(vec![3, 4, 6]);
        assert_eq!(domain.min(), 3);
    }

    #[test]
    fn hash_domain_max() {
        let domain = VecDomain(vec![3, 4, 6]);
        assert_eq!(domain.max(), 6);
    }

    #[test]
    fn hash_domain_values() {
        let domain = VecDomain(vec![3, 4, 6]);
        let mut values = domain.values();
        values.sort();
        assert_eq!(values, vec![3, 4, 6]);
    }
}
