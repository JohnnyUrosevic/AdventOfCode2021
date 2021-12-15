use crate::get_input::get_input;

use std::cmp::min;

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

    let last = *crabs.last().expect("Not Empty");

    let mut min_fuel = i32::MAX;
    for num in crabs[0]..last {
        let fuel_used = crabs.iter()
            .map(|e| {
                let steps = (e-num).abs();
                (1..=steps).sum::<i32>()
            })
            .sum();

        min_fuel = min(min_fuel, fuel_used);
    }

    (median_fuel_used, min_fuel)
}
