use wasm_bindgen::prelude::*;
use js_sys::{Array, Set};

#[wasm_bindgen]
pub fn difference(first_arr: &Array, second_arr: &Array) -> Result<Array, JsValue> {
    let second_set = Set::new(second_arr);

    let result = first_arr
        .iter()
        .filter(|item| !second_set.has(item))
        .collect::<Array>();

    Ok(result)
}