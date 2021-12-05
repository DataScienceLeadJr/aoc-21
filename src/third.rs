use std::{collections::{HashMap}, panic, ops::{Range, RangeTo, RangeFrom}};

use itertools::Itertools;

use crate::util::TaskPart;

pub const DAY: &str = "3rd";


fn process_input(input: String) -> Vec<String> {
    input
    .lines()
    .map(|string| string.to_string())
    .collect()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Third_A!");

    let diagnostic_report = process_input(load_input(DAY, TaskPart::A));

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
        match self.chars().nth(nth) {
            Some(c) => c,
            None => panic!("WTF!? self: {:?}\n\tnth:{}", self, nth)
        }
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

trait DifferenceSplitPoint {
    fn find_difference_point(&self, nth: usize) -> usize;
}
impl DifferenceSplitPoint for Vec<String> {
    fn find_difference_point(&self, nth: usize) -> usize {
        let mut split_point = 1;
        while self[split_point - 1].nth(nth) == self[split_point].nth(nth) {
            split_point += 1;
        }

        split_point
    }
}

trait SplitRange {
    fn into_split_ranges(self, end: usize) -> (Range<usize>, Range<usize>);
}
impl SplitRange for usize {
    fn into_split_ranges(self, end: usize) -> (Range<usize>, Range<usize>) {
        ((0..self), (self..end))
    }
}

trait MostLeast {
    fn as_most_least(self) -> Self;
    fn equal(&self) -> bool;
    fn switch_on_equal(self) -> Self;
}

impl MostLeast for (Range<usize>, Range<usize>) {
    fn equal(&self) -> bool {
        self.0.len() == self.1.len()
    }
    fn as_most_least(self) -> Self {
        if self.0.len() > self.1.len() {
            self
        } else {
            (self.1, self.0)
        }
    }

    fn switch_on_equal(self) -> Self {
        if self.equal() {
            (self.1, self.0)
        } else {
            self
        }
    }
}

fn check_all_equal(binaries: &[String], bit_of_interest: usize) -> bool {
    let bits: Vec<_> = binaries.iter().map(|s| {
        s.nth(bit_of_interest)
    }).collect();
    
    let first = bits[0];
    bits.iter().all(|&item| item == first)
}

fn search_most(values: Vec<String>, bit_level: usize) -> String {    
    if values.len() == 1 {
        return values.first().unwrap().clone();
    } else if bit_level >= values[0].len() {
        panic!("most oh no.... \n\tnum_values: {}", values.len());
    }else if check_all_equal(&values, bit_level) {
        return search_most(values, bit_level + 1)
    }

    let ranges  = values.find_difference_point(bit_level)
    .into_split_ranges(values.len());

    let most_range = if ranges.equal() {ranges.1} else {ranges.as_most_least().0};

    search_most(values[most_range].to_vec(), bit_level + 1)
}

fn search_least(values: Vec<String>, bit_level: usize) -> String {
    if bit_level >= values[0].len() {
        panic!("least oh no.... \n\tnum_values: {}", values.len());
    }
    if values.len() == 1 {
        return values.first().unwrap().clone();
    } else if check_all_equal(&values, bit_level) {
        return search_least(values, bit_level + 1)
    }

    let ranges  = values.find_difference_point(bit_level)
    .into_split_ranges(values.len());

    let most_range = if ranges.equal() {ranges.0} else {ranges.as_most_least().1};

    search_least(values[most_range].to_vec(), bit_level + 1)
}

fn get_most_and_least(values: Vec<String>, bit_level: usize) -> (String, String) {
    let most_least_ranges = 
    values
    .find_difference_point(bit_level)
    .into_split_ranges(values.len())
    .as_most_least()
    .switch_on_equal();

    let most_value = values[most_least_ranges.0].to_vec();
    let most_handle = std::thread::spawn(move || search_most(most_value, bit_level + 1));
    let least_value = values[most_least_ranges.1].to_vec();
    let least_handle = std::thread::spawn(move || search_least(least_value, bit_level + 1));

    (most_handle.join().unwrap(), least_handle.join().unwrap())
}


pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Third_B!");

    let mut diagnostic_report = process_input(load_input(DAY, TaskPart::B));

    // sort for every bit in one go to make it easier.
    diagnostic_report.rev_bin_sort();

    let (o2_bin_num, co2_bin_num) = get_most_and_least(diagnostic_report.clone(), 0);
    
    let o2_gen_rate = isize::from_str_radix(
        &o2_bin_num,
        2)
        .unwrap() as u32;

    let co2_scrub_rate = isize::from_str_radix(
        &co2_bin_num,
        2)
        .unwrap() as u32;

    println!("o2: {}, {:b}, co2: {}, {:b}", o2_gen_rate, o2_gen_rate, co2_scrub_rate, co2_scrub_rate);

    store_output((o2_gen_rate * co2_scrub_rate).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}