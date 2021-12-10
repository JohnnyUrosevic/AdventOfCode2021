use crate::get_input::get_input;

pub fn binary_to_vector(binary: &String) -> Vec<u32> {
    binary.chars()
        .map(|e| e.to_digit(10).expect("Invalid Input"))
        .collect()
}

pub fn binary_diagnostic() -> (u64, u64) {
    let input = get_input(3).expect("Could not get input");

    let numbers: Vec<Vec<u32>> = input.iter()
        .map(binary_to_vector)
        .collect();

    let half: u32 = (numbers.len() / 2).try_into().unwrap();

    let gamma = numbers.iter()
        .fold(vec![0; 12], |mut acc, row| {
            for (i, bit) in row.iter().enumerate() {
                acc[i] += bit;
            }
            acc
        })
        .iter()
        .map(|e| if e > &half {1} else {0})
        .fold(0, |acc, e| (acc << 1) + e);
        
    let mask = 0xFFF;
    ((gamma as u64 * (!gamma & mask) as u64), 0)
}
