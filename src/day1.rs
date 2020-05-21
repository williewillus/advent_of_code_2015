use crate::util;

pub fn run() {
    let input = util::read_all("d1_input.txt").expect("failed to read input");

    let mut floor: i32 = 0;

    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                if floor == 0 {
                    println!("Entering basement at {}", idx + 1);
                }
                floor -= 1
            }
            _ => panic!("unknown character"),
        }
    }

    println!("Final floor: {}", floor);
}
