use crate::get_input::get_input;

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

#[inline(always)]
fn manhattan_distance(i: usize, j: usize, width: usize, height: usize) -> u64 {
  (width - j - 1) as u64 + (height - i - 1) as u64
}

pub fn chiton() -> (u64, u64) {
  let input = get_input(15).expect("Could not get input");

  let grid: Vec<Vec<u64>> = input.iter()
    .map(|row| row.chars()
      .map(|c| c.to_digit(10).expect("NaN") as u64)
      .collect()
    )
    .collect();

  let height = grid.len();
  let width = grid[0].len();

  // A* search
  let get_heap_element = |distance, i, j| -> Reverse<(u64, usize, usize, u64)> {
    Reverse((manhattan_distance(i, j, width, height) + distance + grid[i][j], i, j, distance + grid[i][j]))
  };

  let mut heap = BinaryHeap::new();
  heap.push(get_heap_element(0, 0, 1));
  heap.push(get_heap_element(0, 1, 0));

  let mut closed = HashSet::new();

  let total_distance;
  loop {
    let Reverse((_, i, j, distance)) = heap.pop().expect("No path exists");

    if i == height - 1 && j == width - 1 {
      total_distance = distance;
      break;
    }

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (di, dj) in dirs {
      let y = i as i32 + di;
      let x = j as i32 + dj;

      if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || closed.get(&(y as usize, x as usize)).is_some() {
        continue;
      }

      heap.push(get_heap_element(distance, y as usize, x as usize));
    }

    closed.insert((i, j));
  }

  (total_distance, 0)
}
