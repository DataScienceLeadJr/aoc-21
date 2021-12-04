use std::{collections::{HashMap}, panic, ops::{Range, RangeTo, RangeFrom}};

use itertools::Itertools;

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

    let (draw_numbers,mut boards) = process_input_day_1(load_input(DAY, TaskPart::A));

    let mut winner_board: usize = 1;
    let mut win_draw_num = -1;

    'outer: for n in draw_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y].0 == n {
                        board[x][y].1 = true;
                    }
                }
            }
        }

        for (i, board) in boards.iter_mut().enumerate() {
            let mut win_col = [0; 5];
            let mut win_row = [0; 5]; 
            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y].1 {
                        win_row[x] += 1;
                    }
                    if board[y][x].1 {
                        win_col[x] += 1;
                    }
                }
            }

            if win_col.iter().any(|m| *m == 5) || win_row.iter().any(|m| *m == 5) {
                println!("N = {}", n);
                winner_board = i;
                win_draw_num = n;
                break 'outer;
            }
        }
    }

    println!("winner board is: {}", winner_board);
    println!("winning draw number is: {}", win_draw_num);
    let win_board = boards[winner_board];
    let mut answer_sum = 0;
    for x in 0..5 {
        for y in 0..5 {
            if win_board[x][y].1 {
                continue;
            }
            answer_sum += win_board[x][y].0;
        }
    }
    println!("the answer is: sum:{}, times_draw_num:{}", answer_sum, answer_sum * &win_draw_num);

    store_output((answer_sum * &win_draw_num).to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourth_B!");

    let (draw_numbers,mut boards) = process_input_day_1(load_input(DAY, TaskPart::A));

    let mut loser_board = None;
    let mut win_draw_num = -1;

    'outer: for n in draw_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y].0 == n {
                        board[x][y].1 = true;
                    }
                }
            }
        }

        let mut winners = Vec::new();

        for (i, board) in boards.iter_mut().enumerate() {
            let mut win_col = [0; 5];
            let mut win_row = [0; 5]; 
            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y].1 {
                        win_row[x] += 1;
                    }
                    if board[y][x].1 {
                        win_col[x] += 1;
                    }
                }
            }

            if win_col.iter().any(|m| *m == 5) || win_row.iter().any(|m| *m == 5) {
                winners.push(i);
            }
        }

        if boards.len() == 1  && winners.len() == 1{
            println!("We have a loser!");
            win_draw_num = n;
            loser_board = boards.first();
            break 'outer;
        }

        winners.iter().sorted().rev().for_each(|i| {boards.remove(*i);});

    }

    println!("loser board is: {:?}", loser_board);
    println!("loser's winning draw number is: {}", win_draw_num);
    let board = loser_board.unwrap();
    let mut answer_sum = 0;
    for x in 0..5 {
        for y in 0..5 {
            if board[x][y].1 {
                continue;
            }
            answer_sum += board[x][y].0;
        }
    }
    println!("the answer is: sum:{}, times_draw_num:{}", answer_sum, answer_sum * &win_draw_num);

    store_output((answer_sum * &win_draw_num).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}