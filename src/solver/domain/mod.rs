pub mod bit_domain;
pub mod hash_domain;

pub type Domains = Vec<Box<dyn DomainTrait>>;

pub trait DomainTrait: DomainClone {
    /// Removes a value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    fn remove(&mut self, value: usize) -> Option<bool>;

    /// Removes all values greater than the given value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    fn remove_greater(&mut self, value: usize) -> Option<bool>;

    /// Removes all values lower than the given value from the domain.
    /// Returns None when domain got empty, else returns whether domain changed
    fn remove_lower(&mut self, value: usize) -> Option<bool>;

    /// Returns the minimum value in the domain.
    fn min(&self) -> usize;

    /// Returns the maximum value in the domain.
    fn max(&self) -> usize;

    /// Returns all values from the domain.
    fn values(&self) -> Vec<usize>;

    /// Returns whether the domain is empty
    fn is_empty(&self) -> bool;
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
