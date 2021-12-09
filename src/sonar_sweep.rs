use crate::get_input::get_input;

pub fn sonar_sweep() {
    let input = get_input(1).expect("Could not get input");

    for line in input.iter() {
        println!("{}", line);
    }
}
