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
mod fourteenth;
mod fifteenth;
mod sixteenth;
mod seventeeth;
mod eighteenth;
mod nineteenth;
mod twentieth;
mod twentyfirst;
mod twentysecond;
mod twentythird;
mod twentyfourth;
mod twentyfifth;

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
        "14th" => (fourteenth::a(&input_file, &store_output), fourteenth::b(&input_file, &store_output)),
        "15th" => (fifteenth::a(&input_file, &store_output), fifteenth::b(&input_file, &store_output)),
        "16th" => (sixteenth::a(&input_file, &store_output), sixteenth::b(&input_file, &store_output)),
        "17th" => (seventeenth::a(&input_file, &store_output), seventeenth::b(&input_file, &store_output)),
        "18th" => (eighteenth::a(&input_file, &store_output), eighteenth::b(&input_file, &store_output)),
        "19th" => (nineteenth::a(&input_file, &store_output), nineteenth::b(&input_file, &store_output)),
        "20th" => (twentieth::a(&input_file, &store_output), twentieth::b(&input_file, &store_output)),
        "21th" => (twentyfirst::a(&input_file, &store_output), twentyfirst::b(&input_file, &store_output)),
        "22th" => (twentysecond::a(&input_file, &store_output), twentysecond::b(&input_file, &store_output)),
        "23th" => (twentythird::a(&input_file, &store_output), twentythird::b(&input_file, &store_output)),
        "24th" => (twentyfourth::a(&input_file, &store_output), twentyfourth::b(&input_file, &store_output)),
        "25th" => (twentyfifth::a(&input_file, &store_output), twentyfifth::b(&input_file, &store_output)),
        _ => panic!("WAIHBEPWYUBEPUB!!! WHAT KINDA DAY IS THAT IN THE MONTH OF DECEMBER!?")
    };
}
