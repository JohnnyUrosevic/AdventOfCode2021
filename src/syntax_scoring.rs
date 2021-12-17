use crate::get_input::get_input;

pub fn close_to_open(close: char) -> Option<char> {
  match close {
    ')' => Some('('),
    ']' => Some('['),
    '}' => Some('{'),
    '>' => Some('<'),
    _ => None,
  }
}

pub fn get_score(close: char) -> u64 {
  match close {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!("Not a valid bracket"),
  }
}

pub fn syntax_scoring() -> (u64, u64) {
  let input = get_input(10).expect("Could not get input");

  let mut corrupted_score = 0;
  'outer: for line in input.iter() {
    let mut stack = vec![];

    for c in line.chars() {
      match close_to_open(c) {
        Some(open) => {
          if stack.pop() != Some(open) {
            corrupted_score += get_score(c);
            continue 'outer;
          }
        },
        None => stack.push(c),
      }
    }
  }

  (corrupted_score, 0)
}
