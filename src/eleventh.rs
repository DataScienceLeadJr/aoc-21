use crate::util::TaskPart;

pub const DAY: &str = "4th";

fn process_input(input: String) {
    let mut lines: Vec<_> = input.lines().collect();
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_A!");

    let _ = process_input(load_input(DAY, TaskPart::A));

    println!("the answer is: sum:{}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_B!");

    let _ = process_input(load_input(DAY, TaskPart::B));

    println!("the answer is: sum:{}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}