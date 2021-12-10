use itertools::Itertools;

use crate::util::{
    TaskPart,
};

pub const DAY: &str = "10th";


trait Syntax {
    fn as_illegal_syntax_value(self) -> Result<i32, String>;
}

impl Syntax for char {
    fn as_illegal_syntax_value(self) -> Result<i32, String> {
        match self {
            ')' => Ok(3),
            ']' => Ok(57),
            '}' => Ok(1197),
            '>' => Ok(25137),
            _ => Err("character without any syntax value given!".to_string())
        }
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_A!");

    let lines = load_input(DAY, TaskPart::A).lines();



    println!("minimum fuel usage: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_B!");

    let lines = load_input(DAY, TaskPart::B).lines();

    println!("minimum fuel usage: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}