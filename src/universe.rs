use wasm_bindgen::prelude::*;

use crate::terrain::{generate_terrain, Tile};

#[wasm_bindgen]
pub struct Universe {
  tick: usize,
  tiles: Vec<Tile>,
  size: usize,
}

#[wasm_bindgen]
impl Universe {
  pub fn new(size: usize) -> Universe {
    Self {
      tick: 0,
      tiles: generate_terrain(size),
      size,
    }
  }

  pub fn tick(&mut self) {
    // TODO
  }

  pub fn tiles_ptr(&self) -> *const Tile {
    self.tiles.as_ptr()
  }

  pub fn size(&self) -> usize {
    self.size
  }
}
