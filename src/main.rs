#![warn(clippy::pedantic)]

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
mod dumbo_octopus;
mod passage_pathing;
mod transparent_origami;
mod extended_polymerization;
mod chiton;
mod packet_decoder;
mod trick_shot;
mod snailfish;
mod trench_map;

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
use dumbo_octopus::dumbo_octopus;
use passage_pathing::passage_pathing;
use transparent_origami::transparent_origami;
use extended_polymerization::extended_polymerization;
use chiton::chiton;
use packet_decoder::packet_decoder;
use trick_shot::trick_shot;
use snailfish::snailfish;
use trench_map::trench_map;

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
    print_results(11, dumbo_octopus());
    print_results(12, passage_pathing());
    print_results(13, transparent_origami());
    print_results(14, extended_polymerization());
    print_results(15, chiton());
    print_results(16, packet_decoder());
    print_results(17, trick_shot());
    print_results(18, snailfish());
    print_results(19, (0, 0));
    print_results(20, trench_map());
}
