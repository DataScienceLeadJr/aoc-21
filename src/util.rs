use std::path::{Path, PathBuf};

pub enum TaskPark {
    A,
    B,
}

pub fn input_file(day: &str, which_one: TaskPark) -> PathBuf{
    let path = Path::new("D:\\Projects\\Rust\\aoc-21\\src\\input");
    match which_one {
        TaskPark::A => path.join(format!("{}_a.text", day)),
        TaskPark::B => path.join(format!("{}_b.text", day)),
    }
}