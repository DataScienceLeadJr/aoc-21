use std::collections::HashMap;

use itertools::Itertools;

use crate::util::TaskPart;

pub const DAY: &str = "3rd";


fn process_input_day_1(input: String) -> Vec<String> {
    input
    .lines()
    .map(|string| string.to_string())
    .collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Third_A!");

    let diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::A));

    let mut bit_counts: HashMap<usize, (usize, usize)> = HashMap::with_capacity(diagnostic_report[0].len());

    for bin_num in diagnostic_report.into_iter() {
        let _: () = bin_num.chars().enumerate().map(|(i, c)| {
            if c == '1' {
                bit_counts.entry(i).or_insert((0, 0)).1 += 1;
            } else if c == '0' {
                bit_counts.entry(i).or_insert((0, 0)).0 += 1;
            } else {
                panic!("what the fuck is This: {}", c);
            }
        }).collect();
    }

    let mut bin_num: String = String::new();

    for (_cardinality, counts) in bit_counts.iter().sorted() {
        bin_num += if counts.0 > counts.1 {"0"} else {"1"};
    }
    
    let gamma_rate = isize::from_str_radix(&bin_num, 2).unwrap() as u32;
    let epsilon_rate = gamma_rate ^ (2_u32.pow(bin_num.len() as u32) - 1) as u32;

    println!("hp: {}, {:b}, d: {}, {:b}", gamma_rate, gamma_rate, epsilon_rate, epsilon_rate);

    store_output((gamma_rate * epsilon_rate).to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Third_B!");

    let diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::B));

    let mut bit_counts: HashMap<usize, (usize, usize)> = HashMap::with_capacity(diagnostic_report[0].len());

    for bin_num in diagnostic_report.into_iter() {
        let _: () = bin_num.chars().enumerate().map(|(i, c)| {
            if c == '1' {
                bit_counts.entry(i).or_insert((0, 0)).1 += 1;
            } else if c == '0' {
                bit_counts.entry(i).or_insert((0, 0)).0 += 1;
            } else {
                panic!("what the fuck is This: {}", c);
            }
        }).collect();
    }

    let mut bin_num: String = String::new();

    for (_cardinality, counts) in bit_counts.iter().sorted() {
        bin_num += if counts.0 > counts.1 {"0"} else {"1"};
    }
    
    let gamma_rate = isize::from_str_radix(&bin_num, 2).unwrap() as u32;
    let epsilon_rate = gamma_rate ^ (2_u32.pow(bin_num.len() as u32) - 1) as u32;

    println!("hp: {}, {:b}, d: {}, {:b}", gamma_rate, gamma_rate, epsilon_rate, epsilon_rate);

    store_output((gamma_rate * epsilon_rate).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}