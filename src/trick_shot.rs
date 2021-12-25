use crate::get_input::get_input;

use std::cmp::{max, min};
use std::collections::HashSet;

pub fn trick_shot() -> (i64, i64) {
  let input = get_input(17).expect("Could not get input");
  
  let it = input.iter()
    .next()
    .expect("Bad input")
    .split(' ');

  let x_area: Vec<i64> = it.clone()
    .skip(2)
    .next()
    .expect("Bad input")
    .chars()
    .filter(|c| *c != ',')
    .skip(2)
    .collect::<String>()
    .split("..")
    .map(|e| e.parse().expect("NaN"))
    .collect();

  let y_area: Vec<i64> = it.skip(3)
    .next()
    .expect("Bad input")
    .chars()
    .filter(|c| *c != ',')
    .skip(2)
    .collect::<String>()
    .split("..")
    .map(|e| e.parse().expect("NaN"))
    .collect();
  
  // We want to maximize initial y velocity without overshooting
  // When projectile returns to y=0 velocity = -y0-1
  let y0_max = -y_area[0]-1;
  let max_height = (1..=y0_max).sum();

  let y0_min = y_area[0];

  let mut dist = 0;
  let mut x_min = 0;

  while dist < x_area[0] {
    x_min += 1;
    dist += x_min;
  }

  let x_max = x_area[1];

  let mut possibilities = HashSet::new();
  for vy0 in y0_min..=y0_max {
    let mut t = max(0, vy0) * 2 + (vy0 >= 0) as i64;

    let mut y = 0;
    let mut vy = min(vy0, -vy0-1);
    while y >= y_area[0] {
      if y <= y_area[1] {
        for vx in x_min..=x_max {
          let dist: i64 = (max(0, vx as i64 - t + 1 as i64)..=vx).sum();
          if dist >= x_area[0] && dist <= x_area[1] {
            possibilities.insert((vx, vy0));
          }
        }
      }

      y += vy;
      vy -= 1;
      t += 1;
    }
  }

  (max_height, possibilities.len() as i64)
}
