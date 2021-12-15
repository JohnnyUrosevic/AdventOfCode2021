use crate::get_input::get_input;

pub fn seven_segment_search() -> (usize, usize) {
    let input = get_input(8).expect("Could not get input");

    let easy_outputs = input.iter()
      .map(|e| e.split('|')
         .skip(1)
         .next()
         .expect("Bad Input")
        )
        .map(|e| e.split(' ')
          .filter(|e| match e.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
          })
          .count()
        )
        .sum();

    (easy_outputs, 0)
}
