use crate::util::TaskPart;

pub const DAY: &str = "16th";

fn process_input(input: String) -> String{
    let mut resulting_string = String::new();
    input.lines().next().unwrap().chars()
        .for_each(|hex_num| match hex_num {
            '0' => resulting_string.push_str("0000"),
            '1' => resulting_string.push_str("0001"),
            '2' => resulting_string.push_str("0010"),
            '3' => resulting_string.push_str("0011"),
            '4' => resulting_string.push_str("0100"),
            '5' => resulting_string.push_str("0101"),
            '6' => resulting_string.push_str("0110"),
            '7' => resulting_string.push_str("0111"),
            '8' => resulting_string.push_str("1000"),
            '9' => resulting_string.push_str("1001"),
            'A' => resulting_string.push_str("1010"),
            'B' => resulting_string.push_str("1011"),
            'C' => resulting_string.push_str("1100"),
            'D' => resulting_string.push_str("1101"),
            'E' => resulting_string.push_str("1110"),
            'F' => resulting_string.push_str("1111"),
            _ => panic!("WHAT, THAT'S NOT HEXXXX!!!!")
        });
    
    resulting_string
}

enum ReadState {
    Version,
    TypeID,
    LiteralValue,
    Digit,
    DigitsDone,
    Operator,
    LengthTypeID,
    SubPacket,
    SubPacketLength,
    SubPacketCount
}

trait StateProcess {
    fn ask_for_bits(&self) -> u32;
    fn transition(&self, packet: &str) -> ReadState;
}

impl StateProcess for ReadState {
    fn ask_for_bits(&self) -> u32 {
        match self {
            ReadState::Version => 3,
            ReadState::TypeID => 3,
            ReadState::LiteralValue => 0,
            ReadState::Digit => 5,
            ReadState::DigitsDone => 0,
            ReadState::Operator => 0,
            ReadState::LengthTypeID => 1,
            ReadState::SubPacket => 0,
            ReadState::SubPacketLength => 15,
            ReadState::SubPacketCount => 11,
        }
    }

    fn transition(&self, packet: &str) -> ReadState {
        match self {
            ReadState::Version => {
                // store version number in packet
                ReadState::TypeID
            }
            ReadState::TypeID => {
                // store typeID
                let typeID = packet.parse(base2).unwrap();
                match typeID {
                    4 => ReadState::LiteralValue,
                    _ => ReadState::Operator,
                }
            }
            ReadState::LiteralValue => {
                ReadState::Digit
            },
            ReadState::Digit => {
                let digit = packet[1..].parse(base2);
                // store typeID
                match packet[0] {
                    '1' => ReadState::Digit,
                    '0' => ReadState::DigitsDone,
                    _ => panic!("THIS CAN'T BE!!"),
                }
            },
            ReadState::DigitsDone => {
                ReadState::Version
            }
            ReadState::Operator => {
                ReadState::LengthTypeID
            },
            ReadState::LengthTypeID => {
                match packet {
                    "0" => ReadState::SubPacketLength,
                    "1" => ReadState::SubPacketCount,
                }
            },
            ReadState::SubPacket => {
                ReadState::Version
            },
            ReadState::SubPacketLength => {
                ReadState::Version
            },
            ReadState::SubPacketCount => {
                ReadState::Version
            },
        }
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixteenth_A!");

    let binaries = process_input(load_input(DAY, TaskPart::A));

    // TODO: state machine?

    let state = ReadState::Version;

    println!("{}", binaries);

    println!("num increases: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Sixteenth_B!");

    let binaries = process_input(load_input(DAY, TaskPart::B));

    println!("num increases: {}", 1);

    store_output("1".to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}