use std::{collections::{HashMap}, panic, ops::{Range, RangeTo, RangeFrom}};

use crate::util::TaskPart;

pub const DAY: &str = "4th";

type BingoBoards = Vec<[[(i32, bool); 5]; 5]>;


fn process_input_day_1(input: String) -> (Vec<i32>, BingoBoards) {
    let mut lines: Vec<_> = input.lines().collect();
    let draw_numbers = lines.first().unwrap().split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    lines = lines[1..].to_vec();

    let mut boards: BingoBoards = Vec::new();

    let mut current_board = [[(-1, false); 5]; 5];

    for board_index in (0..lines.len()).step_by(6) {
        for board_line in 1..6 {
            lines[board_index + board_line].split_whitespace().enumerate().for_each(|(i, num)| {current_board[board_line-1][i] = (num.parse().unwrap(), false);});
        }
        boards.push(current_board);
        current_board = [[(-1, false); 5]; 5];
    }

    (draw_numbers, boards)
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_A!");

    let (draw_numbers, boards) = process_input_day_1(load_input(DAY, TaskPart::A));

    // Here we Go!

    store_output((" ").to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_B!");

    let mut diagnostic_report = process_input_day_1(load_input(DAY, TaskPart::B));

    // here we go!

    store_output((" ").to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}