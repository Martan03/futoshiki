use super::Domain;

pub struct BitDomain(pub usize);

impl Domain for BitDomain {
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
        self.0 &= (1 << value) - 1;
        true
    }

    fn remove_lower(&mut self, value: usize) -> bool {
        if value == 0 {
            return false;
        }
        self.0 &= !((1 << value) - 1);
        true
    }

    fn min(&mut self) -> usize {
        if self.0 == 0 {
            return 0;
        }
        self.0 & 1 << self.0.trailing_zeros()
    }

    fn max(&mut self) -> usize {
        if self.0 == 0 {
            return 0;
        }
        self.0 & 1 << (usize::BITS - self.0.leading_zeros())
    }
}
