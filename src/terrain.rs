use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
  Air,
  Water,
  Rock,
}

pub fn generate_terrain(width: usize, height: usize) -> Vec<Tile> {
  if width != height {
    panic!("Sorry, this generator can only handle square universes")
  }

  let mut heightmap = vec![Tile::Air; width * height];
  let first_row = heightmap.chunks_mut(width).next().unwrap();
  for tile in first_row {
    *tile = Tile::Rock;
  }
  let last_row = heightmap.chunks_mut(width).last().unwrap();
  for tile in last_row {
    *tile = Tile::Water;
  }

  heightmap
}
