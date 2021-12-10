use crate::get_input::get_input;

pub fn binary_to_vector(binary: &String) -> Vec<i32> {
    binary.chars()
        .map(|e| e.to_digit(10).expect("Invalid Input") as i32)
        .collect()
}

pub fn most_common_bits(numbers: &Vec<Vec<i32>>) -> Vec<i32> {
    numbers.iter()
    .fold(vec![0; 12], |mut acc, row| {
        for (i, bit) in row.iter().enumerate() {
            // +1 if 1 is found -1 if 0 is found
            acc[i] += 2 * bit - 1;
        }
        acc
    })
    .iter()
    .map(|e| if e >= &0 {1} else {0})
    .collect()
}

pub fn criteria( numbers: &Vec<Vec<i32>>, oxygen: bool) -> u64 {
    let mut candidates = numbers.clone();
    let mut position = 0;
    while candidates.len() > 1 {
        let most_common = most_common_bits(&candidates);
        candidates.retain(|e| oxygen ^ (e[position] == most_common[position]));

        position += 1;
    }

    candidates.iter()
        .next()
        .unwrap()
        .iter()
        .fold(0, |acc, e| (acc << 1) + *e as u64)
}

pub fn binary_diagnostic() -> (u64, u64) {
    let input = get_input(3).expect("Could not get input");

    let numbers: Vec<Vec<i32>> = input.iter()
        .map(binary_to_vector)
        .collect();

    let gamma = most_common_bits(&numbers)
        .iter()
        .fold(0, |acc, e| (acc << 1) + *e as u32);

    let oxygen = criteria(&numbers, true);
    let co2 = criteria(&numbers, false);

    let mask = 0xFFF;
    ((gamma as u64 * (!gamma & mask) as u64), (oxygen * co2).into())
}
