use std::{fs::File, io::Write, path::{Path}};

pub enum TaskPart {
    A,
    B,
}

pub fn input_file(day: &str, which_one: TaskPart) -> String{
    let path = Path::new("D:\\Projects\\Rust\\aoc-21\\input");
    let file_path = match which_one {
        TaskPart::A => path.join(format!("{}_a.txt", day)), // TODO: check the AoC file names for these
        TaskPart::B => path.join(format!("{}_b.txt", day)),
    };

    std::fs::read_to_string(file_path).unwrap()
}

pub fn store_output(task_result: String, day: &str, which_one: TaskPart) -> Result<(), std::io::Error>{
    let out_path = Path::new("D:\\Projects\\Rust\\aoc-21\\output");

    let out_file_path = out_path.join(format!("{}_{}.txt", day, match which_one {
        TaskPart::A => "a",
        TaskPart::B => "b"
    }));

    let mut file = File::create(out_file_path)?;

    file.write_all(task_result.as_bytes())
}