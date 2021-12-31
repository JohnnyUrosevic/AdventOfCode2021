use crate::get_input::get_input;

use std::collections::HashSet;

pub fn reactor_reboot() -> (usize, usize) {
  let input = get_input(22).expect("Could not get input");

  let cuboids: Vec<(bool, Vec<Vec<i64>>)> = input.iter()
    .map(|e| {
      let mut split = e.split(' ');
      let on = split.next().expect("Bad input") == "on";
      let vertices: String = split.next()
        .expect("Bad input")
        .chars()
        .filter(|c| match c {
          '0'..='9' | '.' | ',' | '-' => true,
          _ => false,
        })
        .collect();

      (on, vertices.split(",")
        .map(|e| e.split("..")
          .map(|e| e.parse().expect("NaN"))
          .collect()
      )
      .collect())
    })
    .collect();

  let mut active = HashSet::new();

  for (on, cuboid) in cuboids {
    'outer: for x in cuboid[0][0]..=cuboid[0][1] {
      if x < -50 || x > 50 {
        break 'outer;
      }
      for y in cuboid[1][0]..=cuboid[1][1] {
        if y < -50 || y > 50 {
          break 'outer;
        }
        for z in cuboid[2][0]..=cuboid[2][1] {
          if z < -50 || z > 50 {
            break 'outer;
          }

          if on {
            active.insert((x, y, z));
          }
          else {
            active.remove(&(x, y, z));
          }
        }
      }
    }
  }

  (active.len(), 0)
}
