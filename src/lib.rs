use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn greeting(original: &str) -> String {
    format!("HELLO {}", original.to_uppercase())
}
