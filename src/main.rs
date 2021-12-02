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

use util::{
    input_file,
    store_output,
};

fn main() {
    // TODO: figure out how to do this "morpho module" thing...
    let (a, b) = match std::env::args().collect::<Vec<String>>()[1].as_str() {
        "1st" => (first::a(&input_file, &store_output), first::b(&input_file, &store_output)),
        "2nd" => (second::a(&input_file, &store_output), second::b(&input_file, &store_output)),
        _ => panic!("WAIHBEPWYUBEPUB!!! WHAT KINDA DAY IS THAT IN THE MONTH OF DECEMBER!?")
    };
}
