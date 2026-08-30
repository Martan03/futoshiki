mod wasm_board;

use futoshiki_core::checker::Checker;
use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_board::WasmBoard;

#[wasm_bindgen]
pub fn check_win(board: &WasmBoard) -> bool {
    Checker::check(&board.inner)
}
