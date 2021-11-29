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

fn main() {
    let (a, b) = match std::env::args().collect::<Vec<String>>()[1].as_str() {
        "1st" => (first::a, first::b),
        "2nd" => panic!("It's coming! IT'S COMING!!"),
        _ => panic!("WAIHBEPWYUBEPUB!!! WHAT KINDA DAY IS THAT IN THE MONTH OF DECEMBER!?")
    };

    a();
    b();
}
