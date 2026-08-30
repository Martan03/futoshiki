use rand::{seq::SliceRandom, Rng};

use super::domain::DomainTrait;

pub trait Values {
    /// Returns the values of the domain
    fn get(&mut self, id: usize) -> Vec<usize>;
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
    fn get(&mut self, _id: usize) -> Vec<usize> {
        (1..=self.max).collect()
    }
}

pub struct ConstRngValues<R> {
    max: usize,
    rng: R,
}

impl<R: Rng> ConstRngValues<R> {
    pub fn new(max: usize, rng: R) -> Self {
        Self { max, rng }
    }
}

impl<R: Rng> Values for ConstRngValues<R> {
    fn get(&mut self, _id: usize) -> Vec<usize> {
        let mut domain: Vec<usize> = (1..=self.max).collect();
        domain.shuffle(&mut self.rng);
        domain
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
    fn get(&mut self, id: usize) -> Vec<usize> {
        self.values[id].values()
    }
}
