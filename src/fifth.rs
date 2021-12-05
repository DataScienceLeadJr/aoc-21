use crate::util::TaskPart;

pub const DAY: &str = "5th";


fn process_input_day_1<'a>(input: &'a String) -> Vec<&'a str> {
    let lines: Vec<&str> = input.lines().collect();

    lines
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifh_A!");

    let _ = process_input_day_1(&load_input(DAY, TaskPart::A));

    // println!("the answer is: sum:{}, times_draw_num:{}", answer_sum, answer_sum * &win_draw_num);

    store_output((" ").to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifh_B!");

    let _ = process_input_day_1(&load_input(DAY, TaskPart::A));


    // println!("the answer is: sum:{}, times_draw_num:{}", answer_sum, answer_sum * &win_draw_num);

    store_output((" ").to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}