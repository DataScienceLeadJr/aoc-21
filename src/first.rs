use crate::util::TaskPart;

pub const DAY: &str = "1st";

fn process_input(input: String) -> Vec<i32>{
    input.lines().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_A!");

    let depths = process_input(load_input(DAY, TaskPart::A));

    let mut num_increases = 0;

    for depth in 1..depths.len() {
        if depths[depth] > depths[depth-1] {
            num_increases += 1;
        }
    }

    println!("num increases: {}", num_increases);

    store_output(num_increases.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("First_B!");

    let depths = process_input(load_input(DAY, TaskPart::B));

    let sliding_window = |array: &Vec<i32>, index: usize| {
        if index + 1 >= array.len() {
            return Err(format!("index: {}, not enough entries for a complete new sliding window!", index));
        }

        Ok(vec![array[index -1], array[index], array[index + 1]])
    };

    let mut num_increases = 0;

    for depth in 1..depths.len() - 1 {
        let sum_a = match sliding_window(&depths, depth) {
            Ok(window) => window.iter().sum::<i32>(),
            Err(msg) => {
                println!("{:?}", msg);
                println!("at end of array");
                break;
            }
        };
        let sum_b = match sliding_window(&depths, depth + 1) {
            Ok(window) => window.iter().sum::<i32>(),
            Err(msg) => {
                println!("{:?}", msg);
                println!("at end of array");
                break;
            }
        };

        if sum_b > sum_a {
            num_increases += 1;
        }
    }

    println!("num increases: {}", num_increases);

    store_output(num_increases.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}