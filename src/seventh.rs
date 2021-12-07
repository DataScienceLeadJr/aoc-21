use crate::util::TaskPart;

pub const DAY: &str = "7th";


fn process_input(input: String) -> Vec<i32> {
    let _ = input.lines().next().unwrap();

    Vec::new()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Seventh_A!");

    let _ = process_input(load_input(DAY, TaskPart::A));

    println!("the answer after {} days is: {}", 1, 2);

    store_output(2.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Seventh_B!");

    let _ = process_input(load_input(DAY, TaskPart::B));

    println!("the answer after {} days is: {}", 1, 2);

    store_output(1.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}