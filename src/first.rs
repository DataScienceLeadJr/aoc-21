use crate::util::TaskPart;

pub const DAY: &str = "1st";

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_A!");

    let input = load_input(DAY, TaskPart::A);

    // TODO: do task stuff

    let task_result = |_i| {
        "lol".to_string()
    };

    store_output(task_result(input), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_B!");

    let input = load_input(DAY, TaskPart::B);

    // TODO: do task stuff

    let task_result = |_i| {
        "lol".to_string()
    };

    store_output(task_result(input), DAY, TaskPart::B).expect("funky task not built right... yet?");
}