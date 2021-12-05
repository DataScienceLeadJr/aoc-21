use crate::util::TaskPart;

pub const DAY: &str = "2nd";

enum Direction{
    Forward,
    Up,
    Down,
}

fn process_input(input: String) -> Vec<(Direction, i32)> {
    input
    .lines()
    .map(|command| {
        let parts = command.split(" ").collect::<Vec<&str>>();
        let direction = match parts[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("WTF DIRECTION IS THAT!?"),
        };
        let amount = parts[1].parse::<i32>().unwrap();
        (direction, amount)
    })
    .collect::<Vec<(Direction, i32)>>()
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Second_A!");

    let commands = process_input(load_input(DAY, TaskPart::A));
    let mut hor_pos = 0;
    let mut depth = 0;
    let _result: () = commands.iter().map(|(dir, amount)| {
        match dir {
            Direction::Forward => {hor_pos += amount;}
            Direction::Down => {depth += amount;}
            Direction::Up => {depth -= amount;}
        }
    }).collect();

    println!("hp: {}, d: {}", hor_pos, depth);

    store_output((hor_pos * depth).to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Second_B!");

    let commands = process_input(load_input(DAY, TaskPart::B));

    let mut hor_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let _result: () = commands.iter().map(|(dir, amount)| {
        match dir {
            Direction::Forward => {
                hor_pos += amount;
                depth += aim * amount;
            }
            Direction::Down => {
                aim += *amount;
            }
            Direction::Up => {
                aim -= amount;
            }
        }
    }).collect();

    println!("hp: {}, d: {}", hor_pos, depth);

    store_output((hor_pos * depth).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}