use crate::get_input::get_input;

const A: u8 = 0b1 << 1;
const B: u8 = 0b1 << 2;
const C: u8 = 0b1 << 3;
const D: u8 = 0b1 << 4;
const E: u8 = 0b1 << 5;
const F: u8 = 0b1 << 6;
const G: u8 = 0b1 << 7;

pub fn get_segment_mask(c: char) -> u8 {
  match c {
    'a' => A,
    'b' => B,
    'c' => C,
    'd' => D,
    'e' => E,
    'f' => F,
    'g' => G,
    _ => panic!("Bad Char"),
  }
}

pub fn get_mask(segments: &str) -> u8 {
  segments.chars()
    .map(get_segment_mask)
    .fold(0, |acc, e| acc | e)
}

pub fn letters_to_digit(segments: &str, one: &str, four: &str) -> u64 {
  let mask = get_mask(segments);
  let one_mask = get_mask(one);
  let bd = get_mask(four) - one_mask;

  match segments.len() {
    2 => 1,
    3 => 7,
    4 => 4,
    7 => 8,
    5 => {
      if mask & one_mask == one_mask {
        3
      }
      else if mask & bd == bd {
        5
      }
      else {
        2
      }
    }
    6 => {
      if mask & (one_mask | bd) == one_mask | bd {
        9
      }
      else if mask & one_mask == one_mask {
        0
      }
      else {
        6
      }
    }
    _ => panic!("Bad segments"),
  }
}

pub fn seven_segment_search() -> (u64, u64) {
    let input = get_input(8).expect("Could not get input");

    let easy_outputs: usize = input.iter()
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

    let displays = input.iter()
      .map(|e| {
        let mut split = e.split(" | ");
        let mut observed: Vec<&str> = split.next()
          .expect("Bad Input")
          .split(' ')
          .filter(|e| e.len() == 2 || e.len() == 4)
          .collect();
        observed.sort_by(|a, b| a.len().cmp(&b.len()));

        let one = observed[0];
        let four = observed[1];

        let outputs = split.next()
          .expect("Bad Input")
          .split(' ');

        let mut value = 0;
        for out in outputs.clone() {
          value *= 10;
          value += letters_to_digit(out, one, four);
        }

        value
      })
      .sum();

    (easy_outputs as u64, displays)
}
