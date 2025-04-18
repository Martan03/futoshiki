use super::domain::DomainTrait;

pub trait Values {
    /// Returns the values of the domain
    fn get(&self, id: usize) -> Vec<usize>;
}

pub struct ConstValues {
    max: usize,
}

impl ConstValues {
    /// Creates a new instance of ConstValues
    pub fn new(max: usize) -> Self {
        Self { max }
    }
}

impl Values for ConstValues {
    fn get(&self, _id: usize) -> Vec<usize> {
        (1..=self.max).collect()
    }
}

pub struct DomainValues<D>
where
    D: DomainTrait,
{
    values: Vec<D>,
}

impl<D> DomainValues<D>
where
    D: DomainTrait,
{
    /// Creates a new instance of DomainValues
    pub fn new(values: Vec<D>) -> Self {
        Self { values }
    }
}

impl<D> Values for DomainValues<D>
where
    D: DomainTrait,
{
    fn get(&self, id: usize) -> Vec<usize> {
        self.values[id].values()
    }
}
