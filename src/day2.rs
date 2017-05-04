use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART_2: bool = true;

fn parse_dims(s: String) -> (u32, u32, u32) {
    let dims: Vec<_> = s.split('x').collect();
    let l = dims[0].parse().expect("malformed number");
    let w = dims[1].parse().expect("malformed number");
    let h = dims[2].parse().expect("malformed number");
    (l, w, h)
}

fn calculate_ribbon(s: String) -> u32 {
    let (l, w, h) = parse_dims(s);

    let lw = 2*l + 2*w;
    let lh = 2*l + 2*h;
    let wh = 2*w + 2*h;

    let wrap = min(lw, min(lh, wh));
    let bow = l * w * h;
    wrap + bow
}

fn calculate_wrapping_paper(s: String) -> u32 {
    let (l, w, h) = parse_dims(s);

    let lw = l * w;
    let lh = l * h;
    let wh = w * h;

    let surface_area = 2*lw + 2*lh + 2*wh;
    let extra = min(lw, min(lh, wh));

    surface_area + extra
}

pub fn run() {
    let rdr = BufReader::new(File::open("d2_input.txt").expect("Couldn't read input file"));
    let needed = rdr.lines()
                    .map(|r| r.expect("Failure reading line"))
                    .map(if PART_2 { calculate_ribbon } else { calculate_wrapping_paper })
                    .sum::<u32>();

    println!("Needed {}: {} ft^2", if PART_2 { "ribbon" } else { "wrapping paper" }, needed);
}
