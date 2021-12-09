use std::slice::Iter;

use itertools::{Itertools, Interleave};

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
    beliefs: [String; 7],
}

impl Digit {
    fn new() -> Self {
        let all_posibilities = ALL_POSSIBILITIES;
        let mut array: [String; 7] = Default::default();

        array.iter_mut().for_each(|entry| {
            *entry = all_posibilities.to_string();
        });
        Digit {
            beliefs: array,
        }
    }

    fn update_number_hypothesis(&mut self, reason: &str, num_pattern: [bool; 7]) {
        num_pattern.iter().enumerate().for_each(|(i, seg)| {
            if *seg {
                self.beliefs[i] = reason.union(&self.beliefs[i]);
            } else {
                self.beliefs[i] = self.beliefs[i][..].remove_union(reason);
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

    fn finalize_hypotheses_constraining(&mut self, words: Interleave<Iter<String>, Iter<String>>) {
        words.for_each(|code| {
            match code.len() {
                6 => {
                    let missing_seg = ALL_POSSIBILITIES.remove_union(code);
                    for i in 0..7 {
                        if self.beliefs[i][..].contains(&missing_seg) {
                            let mut possible_number = [true; 7];
                            possible_number[i] = false;
                            if NUMBERS_WITH_SIX.iter().any(|valid| *valid == possible_number) {
                                self.beliefs[i] = missing_seg.clone();
                                for j in (0..7).filter(|j| i != *j) {
                                    if self.beliefs[j][..].contains(&missing_seg) {
                                        self.beliefs[j] = self.beliefs[j][..].remove_union(&missing_seg);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn to_digit(&self, code: &str) -> i32 {
        let digit = (0..7).map(|i| code.contains(&self.beliefs[i][..])).collect::<Vec<bool>>();
        
        LEGAL_NUMBERS.iter().position(|num| *num.to_vec() == digit).unwrap() as i32
    }

    fn to_four_digit_number(&self, codes: &Vec<String>) -> i32 {
        self.to_digit(&codes[0]) * 1000 + self.to_digit(&codes[1]) * 100 + self.to_digit(&codes[2]) * 10 + self.to_digit(&codes[3])
    }
}


pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eigth_B!");

    let patterns = process_input(load_input(DAY, TaskPart::B));

    let mut sum_of_all_outputs = 0;

    patterns.iter().for_each(|(input, output)| {
        let mut segment_hypotheses= Digit::new();
        let word_iter = input.iter().interleave(output.iter());

        word_iter.clone().for_each(|word| {
            segment_hypotheses.restrict_belief_sets_on_given(word);
        });

        segment_hypotheses.finalize_hypotheses_constraining(word_iter);

        assert!(
            segment_hypotheses.beliefs.iter().all(|hypothesis| hypothesis.len() == 1),
            "Didn't manage to constrain all hypothesis down to one thruth!"
        );
        
        let line_output = segment_hypotheses.to_four_digit_number(output);
        println!("LINE OUTPUT: {}", line_output);
        
        sum_of_all_outputs += line_output;
    });

    println!("sum_of_all_outputs: {}", sum_of_all_outputs);

    store_output(sum_of_all_outputs.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}