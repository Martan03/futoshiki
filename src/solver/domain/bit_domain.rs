use super::DomainTrait;

#[derive(Debug, Clone)]
pub struct BitDomain(pub usize);

impl BitDomain {
    pub fn default(max: usize) -> Self {
        Self((1 << max) - 1)
    }
}

impl DomainTrait for BitDomain {
    /// Removes a value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    ///
    /// Can panic when invalid value is given
    fn remove(&mut self, value: usize) -> Option<bool> {
        let prev = self.0;
        self.0 &= !(1 << (value - 1));
        (self.0 != 0).then_some(self.0 != prev)
    }

    /// Removes all values greater than the given value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    ///
    /// Can panic when invalid value is given
    fn remove_greater(&mut self, value: usize) -> Option<bool> {
        let prev = self.0;
        self.0 &= (1 << (value - 1)) - 1;
        (self.0 != 0).then_some(prev != self.0)
    }

    /// Removes all values lower than the given value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    ///
    /// Can panic when invalid value is given
    fn remove_lower(&mut self, value: usize) -> Option<bool> {
        let prev = self.0;
        self.0 &= !((1 << value) - 1);
        (self.0 != 0).then_some(prev != self.0)
    }

    fn min(&self) -> usize {
        if self.0 == 0 {
            return 0;
        }
        self.0.trailing_zeros() as usize + 1
    }

    fn max(&self) -> usize {
        if self.0 == 0 {
            return 0;
        }
        (usize::BITS - self.0.leading_zeros()) as usize
    }

    fn values(&self) -> Vec<usize> {
        let mut values = Vec::new();
        for i in 0..usize::BITS as usize {
            if (self.0 & (1 << i)) != 0 {
                values.push(i + 1);
            }
        }
        values
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::domain::{bit_domain::BitDomain, DomainTrait};

    #[test]
    fn hash_domain_default() {
        let domain = BitDomain::default(5);
        assert_eq!(domain.0, 0b11111);
    }

    #[test]
    fn bit_domain_remove() {
        let mut domain = BitDomain(0b110);

        assert_eq!(domain.remove(3), Some(true));
        assert_eq!(domain.0, 0b10);

        assert_eq!(domain.remove(3), Some(false));
        assert_eq!(domain.0, 0b10);

        assert_eq!(domain.remove(2), None);
        assert_eq!(domain.0, 0);
        assert_eq!(domain.remove(2), None);
    }

    #[test]
    fn bit_domain_remove_greater() {
        let mut domain = BitDomain(0b101100);

        assert_eq!(domain.remove_greater(4), Some(true));
        assert_eq!(domain.0, 0b100);

        assert_eq!(domain.remove_greater(4), Some(false));
        assert_eq!(domain.0, 0b100);

        assert_eq!(domain.remove_greater(1), None);
        assert_eq!(domain.0, 0);
    }

    #[test]
    fn bit_domain_remove_lower() {
        let mut domain = BitDomain(0b111101);

        assert_eq!(domain.remove_lower(4), Some(true));
        assert_eq!(domain.0, 0b110000);

        assert_eq!(domain.remove_lower(4), Some(false));
        assert_eq!(domain.0, 0b110000);

        assert_eq!(domain.remove_lower(6), None);
        assert_eq!(domain.0, 0);
    }

    #[test]
    fn bit_domain_min() {
        let domain = BitDomain(0b101100);
        assert_eq!(domain.min(), 3);
    }

    #[test]
    fn bit_domain_max() {
        let domain = BitDomain(0b101100);
        assert_eq!(domain.max(), 6);
    }

    #[test]
    fn bit_domain_values() {
        let domain = BitDomain(0b101100);
        assert_eq!(domain.values(), vec![3, 4, 6]);
    }

    #[test]
    fn bit_domain_is_empty() {
        let domain = BitDomain(0);
        assert!(domain.is_empty());

        let domain = BitDomain(0b101100);
        assert!(!domain.is_empty());
    }
}
