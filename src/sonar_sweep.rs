use crate::get_input::get_input;

pub fn sonar_sweep() -> i32 {
    let input = get_input(1).expect("Could not get input");
	let parsed: Vec<i32> = input.iter().map(|e| e.parse::<i32>().unwrap()).collect();

    let sequence = parsed.iter().zip(parsed.iter().skip(1));

    let count = sequence.fold(0, |acc, e| {
		let (a, b) = e;
		acc + (a < b) as i32
    });

    count
}
