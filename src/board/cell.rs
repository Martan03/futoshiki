#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    value: usize,
    enabled: bool,
}

impl Cell {
    /// Creates new disabled [`Cell`] with given value
    pub fn new(value: usize) -> Self {
        Self {
            value,
            enabled: false,
        }
    }

    /// Creates new enabled empty [`Cell`]
    pub fn empty() -> Self {
        Cell::default()
    }

    /// Sets [`Cell`] value to given value
    pub fn set(&mut self, value: usize) -> bool {
        if self.enabled {
            self.value = value;
        }
        self.enabled
    }

    /// Gets [`Cell`] value
    pub fn value(&self) -> usize {
        self.value
    }

    /// Gets whether [`Cell`] is enabled
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Enables the [`Cell`]
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the [`Cell`]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: 0,
            enabled: true,
        }
    }
}

impl From<i32> for Cell {
    fn from(value: i32) -> Self {
        Self {
            value: value as usize,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn cell_set_when_enabled() {
        let mut cell = Cell::default();

        assert!(cell.set(10));
        assert_eq!(cell.value, 10);
    }

    #[test]
    fn cell_set_when_disabled() {
        let mut cell = Cell::new(5);

        assert!(!cell.set(10));
        assert_eq!(cell.value, 5);
    }
}
