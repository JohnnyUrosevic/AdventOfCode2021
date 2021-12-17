use crate::get_input::get_input;

use std::collections::{VecDeque, HashSet};
use itertools::iproduct;

pub fn increment(i: usize, j: usize, grid: &mut Vec<Vec<u32>>, 
  queue: &mut VecDeque<(usize, usize)>, flashing: &mut HashSet<(usize, usize)>) {
  if flashing.get(&(i, j)).is_none() {
    grid[i][j] += 1;

    if grid[i][j] > 9 {
      flashing.insert((i, j));

      let range = [-1, 0, 1].iter();
      let dirs = iproduct!(range.clone(), range).filter(|e| e != &(&0, &0));

      for (di, dj) in dirs {
        let x = i as i32 + di;
        let y = j as i32 + dj;

        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
          continue;
        }
  
        increment(x as usize, y as usize, grid, queue, flashing);
      }
    }
  }
}

pub fn dumbo_octopus() -> (usize, usize) {
  let input = get_input(11).expect("Could not get input");

  let mut grid: Vec<Vec<u32>> = input.iter()
    .map(|row|
      row.chars()
        .map(|e| e.to_digit(10).expect("NaN"))
        .collect()
    )
    .collect();

  let mut flashed = 0;
  let mut step = 0;
  loop {
    let mut queue = VecDeque::new();
    let mut flashing = HashSet::new();

    for i in 0..grid.len() {
      for j in 0..grid[0].len() {
        increment(i, j, &mut grid, &mut queue, &mut flashing);
      }
    }

    for (i, j) in flashing.iter() {
      grid[*i][*j] = 0;
    }

    if step < 100 {
      flashed += flashing.len();
    }

    step += 1;
    if flashing.len() == 100 {
      break
    }
  }

  (flashed, step)
}
