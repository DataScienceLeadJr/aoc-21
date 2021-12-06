use crate::util::TaskPart;

pub const DAY: &str = "6th";


fn process_input(input: String) -> Vec<i32> {
    let mut lines: Vec<_> = input.lines().collect();
    let draw_numbers = lines.first().unwrap().split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    draw_numbers
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixth_A!");

    let _ = process_input(load_input(DAY, TaskPart::A));

    println!("the answer is: {}", 1);

    store_output((" ").to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixth_B!");

    let _ = process_input(load_input(DAY, TaskPart::A));

    println!("the answer is: sum:{}", 1);

    store_output((" ").to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}