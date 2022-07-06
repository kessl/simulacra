use wasm_bindgen::prelude::*;

use rand::Rng;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
  Air,
  Water,
  Rock,
}

pub fn generate_terrain(size: usize) -> Vec<Tile> {
  generate_heightmap(size).into_iter().map(height_to_tile).collect()
}

fn height_to_tile(height: i32) -> Tile {
  match height {
    i32::MIN..=-20 => Tile::Water,
    -21..=40 => Tile::Air,
    41.. => Tile::Rock,
  }
}

fn index(width: usize, row: usize, col: usize) -> usize {
  row * width + col
}

fn generate_heightmap(size: usize) -> Vec<i32> {
  let mut heightmap = vec![0; size * size];
  let mut rng = rand::thread_rng();

  // set the randomness bounds, higher values mean rougher landscapes
  let mut randomness = size as i32;

  // set the corner points to the same random value
  let rand = rng.gen_range(-randomness..randomness);
  heightmap[index(size, 0, 0)] = rand;
  heightmap[index(size, size - 1, 0)] = rand;
  heightmap[index(size, 0, size - 1)] = rand;
  heightmap[index(size, size - 1, size - 1)] = rand;

  // we make a pass over the heightmap
  // each time we decrease the side length 2 times
  let mut tile_width = size;
  while tile_width > 1 {
    let half_side = tile_width / 2;

    // diamond step
    for x in (0..size).step_by(tile_width as usize) {
      for y in (0..size).step_by(tile_width as usize) {
        if x + tile_width >= size || y + tile_width >= size {
          continue;
        }

        let corner_sum =
          heightmap[index(size, x, y)] +
          heightmap[index(size, x + tile_width, y)] +
          heightmap[index(size, x, y + tile_width)] +
          heightmap[index(size, x + tile_width, y + tile_width)];
        let avg = corner_sum / 4 + rng.gen_range(-randomness..randomness);

        heightmap[index(size, x + half_side, y + half_side)] = avg;
      }
    }

    // square step
    let mut even = true;
    for y in (0..size).step_by(half_side as usize) {
      let x_start = if even { 0 } else { half_side };
      for x in (x_start..size).step_by(tile_width as usize) {
        if x + tile_width >= size || y + half_side >= size || y < half_side {
          continue;
        }

        let square_sum =
          heightmap[index(size, x, y)] +
          heightmap[index(size, x + tile_width, y)] +
          heightmap[index(size, x + half_side, y - half_side)] +
          heightmap[index(size, x + half_side, y + half_side)];
        let avg = square_sum / 4 + rng.gen_range(-randomness..randomness);

        heightmap[index(size, x + half_side, y)] = avg;
      }
      even = !even;
    }

    // reduce the randomness in each pass, making sure it never gets to 0
    randomness = std::cmp::max(randomness / 2, 1);
    tile_width = half_side;
  }

  heightmap
}
