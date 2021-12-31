use crate::get_input::get_input;

use std::cmp::{min, max};
use itertools::iproduct;

#[derive(Copy, Clone, Debug)]
struct Player {
  position: u32,
  score: u32,
}

pub fn dirac_dice() -> (u64, u64) {
  let input = get_input(21).expect("Could not get input");

  let mut it = input.iter();

  let player1_start = it.next()
    .expect("Bad Input")
    .chars()
    .rev()
    .next()
    .expect("Bad Input")
    .to_digit(10)
    .expect("NaN");

  let player2_start = it.next()
    .expect("Bad Input")
    .chars()
    .rev()
    .next()
    .expect("Bad Input")
    .to_digit(10)
    .expect("NaN");

  let p1 = Player{position: player1_start, score: 0};
  let p2 = Player{position: player2_start, score: 0};

  let mut players = [p1, p2];

  let mut deterministic_dice = (1..=100).cycle();

  let mut turn = 0;
  let mut rolls = 0;
  while players[0].score < 1000 && players[1].score < 1000 {
    let roll: u32 = deterministic_dice.by_ref().take(3).sum();
    players[turn].position = (players[turn].position + roll - 1) % 10 + 1;

    players[turn].score += players[turn].position;
    turn = 1 - turn;
    rolls += 3;
  }

  let total = rolls * min(players[0].score, players[1].score);

  // possible_players[pos1][pos2][score1][score2]
  let mut possible_players = vec![vec![vec![vec![0; 21]; 21]; 11]; 11];
  possible_players[player1_start as usize][player2_start as usize][0][0] = 1;

  let possible_rolls = iproduct!(1..=3, 1..=3, 1..=3);

  let mut wins = [0, 0];
  turn = 0;
  let mut has_players = true;
  while has_players {
    let mut new_possibilities = vec![vec![vec![vec![0; 21]; 21]; 11]; 11];
    has_players = false;

    for rolls in possible_rolls.clone() {
      let roll = rolls.0 + rolls.1 + rolls.2;
      for pos1 in 1..=10 {
        let new_pos = (pos1 + roll - 1) % 10 + 1;
        for pos2 in 1..=10 {
          for score1 in 0..21 {
            for score2 in 0..21 {
              let curr = if turn == 0 {
                possible_players[pos1][pos2][score1][score2]
              }
              else {
                possible_players[pos2][pos1][score2][score1]
              };

              if curr == 0 {
                continue
              }

              if score1 + new_pos >= 21 {
                wins[turn] += curr;
                continue;
              }
              has_players = true;

              if turn == 0 {
                new_possibilities[new_pos][pos2][score1 + new_pos][score2] += curr;
              }
              else {
                new_possibilities[pos2][new_pos][score2][score1 + new_pos] += curr;
              }
            }
          }
        }
      }
    }
    possible_players = new_possibilities;
    turn = 1 - turn;
  }

  let winning_possibilities = max(wins[0], wins[1]);
  (total as u64, winning_possibilities)
}
