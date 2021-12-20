use crate::get_input::get_input;

use std::collections::HashSet;
use either::Either;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
  x: i32,
  y: i32,
}

fn fold(instruction: &String, dots: &mut HashSet<Point>, width: &mut i32, height: &mut i32) {
  let mut line = instruction.split(' ')
    .skip(2)
    .next()
    .expect("Bad input")
    .split('=');

  let axis = line.next().expect("Bad input");
  let value = line.next().expect("Bad input").parse().expect("NaN");

  let clone = dots.clone();
 
  let new_dots = match axis {
    "x" =>
      Either::Left(clone.iter()
        .filter(|e| e.x > value)
        .map(|e| Point{x: 2 * value - e.x, y: e.y})),
    "y" =>
      Either::Right(clone.iter()
        .filter(|e| e.y > value)
        .map(|e| Point{x: e.x, y: 2 * value - e.y})),
    _ => panic!("Bad axis"),
  };

  dots.retain(|e| match axis {
    "x" => e.x < value,
    "y" => e.y < value,  
    _ => panic!("Bad axis"),
  });

  dots.extend(new_dots);

  match axis {
    "x" => *width = value,
    "y" => *height = value,  
    _ => panic!("Bad axis"),
  }
}

pub fn transparent_origami() -> (usize, usize) {
  let input = get_input(13).expect("Could not get input");

  let mut it = input.iter();
  
  let mut dots = HashSet::new();

  it.by_ref()
    .take_while(|e| **e != "".to_string())
    .for_each(|e|  {
      let mut split = e.split(',');
      let x = split.next().expect("Bad input").parse().expect("NaN");
      let y = split.next().expect("Bad input").parse().expect("NaN");

      dots.insert(Point {x, y});
    });

  let mut width = 0;
  let mut height = 0;

  fold(it.next().expect("Bad input"), &mut dots, &mut width, &mut height);
  let first_fold_count = dots.len();
  
  it.for_each(|e| fold(e, &mut dots, &mut width, &mut height));

  for y in 0..height {
    for x in 0..width {
      print!(" {}", if dots.get(&Point{x, y}).is_some() {'#'} else {'.'});
    }
    println!();
  }
  (first_fold_count, 0)
}
