use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(nums: &[f64]) -> f64 {
    nums.iter().sum()
}