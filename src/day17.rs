use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let rdr = BufReader::new(File::open("d17_input.txt").expect("Couldn't read input file"));
    let sizes = rdr
        .lines()
        .map(|r| r.expect("Failure reading line"))
        .map(|s| s.parse::<u32>().expect("bad container size"))
        .collect::<Vec<_>>();

    // todo DP?
    let mut minimal_used = sizes.len() as u32;
    let mut satisfying_masks = Vec::new();

    for mask in 0..(1 << sizes.len()) {
        let mut total = 0;
        let mut used = 0;

        for (idx, size) in sizes.iter().enumerate() {
            if mask & (1 << idx) != 0 {
                total += *size;
                used += 1;
            }
        }

        if total == 150 {
            satisfying_masks.push(mask);
            if used < minimal_used {
                minimal_used = used;
            }
        }
    }

    println!("part 1: {}", satisfying_masks.len());
    println!(
        "part 2: {}",
        satisfying_masks
            .iter()
            .filter(|m: &&usize| m.count_ones() == minimal_used)
            .count()
    )
}
