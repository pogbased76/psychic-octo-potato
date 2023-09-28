use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
export function initializeOrbitDB() {
  // Your JavaScript code here...
}
")]
extern "C" {
    fn initializeOrbitDB();
}

// Call the JavaScript function from Rust
initializeOrbitDB();