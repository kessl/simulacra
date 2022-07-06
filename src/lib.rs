use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct Universe {
  tick: u32,
  cells: Vec<u32>,
}

#[wasm_bindgen]
impl Universe {
  pub fn new() -> Universe {
    Self {
      tick: 0,
      cells: vec![],
    }
  }

  pub fn tick(&mut self) {
    let mut cells = vec![0; 10].into_iter().enumerate().map(|(i, _)| self.tick + i as u32).collect();
    self.cells.append(&mut cells);
    self.tick += 10;
  }

  pub fn cells_ptr(&self) -> *const u32 {
    self.cells.as_ptr()
  }

  pub fn cells_count(&self) -> u32 {
    self.cells.len() as u32
  }
}
