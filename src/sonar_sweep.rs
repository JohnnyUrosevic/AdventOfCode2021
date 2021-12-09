use crate::get_input::get_input;

pub fn sonar_sweep() -> (i32, i32) {
    let input = get_input(1).expect("Could not get input");
	let parsed: Vec<i32> = input.iter().map(|e| e.parse::<i32>().unwrap()).collect();

	let compare = |acc, e| {
		let (a, b) = e;
		acc + (a < b) as i32
    };

    let sequence = parsed.iter().zip(parsed.iter().skip(1));

    let count = sequence.fold(0, compare);

	let triplets = parsed.iter().zip(parsed.iter().skip(1).zip(parsed.iter().skip(2)));
    let summed: Vec<i32> = triplets.map(|e| {
		let (a, (b, c)) = e;
		a + b + c
	}).collect();

	let summed_sequence = summed.iter().zip(summed.iter().skip(1));
	
	let count2 = summed_sequence.fold(0, compare);

    (count, count2)
}
