use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
struct Attributes {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn read_input() -> Vec<Attributes> {
    let pat = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let mut ret = Vec::new();
    for line in crate::util::lines("d15_input.txt").unwrap() {
        let captures = pat.captures(&line).unwrap();
        let name = &captures[1];
        let capacity = captures[2].parse().unwrap();
        let durability = captures[3].parse().unwrap();
        let flavor = captures[4].parse().unwrap();
        let texture = captures[5].parse().unwrap();
        let calories = captures[6].parse().unwrap();
        let attr = Attributes {
            name: name.to_owned(),
            capacity,
            durability,
            flavor,
            texture,
            calories
        };
        ret.push(attr);
    }

    ret
}

pub fn run() {
    let info = read_input();

    
    let mut max_score_p1 = 0;
    let mut max_score_p2 = 0;
    for comb in (0..info.len()).combinations_with_replacement(100) {
        let mut cap = 0;
        let mut dur = 0;
        let mut flv = 0;
        let mut tex = 0;
        let mut cal = 0;
        for i in comb {
            cap += info[i].capacity;
            dur += info[i].durability;
            flv += info[i].flavor;
            tex += info[i].texture;
            cal += info[i].calories;
        }
        let score = cap.max(0) * dur.max(0) * flv.max(0) * tex.max(0);
        max_score_p1 = max_score_p1.max(score);

        if cal == 500 {
            max_score_p2 = max_score_p2.max(score);
        }
    }

    println!("Part 1: {}", max_score_p1);
    println!("Part 2: {}", max_score_p2);
}
