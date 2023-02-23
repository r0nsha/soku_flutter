use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_sudoku(port_: MessagePort) {
    wire_sudoku_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue
