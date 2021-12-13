mod get_input;
mod sonar_sweep;
mod dive;
mod binary_diagnostic;
mod bingo;
mod hydrothermal_venture;

use sonar_sweep::sonar_sweep;
use dive::dive;
use binary_diagnostic::binary_diagnostic;
use bingo::bingo;
use hydrothermal_venture::hydrothermal_venture;

use std::fmt::Display;

fn print_results<T: Display>(day: i32, result: (T, T))
{
    let (part1, part2) = result;
    println!("Day {}: {} {}", day, part1, part2);
}

fn main() {
    print_results(1, sonar_sweep());
    print_results(2, dive());
    print_results(3, binary_diagnostic());
    print_results(4, bingo());
    print_results(4, hydrothermal_venture());
}
