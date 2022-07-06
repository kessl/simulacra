mod universe;
mod terrain;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}
