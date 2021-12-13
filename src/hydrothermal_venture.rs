use crate::get_input::get_input;

struct Point {
  x: i32,
  y: i32,
}

pub fn hydrothermal_venture() -> (i32, i32) {
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
    
    for line in lines.iter() {
      println!("{}", line[0].x);
    }
    (0, 0)
}
