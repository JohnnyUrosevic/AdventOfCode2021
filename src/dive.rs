use crate::get_input::get_input;

pub enum Direction {
    Forward,
    Up,
    Down
}

pub fn string_to_direction(direction: &str) -> Direction {
    match direction {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("Invalid Input"),
    }
}

pub fn dive() -> (i32, i32) {
    let input = get_input(2).expect("Could not get input");
	let parsed: Vec<(Direction, i32)> = input.iter()
        .map(|e| e.split(' '))
        .map(|mut e| {
            let a = e.next().unwrap();
            let b = e.next().unwrap();

            (string_to_direction(a), b.parse::<i32>().unwrap())
        })
        .collect();
    
    let (depth, distance) = parsed.iter()
        .fold((0, 0), |(depth, distance), (dir, value)| {
            match dir {
                Direction::Forward => (depth, distance + value),
                Direction::Up => (depth - value, distance),
                Direction::Down => (depth + value, distance),
            }
        });

    let (depth2, distance2, _) = parsed.iter()
        .fold((0, 0, 0), |(depth, distance, aim), (dir, value)| {
            match dir {
                Direction::Forward => (depth + value * aim, distance + value, aim),
                Direction::Up => (depth, distance, aim - value),
                Direction::Down => (depth, distance, aim + value),
            }
        });

    (depth * distance, depth2 * distance2)
}
