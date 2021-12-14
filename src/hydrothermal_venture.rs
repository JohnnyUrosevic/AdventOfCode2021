use crate::get_input::get_input;

use std::collections::HashSet;
use std::mem;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
  x: i32,
  y: i32,
}

pub fn hydrothermal_venture() -> (usize, usize) {
    let input = get_input(5).expect("Could not get input");
    let lines: Vec<Vec<Point>> = input.iter()
      .map(|e| e.split(" ")
        .filter(|e| e != &"->")
        .map(|e| {
          let mut coords = e.split(',');
          let x = coords.next()
            .expect("Bad input")
            .parse()
            .expect("NaN");
          let y = coords.next()
            .expect("Bad input")
            .parse()
            .expect("NaN");

          Point{x, y}
        })
        .collect()
      )
      .collect();

    let mut straight_seen_points = HashSet::new();
    let mut seen_points = HashSet::new();
    let mut straight_overlap= HashSet::new();
    let mut overlap= HashSet::new();

    for line in lines.iter() {
      let mut x1 = line[0].x;
      let mut x2 = line[1].x;

      if x2 < x1 {
        mem::swap(&mut x2, &mut x1);
      }

      let mut y1 = line[0].y;
      let mut y2 = line[1].y;

      if y2 < y1 {
        mem::swap(&mut y2, &mut y1);
      }

      let straight = x1 == x2 || y1 == y2;

      let dy = y2 - y1;
      let dx = x2 - x1;

      if dy > dx {
        let mut x = x1;
        for y in y1..y2+1 {
          for _ in 0..dx+1 {
            let point = Point{x, y};
            if !seen_points.insert(point.clone()) {
                overlap.insert(point.clone());
            };
            if straight && !straight_seen_points.insert(point.clone()) {
              straight_overlap.insert(point.clone());
            }
            x += !straight as i32;
          }
        }
      }
      else {
        let mut y = y1;
        for x in x1..x2+1 {
          for _ in 0..dy+1 {
            let point = Point{x, y};
            if !seen_points.insert(point.clone()) {
                overlap.insert(point.clone());
            };
            if straight && !straight_seen_points.insert(point.clone()) {
              straight_overlap.insert(point.clone());
            }
            y += !straight as i32;
          }
        }
      }
    }
    (straight_overlap.len(), overlap.len())
}
