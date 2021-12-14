use crate::get_input::get_input;

use std::collections::HashSet;
use either::Either;

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
    let mut straight_overlap= HashSet::new();

    for line in lines.iter() {
      let x1 = line[0].x;
      let x2 = line[1].x;

      let y1 = line[0].y;
      let y2 = line[1].y;

      let dy = y2 - y1;
      let dx = x2 - x1;

      if dy == 0 {
        let y = y1;
        let it = if x2 > x1 {
          Either::Left(x1..=x2)
        } else {
          Either::Right((x2..=x1).rev())
        };
        for x in it {
          let point = Point{x, y};
          if !straight_seen_points.insert(point.clone()) {
            straight_overlap.insert(point.clone());
          }
        }
      }
      else if dx == 0 {
        let x = x1;
        let it = if y2 > y1 {
          Either::Left(y1..=y2)
        } else {
          Either::Right((y2..=y1).rev())
        };

        for y in it {
          let point = Point{x, y};
          if !straight_seen_points.insert(point.clone()) {
            straight_overlap.insert(point.clone());
          }
        }
      }
    }

    let mut seen_points = HashSet::new();
    let mut overlap= HashSet::new();

    for line in lines.iter() {
      let x1 = line[0].x;
      let x2 = line[1].x;

      let y1 = line[0].y;
      let y2 = line[1].y;

      let dy = y2 - y1;
      let dx = x2 - x1;

      if dy != 0 && dx != 0 {
        let x_it = if x2 > x1 {
          Either::Left(x1..=x2)
        } else {
          Either::Right((x2..=x1).rev())
        };

        let y_it = if y2 > y1 {
          Either::Left(y1..=y2)
        } else {
          Either::Right((y2..=y1).rev())
        };

        for (x, y) in x_it.zip(y_it) {
          let point = Point{x, y};
          if straight_seen_points.get(&point).is_some() {
            if straight_overlap.get(&point).is_some() {
              continue;
            }

            overlap.insert(point);
          }
          else {
            if !seen_points.insert(point.clone()) {
              overlap.insert(point);
            }
          }
        }
      }
    }

    (straight_overlap.len(), straight_overlap.len() + overlap.len())
}
