use itertools::Itertools;

use crate::util::{
    TaskPart,
    average,
    median,
    mode,
};

pub const DAY: &str = "7th";


fn process_input(input: String) -> Vec<i32> {
    let line = input.lines().next().unwrap();

    line.split(",").map(|pos| pos.parse::<i32>().unwrap()).collect()
}

struct Statistics {
    average: i32,
    median: i32,
    mode: i32,
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Seventh_A!");

    let crab_pos = process_input(load_input(DAY, TaskPart::A));

    let sorted_crab_pos: Vec<i32> = crab_pos.into_iter().sorted().collect();

    let stats = Statistics {
        average: average(&sorted_crab_pos[..]).round() as i32,
        median: median(&sorted_crab_pos[..]),
        mode: mode(&sorted_crab_pos[..]).unwrap(),
    };
    println!("statistics! mean:{}, median:{}, mode:{}", stats.average, stats.median, stats.mode);

    let avg_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        (pos - stats.average).abs()
    }).sum();

    let median_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        (pos - stats.median).abs()
    }).sum();

    let mode_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        (pos - stats.mode).abs()
    }).sum();

    println!("avg, med, mod fuel usage: {}, {}, {}", avg_fuel_used, median_fuel_used, mode_fuel_used);

    let fuel_used = (avg_fuel_used.min(median_fuel_used)).min(mode_fuel_used);

    println!("minimum fuel usage: {}", fuel_used);

    store_output(fuel_used.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Seventh_B!");

    let crab_pos = process_input(load_input(DAY, TaskPart::B));

    let sorted_crab_pos: Vec<i32> = crab_pos.into_iter().sorted().collect();

    let stats = Statistics {
        average: average(&sorted_crab_pos[..]) as i32,
        median: median(&sorted_crab_pos[..]),
        mode: mode(&sorted_crab_pos[..]).unwrap(),
    };
    println!("statistics! mean:{}, median:{}, mode:{}", stats.average, stats.median, stats.mode);

    let increasing_fuel_usage_amount = |dist: i32| -> i32{
        (0..=dist).sum()
    };

    let avg_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        increasing_fuel_usage_amount((pos - stats.average).abs())
    }).sum();

    let median_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        increasing_fuel_usage_amount((pos - stats.median).abs())
    }).sum();

    let mode_fuel_used: i32 = sorted_crab_pos.iter().map(|pos|{
        increasing_fuel_usage_amount((pos - stats.mode).abs())
    }).sum();

    println!("avg, med, mod fuel usage: {}, {}, {}", avg_fuel_used, median_fuel_used, mode_fuel_used);

    let fuel_used = (avg_fuel_used.min(median_fuel_used)).min(mode_fuel_used);

    println!("minimum fuel usage: {}", fuel_used);

    store_output(fuel_used.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}