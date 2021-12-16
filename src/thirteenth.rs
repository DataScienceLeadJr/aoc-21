use crate::util::TaskPart;

use std::collections::HashMap;

use itertools::Itertools;

pub const DAY: &str = "13th";

#[derive(Debug, Copy, Clone)]
enum FoldAxis {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct InstructionPaper {
    holes: HashMap<(u32, u32), ()>,
    fold_instructions: Vec<(FoldAxis, u32)>,
    height: u32,
    width: u32,
}

impl InstructionPaper {
    fn new() -> Self {
        InstructionPaper {
            holes: HashMap::new(),
            fold_instructions: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn pretty_print(&self) {
        for y in 0..self.height {
            println!("");
            for x in 0..self.width {
                print!("{}", if self.holes.contains_key(&(x,y)) {'#'} else {'.'});
            }
        }
        println!("");
    }

    fn fold(&mut self) {
        let (axis, fold_line) = self.fold_instructions.clone().remove(0);

        println!("folding instruction paper along axis {:?}, at:{}", axis, fold_line);

        let new_height = match axis {
            FoldAxis::Vertical => self.height,
            FoldAxis::Horizontal => fold_line,
        };
        let new_width = match axis {
            FoldAxis::Vertical => fold_line,
            FoldAxis::Horizontal => self.width,
        };

        let mut still_holes = HashMap::new();

        for y in 0..new_height {
            for x in 0..new_width {
                let mirrored_coord = match axis {
                    FoldAxis::Vertical => (self.width - 1 - x, y),
                    FoldAxis::Horizontal => (x, self.height - 1 - y)
                };
                if self.holes.contains_key(&(x,y)) || self.holes.contains_key(&mirrored_coord) {
                    still_holes.insert((x,y), ());
                }
            }
        }

            self.holes = still_holes;
            self.fold_instructions =  self.fold_instructions[1..].to_vec();
            self.height = new_height;
            self.width =  new_width;
    }
}

fn process_input(input: String) -> InstructionPaper{
    let mut paper = InstructionPaper::new();

    let mut max_x = 0;
    let mut max_y = 0;

    input.lines().for_each(|line| {
        if line.is_empty() {

        } else if line.contains("fold") {
            paper.fold_instructions.push(
                (
                    if line.contains("x") { FoldAxis::Vertical}
                    else { FoldAxis::Horizontal },
                    line.split("=").last().unwrap().parse::<u32>().unwrap()
                )
            )
        } else {
            let xy: (u32, u32) = line.split(",").map(|num| num.parse::<u32>().unwrap()).collect_tuple().unwrap();
            if xy.0 > max_x {
                max_x = xy.0;
            }
            if xy.1 > max_y {
                max_y = xy.1;
            }
            paper.holes.insert(xy, ());
        }
    });

    paper.width = max_x + 1;
    paper.height = max_y + 1;

    paper
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Thirteenth_A!");

    let mut instruction_paper = process_input(load_input(DAY, TaskPart::A));
    // instruction_paper.pretty_print();
    let mut visible_holes = instruction_paper.holes.keys().len();
    for _ in 0..1 { //instruction_paper.fold_instructions.len() {
        instruction_paper.fold();
        // instruction_paper.pretty_print();
        visible_holes = instruction_paper.holes.keys().len();
        println!("visible holes: {}", visible_holes);
    }

    store_output(visible_holes.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Thirteenth_B!");

    let mut instruction_paper = process_input(load_input(DAY, TaskPart::B));
    // instruction_paper.pretty_print();
    let mut visible_holes = instruction_paper.holes.keys().len();
    for _ in 0..instruction_paper.fold_instructions.len() {
        instruction_paper.fold();
        // instruction_paper.pretty_print();
        visible_holes = instruction_paper.holes.keys().len();
        println!("visible holes: {}", visible_holes);
    }

    instruction_paper.pretty_print();

    store_output(visible_holes.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}