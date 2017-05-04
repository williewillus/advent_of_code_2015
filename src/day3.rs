use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::collections::HashSet;

fn move_pos(pos: (i32, i32), c: char) -> (i32, i32) {
    match c {
        '^' => (pos.0, pos.1 + 1),
        'v' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        _ => panic!("Invalid direction! {}", c)
    }
}

pub fn run() {
    let mut input = String::new();
    let mut rdr = BufReader::new(File::open("d3_input.txt").expect("Couldn't open input file!"));
    rdr.read_to_string(&mut input).expect("failure reading input");

    let mut cur_pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(cur_pos);

    let mut coop_pos = (0, 0);
    let mut coop_visited = HashSet::new();
    coop_visited.insert(coop_pos);

    let mut robo_pos = (0, 0);
    let mut robo_visited = HashSet::new();
    robo_visited.insert(robo_pos);

    let mut robos_turn = false;

    for c in input.chars() {
        if robos_turn {
            robo_pos = move_pos(robo_pos, c);
            robo_visited.insert(robo_pos);
        } else {
            coop_pos = move_pos(coop_pos, c);
            coop_visited.insert(coop_pos);
        }

        robos_turn = !robos_turn;


        cur_pos = move_pos(cur_pos, c);
        visited.insert(cur_pos);
    }

    println!("{}", visited.len());
    println!("{}", coop_visited.union(&robo_visited).count());
}
