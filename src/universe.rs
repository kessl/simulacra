use wasm_bindgen::prelude::*;

use crate::terrain::{generate_terrain, Tile};

#[wasm_bindgen]
pub struct Universe {
  tick: usize,
  tiles: Vec<Tile>,
  width: usize,
  height: usize,
}

#[wasm_bindgen]
impl Universe {
  pub fn new(width: usize, height: usize) -> Universe {
    Self {
      tick: 0,
      tiles: generate_terrain(width, height),
      width,
      height,
    }
  }

  pub fn tick(&mut self) {
    // TODO
  }

  pub fn tiles_ptr(&self) -> *const Tile {
    self.tiles.as_ptr()
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}
