use crate::get_input::get_input;

use std::{collections::HashMap};

struct Polymer<'a> {
  letter_counts: &'a mut HashMap<char, u64>,
  pair_counts: &'a mut HashMap<String, u64>,
  insertion_rules: &'a mut HashMap<String, char>,
}

impl<'a> Polymer<'a> {
  fn step(&mut self) {
    for (pair, count) in self.pair_counts.clone().iter() {
      let inserted = *self.insertion_rules.get(pair).expect("Insufficient rules");

      *self.letter_counts.entry(inserted).or_insert(0) += count;

      let mut it = pair.chars();
      let first = it.next().expect("Bad Pair");
      let second = it.next().expect("Bad Pair");

      *self.pair_counts.entry(format!("{}{}", first, inserted))
        .or_insert(0) += count;
      *self.pair_counts.entry(format!("{}{}", inserted, second))
        .or_insert(0) += count;

      *self.pair_counts.entry(pair.clone())
        .or_insert(0) -= count;
    }
  }

  fn get_extreme_counts(&self) -> (u64, u64) {
    let most_frequent = self.letter_counts.values()
      .max()
      .expect("Non empty polymer");

    let least_frequent = self.letter_counts.values()
      .min()
      .expect("Non empty polymer");

    (*most_frequent, *least_frequent)
  }
}

pub fn extended_polymerization() -> (u64, u64) {
  let input = get_input(14).expect("Could not get input");
  let mut it = input.iter();

  let initial = it.next().expect("Bad input");

  let mut letter_counts = HashMap::new();
  
  initial.chars()
    .for_each(|c| *letter_counts
      .entry(c)
      .or_insert(0) += 1);

  let mut insertion_rules: HashMap<String, char> = it.skip(1)
    .map(|e| {
      let mut split = e.split(" -> ");

      let pair = split.next().expect("Bad Input").to_string();
      let insert= split.next().expect("Bad Input").chars().next().expect("Empty Rule");

      (pair, insert)
    })
    .collect();

  let mut pair_counts = HashMap::new();
  
  initial.clone()
    .chars()
    .zip(initial.chars().skip(1))
    .for_each(|(a, b)| {
      *pair_counts.entry(format!("{}{}", a, b))
        .or_insert(0) += 1;
    });

  let mut poly = Polymer{
      letter_counts: &mut letter_counts,
      pair_counts: &mut pair_counts,
      insertion_rules: &mut insertion_rules
    };

  for _ in 0..10 {
    poly.step();
  }

  let (most_frequent, least_frequent) = poly.get_extreme_counts();
  
  for _ in 10..40 {
    poly.step();
  }

  let (most_frequent_2, least_frequent_2) = poly.get_extreme_counts();

  (most_frequent - least_frequent, most_frequent_2 - least_frequent_2)
}
