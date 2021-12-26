use crate::get_input::get_input;

struct Map {
  grid: Vec<Vec<u64>>,
  height: usize,
  width: usize,

  enhancement: Vec<u64>,
  outer_val: u64 
}

impl Map {
  fn new(grid: Vec<Vec<u64>>, enhancement: Vec<u64>) -> Self {
    let height = grid.len();
    let width = grid[0].len();
    let outer_val = 0;

    Map{grid, height, width, enhancement, outer_val}
  }

  fn get_value(&self, i: i64, j: i64) -> u64 {
    if i < 0 || i as usize >= self.height || j < 0 || j as usize >= self.width {
      return self.outer_val;
    }

    self.grid[i as usize][j as usize]
  }

  fn get_enhanced(&self, i: i64, j: i64) -> u64 {
    let mut bits = 0;
    for di in -1..=1 {
      for dj in -1..=1 {
        bits = bits << 1;
        bits += self.get_value(i + di, j + dj) as usize;
      }
    }

    self.enhancement[bits]
  }

  fn enhance(&mut self) {
    let mut new_grid = vec![vec![0; self.width+2]; self.height+2];

    for i in -1..self.height as i64+1 {
      for j in -1..self.width as i64+1 {
        new_grid[(i + 1) as usize][(j + 1) as usize] = self.get_enhanced(i, j);
      }
    }

    self.height += 2;
    self.width += 2;

    self.grid = new_grid;

    self.outer_val = 1 - self.outer_val;
  }
}

pub fn trench_map() -> (u64, u64) {
    let input = get_input(20).expect("Could not get input");

    let mut it = input.iter();

    let enhancement = it.next()
      .expect("Bad Enhancement Algorithm")
      .chars()
      .map(|c| match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("Bad Enhancement Algorithm"),
      })
      .collect();

    it.next();

    let grid = it.map(|e| {
      e.chars()
      .map(|c| match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("Bad Image"),
      })
      .collect()
    })
    .collect();

    let mut map = Map::new(grid, enhancement);

    map.enhance();
    map.enhance();

    let lit_pixels = map.grid.iter()
      .map(|e| e.iter().sum::<u64>())
      .sum();

    for _ in 2..50 {
      map.enhance();
    }

    let lit_pixels_2 = map.grid.iter()
      .map(|e| e.iter().sum::<u64>())
      .sum();

    (lit_pixels, lit_pixels_2)
}
