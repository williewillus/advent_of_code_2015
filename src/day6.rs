use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    // no 2d :(
    let mut part1_grid = vec![false; 1000 * 1000].into_boxed_slice();
    let mut part2_grid = vec![0 as u32; 1000 * 1000].into_boxed_slice();

    let rdr = BufReader::new(File::open("d6_input.txt").expect("Couldn't open input file!"));

    let regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();

    for l in rdr.lines() {
        let s = l.expect("invalid");

        for m in regex.captures_iter(&s) {
            let xmin = m[1].parse::<usize>().expect("bad xmin");
            let ymin = m[2].parse::<usize>().expect("bad ymin");
            let xmax = m[3].parse::<usize>().expect("bad xmax");
            let ymax = m[4].parse::<usize>().expect("bad ymax");

            for x in xmin..xmax+1 {
                for y in ymin..ymax+1 {
                    // compiler pls lift the conditional up
                    if s.starts_with("turn on") {
                        part1_grid[x*1000 + y] = true;

                        part2_grid[x*1000 + y] += 1;
                    } else if s.starts_with("turn off") {
                        part1_grid[x*1000 + y] = false;

                        if part2_grid[x*1000 + y] > 0 {
                            part2_grid[x*1000 + y] -= 1;
                        }
                    } else if s.starts_with("toggle") {
                        part1_grid[x*1000 + y] = !part1_grid[x*1000 + y];

                        part2_grid[x*1000 + y] += 2;
                    }
                }
            }
        }
    }

    println!("{} lights on", part1_grid.iter().filter(|&i| *i).count());
    println!("{} total brightness", part2_grid.iter().sum::<u32>());
}
