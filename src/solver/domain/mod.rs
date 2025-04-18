pub mod bit_domain;
pub mod hash_domain;

pub trait DomainTrait: DomainClone {
    /// Removes a value from the domain. Returns whether the value was present
    /// in the domain.
    fn remove(&mut self, value: usize) -> Option<bool>;

    /// Removes all values greater than the given value from the domain.
    /// Returns whether any values were removed.
    fn remove_greater(&mut self, value: usize) -> Option<bool>;

    /// Removes all values lower than the given value from the domain.
    /// Returns whether any values were removed.
    fn remove_lower(&mut self, value: usize) -> Option<bool>;

    /// Returns the minimum value in the domain.
    fn min(&self) -> usize;

    /// Returns the maximum value in the domain.
    fn max(&self) -> usize;

    /// Returns all values from the domain.
    fn values(&self) -> Vec<usize>;
}

pub trait DomainClone {
    /// Clones the domain box
    fn clone_box(&self) -> Box<dyn DomainTrait>;
}

impl<T> DomainClone for T
where
    T: DomainTrait + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn DomainTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DomainTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
