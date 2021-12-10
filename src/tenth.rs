use crate::util::{
    TaskPart,
};

pub const DAY: &str = "10th";

fn process_input(input: String) -> Vec<i32> {
    let line = input.lines().next().unwrap();

    line.split(",").map(|pos| pos.parse::<i32>().unwrap()).collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_A!");

    let _ = process_input(load_input(DAY, TaskPart::A));



    println!("minimum fuel usage: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_B!");

    let _ = process_input(load_input(DAY, TaskPart::B));

    println!("minimum fuel usage: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}