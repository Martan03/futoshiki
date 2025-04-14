#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arc {
    Unique(usize, usize),
    Inequality(usize, usize),
}
