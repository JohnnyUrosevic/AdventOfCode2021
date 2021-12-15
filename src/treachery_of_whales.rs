use crate::get_input::get_input;

use std::cmp::min;

pub fn get_fuel_used(crabs: &Vec<i32>, mean: i32) -> i32 {
    crabs.iter()
        .map(|e| {
            let steps = (e-mean).abs();
            (1..=steps).sum::<i32>()
        })
        .sum()
}

pub fn treachery_of_whales() -> (i32, i32) {
    let input = get_input(7).expect("Could not get input");

    let mut crabs: Vec<i32> = input.iter()
      .next()
      .expect("Bad Input")
      .split(',')
      .map(|e| e.parse().expect("NaN"))
      .collect();

    crabs.sort();

    let median= crabs[crabs.len() / 2];
    
    let median_fuel_used = crabs.iter()
        .map(|e| (e-median).abs())
        .sum();

    let mean = crabs.iter().sum::<i32>() as f32 / crabs.len() as f32;
    let ceil_mean = mean.ceil() as i32;
    let floor_mean = mean.floor() as i32;

    let mean_fuel_used = min(get_fuel_used(&crabs, ceil_mean), get_fuel_used(&crabs, floor_mean));

    (median_fuel_used, mean_fuel_used)
}
