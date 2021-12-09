use itertools::Itertools;

use crate::util::{
    TaskPart,
};

pub const DAY: &str = "8th";

fn process_input(input: String) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().into_iter()
        .map(
            |line| 
                line.split(" | ").into_iter()
                .map(
                    |io| 
                        io.split_whitespace().into_iter()
                        .map(|digit| digit.to_string())
                        .collect::<Vec<String>>())
                .collect_tuple().unwrap())
        .collect()
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

trait StringSet {
    fn union(&self, other: &str) -> String;
    fn remove_union(&self, other: &str) -> String;
}

impl StringSet for str {
    fn union(&self, other: &str) -> String {
        self.chars().filter(|c| {
            other.contains(*c)
        }).collect()
    }

    fn remove_union(&self, other: &str) -> String {
        let union = self.union(other);

        self.chars().filter(|c| {
            !union.contains(*c)
        }).collect()
    }
}


const ZERO: [bool; 7] = [true, true, true, false, true, true, true];
const ONE: [bool; 7] = [false, false, true, false, false, true, false];
const TWO: [bool; 7] = [true, false, true, true, true, false, true];
const THREE: [bool; 7] = [true, false, true, true, false, true, true];
const FOUR: [bool; 7] = [false, true, true, true, false, true, false];
const FIVE: [bool; 7] = [true, true, false, true, false, true, true];
const SIX: [bool; 7] = [true, true, false, true, true, true, true];
const SEVEN: [bool; 7] = [true, false, true, false, false, true, false];
const EIGHT: [bool; 7] = [true, true, true, true, true, true, true];
const NINE: [bool; 7] = [true, true, true, true, false, true, true];

const LEGAL_NUMBERS: [[bool; 7]; 10] = [
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
];

const NUMBERS_WITH_SIX: [[bool; 7]; 3] = [
    ZERO,
    SIX,
    NINE,
];

const ALL_POSSIBILITIES: &str = "abcdefg";

#[derive(Default, Debug, Clone)]
struct Digit {
    segments: [String; 7],
}

impl Digit {
    fn new() -> Self {
        let all_posibilities = ALL_POSSIBILITIES;
        let mut array: [String; 7] = Default::default();

        array.iter_mut().for_each(|entry| {
            *entry = all_posibilities.to_string();
        });
        Digit {
            segments: array,
        }
    }

    fn update_number_hypothesis(&mut self, reason: &str, num_pattern: [bool; 7]) {
        num_pattern.iter().enumerate().for_each(|(i, seg)| {
            if *seg {
                self.segments[i] = reason.union(&self.segments[i]);
            } else {
                self.segments[i] = self.segments[i][..].remove_union(reason);
            }
        });
    }

    fn restrict_belief_sets_on_given(&mut self, reason: &str) {
        match reason.len() {
            1 => panic!("THIS SHOULDN'T BE POSSIBLE! A one-light digit!?"),
            2 => {
                self.update_number_hypothesis(reason, ONE);
            }
            3 => {
                self.update_number_hypothesis(reason, SEVEN);
            }
            4 => {
                self.update_number_hypothesis(reason, FOUR);
            }
            _ => {}
        };
    }

    fn test_remaining_hypothesis(&mut self, input: &Vec<String>, output: &Vec<String>) {
        while self.segments.iter().any(|seg| seg.len() > 1) {
            input.iter().interleave(output.iter()).for_each(|code| {
                match code.len() {
                    6 => {
                        let missing_seg = ALL_POSSIBILITIES.remove_union(code);
                        // println!("fixing based on missing segment: {}", missing_seg);
                        for i in 0..7 {
                            if self.segments[i][..].contains(&missing_seg) {
                                let mut possible_number = [true; 7];
                                possible_number[i] = false;
                                if NUMBERS_WITH_SIX.iter().any(|valid| *valid == possible_number) {
                                    self.segments[i] = missing_seg.clone();
                                    // println!("WOPP!");
                                    // println!("  made segment {} into {}", i, missing_seg);
                                    for j in 0..7 {
                                        if i != j {
                                            if self.segments[j][..].contains(&missing_seg) {
                                                // println!("    and made segment {}, {}", j, self.segments[j]);
                                                self.segments[j] = self.segments[j][..].remove_union(&missing_seg);
                                                // println!("       into {}", self.segments[j]);
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        // println!("{:?}", self.segments);
                    }
                    _ => {}
                }
            });
        }
    }

    fn to_digit(&self, code: &str) -> i32 {
        let mut possible_number = [false; 7];
        for i in 0..7 {
            if code.contains(&self.segments[i][..]) {
                possible_number[i] = true;
            }
        }

        for i in 0..10 {
            if LEGAL_NUMBERS[i] == possible_number {
                return i as i32;
            }
        }
        
        panic!("After all that deduction ended up with No Legal Number: {:?}", possible_number);
    }

    fn to_number(&self, codes: &Vec<String>) -> i32 {
        self.to_digit(&codes[0]) * 1000 + self.to_digit(&codes[1]) * 100 + self.to_digit(&codes[2]) * 10 + self.to_digit(&codes[3])
    }
}


pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eigth_B!");

    let patterns = process_input(load_input(DAY, TaskPart::B));

    let mut sum_of_all_outputs = 0;

    patterns.iter().for_each(|(input, output)| {
        let mut digit_layout= Digit::new();
        input.iter().for_each(|input_digit| {
            digit_layout.restrict_belief_sets_on_given(input_digit);
        });
        output.iter().for_each(|output_digit| {
            digit_layout.restrict_belief_sets_on_given(output_digit);
        });
        if !digit_layout.segments.iter().all(|s| s.len() == 1){
            digit_layout.test_remaining_hypothesis(input, output);
        }

        let line_output = digit_layout.to_number(output);
        println!("LINE OUTPUT: {}", line_output);
        
        sum_of_all_outputs += line_output;
    });


    println!("sum_of_all_outputs: {}", sum_of_all_outputs);

    store_output(sum_of_all_outputs.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}