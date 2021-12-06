use crate::util::TaskPart;

pub const DAY: &str = "6th";


fn process_input(input: String) -> Vec<i32> {
    let starting_lanternfishes = input.lines().next().unwrap();

    starting_lanternfishes.split(",").map(|num| num.parse().unwrap()).collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixth_A!");

    let mut lanternfish = process_input(load_input(DAY, TaskPart::A));

    let days = 80;
    let mut num_fishes_to_add = 0;
    for _day in 0..days {
        lanternfish.iter_mut().for_each(|fish| {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                num_fishes_to_add += 1;
            }
        });
        (0..num_fishes_to_add).for_each(|_| {lanternfish.push(8);});
        num_fishes_to_add = 0;
    }

    println!("the answer after {} days is: {}", days, lanternfish.len());

    store_output(lanternfish.len().to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixth_B!");

    let starting_lanternfish = process_input(load_input(DAY, TaskPart::B));

    let mut spawn_clocks: Vec<(u64, u64)> = (0..7).map(|_| (0, 0)).collect();
    starting_lanternfish.iter().for_each(|spawn_day| spawn_clocks[*spawn_day as usize].0 += 1);

    let mut num_fishes: u64 = starting_lanternfish.len() as u64;

    let days = 256;
    for day in 0..days {
        let clock = day % 7;
        let amount_of_new_spawns = spawn_clocks[clock].0;
        num_fishes += amount_of_new_spawns;
        spawn_clocks[clock].0 += spawn_clocks[clock].1;
        spawn_clocks[(day + 2) % 7].1 = amount_of_new_spawns;
    }

    println!("the answer after {} days is: {}", days, num_fishes);

    store_output(num_fishes.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}