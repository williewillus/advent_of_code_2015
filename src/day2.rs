use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_dims(s: String) -> [u32; 3] {
    let dims: Vec<_> = s.split('x').collect();
    let l = dims[0].parse::<u32>().expect("malformed number");
    let w = dims[1].parse::<u32>().expect("malformed number");
    let h = dims[2].parse::<u32>().expect("malformed number");
    [l, w, h]
}

fn calculate_ribbon(s: String) -> u32 {
    let dims = parse_dims(s);

    let lw = 2*dims[0] + 2*dims[1];
    let lh = 2*dims[0] + 2*dims[2];
    let wh = 2*dims[1] + 2*dims[2];

    let wrap = min(lw, min(lh, wh));
    let bow = dims.iter().product::<u32>();
    wrap + bow
}

fn calculate_wrapping_paper(s: String) -> u32 {
    let dims = parse_dims(s);

    let lw = dims[0] * dims[1];
    let lh = dims[0] * dims[2];
    let wh = dims[1] * dims[2];

    let surface_area = 2*lw + 2*lh + 2*wh;
    let extra = min(lw, min(lh, wh));

    surface_area + extra
}

const PART_2: bool = true;

pub fn run() {
    let rdr = BufReader::new(File::open("d2_input.txt").expect("Couldn't read input file"));
    let needed = rdr.lines()
                    .map(|r| r.expect("Failure reading line"))
                    .map(if PART_2 { calculate_ribbon } else { calculate_wrapping_paper })
                    .sum::<u32>();

    println!("Needed {}: {} ft^2", if PART_2 { "ribbon" } else { "wrapping paper" }, needed);
}