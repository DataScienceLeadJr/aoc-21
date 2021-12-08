use itertools::Itertools;

use crate::util::{
    TaskPart,
};

pub const DAY: &str = "8th";


fn process_input(input: String) -> Vec<(Vec<String>, Vec<String>)> {
    let lines = input.lines();


    lines.into_iter().map(|line| line.split(" | ").into_iter().map(|io| io.split_whitespace().into_iter().map(|digit| digit.to_string()).collect::<Vec<String>>()).collect_tuple().unwrap()).collect()
}


pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eigth_A!");

    let patterns = process_input(load_input(DAY, TaskPart::A));

    let mut digit_count = 0;

    patterns
    .iter().for_each(
        |line| 
            line.1
            .iter().for_each(
                |digit| {
                    match digit.len() {
                        2 | 3 | 4 | 7 => {digit_count += 1;}
                        _ => ()
                    }
                }
            )
    );

    println!("1,4,7 or 8-s in output: {}", digit_count);

    store_output(digit_count.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eigth_B!");

    let patterns = process_input(load_input(DAY, TaskPart::B));

    let mut sum_of_all_outputs = 0;

    println!("sum_of_all_outputs: {}", sum_of_all_outputs);

    store_output(sum_of_all_outputs.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}