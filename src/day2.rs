use std::cmp::min;
use crate::util;

fn parse_dims(s: &str) -> (u32, u32, u32) {
    let dims: Vec<_> = s.split('x').collect();
    let l = dims[0].parse().expect("malformed number");
    let w = dims[1].parse().expect("malformed number");
    let h = dims[2].parse().expect("malformed number");
    (l, w, h)
}

fn calculate_ribbon(s: &str) -> u32 {
    let (l, w, h) = parse_dims(s);

    let lw = 2 * l + 2 * w;
    let lh = 2 * l + 2 * h;
    let wh = 2 * w + 2 * h;

    let wrap = min(lw, min(lh, wh));
    let bow = l * w * h;
    wrap + bow
}

fn calculate_wrapping_paper(s: &str) -> u32 {
    let (l, w, h) = parse_dims(s);

    let lw = l * w;
    let lh = l * h;
    let wh = w * h;

    let surface_area = 2 * lw + 2 * lh + 2 * wh;
    let extra = min(lw, min(lh, wh));

    surface_area + extra
}

pub fn run() {
    let lines = util::lines("d2_input.txt").unwrap();

    let p1 = lines.iter().map(|s| calculate_wrapping_paper(s)).sum::<u32>();
    println!("Part 1: {}", p1);

    let p2 = lines.iter().map(|s| calculate_ribbon(s)).sum::<u32>();
    println!("Part 2: {}", p2);
}
