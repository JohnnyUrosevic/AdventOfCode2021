mod get_input;
mod sonar_sweep;
mod dive;
mod binary_diagnostic;
mod bingo;
mod hydrothermal_venture;
mod lanternfish;
mod treachery_of_whales;
mod seven_segment_search;
mod smoke_basin;
mod syntax_scoring;

use sonar_sweep::sonar_sweep;
use dive::dive;
use binary_diagnostic::binary_diagnostic;
use bingo::bingo;
use hydrothermal_venture::hydrothermal_venture;
use lanternfish::lanternfish;
use treachery_of_whales::treachery_of_whales;
use seven_segment_search::seven_segment_search;
use smoke_basin::smoke_basin;
use syntax_scoring::syntax_scoring;

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
    print_results(5, hydrothermal_venture());
    print_results(6, lanternfish());
    print_results(7, treachery_of_whales());
    print_results(8, seven_segment_search());
    print_results(9, smoke_basin());
    print_results(10, syntax_scoring());
}
