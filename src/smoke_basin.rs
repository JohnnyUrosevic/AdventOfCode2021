use crate::get_input::get_input;

use std::cmp::max;

struct UnionFind {
  groups: Vec<usize>,
  group_sizes: Vec<u64>,
}

impl UnionFind {
  fn new(size: usize) -> Self {
    let groups = (0..size).collect();
    let group_sizes = vec![1; size];

    UnionFind{groups, group_sizes}
  }

  fn find(&mut self, group: usize) -> usize {
    let mut curr: usize = group;

    while curr != self.groups[curr] {
      curr = self.groups[curr];
    }
    let root = curr;

    curr = group;
    while curr != self.groups[curr] {
      self.groups[curr] = root;
      curr = self.groups[curr];
    }

    root
  }

  fn union(&mut self, a: usize, b: usize) {
    let a = self.find(a);
    let b = self.find(b);

    if a == b {
      return;
    }

    if self.group_sizes[a] > self.group_sizes[b] {
      self.groups[b] = a;
      self.group_sizes[a] += self.group_sizes[b];
    }
    else {
      self.groups[a] = b;
      self.group_sizes[b] += self.group_sizes[a];
    }
  }
}

fn get_pos(i: usize, j: usize, width: usize) -> usize {
  i * width + j
}

pub fn smoke_basin() -> (u64, u64) {
  let input = get_input(9).expect("Could not get input");

  let grid: Vec<Vec<u64>> = input.iter()
    .map(|e| e.chars()
      .map(|e| e.to_digit(10).expect("Bad Input") as u64)
      .collect())
    .collect();

  let height = grid.len();
  let width = grid[0].len();

  let mut low_point_sum = 0;

  for (i, row) in grid.iter().enumerate() {
    for (j, val) in row.iter().enumerate() {
      if i != 0 && val >= &grid[i-1][j] {
        continue;
      }

      if i != height - 1 && val >= &grid[i+1][j] {
        continue;
      }

      if j != 0 && val >= &grid[i][j-1] {
        continue;
      }

      if j != width - 1 && val >= &grid[i][j+1] {
        continue;
      }

      low_point_sum += val + 1;
    }
  }

  let mut basins = UnionFind::new(height * width);

  for (i, row) in grid.iter().enumerate() {
    for (j, val) in row.iter().enumerate() {
      if *val == 9 {
        continue;
      }
      
      if i != 0 && grid[i-1][j] != 9 {
        basins.union(get_pos(i, j, width), get_pos(i-1, j, width));
      }

      if j != 0 && grid[i][j-1] != 9 {
        basins.union(get_pos(i, j, width), get_pos(i, j-1, width));
      }
    }
  }

  let basin_product = basins.group_sizes
    .iter()
    .fold([0,0,0], |acc, e| {
      if e >= &acc[0] {
        [*e, acc[0], acc[1]]
      }
      else if e >= &acc[1] {
        [acc[0], *e, acc[1]]
      }
      else {
        [acc[0], acc[1], max(acc[2], *e)]
      }
    })
    .iter()
    .product();

  (low_point_sum, basin_product)
}
