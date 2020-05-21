use crate::util;
use itertools::Itertools;

#[derive(Default)]
struct Sue {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn read_input() -> Vec<Sue> {
    let mut ret = Vec::new();
    for line in util::lines("d16_input.txt").unwrap() {
        let colon = line.find(':').unwrap();
        let mut sue: Sue = Default::default();
        let rest = &line[colon + 1..];
        for kv in rest.split(',') {
            let (k, v) = kv.split(':').next_tuple().unwrap();
            let value = Some(v.trim().parse().unwrap());
            match k.trim() {
                "children" => sue.children = value,
                "cats" => sue.cats = value,
                "samoyeds" => sue.samoyeds = value,
                "pomeranians" => sue.pomeranians = value,
                "akitas" => sue.akitas = value,
                "vizslas" => sue.vizslas = value,
                "goldfish" => sue.goldfish = value,
                "trees" => sue.trees = value,
                "cars" => sue.cars = value,
                "perfumes" => sue.perfumes = value,
                _ => panic!("unknown key {}", k)
            }
        }

        ret.push(sue);
    }
    ret
}

fn matching_keys(sue: &Sue, part2: bool) -> usize {
    let mut ret = 0;
    if let Some(3) = sue.children {
        ret += 1;
    }
    if part2 {
        if let Some(x) = sue.cats {
            if x > 7 {
                ret += 1;
            }
        }
    } else {
        if let Some(7) = sue.cats {
            ret += 1;
        }
    }
    if let Some(2) = sue.samoyeds {
        ret += 1;
    }
    if part2 {
        if let Some(x) = sue.pomeranians {
            if x < 3 {
                ret += 1;
            }
        }
    } else {
        if let Some(3) = sue.pomeranians {
            ret += 1;
        }
    }
    if let Some(0) = sue.akitas {
        ret += 1;
    }
    if let Some(0) = sue.vizslas {
        ret += 1;
    }
    if part2 {
        if let Some(x) = sue.goldfish {
            if x < 5 {
                ret += 1;
            }
        }
    } else {
        if let Some(5) = sue.goldfish {
            ret += 1;
        }
    }
    if part2 {
        if let Some(x) = sue.trees {
            if x > 3 {
                ret += 1;
            }
        }
    } else {
        if let Some(3) = sue.trees {
            ret += 1;
        }
    }
    if let Some(2) = sue.cars {
        ret += 1;
    }
    if let Some(1) = sue.perfumes {
        ret += 1;
    }

    ret
}

pub fn run() {
    let sues = read_input();
    let p1 = sues.iter()
        .enumerate()
        .max_by_key(|&(_, s)| matching_keys(s, false)).unwrap();
    println!("Part 1: {}", p1.0 + 1);

    let p2 = sues.iter()
        .enumerate()
        .max_by_key(|&(_, s)| matching_keys(s, true)).unwrap();
    println!("Part 2: {}", p2.0 + 1);
}
