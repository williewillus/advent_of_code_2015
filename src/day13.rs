use crate::util;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn read_input() -> (HashSet<String>, HashMap<(String, String), i32>) {
    let lines = util::lines("d13_input.txt").unwrap();
    let re = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();

    let mut names = HashSet::new();
    let mut mapping = HashMap::new();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let a = &caps[1];
        let b = &caps[4];
        if !names.contains(a) {
            names.insert(a.to_owned());
        }
        if !names.contains(b) {
            names.insert(b.to_owned());
        }
        let value = {
            let num = caps[3].parse::<i32>().unwrap();
            match &caps[2] {
                "gain" => num,
                "lose" => -num,
                _ => panic!("unknown sign word")
            }
        };
        mapping.insert((a.to_owned(), b.to_owned()), value);
    }
    (names, mapping)
}

fn cost(perm: &[&String], mapping: &HashMap<(String, String), i32>) -> i32 {
    let mut sum = 0;
    for wind in perm.windows(2) {
        let a = wind[0];
        let b = wind[1];
        let fwd = mapping[&(a.to_owned(), b.to_owned())];
        let bwd = mapping[&(b.to_owned(), a.to_owned())];
        sum += fwd + bwd;
    }
    // connect the ends
    let first = perm[0].to_owned();
    let last = perm[perm.len() - 1].to_owned();
    let bwd = mapping[&(first.clone(), last.clone())];
    let fwd = mapping[&(last, first)];
    sum += bwd + fwd;
    sum
}

pub fn run() {
    let (mut names, mut mapping) = read_input();
    let p1 = names.iter().permutations(names.len())
        .map(|perm| cost(&perm, &mapping))
        .max().unwrap();
    println!("Part 1: {}", p1);

    for name in &names {
        mapping.insert((name.to_owned(), "me".to_owned()), 0);
        mapping.insert(("me".to_owned(), name.to_owned()), 0);
    }
    names.insert("me".to_owned());
    let p2 = names.iter().permutations(names.len())
        .map(|perm| cost(&perm, &mapping))
        .max().unwrap();
    println!("Part 2: {}", p2);
}
