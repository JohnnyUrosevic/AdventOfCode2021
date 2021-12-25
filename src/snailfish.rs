use crate::get_input::get_input;
use num::Integer;
use std::fmt;
use std::cmp::max;

#[derive(Clone)]
struct Pair {
  left: Box<Node>,
  right: Box<Node>,
}

#[derive(Clone)]
enum Node {
  Value(u64),
  Pair(Pair)
}

impl fmt::Display for Pair {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = match &*self.left {
          Node::Value(val) => format!("{}", val),
          Node::Pair(pair) => format!("{}", pair),
        };
        let right = match &*self.right {
          Node::Value(val) => format!("{}", val),
          Node::Pair(pair) => format!("{}", pair),
        };
        write!(f, "[{},{}]", left, right)
    }
}

impl Pair {
  fn new(input: &String) -> Self {
    let mut chars = input.chars().peekable();
    chars.next();
    chars.next_back();

    let first = *chars.peek()
      .expect("String must not be empty");

    let left_node = if let Some(parsed) = first.to_digit(10) {
      chars.next();
      Node::Value(parsed as u64)
    }
    else {
      let mut stack = 0;
      let mut left_number: String = chars.by_ref()
        .take_while(|c| {
          if *c == '[' {
            stack += 1;
          }
          else if *c == ']' {
            stack -= 1;
          }

          stack != 0
        })
        .collect();
      left_number.push(']');
      Node::Pair(Pair::new(&left_number))
    };

    //consume comma
    chars.next();

    let right_number: String = chars.collect();
    let right_node = if let Ok(parsed) = right_number.parse() {
      Node::Value(parsed)
    }
    else {
      Node::Pair(Pair::new(&right_number))
    };

    let left = Box::new(left_node);
    let right= Box::new(right_node);

    Pair{left, right}
  }

  fn reduce(&mut self) -> bool {
    let has_exploded = self.do_explode();
    if has_exploded {
      return true
    }

    let has_split = self.split();

    has_split
  }

  fn increase_right(&mut self, amount: u64) {
    match &mut *self.right {
      Node::Value(val) => *self.right = Node::Value(*val + amount),
      Node::Pair(pair) => pair.increase_right(amount),
    }
  }

 fn increase_left(&mut self, amount: u64) {
    match &mut *self.left {
      Node::Value(val) => *self.left = Node::Value(*val + amount),
      Node::Pair(pair) => pair.increase_left(amount),
    }
  }

  fn do_explode(&mut self) -> bool {
    match &mut *self.left {
      Node::Pair(pair) => {
        let (has_exploded, _, right_val) = pair.explode(1);
        if has_exploded {
          match &mut *self.right {
            Node::Pair(pair) => {
              pair.increase_left(right_val);
            },
            Node::Value(val) => {
              *val += right_val;
            }
          };

          return true;
        }
      },
      Node::Value(_) => {},
    };

    match &mut *self.right {
      Node::Pair(pair) => {
        let (has_exploded, left_val, _) = pair.explode(1);
        if has_exploded {
          match &mut *self.left{
            Node::Pair(pair) => {
              pair.increase_right(left_val);
            },
            Node::Value(val) => {
              *val += left_val;
            }
          };

          return true;
        }

        false
      },
      Node::Value(_) => false,
    }
  }

  fn explode(&mut self, depth: u8) -> (bool, u64, u64) {
    if depth >= 4 {
      let left_val = match &mut *self.left {
        Node::Value(val) => val,
        Node::Pair(_) => unreachable!(),
      };

      let right_val = match &mut *self.right {
        Node::Value(val) => val,
        Node::Pair(_) => unreachable!(),
      };

      (true, *left_val, *right_val)
    }
    else {
      match &mut *self.left {
        Node::Value(_) => {},
        Node::Pair(pair) => {
          let (has_exploded, left_val, right_val) = pair.explode(depth + 1);

          if depth == 3 {
            *self.left = Node::Value(0);
          }

          if has_exploded {
            match &mut *self.right {
              Node::Value(val) => {
                let new_right = Node::Value(*val + right_val);
                *self.right = new_right;
                return (true, left_val, 0);
              },
              Node::Pair(right_pair) => {
                right_pair.increase_left(right_val);
                return (true, left_val, 0);
              }
            }
          }
        },
      }

      match &mut *self.right {
        Node::Value(_) => (false, 0, 0),
        Node::Pair(pair) => {
          let (has_exploded, left_val, right_val) = pair.explode(depth + 1);

          if depth == 3 {
            *self.right = Node::Value(0);
          }

          if has_exploded {
            match &mut *self.left {
              Node::Value(val) => {
                let new_left = Node::Value(*val + left_val);
                *self.left = new_left;
                return (true, 0, right_val);
              },
              Node::Pair(left_pair) => {
                left_pair.increase_right(left_val);
                return (true, 0, right_val);
              }
            }
          }

          (false, 0, 0)
        },
      }
    }
  }

  fn split(&mut self) -> bool {
    let has_split = match &mut *self.left {
      Node::Value(value) => {
        if *value >= 10 {
          *self.left = do_split(*value);
          true
        }
        else {
          false
        }
      },
      Node::Pair(pair) => {
        pair.split()
      },
    };

    if has_split {
      return true;
    }

    match &mut *self.right {
      Node::Value(value) => {
        if *value >= 10 {
          *self.right = do_split(*value);
          true
        }
        else {
          false
        }
      },
      Node::Pair(pair) => {
        pair.split()
      },
    }
  }

  fn magnitude(&self) -> u64 {
    let left_val = match &*self.left {
      Node::Value(val) => *val,
      Node::Pair(pair) => pair.magnitude(),
    };

    let right_val = match &*self.right {
      Node::Value(val) => *val,
      Node::Pair(pair) => pair.magnitude(),
    };

    3 * left_val + 2 * right_val
  }
}

fn do_split(value: u64) -> Node {
  let left_value = Node::Value(value / 2);
  let right_value = Node::Value(value.div_ceil(&2));

  let left = Box::new(left_value);
  let right= Box::new(right_value);

  Node::Pair(Pair{left, right})
}

fn add(a: Pair, b: Pair) -> Pair {
  let left = Box::new(Node::Pair(a));
  let right= Box::new(Node::Pair(b));

  let mut new_pair = Pair{left, right};

  while new_pair.reduce() {};

  new_pair
}

pub fn snailfish() -> (u64, u64) {
  let input = get_input(18).expect("Could not get input");
	
  let mut numbers = input.iter()
    .map(|e| Pair::new(e));

  let first_pair = numbers.next()
    .expect("Must be at least one pair");

  let mut pair = first_pair;

  for e in numbers {
    let new_pair = add(pair, e);
    pair = new_pair;
  };

  let mut max_magnitude = 0;

  let numbers: Vec<Pair> = input.iter()
    .map(|e| Pair::new(e))
    .collect();

  for i in 0..numbers.len()-1 {
    for j in 1..numbers.len(){
      let a = numbers[i].clone();
      let b = numbers[j].clone();

      max_magnitude = max(max_magnitude,add(a.clone(), b.clone()).magnitude());
      max_magnitude = max(max_magnitude,add(b.clone(), a.clone()).magnitude());
    }
  }

  (pair.magnitude(), max_magnitude)
}
