use futoshiki_core::board::{board_gen::BoardGen, board_struct::Board};
use wasm_bindgen::prelude::wasm_bindgen;

/// Core [`Board`] wrapper for wasm exposure.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmBoard {
    inner: Board,
}

#[wasm_bindgen]
impl WasmBoard {
    /// Creates new empty board.
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        Self {
            inner: Board::new(size),
        }
    }

    /// Generates new board using core [`BoardGen`].
    pub fn generate(size: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            inner: BoardGen::generate(size, &mut rng),
        }
    }

    /// Gets the size of the board.
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    /// Gets the value of the currently selected cell.
    pub fn get_value(&mut self, x: usize, y: usize) -> usize {
        let sel = x + y * self.size();
        self.inner[sel].value()
    }

    pub fn get_hor_cond(&self, x: usize, y: usize) -> Option<bool> {
        let id = x + y * self.inner.size().saturating_sub(1);
        self.inner.hor_conds.get(id).copied().flatten()
    }

    pub fn get_ver_cond(&self, x: usize, y: usize) -> Option<bool> {
        let id = x + y * self.inner.size();
        self.inner.ver_conds.get(id).copied().flatten()
    }

    /// Sets the value of the cell on given position to the given value.
    pub fn set_value(&mut self, x: usize, y: usize, val: usize) {
        let sel = x + y * self.size();
        _ = self.inner[sel].set(val);
    }
}
