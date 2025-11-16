use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn avg(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}
