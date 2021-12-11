use crate::util::TaskPart;

pub const DAY: &str = "11th";

fn process_input(input: String) -> Vec<Vec<i32>>{
    input.lines()
        .map(
            |line| 
                line.chars()
                    .map(
                        |c| 
                            c.to_string().parse().unwrap())
                    .collect())
        .collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eleventh_A!");

    let _ = process_input(load_input(DAY, TaskPart::A));

    println!("the answer is: sum:{}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eleventh_B!");

    let _ = process_input(load_input(DAY, TaskPart::B));

    println!("the answer is: sum:{}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}