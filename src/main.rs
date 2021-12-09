mod get_input;
use get_input::get_input;

fn main() {
    let input = get_input(1).expect("Could not get input");

    for line in input.iter() {
        println!(line);
    }
}
