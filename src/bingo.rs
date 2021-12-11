use crate::get_input::get_input;

use std::collections::HashMap;

pub fn bingo() -> (i32, i32) {
    let input = get_input(4).expect("Could not get input");
    let mut input_iter = input.iter();
    
    let called_numbers = input_iter.next()
        .expect("Bad input")
        .split(',')
        .map(|e| e.parse::<i32>().expect("NaN"));

    let mut boards: Vec<HashMap<i32, (usize, usize)>> = vec![];
    let mut row: usize = 0;
    let mut boards_len = 0;

    for line in input_iter {
        if line.is_empty() {
            row = 0;
            boards.push(HashMap::new());
            boards_len += 1;
        }
        else {
            for (col, num) in line.split(' ').filter(|e| !e.is_empty()).enumerate() {
                boards[boards_len-1]
                    .insert(num.parse().expect("NaN"), (row, col));
            }
            row += 1;
        }
    }

    let mut row_counts = vec![vec![0; 5]; boards_len];
    let mut col_counts= vec![vec![0; 5]; boards_len];

    let mut score = None;
    let mut last_score = 0;
    let mut has_won = vec![false; boards_len];

    for called in called_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            let (r, c) = match board.remove(&called) {
                Some((r, c)) => (r, c),
                None => continue,
            };

            row_counts[i][r] += 1;
            col_counts[i][c] += 1;

            if (row_counts[i][r] == 5 || col_counts[i][c] == 5) && !has_won[i] {
                let unmarked: i32 = board.keys().sum();
                last_score = unmarked * called; 

                if score == None {
                    score = Some(last_score);
                }

                has_won[i] = true;
            }
        }
    }

    (score.unwrap(), last_score)
}
