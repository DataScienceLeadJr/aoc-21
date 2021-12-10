use std::ops::ControlFlow;

use itertools::Itertools;

use crate::util::{
    TaskPart,
};

pub const DAY: &str = "10th";


trait Syntax {
    fn as_illegal_syntax_value(self) -> Result<i32, String>;
    fn as_opposite(&self) -> Self;
    fn as_completion_value(self) -> i32;
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

    fn as_opposite(&self) -> Self {
        match self {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("can't opposite that one!")
        }
    }

    fn as_completion_value(self) -> i32 {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("That one shouldn't be here!!!")
        }
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_A!");

    let mut invalids = String::new();

    load_input(DAY, TaskPart::A).lines().for_each(|line| {
        let mut looking_for = String::from(line.chars().next().unwrap().as_opposite());
        line.chars().try_for_each(|c| {
            if looking_for.chars().last().unwrap() == c {
                looking_for.pop();
            } else if c.as_illegal_syntax_value().is_err(){
                looking_for.push(c.as_opposite());
            } else {
                invalids.push(c);
                return None;
            }
            // println!("{:?}", looking_for);
            Some(())
        });
    });

    println!("all the found invalids: {:?}", invalids);
    let invalid_sum = invalids.chars().map(|c| c.as_illegal_syntax_value().unwrap()).sum::<i32>();
    println!("total invalid score: {}", invalid_sum);

    store_output(invalid_sum.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Tenth_B!");

    let mut auto_completes: Vec<String> = Vec::new();

    load_input(DAY, TaskPart::B).lines().for_each(|line| {
        let mut looking_for = String::from(line.chars().next().unwrap().as_opposite());
        if line.chars().try_for_each(|c| {
            if looking_for.chars().last().unwrap() == c {
                looking_for.pop();
            } else if c.as_illegal_syntax_value().is_err(){
                looking_for.push(c.as_opposite());
            } else {
                return None;
            }
            // println!("{:?}", looking_for);
            Some(())
        }).is_some() {
            let mut auto_complete: String = looking_for.chars().rev().collect();
            auto_complete.pop();
            auto_completes.push(auto_complete);
        }
    });

    println!("all the found auto_completes:");
    auto_completes.iter().for_each(|line_completion| println!("\t {}", line_completion));

    let mut competion_scores: Vec<i64> = auto_completes.into_iter().map(|completion| {
        let mut completion_score: i64 = 0;
        completion.chars().for_each(|c| {
            completion_score *= 5;
            completion_score += c.as_completion_value() as i64;
        });
        completion_score
    }).collect();
    competion_scores.sort();

    let actual_score = competion_scores[competion_scores.len() / 2];

    println!("total middle completion score: {}", actual_score);

    store_output(actual_score.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}