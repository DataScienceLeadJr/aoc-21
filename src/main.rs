mod util;

mod first;
mod second;
mod third;
mod fourth;
mod fifth;
mod sixth;
mod seventh; 
mod eighth;
mod ninth;
mod tenth;
mod eleventh;
mod twelfth;
mod thirteenth;

use util::{
    input_file,
    store_output,
};

fn main() {
    match std::env::args().collect::<Vec<String>>()[1].as_str() {
        "1st" => (first::a(&input_file, &store_output), first::b(&input_file, &store_output)),
        "2nd" => (second::a(&input_file, &store_output), second::b(&input_file, &store_output)),
        "3rd" => (third::a(&input_file, &store_output), third::b(&input_file, &store_output)),
        "4th" => (fourth::a(&input_file, &store_output), fourth::b(&input_file, &store_output)),
        "5th" => (fifth::a(&input_file, &store_output), fifth::b(&input_file, &store_output)),
        "6th" => (sixth::a(&input_file, &store_output), sixth::b(&input_file, &store_output)),
        "7th" => (seventh::a(&input_file, &store_output), seventh::b(&input_file, &store_output)),
        "8th" => (eighth::a(&input_file, &store_output), eighth::b(&input_file, &store_output)),
        "9th" => (ninth::a(&input_file, &store_output), ninth::b(&input_file, &store_output)),
        "10th" => (tenth::a(&input_file, &store_output), tenth::b(&input_file, &store_output)),
        "11th" => (eleventh::a(&input_file, &store_output), eleventh::b(&input_file, &store_output)),
        "12th" => (twelfth::a(&input_file, &store_output), twelfth::b(&input_file, &store_output)),
        "13th" => (thirteenth::a(&input_file, &store_output), thirteenth::b(&input_file, &store_output)),
        _ => panic!("WAIHBEPWYUBEPUB!!! WHAT KINDA DAY IS THAT IN THE MONTH OF DECEMBER!?")
    };
}
