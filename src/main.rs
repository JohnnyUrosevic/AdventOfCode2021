mod get_input;
mod sonar_sweep;

use sonar_sweep::sonar_sweep;

fn main() {
    println!("Day 1: {}", sonar_sweep());
}
