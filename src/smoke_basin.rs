use crate::get_input::get_input;

pub fn smoke_basin() -> (u64, u64) {
  let input = get_input(9).expect("Could not get input");

  let grid: Vec<Vec<u64>> = input.iter()
    .map(|e| e.chars()
      .map(|e| e.to_digit(10).expect("Bad Input") as u64)
      .collect())
    .collect();
  
  let height = grid.len() - 1;
  let width = grid[0].len() - 1;

  let mut sum =  0;

  for (i, row) in grid.iter().enumerate() {
    for (j, val) in row.iter().enumerate() {
      if i != 0 && val >= &grid[i-1][j] {
        continue;
      }

      if i != height && val >= &grid[i+1][j] {
        continue;
      }

      if j != 0 && val >= &grid[i][j-1] {
        continue;
      }

      if j != width && val >= &grid[i][j+1] {
        continue;
      }

      sum += val + 1;
    }
  }
  

  (sum, 0)
}
