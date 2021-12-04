use std::{collections::{HashMap}, panic, ops::{Range, RangeTo, RangeFrom}};

use itertools::Itertools;

use crate::util::TaskPart;

pub const DAY: &str = "4th";


fn process_input_day_1(input: String) -> Vec<String> {
    input
    .lines()
    .map(|string| string.to_string())
    .collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_A!");

    let diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::A));

    // Here we Go!

    store_output((" ").to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_B!");

    let mut diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::B));

    // here we go!

    store_output((" ").to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}