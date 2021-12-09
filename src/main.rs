mod get_input;
mod sonar_sweep;

use sonar_sweep::sonar_sweep;

fn print_results(day: i32, result: (i32, i32)) {
    let (part1, part2) = result;
    println!("Day {}: {} {}", day, part1, part2);
}

fn main() {
    print_results(1, sonar_sweep());
}
