use crate::util;

pub fn run() {
    let sizes = util::lines("d17_input.txt").unwrap()
        .iter()
        .map(|s| s.parse::<u32>().expect("bad container size"))
        .collect::<Vec<_>>();

    let mut minimal_used = sizes.len() as u32;
    let mut satisfying_masks = Vec::<u32>::new();

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

    println!("Part 1: {}", satisfying_masks.len());
    println!(
        "Part 2: {}",
        satisfying_masks
            .iter()
            .filter(|m| m.count_ones() == minimal_used)
            .count()
    )
}
