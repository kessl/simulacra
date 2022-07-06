use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct Universe {
  tick: usize,
  cells: Vec<usize>,
  width: usize,
  height: usize,
}

#[wasm_bindgen]
impl Universe {
  pub fn new(width: usize, height: usize) -> Universe {
    Self {
      tick: 0,
      cells: Vec::with_capacity(width * height),
      width,
      height,
    }
  }

  pub fn tick(&mut self) {
    let mut cells = vec![0; 10].into_iter().enumerate().map(|(i, _)| self.tick + i).collect();
    self.cells.append(&mut cells);
    self.tick += 10;
  }

  pub fn cells_ptr(&self) -> *const usize {
    self.cells.as_ptr()
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}
