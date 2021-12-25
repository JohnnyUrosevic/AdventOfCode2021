use crate::get_input::get_input;

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
  // When projectile returns to y=0 velocity = -y0
  let y0 = -y_area[0];
  println!("{}", y0);
  let max_height = (1..y0).sum();

  (max_height, 0)
}
