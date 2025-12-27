use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_number(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    match trimmed.parse::<f64>() {
        Ok(n) => n.is_finite(),
        Err(_) => false,
    }
}