use crate::get_input::get_input;

pub fn fish_step(fish: &mut [u64; 9]) {
      let ready_fish = fish[0];

      for i in 1..9 {
        fish[i-1] = fish[i];
      }

      fish[6] += ready_fish;
      fish[8] = ready_fish;
}

pub fn lanternfish() -> (u64, u64) {
    let input = get_input(6).expect("Could not get input");

    let mut fish = [0; 9];

    input.iter()
      .next()
      .expect("Bad Input")
      .split(',')
      .map(|e| e.parse().expect("NaN"))
      .for_each(|e: usize| fish[e] += 1);

    for _ in 0..80 {
      fish_step(&mut fish);
    }

    let mut more_fish = fish.clone();
    for _  in 80..256 {
      fish_step(&mut more_fish);
    }

    (fish.iter().sum(), more_fish.iter().sum())
}
