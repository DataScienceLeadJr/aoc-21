use std::{fs::File, io::Write, path::{Path}, ops::Sub};

pub enum TaskPart {
    A,
    B,
}

pub fn input_file(day: &str, which_one: TaskPart) -> String{
    let path = Path::new("..\\aoc-21\\input");
    let file_path = match which_one {
        TaskPart::A => path.join(format!("{}_a.txt", day)), // TODO: check the AoC file names for these
        TaskPart::B => path.join(format!("{}_b.txt", day)),
    };

    std::fs::read_to_string(file_path).unwrap()
}

pub fn store_output(task_result: String, day: &str, which_one: TaskPart) -> Result<(), std::io::Error>{
    let out_path = Path::new("..\\aoc-21\\output");

    let out_file_path = out_path.join(format!("{}_{}.txt", day, match which_one {
        TaskPart::A => "a",
        TaskPart::B => "b"
    }));

    let mut file = File::create(out_file_path)?;

    file.write_all(task_result.as_bytes())
}

// Statistics
use std::collections::HashMap;

pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn median(sorted_numbers: &[i32]) -> i32 {
    let count = sorted_numbers.len();
    let mid = count / 2;
    if count % 2 == 0 {
        ((sorted_numbers[mid - 1] + sorted_numbers[mid]) as f32 / 2.0) as i32
    } else {
        sorted_numbers[mid]
    }
}

pub fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();

    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

