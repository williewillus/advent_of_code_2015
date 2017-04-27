use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn run() {
    let mut input = String::new();
    let mut rdr = BufReader::new(File::open("d1_input.txt").expect("Couldn't open input file!"));
    rdr.read_to_string(&mut input).expect("failure reading input");

    let mut floor: i32 = 0;

    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                if floor == 0 {
                    println!("Entering basement at {}", idx + 1);
                }
                floor -= 1
            },
            _ => panic!("unknown character")
        }
    }

    println!("Final floor: {}", floor);
}