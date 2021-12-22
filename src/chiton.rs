use crate::get_input::get_input;

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct AStar {
  heap: BinaryHeap<Reverse<(u64, usize, usize, u64)>>,
  costs: HashMap<(usize, usize), u64>,
  grid: Vec<Vec<u64>>,
  height: usize,
  width: usize,
  extended: bool,
}

impl AStar {
  fn new(height: usize, width: usize, grid: Vec<Vec<u64>>, extended: bool) -> Self {
    let heap = BinaryHeap::new();
    let costs = HashMap::new();

    let mut a_star = AStar{heap, costs, grid, height, width, extended};

    a_star.heap.push(a_star.get_heap_element(0, 0, 1));
    a_star.heap.push(a_star.get_heap_element(0, 1, 0));

    a_star
  }

  fn manhattan_distance(&self, i: usize, j: usize) -> u64 {
    let mut width = self.width;
    let mut height= self.height;

    if self.extended {
      width *= 5;
      height *= 5;
    }

    (width - j - 1) as u64 + (height - i - 1) as u64
  }

  fn get_risk(&self, i: usize, j: usize) -> u64 {
    let increment = self.grid[i % self.height][j % self.width] + (i / self.height) as u64 + (j / self.width) as u64;
    (increment - 1) % 9 + 1
  }

  fn get_heap_element(&self, distance: u64, i: usize, j: usize) -> Reverse<(u64, usize, usize, u64)> {
    let element = self.get_risk(i, j);
    Reverse((self.manhattan_distance(i, j) + distance + element, i, j, distance + element))
  }

  fn search(&mut self) -> u64 {
    let mut width = self.width;
    let mut height= self.height;

    if self.extended {
      width *= 5;
      height *= 5;
    }

    loop {
      let Reverse((_, i, j, distance)) = self.heap.pop().expect("No path exists");

      if i == height - 1 && j == width - 1 {
        return distance;
      }

      let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
      for (di, dj) in dirs {
        let y = i as i32 + di;
        let x = j as i32 + dj;

        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
          continue;
        }

        let y = y as usize;
        let x = x as usize;

        let new_cost = distance + self.get_risk(y, x);
        
        let c = self.costs.entry((x, y)).or_insert(u64::MAX);

        if new_cost < *c {
          *c = new_cost;
          self.heap.push(self.get_heap_element(distance, y as usize, x as usize));
        }
      }
    }
  }
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

  let mut a_star = AStar::new(height, width, grid.clone(), false);

  let total_distance = a_star.search();

  let mut a_star_extended = AStar::new(height, width, grid, true);
  let extended_distance = a_star_extended.search();

  (total_distance, extended_distance)
}
