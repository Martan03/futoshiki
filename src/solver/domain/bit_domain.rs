use super::DomainTrait;

#[derive(Debug, Clone)]
pub struct BitDomain(pub usize);

impl BitDomain {
    pub fn default(max: usize) -> Self {
        Self((1 << max) - 1)
    }
}

impl DomainTrait for BitDomain {
    fn remove(&mut self, value: usize) -> Option<bool> {
        let prev = self.0;
        self.0 &= !(1 << value - 1);
        (self.0 != 0).then_some(self.0 != prev)
    }

    fn remove_greater(&mut self, value: usize) -> Option<bool> {
        let prev = self.0;
        self.0 &= (1 << (value - 1)) - 1;
        (self.0 != 0).then_some(prev != self.0)
    }

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
        let mut bits = self.0;
        let mut values = Vec::new();
        for i in 0..usize::BITS as usize {
            if bits == 0 {
                break;
            }

            if (bits & 1) != 0 {
                values.push(i + 1);
            }
            bits >>= 1;
        }
        values
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
        assert_eq!(domain.remove(1), None);
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
        assert_eq!(domain.remove_greater(5), None);
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
        assert_eq!(domain.remove_lower(3), None);
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
