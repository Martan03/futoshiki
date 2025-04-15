use super::DomainTrait;

#[derive(Debug, Clone)]
pub struct BitDomain(pub usize);

impl DomainTrait for BitDomain {
    fn remove(&mut self, value: usize) -> bool {
        let mask = 1 << value.saturating_sub(1);
        let exists = self.0 & mask != 0;
        self.0 &= !mask;
        exists
    }

    fn remove_greater(&mut self, value: usize) -> bool {
        if value == 0 {
            return false;
        }
        let prev = self.0;
        self.0 &= (1 << (value - 1)) - 1;
        prev != self.0
    }

    fn remove_lower(&mut self, value: usize) -> bool {
        if value == 0 {
            return false;
        }
        let prev = self.0;
        self.0 &= !((1 << value) - 1);
        prev != self.0
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
            if self.0 & (1 << i) != 0 {
                values.push(i + 1);
            }
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::domain::{bit_domain::BitDomain, DomainTrait};

    #[test]
    fn bit_domain_remove() {
        let mut domain = BitDomain(0b1111);

        assert!(domain.remove(3));
        assert_eq!(domain.0, 0b1011);

        assert!(!domain.remove(3));
        assert_eq!(domain.0, 0b1011);
    }

    #[test]
    fn bit_domain_remove_greater() {
        let mut domain = BitDomain(0b101100);

        assert!(domain.remove_greater(4));
        assert_eq!(domain.0, 0b100);

        assert!(!domain.remove_greater(4));
        assert_eq!(domain.0, 0b100);
    }

    #[test]
    fn bit_domain_remove_lower() {
        let mut domain = BitDomain(0b111101);

        assert!(domain.remove_lower(4));
        assert_eq!(domain.0, 0b110000);

        assert!(!domain.remove_lower(4));
        assert_eq!(domain.0, 0b110000);
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
}
