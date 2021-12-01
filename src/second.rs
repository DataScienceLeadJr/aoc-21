use crate::util::TaskPart;

pub const DAY: &str = "2nd";

fn process_input_day_1(input: String) -> Vec<i32> {
    // input.split("\n").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    Vec::new()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_A!");

    // let depths = process_input_day_1(load_input(DAY, TaskPart::A));

    // store_output(num_increases.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_B!");

    // let depths = process_input_day_1(load_input(DAY, TaskPart::B));

    // store_output(num_increases.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}