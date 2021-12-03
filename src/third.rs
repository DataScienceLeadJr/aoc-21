use std::{collections::HashMap, panic};

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

    println!("g: {}, {:b}, e: {}, {:b}", gamma_rate, gamma_rate, epsilon_rate, epsilon_rate);

    store_output((gamma_rate * epsilon_rate).to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

trait Nth {
    fn nth(&self, nth: usize) -> char;
}

impl Nth for String {
    fn nth(&self, nth: usize) -> char {
        self.chars().nth(nth).unwrap()
    }
}

trait BinarySortMembers {
    fn rev_bin_sort(&mut self);
}

impl BinarySortMembers for Vec<String> {
    fn rev_bin_sort(&mut self) {
        (0..self.len()).rev().for_each(
          |i| {
            self.sort_by(|a, b| 
                a.chars().nth(i).partial_cmp(&b.chars().nth(i)).unwrap())
          }
        );
    }
}

fn check_all_equal(binaries: &[String], bit_of_interest: usize) -> bool {
    let bits: Vec<_> = binaries.iter().map(|s| {
        s.nth(bit_of_interest)
    }).collect();
    
    let first = bits[0];
    bits.iter().all(|&item| item == first)
}

fn get_split_point(binaries: &[String], rec: usize) -> usize {
    let mut split_point = binaries.len() as f32 / 2.0;

    let mut step = split_point * 0.5;

    let mut bit_of_interest = rec;

    let mut left = binaries[split_point as usize - 1].nth(bit_of_interest);
    let mut right = binaries[split_point as usize].nth(bit_of_interest);

    while left == right{

        // search both ways
       
        step *= 0.5;
        bit_of_interest += 1;

        left = binaries[split_point as usize - 1].nth(bit_of_interest);
        right = binaries[split_point as usize].nth(bit_of_interest);
    }

    return split_point as usize;
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Third_B!");

    let mut diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::B));

    // sort for every bit in one go to make it easier.
    diagnostic_report.rev_bin_sort();

    diagnostic_report.iter().for_each(|n| {println!("{:?}", n)});

    let max_rec = diagnostic_report[0].len();

    let (o2_bin_num, co2_bin_num) = ("110", "001");
    
    let o2_gen_rate = isize::from_str_radix(
        o2_bin_num,
        2)
        .unwrap() as u32;

    let co2_scrub_rate = isize::from_str_radix(
        co2_bin_num,
        2)
        .unwrap() as u32;

    println!("o2: {}, {:b}, co2: {}, {:b}", o2_gen_rate, o2_gen_rate, co2_scrub_rate, co2_scrub_rate);

    assert_eq!(o2_gen_rate * co2_scrub_rate, 230);

    store_output((o2_gen_rate * co2_scrub_rate).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}