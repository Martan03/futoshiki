pub mod bit_domain;
pub mod hash_domain;

pub trait Domain {
    /// Removes a value from the domain. Returns whether the value was present
    /// in the domain.
    fn remove(&mut self, value: usize) -> bool;

    /// Removes all values greater than the given value from the domain.
    /// Returns whether any values were removed.
    fn remove_greater(&mut self, value: usize) -> bool;

    /// Removes all values lower than the given value from the domain.
    /// Returns whether any values were removed.
    fn remove_lower(&mut self, value: usize) -> bool;

    /// Returns the minimum value in the domain.
    fn min(&mut self) -> usize;

    /// Returns the maximum value in the domain.
    fn max(&mut self) -> usize;
}
