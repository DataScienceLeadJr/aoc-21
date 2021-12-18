use crate::util::TaskPart;

pub const DAY: &str = "19th";

fn process_input(input: String) -> Vec<i32>{
    input.lines().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Nineteenth_A!");

    let _graph = process_input(load_input(DAY, TaskPart::A));

    println!("num increases: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Nineteenth_A!");

    let _graph = process_input(load_input(DAY, TaskPart::B));

    println!("num increases: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}